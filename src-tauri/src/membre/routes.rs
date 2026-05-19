use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{get, post, put},
    Json, Router,
};
use sqlx::SqlitePool;

use super::auth::{member_jwt_secret, member_profile_id, MemberAuth};
use super::models::*;
use crate::{
    admin::auth::verify_password,
    crypto::{email_hash, parse_encryption_key},
    forge::models::{CcnExpertise, CcnLevel, PaysExpertise},
};

type Db = Arc<SqlitePool>;

const LA_FORGE_HTML: &str = include_str!("la_forge.html");

fn sanitiser(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, '<' | '>') && (!c.is_control() || matches!(c, '\n' | '\r' | '\t' | ' ')))
        .collect::<String>()
        .trim()
        .to_string()
}

// ── GET /la_forge ─────────────────────────────────────────────────────────────

async fn la_forge_page() -> Html<&'static str> {
    Html(LA_FORGE_HTML)
}

// ── POST /la_forge/login ──────────────────────────────────────────────────────

async fn login(
    State(pool): State<Db>,
    Json(req): Json<MemberLoginReq>,
) -> Result<Json<MemberLoginResp>, (StatusCode, &'static str)> {
    let enc_key = parse_encryption_key();
    let e_hash  = email_hash(&req.email, &enc_key);

    let row = sqlx::query_as::<_, (i64, Option<String>, i64, String, String)>(
        "SELECT u.id, u.password_hash, u.email_verified, cp.status, cp.pseudo
         FROM users u
         INNER JOIN contributor_profiles cp ON cp.user_id = u.id
         WHERE u.email_hash = ?",
    )
    .bind(&e_hash)
    .fetch_optional(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne"))?
    .ok_or((StatusCode::UNAUTHORIZED, "Email ou mot de passe incorrect"))?;

    let (user_id, pwd_hash_opt, email_verified, status, pseudo) = row;

    if email_verified == 0 {
        return Err((StatusCode::FORBIDDEN, "Email non vérifié. Vérifiez votre boîte mail."));
    }
    if status != "approved" {
        return Err((StatusCode::FORBIDDEN, "Votre compte est en attente de validation."));
    }

    let pwd_hash = pwd_hash_opt
        .ok_or((StatusCode::UNAUTHORIZED, "Email ou mot de passe incorrect"))?;

    if !verify_password(&req.password, &pwd_hash) {
        return Err((StatusCode::UNAUTHORIZED, "Email ou mot de passe incorrect"));
    }

    // Sub = profile_id
    let profile_id: i64 = sqlx::query_scalar(
        "SELECT id FROM contributor_profiles WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne"))?;

    let secret = member_jwt_secret();
    // JWT 7 jours
    let exp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 7 * 24 * 3600;

    let claims = crate::admin::auth::Claims { sub: profile_id.to_string(), exp };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur JWT"))?;

    Ok(Json(MemberLoginResp { token, pseudo }))
}

// ── GET /la_forge/me ─────────────────────────────────────────────────────────

