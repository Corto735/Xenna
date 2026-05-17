use serde::{Deserialize, Serialize};
use crate::forge::models::{CcnExpertise, CcnExpertiseInput, PaysExpertise, PaysExpertiseInput};

#[derive(Debug, Deserialize)]
pub struct MemberLoginReq {
    pub email:    String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct MemberLoginResp {
    pub token:  String,
    pub pseudo: String,
}

#[derive(Debug, Serialize)]
pub struct MemberProfileData {
    pub id:               i64,
    pub pseudo:           String,
    pub poste:            String,
    pub poste_est_actuel: bool,
    pub linkedin_url:     Option<String>,
    pub paie_fr_niveau:   Option<String>,
    pub posts_count:      i64,
    pub topics_count:     i64,
    pub votes_received:   i64,
    pub expertises:       Vec<CcnExpertise>,
    pub pays:             Vec<PaysExpertise>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileReq {
    pub poste:            String,
    pub poste_est_actuel: bool,
    pub linkedin_url:     Option<String>,
    pub paie_fr_niveau:   Option<String>,
    #[serde(default)]
    pub expertises:       Vec<CcnExpertiseInput>,
    #[serde(default)]
    pub pays:             Vec<PaysExpertiseInput>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TopicResume {
    pub id:            i64,
    pub titre:         String,
    pub author_pseudo: String,
    pub created_at:    String,
    pub reply_count:   i64,
    pub votes:         i64,
}

#[derive(Debug, Serialize)]
pub struct TopicDetail {
    pub id:            i64,
    pub titre:         String,
    pub contenu:       String,
    pub author_pseudo: String,
    pub created_at:    String,
    pub reply_count:   i64,
    pub votes:         i64,
    pub replies:       Vec<ReplyDto>,
    pub user_voted:    bool,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ReplyDto {
    pub id:            i64,
    pub contenu:       String,
    pub author_pseudo: String,
    pub created_at:    String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTopicReq {
    pub titre:   String,
    pub contenu: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReplyReq {
    pub contenu: String,
}
