use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use super::models::{
    CcnExpertise, CcnLevel, ContributorProfile, CreerProfilReq, PaysExpertise, ProfilComplet,
};

type Db = Arc<SqlitePool>;

// ── Erreurs métier ─────────────────────────────────────────────────────────────

enum ForgeError {
    Introuvable(String),
    DejaVote,
    AutoVote,
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
            ForgeError::AutoVote => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Impossible de voter pour son propre sujet",
            )
                .into_response(),
            ForgeError::Conflit(msg) => (StatusCode::CONFLICT, msg).into_response(),
            ForgeError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response(),
            ForgeError::Db(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
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
                posts_count, topics_count, votes_received, votes_given, created_at
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

async fn liste_contributeurs(
    State(pool): State<Db>,
) -> Result<Json<Vec<ProfilComplet>>, ForgeError> {
    let profils = sqlx::query_as::<_, ContributorProfile>(
        "SELECT id, user_id, pseudo, linkedin_url, poste, poste_est_actuel,
                paie_fr_niveau,
                posts_count, topics_count, votes_received, votes_given, created_at
         FROM contributor_profiles
         ORDER BY votes_received DESC, pseudo ASC",
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
) -> Result<(StatusCode, Json<ProfilComplet>), ForgeError> {
    // Validation des niveaux avant toute écriture
    if let Some(ref n) = req.paie_fr_niveau {
        CcnLevel::try_from(n.as_str()).map_err(ForgeError::Validation)?;
    }
    for exp in &req.expertises {
        CcnLevel::try_from(exp.niveau.as_str()).map_err(ForgeError::Validation)?;
    }
    for p in &req.pays {
        CcnLevel::try_from(p.niveau.as_str()).map_err(ForgeError::Validation)?;
    }

    let mut tx = pool.begin().await.map_err(ForgeError::Db)?;

    sqlx::query("INSERT OR IGNORE INTO users (email) VALUES (?)")
        .bind(&req.email)
        .execute(&mut *tx)
        .await
        .map_err(ForgeError::Db)?;

    let user_id: i64 = sqlx::query_scalar("SELECT id FROM users WHERE email = ?")
        .bind(&req.email)
        .fetch_one(&mut *tx)
        .await
        .map_err(ForgeError::Db)?;

    sqlx::query(
        "INSERT INTO contributor_profiles
             (user_id, pseudo, poste, linkedin_url, poste_est_actuel, paie_fr_niveau)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(&req.pseudo)
    .bind(&req.poste)
    .bind(req.linkedin_url.as_deref())
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

    let profil     = profil_par_pseudo(&pool, &req.pseudo).await?;
    let expertises = charger_expertises(&pool, profil.id).await?;
    let pays       = charger_pays(&pool, profil.id).await?;
    Ok((StatusCode::CREATED, Json(ProfilComplet { profil, expertises, pays })))
}

// ── POST /forge/voter ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct VoteReq {
    voter_pseudo:  String,
    auteur_pseudo: String,
    topic_id:      i64,
}

async fn handle_voter(
    State(pool): State<Db>,
    Json(req): Json<VoteReq>,
) -> Result<StatusCode, ForgeError> {
    let voter  = profil_par_pseudo(&pool, &req.voter_pseudo).await?;
    let auteur = profil_par_pseudo(&pool, &req.auteur_pseudo).await?;

    if voter.id == auteur.id {
        return Err(ForgeError::AutoVote);
    }

    sqlx::query(
        "INSERT INTO forge_votes (voter_id, topic_author_id, topic_id) VALUES (?, ?, ?)",
    )
    .bind(voter.id)
    .bind(auteur.id)
    .bind(req.topic_id)
    .execute(&*pool)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

// ── Router exporté ─────────────────────────────────────────────────────────────

pub fn forge_router() -> Router<Db> {
    Router::new()
        .route("/profil/:pseudo",      get(get_profil))
        .route("/forge/contributeurs", get(liste_contributeurs))
        .route("/forge/profil",        post(creer_profil))
        .route("/forge/voter",         post(handle_voter))
}
