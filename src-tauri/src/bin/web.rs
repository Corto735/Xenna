//! Serveur HTTP standalone — Railway / Docker.
//! Expose les mêmes commandes que Tauri via POST JSON sur /api/{commande}.

use std::{net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};

use axum::{
    extract::{ConnectInfo, Request, State},
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
    contrat::{pdf as contrat_pdf, ContratPdf, ReponsePdf},
    paie_pdf::{pdf as bulletin_pdf, BulletinPdf},
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

    // Le repli de développement n'existe QUE dans un binaire de debug. Compilé
    // en release, `XENNA_DEV_MODE` ne veut plus rien dire : une variable
    // d'environnement égarée dans la config de prod ne peut plus faire démarrer
    // le serveur avec le secret JWT public de `admin/auth.rs` (forge de jeton
    // admin), une clé de chiffrement nulle et le captcha désactivé.
    #[cfg(debug_assertions)]
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
         XENNA_DEV_MODE=1 lève ce contrôle, mais uniquement dans un binaire de\n\
         debug (cargo run) — jamais dans une build release."
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

// ── Middleware : quota par adresse IP ─────────────────────────────────────────
// Le compteur d'échecs de `ratelimit` est indexé par compte : changer
// d'identifiant à chaque essai le contournait entièrement, alors que chaque
// tentative coûte un Argon2id (~19 Mio). Ce quota-ci est indexé par IP et
// s'applique avant d'atteindre le handler.

/// Fenêtre et plafond selon la route. `None` = pas de quota.
fn quota_pour(chemin: &str) -> Option<(u32, Duration)> {
    // Authentification et inscription : coûteuses (Argon2id) et sensibles.
    let auth = chemin.ends_with("/login")
        || chemin == "/forge/profil"
        || chemin == "/la_forge/login";
    if auth {
        return Some((15, Duration::from_secs(15 * 60)));
    }
    // Écritures publiques sans authentification : bornées plus large.
    if chemin == "/api/meliinda/record"
        || chemin == "/quizz/score"
        || chemin == "/quizz/suggestion"
        || chemin.starts_with("/quizz/vote/")
    {
        return Some((30, Duration::from_secs(60)));
    }
    // Calculs : bon marché à l'unité, mais c'est le gros du trafic anonyme.
    if chemin.starts_with("/api/calculer_bulletin") || chemin.starts_with("/api/simuler_annee") {
        return Some((120, Duration::from_secs(60)));
    }
    None
}

/// IP du client. Derrière le proxy Clever Cloud, `x-forwarded-for` porte
/// l'adresse réelle en première position ; en direct on retombe sur la socket.
/// Un en-tête absent ne doit pas faire partager un même compteur à tout le
/// monde : sans IP identifiable, on n'applique pas de quota.
fn ip_client(req: &Request) -> Option<String> {
    if let Some(xff) = req.headers().get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(premier) = xff.split(',').next().map(str::trim).filter(|s| !s.is_empty()) {
            return Some(premier.to_string());
        }
    }
    req.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0.ip().to_string())
}

async fn limite_par_ip(req: Request, next: Next) -> Response {
    if let Some((max, fenetre)) = quota_pour(req.uri().path()) {
        if let Some(ip) = ip_client(&req) {
            let cle = format!("ip:{ip}:{}", req.uri().path());
            if !xenna_paie_lib::ratelimit::quota_autorise(&cle, max, fenetre) {
                tracing::warn!("Quota dépassé — {ip} sur {}", req.uri().path());
                return (
                    StatusCode::TOO_MANY_REQUESTS,
                    "Trop de requêtes. Réessayez dans quelques minutes.",
                )
                    .into_response();
            }
        }
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

// ── POST /api/generer_contrat_pdf ─────────────────────────────────────────────
// Le contrat arrive rédigé : le back n'en compose que la mise en page. Rien n'est
// écrit sur disque ni en base, et le corps de la requête — qui porte le NIR,
// l'adresse et la rémunération — n'est jamais journalisé, y compris en cas
// d'échec : seule la cause technique l'est.
async fn handle_contrat_pdf(
    Json(req): Json<ContratReq>,
) -> Result<impl IntoResponse, ApiError> {
    let (pdf_base64, pages) = contrat_pdf::generer_base64(&req.contrat).map_err(|e| {
        tracing::error!("generer_contrat_pdf: {e}");
        ApiError("Le moteur PDF n'a pas pu composer le document".into())
    })?;
    Ok(Json(ReponsePdf { pdf_base64, pages }))
}

#[derive(Deserialize)]
struct ContratReq {
    contrat: ContratPdf,
}

// ── POST /api/generer_bulletin_pdf ────────────────────────────────────────────
// Même contrat que ci-dessus : le bulletin arrive composé — regroupement
// réglementaire, libellés et montants formatés —, le back n'en fait que la mise
// en page. Le corps de la requête porte un nom, une adresse et une rémunération :
// il n'est jamais journalisé, y compris en cas d'échec, où seule la cause
// technique l'est.
async fn handle_bulletin_pdf(
    Json(req): Json<BulletinPdfReq>,
) -> Result<impl IntoResponse, ApiError> {
    let (pdf_base64, pages) = bulletin_pdf::generer_base64(&req.bulletin).map_err(|e| {
        tracing::error!("generer_bulletin_pdf: {e}");
        ApiError("Le moteur PDF n'a pas pu composer le bulletin".into())
    })?;
    Ok(Json(ReponsePdf { pdf_base64, pages }))
}

#[derive(Deserialize)]
struct BulletinPdfReq {
    bulletin: BulletinPdf,
}
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
        .route("/api/generer_contrat_pdf", post(handle_contrat_pdf))
        .route("/api/generer_bulletin_pdf", post(handle_bulletin_pdf))
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
        // Ordre : le quota est évalué avant tout le reste (couche la plus
        // externe après la redirection HTTPS), pour que les requêtes refusées
        // ne touchent jamais un handler ni la base.
        .layer(middleware::from_fn(limite_par_ip))
        .layer(middleware::from_fn(https_redirect))
        .with_state(pool);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Xenna web → http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // `with_connect_info` : sans lui, `limite_par_ip` n'a aucune IP de repli
    // quand `x-forwarded-for` est absent (accès direct, hors proxy).
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
