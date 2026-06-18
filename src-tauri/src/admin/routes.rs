use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Json, Router,
};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use super::auth::{generate_jwt, jwt_secret, verify_password, AdminAuth};
use super::models::{
    AproposPost, DashboardData, InscriptionAdmin, LoginReq, LoginResp, PublishAproposReq,
    PublishAproposResp, QuizzSuggestionAdmin,
};
use crate::crypto::{decrypt_email, parse_encryption_key};

type Db = Arc<SqlitePool>;

const ADMIN_HTML: &str = include_str!("admin.html");

// ── GET /admin ─────────────────────────────────────────────────────────────────

async fn admin_page() -> Html<&'static str> {
    Html(ADMIN_HTML)
}

// ── POST /admin/login ─────────────────────────────────────────────────────────

async fn login(
    State(pool): State<Db>,
    Json(req): Json<LoginReq>,
) -> Result<Json<LoginResp>, (StatusCode, &'static str)> {
    let row = sqlx::query_as::<_, (String,)>(
        "SELECT password_hash FROM admin_users WHERE username = ?",
    )
    .bind(&req.username)
    .fetch_optional(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne"))?;

    let (hash,) = row.ok_or((StatusCode::UNAUTHORIZED, "Identifiants incorrects"))?;

    if !verify_password(&req.password, &hash) {
        return Err((StatusCode::UNAUTHORIZED, "Identifiants incorrects"));
    }

    let token = generate_jwt(&req.username, &jwt_secret())
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur JWT"))?;

    Ok(Json(LoginResp { token }))
}

// ── GET /admin/dashboard ──────────────────────────────────────────────────────

async fn dashboard(
    State(pool): State<Db>,
    _auth: AdminAuth,
) -> Result<Json<DashboardData>, (StatusCode, &'static str)> {
    let enc_key = parse_encryption_key();

    // Profils en attente avec email chiffré
    let rows = sqlx::query_as::<_, (i64, String, String, Option<String>, String, String)>(
        "SELECT cp.id, cp.pseudo, cp.poste, u.email_enc, cp.status, cp.created_at
         FROM contributor_profiles cp
         INNER JOIN users u ON u.id = cp.user_id
         WHERE cp.status = 'pending'
         ORDER BY cp.created_at ASC",
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    let inscriptions_pending: Vec<InscriptionAdmin> = rows
        .into_iter()
        .map(|(id, pseudo, poste, email_enc, status, created_at)| {
            let email_clair = email_enc
                .and_then(|enc| decrypt_email(&enc, &enc_key).ok())
                .unwrap_or_else(|| "— chiffrement indisponible —".into());
            InscriptionAdmin { id, pseudo, poste, email_clair, status, created_at }
        })
        .collect();

    // Suggestions quizz en attente
    let quizz_pending = sqlx::query_as::<_, QuizzSuggestionAdmin>(
        "SELECT id, pays, question, reponse, reps_alt, mauvaises_rep, source, pseudo, created_at, votes
         FROM quizz_suggestions
         WHERE admin_status = 'pending'
         ORDER BY votes DESC, created_at ASC",
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    Ok(Json(DashboardData { inscriptions_pending, quizz_pending }))
}

// ── POST /admin/inscription/:id/approve ──────────────────────────────────────

async fn approve_inscription(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    sqlx::query("UPDATE contributor_profiles SET status = 'approved' WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

// ── POST /admin/inscription/:id/reject ───────────────────────────────────────

async fn reject_inscription(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    sqlx::query("UPDATE contributor_profiles SET status = 'rejected' WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

// ── POST /admin/quizz/:id/approve ────────────────────────────────────────────

async fn approve_quizz(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    sqlx::query("UPDATE quizz_suggestions SET admin_status = 'approved' WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

// ── POST /admin/quizz/:id/reject ─────────────────────────────────────────────

async fn reject_quizz(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    sqlx::query("UPDATE quizz_suggestions SET admin_status = 'rejected' WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

// ── POST /admin/apropos/publish ──────────────────────────────────────────────

async fn publish_apropos(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Json(req): Json<PublishAproposReq>,
) -> Result<Json<PublishAproposResp>, (StatusCode, &'static str)> {
    let contenu = req.contenu.trim();
    if contenu.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Contenu vide"));
    }

    let id = Uuid::new_v4().to_string();
    let events_json = serde_json::to_string(&req.events)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Events invalides"))?;
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO apropos_posts (id, contenu, events, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(contenu)
    .bind(&events_json)
    .bind(&now)
    .execute(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    Ok(Json(PublishAproposResp { id }))
}

// ── DELETE /admin/apropos/:id ─────────────────────────────────────────────────

async fn delete_apropos(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Path(id): Path<String>,
) -> impl IntoResponse {
    sqlx::query("DELETE FROM apropos_posts WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

// ── GET /api/apropos/posts (public) ──────────────────────────────────────────

async fn list_apropos_posts(
    State(pool): State<Db>,
) -> Result<Json<Vec<AproposPost>>, (StatusCode, &'static str)> {
    let rows = sqlx::query_as::<_, (String, String, String, String)>(
        "SELECT id, contenu, events, created_at FROM apropos_posts ORDER BY created_at DESC",
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    let posts = rows
        .into_iter()
        .map(|(id, contenu, events, created_at)| {
            let events = serde_json::from_str(&events).unwrap_or(serde_json::Value::Null);
            AproposPost { id, contenu, events, created_at }
        })
        .collect();

    Ok(Json(posts))
}

// ── Router exporté ────────────────────────────────────────────────────────────

pub fn admin_router() -> Router<Db> {
    Router::new()
        .route("/admin",                            get(admin_page))
        .route("/admin/login",                      post(login))
        .route("/admin/dashboard",                  get(dashboard))
        .route("/admin/inscription/{id}/approve",   post(approve_inscription))
        .route("/admin/inscription/{id}/reject",    post(reject_inscription))
        .route("/admin/quizz/{id}/approve",         post(approve_quizz))
        .route("/admin/quizz/{id}/reject",          post(reject_quizz))
        .route("/admin/apropos/publish",            post(publish_apropos))
        .route("/admin/apropos/{id}",               delete(delete_apropos))
        .route("/api/apropos/posts",                get(list_apropos_posts))
}
