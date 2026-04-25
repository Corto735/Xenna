use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Statut {
    Cadre,
    NonCadre,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Salarie {
    pub nom:          String,
    pub prenom:       String,
    #[serde(with = "rust_decimal::serde::str")]
    pub salaire_brut: Decimal,
    pub statut:       Statut,
    #[serde(default)]
    pub alsace_moselle: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LigneCotisation {
    pub code:        String,
    pub libelle:     String,
    #[serde(with = "rust_decimal::serde::str")]
    pub base:        Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub taux_sal:    Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub montant_sal: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub taux_pat:    Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub montant_pat: Decimal,
    pub explication: String,
    pub loi_ref:     Option<String>,
    pub categorie:   String,
}

/// Une ligne dans la simulation annuelle (un mois).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LigneMensuelle {
    pub mois:         u32,
    pub mois_libelle: String,
    #[serde(with = "rust_decimal::serde::str")] pub brut:             Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub smic:             Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub pmss:             Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub total_sal:        Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub total_pat_brut:   Decimal,
    /// Fillon calculé mois par mois sans régularisation (formule mensuelle simple).
    #[serde(with = "rust_decimal::serde::str")] pub fillon_simple:    Decimal,
    /// Fillon après régularisation annuelle cumulée.
    #[serde(with = "rust_decimal::serde::str")] pub fillon_regularise: Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub net_a_payer:      Decimal,
    /// Coût employeur avec Fillon régularisé.
    #[serde(with = "rust_decimal::serde::str")] pub cout_employeur:   Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationAnnuelle {
    pub annee:  i32,
    pub lignes: Vec<LigneMensuelle>,
    #[serde(with = "rust_decimal::serde::str")] pub total_brut:     Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub total_fillon:   Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub total_net:      Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub total_cout:     Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub total_sal:      Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub total_pat_brut: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bulletin {
    pub salarie:       Salarie,
    pub cotisations:   Vec<LigneCotisation>,
    #[serde(with = "rust_decimal::serde::str")]
    pub brut:          Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub net_imposable: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub net_a_payer:   Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub cout_total_employeur: Decimal,
}
