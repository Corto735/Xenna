use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use tracing;

use super::models::{
    CcnExpertise, CcnLevel, ContributorProfile, CreerProfilReq, PaysExpertise, ProfilComplet,
};
use crate::{
    admin::auth::hash_password,
    altcha::verify_solution,
    crypto::{email_hash, encrypt_email, parse_encryption_key},
};

type Db = Arc<SqlitePool>;

// Texte brut uniquement : retire <>, les caractères de contrôle, et trime.
fn sanitiser(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, '<' | '>') && (!c.is_control() || matches!(c, '\n' | '\r' | '\t' | ' ')))
        .collect::<String>()
        .trim()
        .to_string()
}

// ── Erreurs métier ─────────────────────────────────────────────────────────────

enum ForgeError {
    Introuvable(String),
    DejaVote,
    Conflit(String),
    Validation(String),
    Db(sqlx::Error),
}

impl IntoResponse for ForgeError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ForgeError::Introuvable(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            ForgeError::DejaVote => {
                (StatusCode::CONFLICT, "Vous avez déjà voté pour ce sujet").into_response()
            }
            ForgeError::Conflit(msg) => (StatusCode::CONFLICT, msg).into_response(),
            ForgeError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response(),
            ForgeError::Db(e) => {
                tracing::error!("Forge DB error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne du serveur").into_response()
            }
        }
    }
}

impl From<sqlx::Error> for ForgeError {
    fn from(e: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref db_err) = e {
            if db_err.is_unique_violation() {
                return ForgeError::DejaVote;
            }
        }
        ForgeError::Db(e)
    }
}

// ── Helpers DB ─────────────────────────────────────────────────────────────────

async fn profil_par_pseudo(pool: &SqlitePool, pseudo: &str) -> Result<ContributorProfile, ForgeError> {
    sqlx::query_as::<_, ContributorProfile>(
        "SELECT id, user_id, pseudo, linkedin_url, poste, poste_est_actuel,
                paie_fr_niveau,
                posts_count, topics_count, votes_received, votes_given, created_at, status
         FROM contributor_profiles
         WHERE pseudo = ? COLLATE NOCASE",
    )
    .bind(pseudo)
    .fetch_optional(pool)
    .await
    .map_err(ForgeError::Db)?
    .ok_or_else(|| ForgeError::Introuvable(format!("Profil '{pseudo}' introuvable")))
}

async fn charger_expertises(pool: &SqlitePool, profile_id: i64) -> Result<Vec<CcnExpertise>, ForgeError> {
    sqlx::query_as::<_, CcnExpertise>(
        "SELECT id, profile_id, ccn_idcc, ccn_libelle, niveau
         FROM contributor_ccn_expertises
         WHERE profile_id = ?
         ORDER BY CASE niveau
             WHEN 'Maîtrisée' THEN 1
             WHEN 'Pratiquée' THEN 2
             WHEN 'Connue'    THEN 3
             ELSE 4
         END",
    )
    .bind(profile_id)
    .fetch_all(pool)
    .await
    .map_err(ForgeError::Db)
}

async fn charger_pays(pool: &SqlitePool, profile_id: i64) -> Result<Vec<PaysExpertise>, ForgeError> {
    sqlx::query_as::<_, PaysExpertise>(
        "SELECT id, profile_id, pays_code, pays_libelle, niveau
         FROM contributor_pays_expertises
         WHERE profile_id = ?
         ORDER BY CASE niveau
             WHEN 'Maîtrisée' THEN 1
             WHEN 'Pratiquée' THEN 2
             WHEN 'Connue'    THEN 3
             ELSE 4
         END",
    )
    .bind(profile_id)
    .fetch_all(pool)
    .await
    .map_err(ForgeError::Db)
}

// ── GET /profil/:pseudo ────────────────────────────────────────────────────────

async fn get_profil(
    State(pool): State<Db>,
    Path(pseudo): Path<String>,
) -> Result<Json<ProfilComplet>, ForgeError> {
    let profil     = profil_par_pseudo(&pool, &pseudo).await?;
    let expertises = charger_expertises(&pool, profil.id).await?;
    let pays       = charger_pays(&pool, profil.id).await?;
    Ok(Json(ProfilComplet { profil, expertises, pays }))
}

