//! Serveur HTTP standalone — Railway / Docker.
//! Expose les mêmes commandes que Tauri via POST JSON sur /api/{commande}.
//!
//! Les routes correspondent exactement aux noms de commandes Tauri :
//!   POST /api/calculer_bulletin  ←→  tauri::command calculer_bulletin
//!   POST /api/simuler_annee      ←→  tauri::command simuler_annee
//!
//! Le frontend détecte le mode via window.__TAURI__ (absent en web) et bascule
//! entre invoke() et fetch() de façon transparente (voir src/main.js).

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::SqlitePool;
use tower_http::{cors::CorsLayer, services::ServeDir};

use xenna_paie_lib::{
    calculs::{generer_annee, generer_bulletin},
    db::{init_db, ContextPaie},
    forge::forge_router,
    models::{Salarie, Statut},
};

// ── State ─────────────────────────────────────────────────────────────────────
type Db = Arc<SqlitePool>;

// ── Corps de requête ──────────────────────────────────────────────────────────
// Le JS envoie les champs en camelCase (convention Tauri) → on rename ici.
// Si la désérialisation échoue (champ manquant, mauvais type), Axum retourne
// automatiquement un 422 avec le message d'erreur serde en plain text.
// Ce n'est PAS capturé par ApiError ci-dessous — c'est voulu.
#[derive(Deserialize)]
struct BulletinReq {
    salarie: Salarie,
    #[serde(rename = "datePaie")]
    date_paie: String,
}

#[derive(Deserialize)]
struct AnneeReq {
    annee: i32,
    #[serde(rename = "salaireBrut")]
    salaire_brut: String,
    statut: Statut,
}

// ── Erreur métier (400 Bad Request, corps = texte lisible) ────────────────────
// Distinct des erreurs de désérialisation JSON (422, gérées par Axum).
struct ApiError(String);

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────
async fn handle_bulletin(
    State(pool): State<Db>,
    Json(req): Json<BulletinReq>,
) -> Result<impl IntoResponse, ApiError> {
    let date = NaiveDate::parse_from_str(&req.date_paie, "%Y-%m-%d")
        .map_err(|_| ApiError(format!("Date invalide : '{}'", req.date_paie)))?;

    let ctx = ContextPaie::charger(&pool, date)
        .await
        .map_err(|e| ApiError(e.to_string()))?;

    Ok(Json(generer_bulletin(req.salarie, &ctx)))
}

async fn handle_annee(
    State(pool): State<Db>,
    Json(req): Json<AnneeReq>,
) -> Result<impl IntoResponse, ApiError> {
    let brut: Decimal = req
        .salaire_brut
        .parse()
        .map_err(|_| ApiError(format!("Salaire invalide : '{}'", req.salaire_brut)))?;

    let sim = generer_annee(&pool, brut, req.statut, req.annee)
        .await
        .map_err(|e| ApiError(e.to_string()))?;

    Ok(Json(sim))
}

// ── Main ──────────────────────────────────────────────────────────────────────
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_path: PathBuf = std::env::var("DATABASE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("xenna.db"));

    let pool = Arc::new(
        init_db(&db_path)
            .await
            .expect("Impossible d'initialiser la base de données"),
    );

    let dist = std::env::var("DIST_DIR").unwrap_or_else(|_| "../dist".to_string());

    let app = Router::new()
        .route("/api/calculer_bulletin", post(handle_bulletin))
        .route("/api/simuler_annee", post(handle_annee))
        .merge(forge_router())
        .fallback_service(ServeDir::new(&dist))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Xenna web → http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
