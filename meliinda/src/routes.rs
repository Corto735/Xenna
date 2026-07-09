use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{RecordRequest, RecordResponse, Sequence};

pub type Db = Arc<SqlitePool>;

pub struct ApiError(String);

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

/// POST /api/meliinda/record — enregistre une séquence de frappes.
pub async fn record(
    State(pool): State<Db>,
    Json(req): Json<RecordRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Bornes anti-abus : l'endpoint est public, sans elles n'importe qui
    // remplit la base avec des séquences arbitraires.
    if req.events.is_empty() || req.events.len() > 10_000 {
        return Err(ApiError("Séquence invalide : 1 à 10 000 événements".into()));
    }
    if req.label.as_deref().is_some_and(|l| l.len() > 120) {
        return Err(ApiError("Label : 120 caractères maximum".into()));
    }

    let id = Uuid::new_v4();
    let events_json = serde_json::to_string(&req.events)
        .map_err(|e| ApiError(e.to_string()))?;
    let now = Utc::now().to_rfc3339();
    let id_str = id.to_string();

    sqlx::query(
        "INSERT INTO meliinda_sequences (id, label, events, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&id_str)
    .bind(&req.label)
    .bind(&events_json)
    .bind(&now)
    .execute(pool.as_ref())
    .await
    .map_err(|e| ApiError(e.to_string()))?;

    Ok(Json(RecordResponse { id }))
}

/// GET /api/meliinda/sequence/:id — récupère une séquence pour replay.
pub async fn get_sequence(
    State(pool): State<Db>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let row = sqlx::query_as::<_, (String, Option<String>, String, String)>(
        "SELECT id, label, events, created_at FROM meliinda_sequences WHERE id = ?",
    )
    .bind(&id)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|e| ApiError(e.to_string()))?
    .ok_or_else(|| ApiError("Séquence introuvable".into()))?;

    let events = serde_json::from_str(&row.2).map_err(|e| ApiError(e.to_string()))?;
    let created_at = chrono::DateTime::parse_from_rfc3339(&row.3)
        .map_err(|e| ApiError(e.to_string()))?
        .with_timezone(&Utc);

    let seq = Sequence {
        id:         row.0.parse().map_err(|e: uuid::Error| ApiError(e.to_string()))?,
        label:      row.1,
        events,
        created_at,
    };

    Ok(Json(seq))
}

/// DELETE /api/meliinda/sequence/:id — supprime une séquence.
pub async fn delete_sequence(
    State(pool): State<Db>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let result = sqlx::query("DELETE FROM meliinda_sequences WHERE id = ?")
        .bind(&id)
        .execute(pool.as_ref())
        .await
        .map_err(|e| ApiError(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApiError("Séquence introuvable".into()));
    }
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/meliinda/sequences — liste toutes les séquences (sans les events).
pub async fn list_sequences(
    State(pool): State<Db>,
) -> Result<impl IntoResponse, ApiError> {
    let rows = sqlx::query_as::<_, (String, Option<String>, String)>(
        "SELECT id, label, created_at FROM meliinda_sequences ORDER BY created_at DESC",
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| ApiError(e.to_string()))?;

    let list: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|(id, label, created_at)| {
            serde_json::json!({ "id": id, "label": label, "created_at": created_at })
        })
        .collect();

    Ok(Json(list))
}
