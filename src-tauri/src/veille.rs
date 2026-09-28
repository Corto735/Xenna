// Veille réglementaire : fraîcheur des barèmes, pays par pays.
//
// Un simulateur qui refuse d'inventer un chiffre ne doit pas non plus faire
// passer un chiffre périmé pour un chiffre à jour. Or une date de paie au-delà
// des barèmes intégrés ne déclenche aucune erreur : chaque calculateur retombe
// sur sa dernière branche (`_ =>`, `annee >= …`) ou sur la dernière période en
// base (`date_fin NULL`). Ce module DÉCLARE ce qui est réellement dans le code,
// pour que l'interface puisse le dire.
//
// Ce n'est PAS un calcul et rien ici n'est déduit automatiquement : une date de
// dernière modification en base ne prouve pas qu'un taux stable ait été
// revérifié, et les barèmes d'impôt vivent en Rust, hors de toute table. La
// vérité est donc un relevé, daté par `AUDIT_DU`, à refaire à chaque intégration
// de barèmes : quand un pays reçoit ses valeurs d'une nouvelle année, on relève
// `integre_jusqu_a` et on retire les lacunes comblées.
//
// Le `match` de `veille()` est exhaustif : un pays ajouté à l'enum sans sa
// déclaration de fraîcheur casse la compilation.

use serde::Serialize;

use crate::models::Pays;

/// Date du relevé (code et base relus pays par pays).
pub const AUDIT_DU: &str = "2026-09-25";

#[derive(Debug, Clone, Serialize)]
pub struct Veille {
    /// Dernière année civile dont TOUS les barèmes (taux, plafonds, impôt) sont
    /// intégrés. Au-delà, le calculateur prolonge les valeurs antérieures.
    pub integre_jusqu_a: i32,
    /// Ce qui manque au-delà de `integre_jusqu_a`, daté dans le texte même
    /// (une lacune peut ne porter que sur une partie de l'année).
    pub lacunes: &'static [&'static str],
    pub audit_du: &'static str,
    /// Date de la dernière entrée du `JOURNAL` pour ce régime, s'il y en a une.
    pub derniere_maj: Option<&'static str>,
}

/// Une mise à jour de barèmes, datée et sourcée : ce que le visiteur lit pour
/// juger de la fraîcheur d'un régime. On n'y inscrit que ce qui a réellement
/// été intégré au code ou à la base, avec la source consultée.
#[derive(Debug, Clone, Serialize)]
pub struct MiseAJour {
    /// Date de l'intégration (AAAA-MM-JJ).
    pub date: &'static str,
    pub pays: Pays,
    /// Ce qui a changé, valeurs et date d'effet comprises.
    pub objet: &'static str,
    /// Source(s) officielle(s) ou, à défaut, presse spécialisée concordante.
    pub sources: &'static [&'static str],
}