// ── GET /forge/contributeurs ───────────────────────────────────────────────────
// Seuls les profils approuvés par l'admin ET avec email vérifié sont listés.

async fn liste_contributeurs(
    State(pool): State<Db>,
) -> Result<Json<Vec<ProfilComplet>>, ForgeError> {
    let profils = sqlx::query_as::<_, ContributorProfile>(
        "SELECT cp.id, cp.user_id, cp.pseudo, cp.linkedin_url, cp.poste, cp.poste_est_actuel,
                cp.paie_fr_niveau,
                cp.posts_count, cp.topics_count, cp.votes_received, cp.votes_given,
                cp.created_at, cp.status
         FROM contributor_profiles cp
         INNER JOIN users u ON u.id = cp.user_id
         WHERE cp.status = 'approved' AND u.email_verified = 1
         ORDER BY cp.votes_received DESC, cp.pseudo ASC",
    )
    .fetch_all(&*pool)
    .await
    .map_err(ForgeError::Db)?;

    let mut complets = Vec::with_capacity(profils.len());
    for profil in profils {
        let expertises = charger_expertises(&pool, profil.id).await?;
        let pays       = charger_pays(&pool, profil.id).await?;
        complets.push(ProfilComplet { profil, expertises, pays });
    }
    Ok(Json(complets))
}

// ── POST /forge/profil ─────────────────────────────────────────────────────────

