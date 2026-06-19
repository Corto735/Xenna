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
    /// Salariés du secteur privé anglais (NI Class 1 + Income Tax PAYE).
    /// Données disponibles depuis le 06/04/2024 (année fiscale 2024/25).
    Angleterre,
    /// Salariés du secteur privé japonais (健康/介護/厚生/雇用/労災 + 所得税 + 住民税).
    /// Régime 協会けんぽ Tokyo. Hypothèse : salarié ≥ 40 ans.
    /// Données disponibles depuis le 01/04/2024.
    Japon,
    /// Salariés du secteur privé chinois, base Pékin (五险一金 + 个人所得税).
    /// Données disponibles depuis le 01/01/2024.
    Chine,
    /// Salariés du secteur privé néerlandais, sous l'âge AOW (régime général).
    /// Net = loonheffing (loonbelasting + premies volksverzekeringen AOW/Anw/Wlz)
    /// − heffingskortingen (algemene heffingskorting + arbeidskorting).
    /// Premies werknemersverzekeringen (AWf/Aof/Whk) et Zvw : 100 % patronales.
    /// Données disponibles depuis le 01/01/2026 (extension 2015-2025 en cours).
    PaysBas,
    /// Salariés du secteur privé australien, résidents fiscaux (PAYG + Medicare + Super).
    /// Données disponibles pour l'exercice 2025-26 (dates 2026). Devise AUD.
    Australie,
    /// Salariés néo-zélandais, résidents fiscaux (PAYE + ACC earner's levy + KiwiSaver).
    /// Données disponibles pour l'année fiscale 2025-26 (dates 2026). Devise NZD.
    NouvelleZelande,
    /// Salariés polonais (umowa o pracę) : ZUS + składka zdrowotna + PIT 12/32.
    /// Données disponibles pour 2025. Devise PLN.
    Pologne,
    /// Salariés sud-coréens : 4대보험 (NPS, NHI+장기요양, 고용, 산재) + 소득세 + 지방소득세.
    /// Données disponibles pour 2025. Devise KRW.
    CoreeDuSud,
    /// Salariés andorrans : CASS 6,5 % + IRPF (0 / 5 / 10 %, depuis 2015). Devise EUR.
    /// Données disponibles pour 2025.
    Andorre,
    /// Salariés monégasques : CAR (retraite) + chômage ; pas d'IR pour les résidents
    /// (sauf nationaux français). CCSS 100 % patronale. Devise EUR. Données : 2025.
    Monaco,
    /// Salariés danois : AM-bidrag 8 % + ATP + impôt (bund/kommune/topskat). Devise DKK.
    /// Données disponibles pour 2025.
    Danemark,
    /// Salariés finlandais (17-68 ans) : TyEL + chômage + assurance maladie
    /// + impôt d'État progressif + impôt communal moyen. Devise EUR. Données : 2026.
    Finlande,
    /// Salariés suédois : arbetsgivaravgifter 31,42 % (100 % patronales) + impôt
    /// communal moyen + impôt d'État 20 %. Devise SEK. Données : 2025.
    Suede,
    /// Salariés estoniens : sotsiaalmaks 33 % (pat) + chômage + 2ᵉ pilier 2 %
    /// + impôt 22 % (exonération de base dégressive). Devise EUR. Données : 2025.
    Estonie,
    /// Salariés lettons : VSAOI 10,5 % sal / 23,59 % pat + IIN 25,5 % / 33 %.
    /// Minimum non imposable 510 €/mois. Devise EUR. Données : 2025.
    Lettonie,
    /// Salariés lituaniens : Sodra 19,5 % sal + GPM 20 % / 32 % (NPD dégressif).
    /// Devise EUR. Données : 2025.
    Lituanie,
    /// Salariés autrichiens : Sozialversicherung 18,07 % sal (plafonnée) + Lohnsteuer
    /// progressif (0 à 55 %). Devise EUR. Données : 2025.
    Autriche,
    /// Salariés tchèques : sociální 7,1 % + zdravotní 4,5 % sal + daň 15 % / 23 %
    /// (sleva 2 570 CZK/mois). Devise CZK. Données : 2025.
    Tchequie,
    /// Salariés slovaques : santé 4 % + sociální 9,4 % sal + daň 19 % / 25 %
    /// (časť nezdaniteľná). Devise EUR. Données : 2025.
    Slovaquie,
    /// Salariés hongrois : TB 18,5 % sal + SZJA 15 % (flat) ; szocho 13 % pat.
    /// Devise HUF. Données : 2025.
    Hongrie,
    /// Salariés slovènes : cotisations 22,1 % sal / 16,1 % pat + dohodnina (16 à 50 %).
    /// Devise EUR. Données : 2025.
    Slovenie,
    /// Salariés grecs : EFKA 13,87 % sal (plafonné) + impôt progressif (9 à 44 %).
    /// Devise EUR. Données : 2025.
    Grece,
    /// Salariés chypriotes : assurance sociale + GESY 11,45 % sal + impôt (0 à 35 %).
    /// Devise EUR. Données : 2025.
    Chypre,
    /// Salariés maltais : SSC 10 % sal (plafonné) + impôt (0 à 35 %, barème single).
    /// Devise EUR. Données : 2025.
    Malte,
    /// Salariés croates : retraite 20 % sal + porez na dohodak (20 % / 30 %) ;
    /// santé 16,5 % pat. Devise EUR. Données : 2025.
    Croatie,
    /// Salariés irlandais : PRSI 4,1 % + USC + Income Tax 20 % / 40 % (crédits).
    /// Devise EUR. Données : 2025.
    Irlande,
    /// Salariés roumains : CAS 25 % + CASS 10 % sal + impôt 10 % ; CAM 2,25 % pat.
    /// Devise RON. Données : 2018-2025 (taux figés depuis OUG 79/2017).
    Roumanie,
    /// Salariés bulgares : cotisations 13,78 % sal (plafonnées) + impôt 10 %.
    /// Devise BGN (euro au 01/01/2026). Données : 2025.
    Bulgarie,
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
    /// Quotité de travail en % (100 = temps plein, 80 = 4/5, 50 = mi-temps…).
    /// Sert à proratiser le SMIC dans la formule Fillon (§670 BOSS / CSS art. L241-13).
    #[serde(default = "etp_default")]
    pub etp: f64,
    /// Vrai si l'employeur est une entreprise adaptée (EA) → aide au poste versée
    /// par l'État (ASP) au titre de l'emploi d'un travailleur handicapé (RQTH).
    /// France privé uniquement ; cumulable avec Alsace-Moselle, incompatible FPT.
    #[serde(default)]
    pub entreprise_adaptee: bool,
    /// Tranche d'âge pour l'aide au poste EA : "m50" | "50_55" | "56p". Défaut "m50".
    #[serde(default)]
    pub tranche_age_ea: Option<String>,
    /// Nombre d'heures supplémentaires du mois (temps plein). Majoration légale :
    /// 8 premières à +25 %, au-delà +50 % (seuil mensuel forfaitaire dans le simulateur).
    #[serde(default)]
    pub heures_supp: f64,
    /// Nombre d'heures complémentaires du mois (temps partiel). +10 % dans la limite
    /// du dixième des heures contractuelles, +25 % au-delà.
    #[serde(default)]
    pub heures_comp: f64,
    /// Salaire de base mensuel (hors primes/HS), sert à dériver le taux horaire
    /// = salaire_base / (151,67 × ETP/100). TEXT pour précision ; fallback salaire_brut si absent.
    #[serde(default)]
    pub salaire_base: Option<String>,
    /// Effectif de l'entreprise pour la déduction forfaitaire patronale HS :
    /// "moins20" | "20_249" | "250p". Défaut "moins20".
    #[serde(default)]
    pub effectif: Option<String>,
}