async fn get_me(
    State(pool): State<Db>,
    auth: MemberAuth,
) -> Result<Json<MemberProfileData>, (StatusCode, &'static str)> {
    let profile_id = member_profile_id(&auth)?;

    let row = sqlx::query_as::<_, (i64, String, String, bool, Option<String>, Option<String>, i64, i64, i64)>(
        "SELECT id, pseudo, poste, poste_est_actuel, linkedin_url, paie_fr_niveau,
                posts_count, topics_count, votes_received
         FROM contributor_profiles WHERE id = ?",
    )
    .bind(profile_id)
    .fetch_optional(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?
    .ok_or((StatusCode::NOT_FOUND, "Profil introuvable"))?;

    let (id, pseudo, poste, poste_est_actuel, linkedin_url, paie_fr_niveau,
         posts_count, topics_count, votes_received) = row;

    let expertises = charger_expertises(&pool, id).await?;
    let pays       = charger_pays(&pool, id).await?;

    Ok(Json(MemberProfileData {
        id, pseudo, poste, poste_est_actuel, linkedin_url, paie_fr_niveau,
        posts_count, topics_count, votes_received, expertises, pays,
    }))
}

// ── PUT /la_forge/me ─────────────────────────────────────────────────────────

async fn update_me(
    State(pool): State<Db>,
    auth: MemberAuth,
    Json(req): Json<UpdateProfileReq>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    let profile_id = member_profile_id(&auth)?;

    if req.poste.trim().is_empty() || req.poste.len() > 100 {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Poste : 1 à 100 caractères"));
    }
    if req.expertises.len() > 20 {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Maximum 20 CCN"));
    }
    if req.pays.len() > 20 {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Maximum 20 pays"));
    }
    for exp in &req.expertises {
        CcnLevel::try_from(exp.niveau.as_str())
            .map_err(|_| (StatusCode::UNPROCESSABLE_ENTITY, "Niveau CCN invalide"))?;
    }
    for p in &req.pays {
        CcnLevel::try_from(p.niveau.as_str())
            .map_err(|_| (StatusCode::UNPROCESSABLE_ENTITY, "Niveau pays invalide"))?;
    }

    let mut tx = pool.begin().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    sqlx::query(
        "UPDATE contributor_profiles
         SET poste = ?, poste_est_actuel = ?, linkedin_url = ?, paie_fr_niveau = ?
         WHERE id = ?",
    )
    .bind(req.poste.trim())
    .bind(req.poste_est_actuel)
    .bind(req.linkedin_url.as_deref())
    .bind(req.paie_fr_niveau.as_deref())
    .bind(profile_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    sqlx::query("DELETE FROM contributor_ccn_expertises WHERE profile_id = ?")
        .bind(profile_id).execute(&mut *tx).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    for exp in &req.expertises {
        sqlx::query(
            "INSERT INTO contributor_ccn_expertises (profile_id, ccn_idcc, ccn_libelle, niveau)
             VALUES (?, ?, ?, ?)",
        )
        .bind(profile_id).bind(&exp.ccn_idcc).bind(&exp.ccn_libelle).bind(&exp.niveau)
        .execute(&mut *tx).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;
    }

    sqlx::query("DELETE FROM contributor_pays_expertises WHERE profile_id = ?")
        .bind(profile_id).execute(&mut *tx).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    for p in &req.pays {
        sqlx::query(
            "INSERT INTO contributor_pays_expertises (profile_id, pays_code, pays_libelle, niveau)
             VALUES (?, ?, ?, ?)",
        )
        .bind(profile_id).bind(&p.pays_code).bind(&p.pays_libelle).bind(&p.niveau)
        .execute(&mut *tx).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;
    }

    tx.commit().await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;
    Ok(StatusCode::NO_CONTENT)
}

// ── GET /la_forge/forum ───────────────────────────────────────────────────────

async fn liste_topics(
    State(pool): State<Db>,
    _auth: MemberAuth,
) -> Result<Json<Vec<TopicResume>>, (StatusCode, &'static str)> {
    let topics = sqlx::query_as::<_, TopicResume>(
        "SELECT ft.id, ft.titre, cp.pseudo AS author_pseudo, ft.created_at, ft.reply_count,
                COALESCE((SELECT COUNT(*) FROM forge_votes WHERE topic_id = ft.id), 0) AS votes
         FROM forum_topics ft
         INNER JOIN contributor_profiles cp ON cp.id = ft.author_id
         ORDER BY ft.created_at DESC
         LIMIT 100",
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    Ok(Json(topics))
}

// ── POST /la_forge/forum/topic ────────────────────────────────────────────────

async fn creer_topic(
    State(pool): State<Db>,
    auth: MemberAuth,
    Json(req): Json<CreateTopicReq>,
) -> Result<(StatusCode, Json<TopicDetail>), (StatusCode, &'static str)> {
    let profile_id = member_profile_id(&auth)?;

    let titre   = sanitiser(&req.titre);
    let contenu = sanitiser(&req.contenu);

    if titre.is_empty() || titre.len() > 200 {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Titre : 1 à 200 caractères"));
    }
    if contenu.is_empty() || contenu.len() > 10_000 {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Contenu : 1 à 10 000 caractères"));
    }

    sqlx::query("INSERT INTO forum_topics (author_id, titre, contenu) VALUES (?, ?, ?)")
        .bind(profile_id).bind(&titre).bind(&contenu)
        .execute(&*pool).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    let topic_id: i64 = sqlx::query_scalar("SELECT last_insert_rowid()")
        .fetch_one(&*pool).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    let detail = charger_topic_detail(&pool, topic_id, profile_id).await?;
    Ok((StatusCode::CREATED, Json(detail)))
}

// ── GET /la_forge/forum/topic/:id ────────────────────────────────────────────

async fn get_topic(
    State(pool): State<Db>,
    auth: MemberAuth,
    Path(id): Path<i64>,
) -> Result<Json<TopicDetail>, (StatusCode, &'static str)> {
    let profile_id = member_profile_id(&auth)?;
    let detail = charger_topic_detail(&pool, id, profile_id).await?;
    Ok(Json(detail))
}

// ── POST /la_forge/forum/topic/:id/reply ─────────────────────────────────────

async fn creer_reply(
    State(pool): State<Db>,
    auth: MemberAuth,
    Path(topic_id): Path<i64>,
    Json(req): Json<CreateReplyReq>,
) -> Result<(StatusCode, Json<ReplyDto>), (StatusCode, &'static str)> {
    let profile_id = member_profile_id(&auth)?;
    let contenu = sanitiser(&req.contenu);

    if contenu.is_empty() || contenu.len() > 5_000 {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Réponse : 1 à 5 000 caractères"));
    }

    // Vérifier que le topic existe
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM forum_topics WHERE id = ?)")
        .bind(topic_id)
        .fetch_one(&*pool).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    if !exists {
        return Err((StatusCode::NOT_FOUND, "Sujet introuvable"));
    }

    sqlx::query("INSERT INTO forum_replies (topic_id, author_id, contenu) VALUES (?, ?, ?)")
        .bind(topic_id).bind(profile_id).bind(&contenu)
        .execute(&*pool).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    let reply_id: i64 = sqlx::query_scalar("SELECT last_insert_rowid()")
        .fetch_one(&*pool).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    let reply = sqlx::query_as::<_, ReplyDto>(
        "SELECT fr.id, fr.contenu, cp.pseudo AS author_pseudo, fr.created_at
         FROM forum_replies fr
         INNER JOIN contributor_profiles cp ON cp.id = fr.author_id
         WHERE fr.id = ?",
    )
    .bind(reply_id)
    .fetch_one(&*pool).await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    Ok((StatusCode::CREATED, Json(reply)))
}

// ── POST /la_forge/forum/topic/:id/vote ──────────────────────────────────────

async fn voter_topic(
    State(pool): State<Db>,
    auth: MemberAuth,
    Path(topic_id): Path<i64>,
) -> Result<Json<i64>, (StatusCode, &'static str)> {
    let profile_id = member_profile_id(&auth)?;

    let author_id: i64 = sqlx::query_scalar("SELECT author_id FROM forum_topics WHERE id = ?")
        .bind(topic_id)
        .fetch_optional(&*pool).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?
        .ok_or((StatusCode::NOT_FOUND, "Sujet introuvable"))?;

    if author_id == profile_id {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Impossible de voter pour son propre sujet"));
    }

    sqlx::query(
        "INSERT OR IGNORE INTO forge_votes (voter_id, topic_author_id, topic_id) VALUES (?, ?, ?)",
    )
    .bind(profile_id).bind(author_id).bind(topic_id)
    .execute(&*pool).await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM forge_votes WHERE topic_id = ?")
        .bind(topic_id)
        .fetch_one(&*pool).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    Ok(Json(count))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

async fn charger_expertises(
    pool: &SqlitePool,
    profile_id: i64,
) -> Result<Vec<CcnExpertise>, (StatusCode, &'static str)> {
    sqlx::query_as::<_, CcnExpertise>(
        "SELECT id, profile_id, ccn_idcc, ccn_libelle, niveau
         FROM contributor_ccn_expertises
         WHERE profile_id = ?
         ORDER BY CASE niveau WHEN 'Maîtrisée' THEN 1 WHEN 'Pratiquée' THEN 2 ELSE 3 END",
    )
    .bind(profile_id)
    .fetch_all(pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))
}

async fn charger_pays(
    pool: &SqlitePool,
    profile_id: i64,
) -> Result<Vec<PaysExpertise>, (StatusCode, &'static str)> {
    sqlx::query_as::<_, PaysExpertise>(
        "SELECT id, profile_id, pays_code, pays_libelle, niveau
         FROM contributor_pays_expertises
         WHERE profile_id = ?
         ORDER BY CASE niveau WHEN 'Maîtrisée' THEN 1 WHEN 'Pratiquée' THEN 2 ELSE 3 END",
    )
    .bind(profile_id)
    .fetch_all(pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))
}

async fn charger_topic_detail(
    pool: &SqlitePool,
    topic_id: i64,
    viewer_id: i64,
) -> Result<TopicDetail, (StatusCode, &'static str)> {
    let row = sqlx::query_as::<_, (i64, String, String, String, String, i64)>(
        "SELECT ft.id, ft.titre, ft.contenu, cp.pseudo, ft.created_at, ft.reply_count
         FROM forum_topics ft
         INNER JOIN contributor_profiles cp ON cp.id = ft.author_id
         WHERE ft.id = ?",
    )
    .bind(topic_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?
    .ok_or((StatusCode::NOT_FOUND, "Sujet introuvable"))?;

    let (id, titre, contenu, author_pseudo, created_at, reply_count) = row;

    let votes: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM forge_votes WHERE topic_id = ?")
        .bind(topic_id)
        .fetch_one(pool).await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    let user_voted: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM forge_votes WHERE topic_id = ? AND voter_id = ?)",
    )
    .bind(topic_id).bind(viewer_id)
    .fetch_one(pool).await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    let replies = sqlx::query_as::<_, ReplyDto>(
        "SELECT fr.id, fr.contenu, cp.pseudo AS author_pseudo, fr.created_at
         FROM forum_replies fr
         INNER JOIN contributor_profiles cp ON cp.id = fr.author_id
         WHERE fr.topic_id = ?
         ORDER BY fr.created_at ASC",
    )
    .bind(topic_id)
    .fetch_all(pool).await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    Ok(TopicDetail { id, titre, contenu, author_pseudo, created_at, reply_count, votes, replies, user_voted })
}

// ── Router exporté ────────────────────────────────────────────────────────────

pub fn membre_router() -> Router<Db> {
    Router::new()
        .route("/la_forge",                              get(la_forge_page))
        .route("/la_forge/login",                        post(login))
        .route("/la_forge/me",                           get(get_me))
        .route("/la_forge/me",                           put(update_me))
        .route("/la_forge/forum",                        get(liste_topics))
        .route("/la_forge/forum/topic",                  post(creer_topic))
        .route("/la_forge/forum/topic/{id}",             get(get_topic))
        .route("/la_forge/forum/topic/{id}/reply",       post(creer_reply))
        .route("/la_forge/forum/topic/{id}/vote",        post(voter_topic))
}