async fn creer_profil(
    State(pool): State<Db>,
    Json(req): Json<CreerProfilReq>,
) -> Result<(StatusCode, Json<serde_json::Value>), ForgeError> {
    // 1. Vérification CAPTCHA Altcha
    let altcha_secret = std::env::var("ALTCHA_SECRET").unwrap_or_default();
    if !altcha_secret.is_empty() {
        let payload = req.altcha.as_deref().unwrap_or("");
        if payload.is_empty() || !verify_solution(payload, &altcha_secret) {
            return Err(ForgeError::Validation(
                "Vérification anti-bot échouée. Veuillez réessayer.".into(),
            ));
        }
    }

    // 2. Validation des champs (versions sanitisées : ce sont ELLES qui sont insérées)
    let pseudo = sanitiser(&req.pseudo);
    if pseudo.is_empty() || pseudo.len() > 50 {
        return Err(ForgeError::Validation("Pseudo : 1 à 50 caractères".into()));
    }
    let poste = sanitiser(&req.poste);
    if poste.len() > 100 {
        return Err(ForgeError::Validation("Poste : 100 caractères maximum".into()));
    }
    if !req.email.contains('@') || req.email.len() > 254 {
        return Err(ForgeError::Validation("Adresse email invalide".into()));
    }
    if req.password.len() < 8 {
        return Err(ForgeError::Validation("Mot de passe : 8 caractères minimum".into()));
    }
    if req.password != req.password_confirm {
        return Err(ForgeError::Validation("Les mots de passe ne correspondent pas".into()));
    }
    if req.expertises.len() > 20 {
        return Err(ForgeError::Validation("Maximum 20 CCN par profil".into()));
    }
    if req.pays.len() > 20 {
        return Err(ForgeError::Validation("Maximum 20 pays par profil".into()));
    }
    let linkedin_url = match req.linkedin_url.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(url) => {
            if !(url.starts_with("https://") || url.starts_with("http://")) || url.len() > 300 {
                return Err(ForgeError::Validation(
                    "URL LinkedIn invalide (doit commencer par https://)".into(),
                ));
            }
            Some(url.to_string())
        }
        None => None,
    };
    if let Some(ref n) = req.paie_fr_niveau {
        CcnLevel::try_from(n.as_str()).map_err(ForgeError::Validation)?;
    }
    for exp in &req.expertises {
        CcnLevel::try_from(exp.niveau.as_str()).map_err(ForgeError::Validation)?;
    }
    for p in &req.pays {
        CcnLevel::try_from(p.niveau.as_str()).map_err(ForgeError::Validation)?;
    }

    // 3. Chiffrement email + hash mot de passe
    let enc_key  = parse_encryption_key();
    let e_hash   = email_hash(&req.email, &enc_key);
    let e_enc    = encrypt_email(&req.email, &enc_key);
    let pwd_hash = hash_password(&req.password);

    // 4. Génération token de vérification email
    let smtp_configured = std::env::var("SMTP_HOST").is_ok();
    let verification_token = uuid::Uuid::new_v4().to_string();
    let token_expires_at = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default();

    let mut tx = pool.begin().await.map_err(ForgeError::Db)?;

    // 5. Insérer ou récupérer l'utilisateur (via email_hash pour déduplication)
    sqlx::query(
        "INSERT OR IGNORE INTO users
             (email_hash, email_enc, password_hash, email_verified, verification_token, token_expires_at)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&e_hash)
    .bind(&e_enc)
    .bind(&pwd_hash)
    .bind(if smtp_configured { 0 } else { 1 })
    .bind(&verification_token)
    .bind(&token_expires_at)
    .execute(&mut *tx)
    .await
    .map_err(ForgeError::Db)?;

    let user_id: i64 = sqlx::query_scalar("SELECT id FROM users WHERE email_hash = ?")
        .bind(&e_hash)
        .fetch_one(&mut *tx)
        .await
        .map_err(ForgeError::Db)?;

    // 6. Créer le profil avec status='pending'
    sqlx::query(
        "INSERT INTO contributor_profiles
             (user_id, pseudo, poste, linkedin_url, poste_est_actuel, paie_fr_niveau, status)
         VALUES (?, ?, ?, ?, ?, ?, 'pending')",
    )
    .bind(user_id)
    .bind(&pseudo)
    .bind(&poste)
    .bind(linkedin_url.as_deref())
    .bind(req.poste_est_actuel)
    .bind(req.paie_fr_niveau.as_deref())
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(ref db_err) = e {
            if db_err.is_unique_violation() {
                let msg = db_err.message();
                if msg.contains("user_id") {
                    return ForgeError::Conflit("Vous avez déjà un profil avec cet email".into());
                }
                return ForgeError::Conflit("Ce pseudo est déjà pris".into());
            }
        }
        ForgeError::Db(e)
    })?;

    let profile_id: i64 = sqlx::query_scalar("SELECT last_insert_rowid()")
        .fetch_one(&mut *tx)
        .await
        .map_err(ForgeError::Db)?;

    for exp in &req.expertises {
        sqlx::query(
            "INSERT INTO contributor_ccn_expertises (profile_id, ccn_idcc, ccn_libelle, niveau)
             VALUES (?, ?, ?, ?)",
        )
        .bind(profile_id)
        .bind(&exp.ccn_idcc)
        .bind(&exp.ccn_libelle)
        .bind(&exp.niveau)
        .execute(&mut *tx)
        .await
        .map_err(ForgeError::Db)?;
    }

    for p in &req.pays {
        sqlx::query(
            "INSERT INTO contributor_pays_expertises (profile_id, pays_code, pays_libelle, niveau)
             VALUES (?, ?, ?, ?)",
        )
        .bind(profile_id)
        .bind(&p.pays_code)
        .bind(&p.pays_libelle)
        .bind(&p.niveau)
        .execute(&mut *tx)
        .await
        .map_err(ForgeError::Db)?;
    }

    tx.commit().await.map_err(ForgeError::Db)?;

    // 7. Envoyer email de vérification si SMTP configuré
    if smtp_configured {
        let email_for_send = req.email.clone();
        let token_for_send = verification_token.clone();
        tokio::spawn(async move {
            if let Err(e) = send_verification_email(&email_for_send, &token_for_send).await {
                tracing::warn!("Échec envoi email de vérification : {e}");
            }
        });
    }

    let msg = if smtp_configured {
        "Profil créé. Vérifiez votre email pour activer votre compte, puis l'équipe Xenna validera votre inscription."
    } else {
        "Profil créé et en attente de validation par l'équipe Xenna."
    };

    Ok((
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "pseudo": pseudo,
            "status": "pending",
            "message": msg
        })),
    ))
}

// ── Envoi email de vérification ───────────────────────────────────────────────

