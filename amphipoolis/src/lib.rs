//! Amphipoolis — espace de discussion sous pseudonyme, modéré message par message.
//!
//! Prototype. Deux promesses, qui structurent tout le module :
//!
//! 1. **Le pseudonyme est strictement respecté.** On entre avec un pseudonyme
//!    et une phrase secrète ; personne d'autre ne peut parler sous ce nom, et
//!    rien ne relie le pseudonyme à une personne (ni e-mail, ni IP en base).
//! 2. **Seuls les messages sont filtrés.** Chaque sujet et chaque message
//!    attend qu'un modérateur le publie ou le refuse (motif court, visible du
//!    seul auteur). Un pseudonyme discourtois peut être refusé : son titulaire
//!    en choisit un autre, l'ancien nom reste réservé à jamais.
//!
//! Le rôle (participant ou modérateur) se choisit à la création du pseudonyme,
//! un de chaque par adresse et par jour. Un modérateur ne valide jamais ce qu'il
//! a lui-même écrit ; un seul suffit, le premier qui tranche l'emporte.
//!
//! Rien n'est éternel : un texte publié vit 7 jours, chaque +1 d'un autre
//! pseudonyme les rallonge de 7, dans la limite de 30 jours après publication.

pub mod admin;
pub mod db;
pub mod routes;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::SqlitePool;

pub use routes::{Captcha, IpClient};

/// Initialise le schéma et retourne le routeur à merger dans le serveur web.
///
/// `captcha` vérifie la preuve de travail ALTCHA envoyée à la création d'un
/// pseudonyme : la vérification vit côté Xenna (secret, anti-rejeu), la crate
/// ne la connaît que par cette fonction.
///
/// Le caller doit appeler `.with_state(pool)` sur le routeur principal.
pub async fn amphipoolis_router(
    pool: Arc<SqlitePool>,
    captcha: Captcha,
) -> Result<Router<Arc<SqlitePool>>, sqlx::Error> {
    db::run_migrations(&pool).await?;

    // Ménage : au démarrage, puis toutes les 10 minutes.
    let p = pool.clone();
    tokio::spawn(async move {
        let mut t = tokio::time::interval(std::time::Duration::from_secs(600));
        loop {
            t.tick().await;
            if let Err(e) = db::purger(&p).await {
                eprintln!("[amphipoolis] purge : {e}");
            }
        }
    });

    let r = Router::new()
        .route("/api/amphipoolis/creer",                post(routes::creer))
        .route("/api/amphipoolis/entrer",               post(routes::entrer))
        .route("/api/amphipoolis/sortir",               post(routes::sortir))
        .route("/api/amphipoolis/moi",                  get(routes::moi))
        .route("/api/amphipoolis/renommer",             post(routes::renommer))
        .route("/api/amphipoolis/sujets",               get(routes::liste_sujets).post(routes::creer_sujet))
        .route("/api/amphipoolis/sujets/{id}",          get(routes::fil))
        .route("/api/amphipoolis/sujets/{id}/messages", post(routes::ecrire))
        .route("/api/amphipoolis/moderation",           get(routes::file_moderation))
        .route("/api/amphipoolis/moderation/decision",  post(routes::decider))
        .route("/api/amphipoolis/moderation/pseudo",    post(routes::refuser_pseudo))
        .route("/api/amphipoolis/plus1",                post(routes::plus1))
        .layer(Extension(captcha));

    Ok(r)
}
