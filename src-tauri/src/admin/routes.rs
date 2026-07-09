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

use super::auth::{dummy_hash, generate_jwt, hash_password, jwt_secret, verify_password, AdminAuth};
use super::models::{
    AproposPost, ChangePasswordReq, DashboardData, InscriptionAdmin, LoginReq, LoginResp,
    PublishAproposReq, PublishAproposResp, QuizzSuggestionAdmin,
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
    let cle = format!("admin:{}", req.username.to_lowercase());
    if !crate::ratelimit::tentative_autorisee(&cle) {
        return Err((StatusCode::TOO_MANY_REQUESTS, "Trop de tentatives. Réessayez dans 15 minutes."));
    }

    let row = sqlx::query_as::<_, (String,)>(
        "SELECT password_hash FROM admin_users WHERE username = ?",
    )
    .bind(&req.username)
    .fetch_optional(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne"))?;

    // Vérification systématique (hash factice si compte inconnu) : le temps de
    // réponse ne doit pas révéler l'existence d'un identifiant.
    let valide = match &row {
        Some((hash,)) => verify_password(&req.password, hash),
        None => {
            verify_password(&req.password, dummy_hash());
            false
        }
    };

    if !valide {
        crate::ratelimit::enregistrer_echec(&cle);
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        return Err((StatusCode::UNAUTHORIZED, "Identifiants incorrects"));
    }
    crate::ratelimit::reinitialiser(&cle);

    let token = generate_jwt(&req.username, &jwt_secret())
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur JWT"))?;

    Ok(Json(LoginResp { token }))
}

// ── POST /admin/password ──────────────────────────────────────────────────────

async fn change_password(
    State(pool): State<Db>,
    auth: AdminAuth,
    Json(req): Json<ChangePasswordReq>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    // L'utilisateur est celui du JWT (claim `sub`) — on ne change QUE son propre mot de passe.
    let username = auth.0.sub;

    if req.new_password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, "Le nouveau mot de passe doit faire au moins 8 caractères"));
    }

    let row = sqlx::query_as::<_, (String,)>(
        "SELECT password_hash FROM admin_users WHERE username = ?",
    )
    .bind(&username)
    .fetch_optional(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne"))?;

    let (hash,) = row.ok_or((StatusCode::UNAUTHORIZED, "Compte introuvable"))?;

    // 403 (et non 401) : la session est valide, c'est le mot de passe actuel qui
    // est faux. Permet au front de distinguer « session expirée » d'« ancien code faux ».
    if !verify_password(&req.current_password, &hash) {
        return Err((StatusCode::FORBIDDEN, "Mot de passe actuel incorrect"));
    }

    let new_hash = hash_password(&req.new_password);
    sqlx::query("UPDATE admin_users SET password_hash = ? WHERE username = ?")
        .bind(&new_hash)
        .bind(&username)
        .execute(&*pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    Ok(StatusCode::NO_CONTENT)
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
    // Seules deux destinations sont valides ; toute autre valeur retombe sur "apropos".
    let destination = match req.destination.as_deref() {
        Some("carnet") => "carnet",
        _ => "apropos",
    };

    sqlx::query(
        "INSERT INTO apropos_posts (id, contenu, events, created_at, destination) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(contenu)
    .bind(&events_json)
    .bind(&now)
    .bind(destination)
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
    let rows = sqlx::query_as::<_, (String, String, String, String, String)>(
        "SELECT id, contenu, events, created_at, destination FROM apropos_posts ORDER BY created_at DESC",
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Erreur DB"))?;

    let posts = rows
        .into_iter()
        .map(|(id, contenu, events, created_at, destination)| {
            let events = serde_json::from_str(&events).unwrap_or(serde_json::Value::Null);
            AproposPost { id, contenu, events, created_at, destination }
        })
        .collect();

    Ok(Json(posts))
}

// ── Router exporté ────────────────────────────────────────────────────────────

// NB : préfixe volontairement trompeur (« archives-bareme-1997 » — une page
// d'archives poussiéreuse qui n'intéresse ni les bots ni les curieux) pour
// réduire le bruit des scanners qui cherchent /admin. Ce n'est PAS une
// protection : c'est le JWT (AdminAuth) qui protège les routes.
// admin.html dérive son préfixe de location.pathname : renommer ici suffit
// (plus le lien base64 dans src/main.js, côté burger Leeloo).
const PREFIXE_ADMIN: &str = "/archives-bareme-1997";

pub fn admin_router() -> Router<Db> {
    let p = |suffixe: &str| format!("{PREFIXE_ADMIN}{suffixe}");
    Router::new()
        .route(&p(""),                            get(admin_page))
        .route(&p("/login"),                      post(login))
        .route(&p("/password"),                   post(change_password))
        .route(&p("/dashboard"),                  get(dashboard))
        .route(&p("/inscription/{id}/approve"),   post(approve_inscription))
        .route(&p("/inscription/{id}/reject"),    post(reject_inscription))
        .route(&p("/quizz/{id}/approve"),         post(approve_quizz))
        .route(&p("/quizz/{id}/reject"),          post(reject_quizz))
        .route(&p("/apropos/publish"),            post(publish_apropos))
        .route(&p("/apropos/{id}"),               delete(delete_apropos))
        .route("/api/apropos/posts",              get(list_apropos_posts))
}