async fn send_verification_email(email: &str, token: &str) -> Result<(), String> {
    use lettre::{
        message::header::ContentType,
        transport::smtp::authentication::Credentials,
        AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    };

    let smtp_host = std::env::var("SMTP_HOST").map_err(|_| "SMTP_HOST non défini".to_string())?;
    let smtp_port: u16 = std::env::var("SMTP_PORT")
        .unwrap_or_else(|_| "587".into())
        .parse()
        .unwrap_or(587);
    let smtp_user = std::env::var("SMTP_USER").map_err(|_| "SMTP_USER non défini".to_string())?;
    let smtp_pass = std::env::var("SMTP_PASS").map_err(|_| "SMTP_PASS non défini".to_string())?;
    let smtp_from = std::env::var("SMTP_FROM")
        .unwrap_or_else(|_| "noreply@payetonbulletin.fr".into());

    let base_url = std::env::var("BASE_URL")
        .unwrap_or_else(|_| "https://payetonbulletin.fr".into());
    let verify_url = format!("{base_url}/forge/verifier-email?token={token}");
    let body = format!(
        "Bienvenue dans La Forge Métier !\n\nCliquez sur ce lien pour vérifier votre email :\n{verify_url}\n\nCe lien expire dans 24 heures.\n\nL'équipe Xenna Paie"
    );

    let from_addr: lettre::Address = smtp_from.parse().map_err(|e: lettre::address::AddressError| e.to_string())?;
    let to_addr:   lettre::Address = email.parse().map_err(|e: lettre::address::AddressError| e.to_string())?;

    let email_msg = Message::builder()
        .from(lettre::message::Mailbox::new(None, from_addr))
        .to(lettre::message::Mailbox::new(None, to_addr))
        .subject("Xenna Paie — Vérifiez votre email")
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .map_err(|e| e.to_string())?;

    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_host)
            .map_err(|e| e.to_string())?
            .port(smtp_port)
            .credentials(creds)
            .build();

    mailer.send(email_msg).await.map_err(|e| e.to_string())?;
    Ok(())
}

// ── GET /forge/verifier-email?token=<uuid> ────────────────────────────────────

#[derive(Deserialize)]
struct VerifQuery {
    token: String,
}

async fn verifier_email(
    State(pool): State<Db>,
    Query(q): Query<VerifQuery>,
) -> Html<String> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let result = sqlx::query(
        "UPDATE users SET email_verified = 1, verification_token = NULL, token_expires_at = NULL
         WHERE verification_token = ? AND token_expires_at >= ?",
    )
    .bind(&q.token)
    .bind(&now)
    .execute(&*pool)
    .await;

    let (title, msg, color) = match result {
        Ok(r) if r.rows_affected() > 0 => (
            "Email vérifié ✓",
            "Votre email a été vérifié. Votre profil est en attente de validation par l'équipe Xenna.",
            "#22c55e",
        ),
        Ok(_) => (
            "Lien invalide ou expiré",
            "Ce lien de vérification est invalide ou a expiré (24h). Contactez-nous si besoin.",
            "#ef4444",
        ),
        Err(_) => ("Erreur", "Une erreur est survenue. Veuillez réessayer.", "#f59e0b"),
    };

    Html(format!(
        r#"<!DOCTYPE html><html lang="fr"><head><meta charset="UTF-8">
        <title>Xenna — {title}</title>
        <style>body{{background:#0a0e1a;color:#c8d0e0;font-family:monospace;display:flex;align-items:center;justify-content:center;min-height:100vh;margin:0}}
        .box{{max-width:400px;padding:2rem;border:1px solid #1e2a3a;border-top:3px solid {color};background:#0d1221;text-align:center}}
        h1{{color:{color};font-size:1rem;letter-spacing:.1em;margin-bottom:1rem}}p{{font-size:.8rem;color:#718096;line-height:1.6}}
        a{{color:#4a9eff;text-decoration:none}}a:hover{{text-decoration:underline}}</style></head>
        <body><div class="box"><h1>{title}</h1><p>{msg}</p>
        <p style="margin-top:1.5rem"><a href="/">← Retour à Xenna Paie</a></p></div></body></html>"#
    ))
}

// ── Router exporté ─────────────────────────────────────────────────────────────
// NB : l'ancien POST /forge/voter (vote par pseudo, sans authentification) a été
// supprimé : n'importe qui pouvait voter au nom de n'importe quel profil. Le vote
// authentifié vit dans membre::routes (POST /la_forge/forum/topic/:id/vote).

pub fn forge_router() -> Router<Db> {
    Router::new()
        .route("/profil/{pseudo}",           get(get_profil))
        .route("/forge/contributeurs",       get(liste_contributeurs))
        .route("/forge/profil",              post(creer_profil))
        .route("/forge/verifier-email",      get(verifier_email))
}
