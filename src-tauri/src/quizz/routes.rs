use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

type Db = Arc<SqlitePool>;

const PAYS_VALIDES: &[&str] = &["fr", "fpt", "ch", "lu", "it", "ca", "qc"];

// ── Error ─────────────────────────────────────────────────────────────────────

enum QuizzError {
    Validation(String),
    Db(sqlx::Error),
}

impl IntoResponse for QuizzError {
    fn into_response(self) -> axum::response::Response {
        match self {
            QuizzError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response(),
            QuizzError::Db(e) => {
                tracing::error!("Quizz DB error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne du serveur").into_response()
            }
        }
    }
}

impl From<sqlx::Error> for QuizzError {
    fn from(e: sqlx::Error) -> Self {
        QuizzError::Db(e)
    }
}

// ── DTOs ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScoreReq {
    pseudo: String,
    pays:   String,
    score:  i64,
    total:  i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SuggestionReq {
    pays:     String,
    question: String,
    reponse:  Option<String>,
    source:   Option<String>,
    pseudo:   Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
struct EntreeLeaderboard {
    rang:   i64,
    pseudo: String,
    score:  i64,
    total:  i64,
    pct:    f64,
    date:   String,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn soumettre_score(
    State(pool): State<Db>,
    Json(req): Json<ScoreReq>,
) -> Result<StatusCode, QuizzError> {
    let pseudo = req.pseudo.trim();
    if pseudo.is_empty() || pseudo.len() > 50 {
        return Err(QuizzError::Validation("Pseudo : 1 à 50 caractères".into()));
    }
    if !PAYS_VALIDES.contains(&req.pays.as_str()) {
        return Err(QuizzError::Validation(format!("Pays invalide : '{}'", req.pays)));
    }
    if req.total < 5 {
        return Err(QuizzError::Validation("Minimum 5 questions requises".into()));
    }
    if req.score < 0 || req.score > req.total {
        return Err(QuizzError::Validation("Score invalide".into()));
    }

    sqlx::query("INSERT INTO quizz_scores (pseudo, pays, score, total) VALUES (?, ?, ?, ?)")
        .bind(pseudo)
        .bind(&req.pays)
        .bind(req.score)
        .bind(req.total)
        .execute(&*pool)
        .await?;

    Ok(StatusCode::CREATED)
}

async fn leaderboard(
    State(pool): State<Db>,
    Path(pays): Path<String>,
) -> Result<Json<Vec<EntreeLeaderboard>>, QuizzError> {
    if !PAYS_VALIDES.contains(&pays.as_str()) {
        return Err(QuizzError::Validation(format!("Pays invalide : '{pays}'")));
    }

    let rows = sqlx::query_as::<_, EntreeLeaderboard>(
        "SELECT
             ROW_NUMBER() OVER (ORDER BY score*1.0/total DESC, created_at ASC) AS rang,
             pseudo,
             score,
             total,
             ROUND(score*100.0/total, 1) AS pct,
             DATE(created_at) AS date
         FROM quizz_scores
         WHERE pays = ?
         ORDER BY score*1.0/total DESC, created_at ASC
         LIMIT 10",
    )
    .bind(&pays)
    .fetch_all(&*pool)
    .await?;

    Ok(Json(rows))
}

async fn soumettre_suggestion(
    State(pool): State<Db>,
    Json(req): Json<SuggestionReq>,
) -> Result<StatusCode, QuizzError> {
    if !PAYS_VALIDES.contains(&req.pays.as_str()) {
        return Err(QuizzError::Validation(format!("Pays invalide : '{}'", req.pays)));
    }
    let question = req.question.trim();
    if question.is_empty() || question.len() > 500 {
        return Err(QuizzError::Validation("Question : 1 à 500 caractères".into()));
    }
    if let Some(ref p) = req.pseudo {
        if p.len() > 50 {
            return Err(QuizzError::Validation("Pseudo : 50 caractères maximum".into()));
        }
    }

    sqlx::query(
        "INSERT INTO quizz_suggestions (pays, question, reponse, source, pseudo)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&req.pays)
    .bind(question)
    .bind(req.reponse.as_deref())
    .bind(req.source.as_deref())
    .bind(req.pseudo.as_deref())
    .execute(&*pool)
    .await?;

    Ok(StatusCode::CREATED)
}

// ── Router ────────────────────────────────────────────────────────────────────

pub fn quizz_router() -> Router<Db> {
    Router::new()
        .route("/quizz/score", post(soumettre_score))
        .route("/quizz/leaderboard/{pays}", get(leaderboard))
        .route("/quizz/suggestion", post(soumettre_suggestion))
}
