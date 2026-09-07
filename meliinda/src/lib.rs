pub mod db;
pub mod models;
pub mod routes;

use std::sync::Arc;
use axum::{routing::{get, post}, Router};
use sqlx::SqlitePool;

/// Initialise le schéma Meliinda et retourne le router Axum à merger.
/// Le caller doit appeler `.with_state(pool)` sur le routeur principal.
///
/// # Usage dans Xenna
/// ```rust
/// let meliinda = meliinda::meliinda_router(pool.clone()).await.unwrap();
/// let app = Router::new()
///     .merge(meliinda)
///     // …autres routes…
///     .with_state(pool);
/// ```
pub async fn meliinda_router(pool: Arc<SqlitePool>) -> Result<Router<Arc<SqlitePool>>, sqlx::Error> {
    db::run_migrations(&pool).await?;

    // NB : `routes::delete_sequence` n'est délibérément PAS routée ici. Ce
    // routeur ne porte aucune authentification ; exposer la suppression
    // laissait n'importe qui vider la table. Au caller de la brancher derrière
    // sa propre auth (côté Xenna : espace admin).
    let router = Router::new()
        .route("/api/meliinda/record",        post(routes::record))
        .route("/api/meliinda/sequences",     get(routes::list_sequences))
        .route("/api/meliinda/sequence/{id}", get(routes::get_sequence));

    Ok(router)
}