fn etp_default() -> f64 { 100.0 }

/// Spécification d'une absence maladie envoyée par le front (snake_case,
/// comme les champs de Salarie). Tous les champs hors dates ont un défaut
/// pour rester rétro-compatible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsenceInput {
    /// Type d'arrêt. Seule "maladie" (non professionnelle) est gérée pour l'instant.
    #[serde(default)]
    pub type_arret: String,
    /// Dates ISO YYYY-MM-DD.
    pub date_debut: String,
    pub date_fin:   String,
    /// "calendaire" | "moyens" | "heures" (+ anciens "ouvrables"/"ouvres").
    #[serde(default)]
    pub methode:    String,
    /// "ouvres" | "ouvrables" — pilote le diviseur pour "moyens"/"heures".
    #[serde(default)]
    pub jours_type: String,
    /// Heures/mois (méthode "heures"). Défaut 151,67 si absent.
    #[serde(default)]
    pub heures_mois: Option<f64>,
    /// Code IDCC de la convention collective. Défaut "0016" (transport).
    #[serde(default)]
    pub convention_idcc: Option<String>,
}

/// Résultat du calcul d'absence maladie (retenue + maintien employeur + IJSS),
/// renvoyé dans le Bulletin pour affichage. Voir crate::calculs::absence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsenceResult {
    #[serde(with = "rust_decimal::serde::str")] pub retenue:   Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub maintien:  Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub ijss_brut: Decimal,
    #[serde(with = "rust_decimal::serde::str")] pub ijss_net:  Decimal,
    pub jours_absence:  i64,
    pub jours_ijss:     i64,
    pub jours_maintien: i64,
    /// Ex. "maladie · ÷21,67 ouvrés".
    pub libelle:    String,
    /// Ex. "IDCC 0016".
    pub convention: String,
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
    /// Détail absence maladie (retenue, maintien, IJSS) si une absence est saisie.
    /// Absent du JSON sinon. France uniquement pour l'instant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub absence: Option<AbsenceResult>,
    /// Détail heures supplémentaires/complémentaires (gains majorés + exonérations)
    /// si des heures sont saisies. Absent du JSON sinon. France uniquement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heures_sup: Option<HeuresSupResult>,
}

/// Résultat du calcul des heures supplémentaires/complémentaires (gains majorés
/// par tranche + exonérations), renvoyé dans le Bulletin pour affichage.
/// Voir crate::calculs::heures_sup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeuresSupResult {
    #[serde(with = "rust_decimal::serde::str")] pub taux_horaire: Decimal,
    /// Heures supp à +25 % (≤ 8) et leur gain brut.
    pub h_supp_25: f64,
    pub h_supp_50: f64,
    #[serde(with = "rust_decimal::serde::str")] pub gain_hs: Decimal,
    /// Heures complémentaires à +10 % (≤ 1/10 contractuel) et à +25 %.
    pub h_comp_10: f64,
    pub h_comp_25: f64,
    #[serde(with = "rust_decimal::serde::str")] pub gain_hc: Decimal,
    /// Réduction de cotisations salariales (montant positif retranché du net salarial).
    #[serde(with = "rust_decimal::serde::str")] pub reduction_salariale: Decimal,
    /// Déduction forfaitaire patronale (montant positif retranché du coût employeur).
    #[serde(with = "rust_decimal::serde::str")] pub deduction_patronale: Decimal,
    /// Net imposable exonéré d'impôt sur le revenu au titre des HS/HC.
    #[serde(with = "rust_decimal::serde::str")] pub exo_fiscale: Decimal,
    /// Plafond annuel d'exonération applicable à la date (info).
    #[serde(with = "rust_decimal::serde::str")] pub exo_plafond: Decimal,
}
