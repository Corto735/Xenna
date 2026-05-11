use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Statut {
    Cadre,
    NonCadre,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Pays {
    #[default]
    France,
    Suisse,
    Luxembourg,
    /// Agents titulaires de la Fonction Publique Territoriale (FPT).
    /// Données historiques disponibles depuis le 01/01/2016.
    FonctionPublique,
    /// Salariés du secteur privé italien (INPS + INAIL + IRPEF).
    /// Données disponibles depuis le 01/01/2015.
    Italia,
    /// Salariés canadiens hors Québec (RPC, AE, impôt fédéral + Ontario par défaut).
    /// Données disponibles depuis le 01/01/2019.
    Canada,
    /// Salariés québécois (RRQ remplace RPC, AE réduit, RQAP, FSS, impôt fédéral + QC).
    /// Données disponibles depuis le 01/01/2019.
    Quebec,
    /// Salariés du secteur privé allemand (SGB V/VI/III/XI/VII + Lohnsteuer + Kirchensteuer).
    /// Données disponibles depuis le 01/01/2015.
    Allemagne,
    /// Salariés du secteur privé espagnol (régime général, contrato indefinido).
    /// Cotisations : CC, Desempleo, FOGASA, FP, MEI (depuis 2023).
    /// Données disponibles depuis le 01/01/2015.
    Espagne,
    /// Salariés du secteur privé portugais (regime geral de segurança social).
    /// Cotisations : SS (TSU), Acidentes de Trabalho, FCT, FGCT, IRS retenção.
    /// Données disponibles depuis le 01/01/2015.
    Portugal,
    /// Salariés du secteur privé belge (régime général ONSS).
    /// Cotisations : ONSS sal/pat, bonus emploi, réd. structurelle, PP/BV (régional).
    /// region_be : "wallonie" | "flandre" | "bruxelles".
    /// Données disponibles depuis le 01/01/2015.
    Belgique,
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
    #[serde(default)]
    pub pays: Pays,
    /// Code canton suisse à 2 lettres (ex. "GE", "ZH") — utilisé pour l'IS.
    #[serde(default)]
    pub canton: Option<String>,
    /// Code tarif IS ORIS (A0, A1, A2, B0, B1, B2, C0, C1, C2, H0, H1, H2).
    #[serde(default)]
    pub tarif_is: Option<String>,
    /// Vrai si le salarié est soumis à la retenue à la source (IS) suisse.
    #[serde(default)]
    pub assujetti_is: bool,
    /// Code région italienne à 2 lettres (ex. "LO" Lombardia, "LA" Lazio).
    /// Utilisé pour le calcul de l'addizionale regionale IRPEF.
    #[serde(default)]
    pub regione: Option<String>,
    /// Vrai pour les contrats à durée déterminée italiens (contratto a tempo determinato).
    /// Déclenche la majoration NASpI CDD (+1,40 % patronal).
    #[serde(default)]
    pub contratto_termine: bool,
    /// Code province canadienne à 2 lettres (ex. "AB", "BC", "ON").
    /// Détermine l'impôt provincial pour Pays::Canada.
    /// Non utilisé pour Pays::Quebec (régime distinct).
    #[serde(default)]
    pub province: Option<String>,
    /// Steuerklasse allemande (1-6). Défaut : 1 (célibataire).
    #[serde(default)]
    pub steuerklasse: Option<u8>,
    /// Vrai si le salarié est sans enfant (>23 ans) → Kinderlosenzuschlag PV actif.
    #[serde(default)]
    pub kinderlos: Option<bool>,
    /// Code Land allemand à 2 lettres (ex. "BY", "BW", "NW"). Kirchensteuer.
    #[serde(default)]
    pub land: Option<String>,
    /// Vrai si membre d'une église → Kirchensteuer prélevée.
    #[serde(default)]
    pub kirchenmitglied: Option<bool>,
    /// Région belge pour le précompte professionnel (PP/BV).
    /// Valeurs : "wallonie", "flandre", "bruxelles" (défaut : "bruxelles").
    #[serde(default)]
    pub region_be: Option<String>,
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
    /// "EUR" pour la France, "CHF" pour la Suisse.
    pub devise: String,
}
