use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResp {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct InscriptionAdmin {
    pub id:          i64,
    pub pseudo:      String,
    pub poste:       String,
    pub email_clair: String,
    pub status:      String,
    pub created_at:  String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct QuizzSuggestionAdmin {
    pub id:            i64,
    pub pays:          String,
    pub question:      String,
    pub reponse:       Option<String>,
    pub reps_alt:      Option<String>,
    pub mauvaises_rep: Option<String>,
    pub source:        Option<String>,
    pub pseudo:        Option<String>,
    pub created_at:    String,
    pub votes:         i64,
}

#[derive(Debug, Serialize)]
pub struct DashboardData {
    pub inscriptions_pending: Vec<InscriptionAdmin>,
    pub quizz_pending:        Vec<QuizzSuggestionAdmin>,
}
