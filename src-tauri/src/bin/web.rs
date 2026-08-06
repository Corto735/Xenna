//! Serveur HTTP standalone — Railway / Docker.
//! Expose les mêmes commandes que Tauri via POST JSON sur /api/{commande}.

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Request, State},
    http::{header, HeaderValue, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Deserialize;
use sqlx::SqlitePool;
use tower_http::{cors::CorsLayer, services::ServeDir};

use xenna_paie_lib::{
    admin::admin_router,
    altcha::{generate_challenge, AltchaChallenge},
    calculs::{generer_annee, generer_bulletin},
    ccn::ccn_router,
    db::{init_db, ContextPaie},
    forge::forge_router,
    membre::membre_router,
    quizz::quizz_router,
    models::{AbsenceInput, Salarie, Statut},
};
use meliinda::meliinda_router;

type Db = Arc<SqlitePool>;

#[derive(Deserialize)]
struct BulletinReq {
    salarie: Salarie,
    #[serde(rename = "datePaie")]
    date_paie: String,
    // Langue d'affichage des libellés/explications ("fr" par défaut).
    #[serde(default)]
    lang: Option<String>,
    // Absence maladie éventuelle (retenue + maintien + IJSS).
    #[serde(default)]
    absence: Option<AbsenceInput>,
    // Paye inversée : net souhaité AVANT impôt à la source. Si présent,
    // salaire_brut est ignoré et le brut est reconstitué par dichotomie.
    #[serde(default, rename = "netCible")]
    net_cible: Option<String>,
}

#[derive(Deserialize)]
struct AnneeReq {
    annee: i32,
    #[serde(rename = "salaireBrut")]
    salaire_brut: String,
    statut: Statut,
    #[serde(default = "default_etp_100")]
    etp: f64,
}

fn default_etp_100() -> f64 { 100.0 }

struct ApiError(String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

// ── Vérification des secrets au démarrage ─────────────────────────────────────
// Chaque secret absent avait un repli de développement silencieux (JWT forgeable,
// emails chiffrés avec une clé nulle, captcha désactivé). En production on
// refuse de démarrer plutôt que de tourner vulnérable.
fn verifier_secrets_ou_quitter() {
    const REQUIS: [&str; 4] = ["ADMIN_JWT_SECRET", "MEMBER_JWT_SECRET", "ENCRYPTION_KEY", "ALTCHA_SECRET"];
    let manquants: Vec<&str> = REQUIS
        .iter()
        .copied()
        .filter(|v| std::env::var(v).map(|s| s.trim().is_empty()).unwrap_or(true))
        .collect();

    if manquants.is_empty() {
        // Valide aussi le format (base64, 32 octets) : panique ici plutôt qu'à
        // la première inscription.
        let _ = xenna_paie_lib::crypto::parse_encryption_key();
        return;
    }

    if std::env::var("XENNA_DEV_MODE").is_ok() {
        tracing::warn!(
            "XENNA_DEV_MODE actif — secrets manquants tolérés (JAMAIS en production) : {manquants:?}"
        );
        return;
    }

    eprintln!(
        "ERREUR : variables d'environnement de sécurité manquantes : {manquants:?}\n\
         Générer chaque valeur avec : openssl rand -base64 32\n\
         (ENCRYPTION_KEY doit décoder exactement 32 octets)\n\
         Pour un poste de développement local uniquement : XENNA_DEV_MODE=1."
    );
    std::process::exit(1);
}

// ── Middleware : redirection HTTP → HTTPS (via X-Forwarded-Proto) ─────────────
async fn https_redirect(req: Request, next: Next) -> Response {
    if req.headers()
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        == Some("http")
    {
        // Hôte issu d'une allowlist : un en-tête Host arbitraire ne doit pas
        // transformer la redirection en open redirect.
        let host = req.headers()
            .get("host")
            .and_then(|v| v.to_str().ok())
            .filter(|h| {
                matches!(*h, "www.payetonbulletin.fr" | "payetonbulletin.fr")
                    || h.ends_with(".cleverapps.io")
            })
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
        // worker-src 'self' blob: — requis par le widget Altcha (Web Worker)
        HeaderValue::from_static(
            "default-src 'self'; \
             style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; \
             font-src 'self' https://fonts.gstatic.com; \
             script-src 'self' 'unsafe-inline'; \
             worker-src 'self' blob:; \
             connect-src 'self' https://api.mymemory.translated.net; \
             img-src 'self' data:; \
             frame-ancestors 'none'"
        ),
    );
    res
}

// ── GET /altcha/challenge ──────────────────────────────────────────────────────
async fn altcha_challenge() -> Json<AltchaChallenge> {
    let secret = std::env::var("ALTCHA_SECRET").unwrap_or_else(|_| "dev_altcha_secret".into());
    Json(generate_challenge(&secret))
}

// ── Handlers ──────────────────────────────────────────────────────────────────
async fn handle_bulletin(
    State(pool): State<Db>,
    Json(req): Json<BulletinReq>,
) -> Result<impl IntoResponse, ApiError> {
    let date = NaiveDate::parse_from_str(&req.date_paie, "%Y-%m-%d")
        .map_err(|_| ApiError(format!("Date invalide : '{}'", req.date_paie)))?;

    let mut ctx = ContextPaie::charger(&pool, date)
        .await
        .map_err(|e| {
            tracing::error!("ContextPaie::charger error: {:?}", e);
            ApiError("Erreur interne du serveur".into())
        })?;
    ctx.lang = req.lang.unwrap_or_else(|| "fr".into());

    match req.net_cible {
        Some(n) => {
            let net: rust_decimal::Decimal = n
                .parse()
                .map_err(|_| ApiError(format!("Net cible invalide : '{n}'")))?;
            if net <= rust_decimal::Decimal::ZERO {
                return Err(ApiError("Net cible invalide — saisir un montant positif.".into()));
            }
            Ok(Json(xenna_paie_lib::calculs::paye_inverse::resoudre_brut_pour_net(
                net, &req.salarie, &ctx, req.absence.as_ref(),
            )))
        }
        None => Ok(Json(generer_bulletin(req.salarie, &ctx, req.absence.as_ref()))),
    }
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

    let sim = generer_annee(&pool, brut, req.statut, req.annee, req.etp)
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

    verifier_secrets_ou_quitter();

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

    let meliinda = meliinda_router(pool.clone())
        .await
        .expect("Impossible d'initialiser Meliinda");

    let app = Router::new()
        .route("/api/calculer_bulletin", post(handle_bulletin))
        .route("/api/simuler_annee", post(handle_annee))
        .route("/altcha/challenge", get(altcha_challenge))
        .merge(forge_router())
        .merge(quizz_router())
        .merge(ccn_router())
        .merge(admin_router())
        .merge(membre_router())
        .merge(meliinda)
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