/// Journal des mises à jour, du plus récent au plus ancien. Qui intègre des
/// barèmes y ajoute une ligne dans le même commit (voir `CLAUDE.md`).
pub const JOURNAL: &[MiseAJour] = &[
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Irlande,
        objet: "PRSI Class A : 4,2 % / 11,25 % dès le 01/10/2025 et 4,35 % / 11,40 % dès le 01/10/2026 (au lieu de 4,2 % / 11,15 % toute l'année) ; taux employeur réduit sous 552 €/semaine et crédit PRSI salarié désormais appliqués",
        sources: &[
            "https://assets.gov.ie/static/documents/cb168977/PRSI_C20260116_Contribution_Rates_and_User_Guide_-_SW_14_-_English_Version_-_January_2026_.pdf-web.pdf",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::NouvelleZelande,
        objet: "Exercice 2026-27 (dès le 01/04/2026) : ACC earner's levy 1,75 % plafonné à 156 641 $ ; KiwiSaver employeur 3,5 % (4 % au 01/04/2028) ; barème PAYE inchangé ; calcul désormais par exercice (avril-mars)",
        sources: &[
            "https://www.ird.govt.nz/kiwisaver-changes",
            "https://www.ird.govt.nz/updates/news-folder/2026/changes-to-the-kiwisaver-contribution-rate",
            "https://nztax.tools/tax-insights/acc-earner-levy-2026-27/",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Angleterre,
        objet: "Exercices 2025/26 et 2026/27 : NI employeur 15 % au-delà d'un Secondary Threshold de £5 000 (au lieu de 13,8 % / £9 100) ; seuils salariaux et d'impôt gelés ; calcul désormais par exercice fiscal (6 avril)",
        sources: &[
            "https://www.gov.uk/guidance/rates-and-thresholds-for-employers-2026-to-2027",
            "https://www.gov.uk/guidance/rates-and-thresholds-for-employers-2025-to-2026",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Italia,
        objet: "IRPEF 2026 : 2ᵉ tranche (28 000-50 000 €) ramenée de 35 à 33 % au 01/01/2026 — L. 199/2025 (Bilancio 2026)",
        sources: &[
            "https://www.mef.gov.it/focus/Principali-misure-della-legge-di-bilancio-2026/",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::FonctionPublique,
        objet: "CNRACL : taux employeur 34,65 % (2025), 37,65 % (2026), 40,65 % (2027), 43,65 % (2028) — décret n° 2025-86 ; part agent inchangée (11,10 %)",
        sources: &["https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000051070354"],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::France,
        objet: "AGS : historique corrigé 2015-2026 — 0,30 % (2015), 0,25 % (2016), 0,20 % (1er sem. 2017), 0,15 % (07/2017-2023), 0,20 % (1er sem. 2024), 0,25 % depuis le 01/07/2024, maintenu en 2026",
        sources: &[
            "https://entreprendre.service-public.gouv.fr/actualites/A17906",
            "https://www.legisocial.fr/actualites-sociales/1751-la-cotisation-ags-passe-025-au-1er-janvier-2016.html",
            "https://www.legisocial.fr/actualites-sociales/2262-diminution-de-la-cotisation-ags-qui-passe-015-au-1er-juillet-2017.html",
        ],
    },
];

/// Le journal complet (tableau « À propos » du front).
pub fn journal() -> &'static [MiseAJour] {
    JOURNAL
}

/// Veille d'un régime, étiquetée de son pays (tableau récapitulatif du front).
#[derive(Debug, Clone, Serialize)]
pub struct VeillePays {
    pub pays: Pays,
    #[serde(flatten)]
    pub veille: Veille,
}

/// La veille de tous les régimes, dans l'ordre de `Pays::TOUS`.
pub fn veille_tous() -> Vec<VeillePays> {
    Pays::TOUS.iter()
        .map(|p| VeillePays { pays: p.clone(), veille: veille(p) })
        .collect()
}

const fn v(integre_jusqu_a: i32, lacunes: &'static [&'static str]) -> Veille {
    Veille { integre_jusqu_a, lacunes, audit_du: AUDIT_DU, derniere_maj: None }
}

pub fn veille(pays: &Pays) -> Veille {
    let mut v = declaree(pays);
    v.derniere_maj = JOURNAL.iter().filter(|m| m.pays == *pays).map(|m| m.date).max();
    v
}

/// Fraîcheur déclarée d'un régime, relevée à la main (voir l'en-tête).
fn declaree(pays: &Pays) -> Veille {
    match pays {
        Pays::France => v(2026, &[]),
        Pays::FonctionPublique => v(2026, &[]),
        // ch_is.rs : « valeurs 2025 » (ORIS 2025) ; cotisations 2026 en base.
        Pays::Suisse => v(2025, &[
            "impôt à la source : barèmes cantonaux 2026 non intégrés — barèmes 2025 appliqués",
        ]),
        Pays::Luxembourg => v(2026, &[]),
        Pays::Italia => v(2026, &[]),
        // ca_impot.rs : branche « 2024+ » (fédéral) et MPB Ontario 2024 ;
        // cotisations RPC/AE jusqu'en 2026.
        Pays::Canada => v(2024, &[
            "impôt fédéral et de l'Ontario : barèmes 2025 et 2026 non intégrés — barème 2024 appliqué",
        ]),
        Pays::Quebec => v(2024, &[
            "impôt fédéral et du Québec : barèmes 2025 et 2026 non intégrés — barème 2024 appliqué",
        ]),
        // de_lohnsteuer.rs : les paramètres 2026 portent « estimation » /
        // « approximatif » en commentaire. Cotisations 2026 en base.
        Pays::Allemagne => v(2025, &[
            "Lohnsteuer 2026 : Grundfreibetrag, zones de progression et Abzugsbetrag notés « estimation » dans le code — à confirmer sur source officielle",
        ]),
        // es_cotizaciones.rs : « 2025+ » ; ES_BASE_MAX / ES_MEI depuis 2025.
        Pays::Espagne => v(2025, &[
            "SMI, bases minimale et maximale et MEI 2026 non intégrés — valeurs 2025 appliquées",
        ]),
        // pt_irs.rs jusqu'en 2025 ; PT_SMN depuis 2025.
        Pays::Portugal => v(2025, &[
            "SMN et barème IRS 2026 non intégrés — valeurs 2025 appliquées",
        ]),
        // be_pp.rs : branches jusqu'à 2025.
        Pays::Belgique => v(2025, &[
            "précompte professionnel 2026 non intégré — barème 2025 appliqué",
        ]),
        Pays::Angleterre => v(2026, &[]),
        // JP_KENPO / JP_KOYO depuis 2024, plafonds 2024, 基礎控除 2024.
        Pays::Japon => v(2024, &[
            "協会けんぽ, 雇用保険 et 子育て支援金 postérieurs à 2024 non intégrés — taux et plafonds 2024 appliqués",
        ]),
        // CN_* et CN_BASE_* depuis le 01/01/2024.
        Pays::Chine => v(2024, &[
            "assurance maladie employeur (6 % en 2026) et bases de Pékin postérieures à 2024 non intégrées — valeurs 2024 appliquées",
        ]),
        Pays::PaysBas => v(2026, &[]),
        // au_bulletin.rs : l'année civile 2026 est servie par l'exercice 2025-26.
        Pays::Australie => v(2025, &[
            "exercice 2026-27 (dès le 01/07/2026, 2ᵉ tranche de 16 à 15 %) non intégré — barème 2025-26 appliqué à toute l'année 2026",
        ]),
        Pays::NouvelleZelande => v(2026, &[]),
        Pays::Pologne => v(2026, &[]),
        Pays::CoreeDuSud => v(2026, &[]),
        Pays::Andorre => v(2026, &[]),
        Pays::Monaco => v(2026, &[]),
        Pays::Danemark => v(2026, &[]),
        Pays::Finlande => v(2026, &[]),
        Pays::Suede => v(2026, &[]),
        Pays::Estonie => v(2026, &[]),
        Pays::Lettonie => v(2026, &[]),
        Pays::Lituanie => v(2026, &[]),
        Pays::Autriche => v(2026, &[]),
        Pays::Tchequie => v(2026, &[]),
        // sk_bulletin.rs : plafond social 2025 reconduit faute de valeur sourcée.
        Pays::Slovaquie => v(2025, &[
            "plafond social 2026 non sourcé — plafond 2025 (15 730 €/mois) reconduit ; seuls les salaires au-delà sont concernés",
        ]),
        Pays::Hongrie => v(2026, &[]),
        Pays::Slovenie => v(2026, &[]),
        Pays::Grece => v(2026, &[]),
        Pays::Chypre => v(2026, &[]),
        Pays::Malte => v(2026, &[]),
        Pays::Croatie => v(2026, &[]),
        Pays::Irlande => v(2026, &[]),
        Pays::Roumanie => v(2026, &[]),
        Pays::Bulgarie => v(2026, &[]),
        // us_impot.rs : « 2026 reconduit sur le barème 2025 ».
        Pays::EtatsUnis => v(2025, &[
            "barème fédéral 2026 non intégré — barème 2025 reconduit",
        ]),
        // mx_bulletin.rs : « Données : 2025 (2026 reconduit) ».
        Pays::Mexique => v(2025, &[
            "UMA et barèmes 2026 non intégrés — valeurs 2025 reconduites",
        ]),
        // br_bulletin.rs : « Données : 2025 ».
        Pays::Bresil => v(2025, &[
            "tranches INSS et barème IRRF 2026 non intégrés (portaria 2026 absente) — valeurs 2025 reconduites",
        ]),
        // ae_bulletin.rs : « Données : 2025 ».
        Pays::Emirats => v(2025, &[
            "paramètres GPSSA non relevés pour 2026 — valeurs 2025 appliquées (5 % / 12,5 %, plafond 50 000 AED)",
        ]),
        // in_bulletin.rs : « Données : FY 2025-26 ».
        Pays::Inde => v(2025, &[
            "exercice 2026-27 (dès le 01/04/2026) non intégré — barème 2025-26 appliqué",
        ]),
    }
}
