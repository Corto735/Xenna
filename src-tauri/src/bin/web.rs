//! Serveur HTTP standalone — Railway / Docker.
//! Expose les mêmes commandes que Tauri via POST JSON sur /api/{commande}.

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Request, State},
    http::{header, HeaderValue, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::post,
    Json, Router,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Deserialize;
use sqlx::SqlitePool;
use tower_http::{cors::CorsLayer, services::ServeDir};

use xenna_paie_lib::{
    calculs::{generer_annee, generer_bulletin},
    db::{init_db, ContextPaie},
    forge::forge_router,
    models::{Salarie, Statut},
};

type Db = Arc<SqlitePool>;

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

struct ApiError(String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

// ── Middleware : redirection HTTP → HTTPS (via X-Forwarded-Proto) ─────────────
async fn https_redirect(req: Request, next: Next) -> Response {
    if req.headers()
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        == Some("http")
    {
        let host = req.headers()
            .get("host")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("www.payetonbulletin.fr");
        let path_query = req.uri().path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/");
        let location = format!("https://{host}{path_query}");
        return Redirect::permanent(&location).into_response();
    }
    next.run(req).await
}

// ── Middleware : en-têtes de sécurité ─────────────────────────────────────────
async fn security_headers(req: Request, next: Next) -> Response {
    let mut res = next.run(req).await;
    let h = res.headers_mut();
    h.insert("x-frame-options",        HeaderValue::from_static("DENY"));
    h.insert("x-content-type-options", HeaderValue::from_static("nosniff"));
    h.insert("referrer-policy",        HeaderValue::from_static("strict-origin-when-cross-origin"));
    h.insert("permissions-policy",     HeaderValue::from_static("geolocation=(), camera=(), microphone=()"));
    h.insert("strict-transport-security", HeaderValue::from_static("max-age=63072000; includeSubDomains"));
    h.insert(
        "content-security-policy",
        HeaderValue::from_static(
            "default-src 'self'; \
             style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; \
             font-src 'self' https://fonts.gstatic.com; \
             script-src 'self' 'unsafe-inline'; \
             connect-src 'self' https://api.mymemory.translated.net; \
             img-src 'self' data:; \
             frame-ancestors 'none'"
        ),
    );
    res
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
        .map_err(|e| {
            tracing::error!("ContextPaie::charger error: {:?}", e);
            ApiError("Erreur interne du serveur".into())
        })?;

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

    if brut <= Decimal::ZERO || brut > dec!(1_000_000) {
        return Err(ApiError(
            "Salaire brut hors limites (0 < salaire ≤ 1 000 000 €)".into(),
        ));
    }

    let sim = generer_annee(&pool, brut, req.statut, req.annee)
        .await
        .map_err(|e| {
            tracing::error!("generer_annee error: {:?}", e);
            ApiError("Erreur interne du serveur".into())
        })?;

    Ok(Json(sim))
}

// ── Main ──────────────────────────────────────────────────────────────────────
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "xenna_paie=info,warn".parse().unwrap()),
        )
        .init();

    let db_path: PathBuf = std::env::var("DATABASE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("xenna.db"));

    let pool = Arc::new(
        init_db(&db_path)
            .await
            .expect("Impossible d'initialiser la base de données"),
    );

    let dist = std::env::var("DIST_DIR").unwrap_or_else(|_| "../dist".to_string());

    // CORS : autorise seulement l'origine configurée (même serveur en prod)
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/calculer_bulletin", post(handle_bulletin))
        .route("/api/simuler_annee", post(handle_annee))
        .merge(forge_router())
        .fallback_service(ServeDir::new(&dist))
        .layer(middleware::from_fn(security_headers))
        .layer(cors)
        .layer(middleware::from_fn(https_redirect))
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
