use serde::{Deserialize, Serialize};

// ── Niveau de maîtrise (partagé CCN et pays) ─────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CcnLevel {
    Connue,
    #[serde(rename = "Pratiquée")]
    Pratiquee,
    #[serde(rename = "Maîtrisée")]
    Maitrisee,
}

impl CcnLevel {
    pub fn as_db_str(&self) -> &'static str {
        match self {
            CcnLevel::Connue    => "Connue",
            CcnLevel::Pratiquee => "Pratiquée",
            CcnLevel::Maitrisee => "Maîtrisée",
        }
    }

    pub fn ordre(&self) -> u8 {
        match self {
            CcnLevel::Maitrisee => 1,
            CcnLevel::Pratiquee => 2,
            CcnLevel::Connue    => 3,
        }
    }
}

impl TryFrom<&str> for CcnLevel {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Connue"     => Ok(CcnLevel::Connue),
            "Pratiquée"  => Ok(CcnLevel::Pratiquee),
            "Maîtrisée"  => Ok(CcnLevel::Maitrisee),
            other        => Err(format!("Niveau inconnu : '{other}'")),
        }
    }
}

// ── Expertise CCN ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CcnExpertise {
    pub id:          i64,
    pub profile_id:  i64,
    pub ccn_idcc:    String,
    pub ccn_libelle: String,
    pub niveau:      String,
}

// ── Expertise pays (paie internationale) ──────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PaysExpertise {
    pub id:           i64,
    pub profile_id:   i64,
    pub pays_code:    String,
    pub pays_libelle: String,
    pub niveau:       String,
}

// ── Profil contributeur ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContributorProfile {
    pub id:               i64,
    pub user_id:          i64,
    pub pseudo:           String,
    pub linkedin_url:     Option<String>,
    pub poste:            String,
    pub poste_est_actuel: bool,
    pub paie_fr_niveau:   Option<String>,
    pub posts_count:      i64,
    pub topics_count:     i64,
    pub votes_received:   i64,
    pub votes_given:      i64,
    pub created_at:       String,
}

// ── Vue agrégée ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilComplet {
    #[serde(flatten)]
    pub profil:     ContributorProfile,
    pub expertises: Vec<CcnExpertise>,
    pub pays:       Vec<PaysExpertise>,
}

// ── DTOs de création (POST /forge/profil) ─────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CcnExpertiseInput {
    pub ccn_idcc:    String,
    pub ccn_libelle: String,
    pub niveau:      String,
}

#[derive(Debug, Deserialize)]
pub struct PaysExpertiseInput {
    pub pays_code:    String,
    pub pays_libelle: String,
    pub niveau:       String,
}

#[derive(Debug, Deserialize)]
pub struct CreerProfilReq {
    pub email:            String,
    pub pseudo:           String,
    pub poste:            String,
    pub linkedin_url:     Option<String>,
    #[serde(default = "default_actuel")]
    pub poste_est_actuel: bool,
    pub paie_fr_niveau:   Option<String>,
    #[serde(default)]
    pub expertises:       Vec<CcnExpertiseInput>,
    #[serde(default)]
    pub pays:             Vec<PaysExpertiseInput>,
}

fn default_actuel() -> bool { true }
