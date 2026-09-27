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
    Veille { integre_jusqu_a, lacunes, audit_du: AUDIT_DU }
}

pub fn veille(pays: &Pays) -> Veille {
    match pays {
        // AGS : 0,15 % en base depuis 2024 ; le dossier de complétion relève 0,20 %
        // au 01/07/2024 puis 0,25 % (CA de l'AGS du 16/12/2025).
        Pays::France => v(2023, &[
            "AGS : 0,20 % au 01/07/2024 puis 0,25 % (CA de l'AGS du 16/12/2025) non intégrés — la base applique 0,15 %",
        ]),
        // FPT_CNRACL : 30,65 % patronal en base depuis le 01/01/2019.
        Pays::FonctionPublique => v(2024, &[
            "CNRACL : relèvement pluriannuel du taux employeur à partir de 2025 non intégré — la base applique 30,65 % (taux 2019) ; texte à sourcer",
        ]),
        // ch_is.rs : « valeurs 2025 » (ORIS 2025) ; cotisations 2026 en base.
        Pays::Suisse => v(2025, &[
            "impôt à la source : barèmes cantonaux 2026 non intégrés — barèmes 2025 appliqués",
        ]),
        Pays::Luxembourg => v(2026, &[]),
        // it_irpef.rs : branches jusqu'à 2025.
        Pays::Italia => v(2025, &[
            "IRPEF 2026 : 2ᵉ tranche ramenée de 35 à 33 % non intégrée — barème 2025 appliqué",
        ]),
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
        // uk_cotisations.rs : exercice 2024/25 ; UK_NI_PAT 13,8 % depuis le 06/04/2024.
        Pays::Angleterre => v(2024, &[
            "exercices 2025/26 et 2026/27 non intégrés (NI employeur 15 % et seuil £5 000 depuis le 06/04/2025) — seuils et taux 2024/25 appliqués",
        ]),
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
        // nz_bulletin.rs : ACC 1,67 % pour 2026 ; NZ_KIWISAVER_EMP 3 % en base.
        Pays::NouvelleZelande => v(2025, &[
            "ACC 1,75 % et KiwiSaver 3,5 % (dès le 01/04/2026) non intégrés — 1,67 % et 3 % appliqués",
        ]),
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
        // ie_bulletin.rs : « la hausse PRSI de +0,15 % au 1ᵉʳ octobre 2026 n'est pas modélisée ».
        Pays::Irlande => v(2025, &[
            "PRSI : hausse de 0,15 point au 01/10/2026 non modélisée — 4,2 % appliqué toute l'année",
        ]),
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
