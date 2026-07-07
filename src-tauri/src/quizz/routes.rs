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

const PAYS_VALIDES: &[&str] = &["fr", "fpt", "ch", "lu", "it", "ca", "qc", "frh"];

// ── Sanitisation ──────────────────────────────────────────────────────────────
// Seul du texte brut est accepté : on retire les balises HTML (<>),
// les caractères de contrôle (sauf espaces courants) et on trime.

fn sanitiser(s: &str) -> String {
    s.chars()
        .filter(|c| match c {
            '<' | '>' => false,
            c => !c.is_control() || matches!(c, '\n' | '\r' | '\t' | ' '),
        })
        .collect::<String>()
        .trim()
        .to_string()
}

fn sanitiser_opt(s: Option<&str>) -> Option<String> {
    s.map(sanitiser).filter(|s| !s.is_empty())
}

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
    pays:         String,
    question:     String,
    reponse:      Option<String>,
    reps_alt:     Option<String>,
    mauvaises_rep: Option<String>,
    source:       Option<String>,
    pseudo:       Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoteReq {
    voter_id: String,
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

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
struct SuggestionPublique {
    id:       i64,
    question: String,
    votes:    i64,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn soumettre_score(
    State(pool): State<Db>,
    Json(req): Json<ScoreReq>,
) -> Result<StatusCode, QuizzError> {
    let pseudo = sanitiser(&req.pseudo);
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
        .bind(&pseudo)
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
    let question = sanitiser(&req.question);
    if question.is_empty() || question.len() > 500 {
        return Err(QuizzError::Validation("Question : 1 à 500 caractères".into()));
    }
    let reponse      = sanitiser_opt(req.reponse.as_deref());
    let reps_alt     = sanitiser_opt(req.reps_alt.as_deref());
    let mauvaises    = sanitiser_opt(req.mauvaises_rep.as_deref());
    let source       = sanitiser_opt(req.source.as_deref());
    let pseudo       = sanitiser_opt(req.pseudo.as_deref());

    if let Some(ref p) = pseudo {
        if p.len() > 50 {
            return Err(QuizzError::Validation("Pseudo : 50 caractères maximum".into()));
        }
    }

    sqlx::query(
        "INSERT INTO quizz_suggestions (pays, question, reponse, reps_alt, mauvaises_rep, source, pseudo)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&req.pays)
    .bind(&question)
    .bind(reponse.as_deref())
    .bind(reps_alt.as_deref())
    .bind(mauvaises.as_deref())
    .bind(source.as_deref())
    .bind(pseudo.as_deref())
    .execute(&*pool)
    .await?;

    Ok(StatusCode::CREATED)
}

async fn lister_suggestions(
    State(pool): State<Db>,
    Path(pays): Path<String>,
) -> Result<Json<Vec<SuggestionPublique>>, QuizzError> {
    if !PAYS_VALIDES.contains(&pays.as_str()) {
        return Err(QuizzError::Validation(format!("Pays invalide : '{pays}'")));
    }

    let rows = sqlx::query_as::<_, SuggestionPublique>(
        "SELECT id, question, COALESCE(votes, 0) AS votes
         FROM quizz_suggestions
         WHERE pays = ?
         ORDER BY COALESCE(votes, 0) DESC, created_at ASC
         LIMIT 15",
    )
    .bind(&pays)
    .fetch_all(&*pool)
    .await?;

    Ok(Json(rows))
}

async fn voter(
    State(pool): State<Db>,
    Path(id): Path<i64>,
    Json(req): Json<VoteReq>,
) -> Result<Json<i64>, QuizzError> {
    let voter_id = sanitiser(&req.voter_id);
    if voter_id.is_empty() || voter_id.len() > 64 {
        return Err(QuizzError::Validation("voter_id invalide".into()));
    }

    // Vérifier que la suggestion existe
    let existe: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM quizz_suggestions WHERE id = ?)")
        .bind(id)
        .fetch_one(&*pool)
        .await?;
    if !existe {
        return Err(QuizzError::Validation("Suggestion introuvable".into()));
    }

    // Insérer le vote — la PRIMARY KEY (suggestion_id, voter_id) empêche les doublons
    let result = sqlx::query(
        "INSERT OR IGNORE INTO quizz_suggestion_votes (suggestion_id, voter_id) VALUES (?, ?)",
    )
    .bind(id)
    .bind(&voter_id)
    .execute(&*pool)
    .await?;

    // Incrémenter le compteur seulement si le vote n'existait pas déjà
    if result.rows_affected() > 0 {
        sqlx::query("UPDATE quizz_suggestions SET votes = COALESCE(votes, 0) + 1 WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await?;
    }

    let votes: i64 = sqlx::query_scalar("SELECT COALESCE(votes, 0) FROM quizz_suggestions WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await?;

    Ok(Json(votes))
}

// ── Router ────────────────────────────────────────────────────────────────────

pub fn quizz_router() -> Router<Db> {
    Router::new()
        .route("/quizz/score",               post(soumettre_score))
        .route("/quizz/leaderboard/{pays}",  get(leaderboard))
        .route("/quizz/suggestion",          post(soumettre_suggestion))
        .route("/quizz/suggestions/{pays}",  get(lister_suggestions))
        .route("/quizz/vote/{id}",           post(voter))
}
