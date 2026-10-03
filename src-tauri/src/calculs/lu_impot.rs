// ── Luxembourg — retenue d'impôt sur les salaires (classe d'impôt 1) ─────────
//
// Années 2025 et 2026 (le barème 2025 reste applicable en 2026 ; prochaine
// adaptation prévue en 2028). Salarié célibataire, classe 1, sans enfant :
//   revenu imposable = brut annuel − cotisations pension et maladie (la
//   contribution dépendance n'est pas déductible) − frais d'obtention forfaitaires
//   540 € − dépenses spéciales forfaitaires 480 € ;
//   impôt = barème (23 tranches de 0 à 42 %), majoré de la contribution au fonds
//   pour l'emploi (7 %, 9 % au-delà de 150 000 €) ;
//   moins le crédit d'impôt salarié (CIS) et le crédit d'impôt CO2 salarié,
//   versés par l'employeur sur la fiche de paie.
// Sources : ACD, « Tarif de base applicable aux personnes physiques » (2025) ;
// ACD, « CIS et CI-CO2 salarié » (années d'imposition 2025 et 2026).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

/// Barème 2025 (borne haute de chaque tranche, taux).
const TARIF_2025: [(u32, u32); 23] = [
    (13_230, 0), (15_435, 8), (17_640, 9), (19_845, 10), (22_050, 11),
    (24_255, 12), (26_550, 14), (28_845, 16), (31_140, 18), (33_435, 20),
    (35_730, 22), (38_025, 24), (40_320, 26), (42_615, 28), (44_910, 30),
    (47_205, 32), (49_500, 34), (51_795, 36), (54_090, 38), (117_450, 39),
    (176_160, 40), (234_870, 41), (u32::MAX, 42),
];

fn bareme(revenu: Decimal) -> Decimal {
    let mut impot = Decimal::ZERO;
    let mut bas = Decimal::ZERO;
    for (haut, taux) in TARIF_2025 {
        if revenu <= bas { break; }
        let haut = Decimal::from(haut);
        impot += (revenu.min(haut) - bas) * Decimal::from(taux) / dec!(100);
        bas = haut;
    }
    impot
}

/// Crédit d'impôt salarié (CIS), annuel, selon le salaire brut annuel.
fn cis(brut_an: Decimal) -> Decimal {
    if brut_an < dec!(936) {
        Decimal::ZERO
    } else if brut_an <= dec!(11265) {
        dec!(300) + (brut_an - dec!(936)) * dec!(0.029)
    } else if brut_an <= dec!(40000) {
        dec!(600)
    } else if brut_an < dec!(80000) {
        dec!(600) - (brut_an - dec!(40000)) * dec!(0.015)
    } else {
        Decimal::ZERO
    }
}

/// Crédit d'impôt CO2 salarié, annuel : 192 € (2025), 216 € (2026), dégressif
/// de 40 000 à 80 000 €.
fn ci_co2(brut_an: Decimal, annee: i32) -> Decimal {
    let (plein, pente) = if annee >= 2026 { (dec!(216), dec!(0.0054)) } else { (dec!(192), dec!(0.0048)) };
    if brut_an < dec!(936) {
        Decimal::ZERO
    } else if brut_an <= dec!(40000) {
        plein
    } else if brut_an < dec!(80000) {
        plein - (brut_an - dec!(40000)) * pente
    } else {
        Decimal::ZERO
    }
}

/// Ligne d'impôt, ou None avant 2025 (barèmes antérieurs non intégrés).
/// `cotisations_deductibles` = cotisations pension + maladie du mois.
pub fn impot_lu(brut: Decimal, cotisations_deductibles: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let annee = ctx.date_paie.year();
    if annee < 2025 {
        return None;
    }
    let brut_an = brut * dec!(12);
    let revenu = (brut_an - cotisations_deductibles * dec!(12) - dec!(540) - dec!(480)).max(Decimal::ZERO);
    let impot_base = bareme(revenu);
    let taux_fe = if revenu > dec!(150000) { dec!(0.09) } else { dec!(0.07) };
    let impot_fe = (impot_base * (Decimal::ONE + taux_fe)).round_dp(2);
    let credits = (cis(brut_an) + ci_co2(brut_an, annee)).round_dp(2);
    let annuel = (impot_fe - credits).max(Decimal::ZERO);
    let mensuel = (annuel / dec!(12)).round_dp(2);
    let taux = if brut > Decimal::ZERO { (mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    Some(LigneCotisation {
        code:        "LU_IMPOT".into(),
        libelle:     ctx.libelle("LU_IMPOT", "Impôt sur le revenu — retenue sur salaires (classe 1)"),
        base:        brut,
        taux_sal:    taux,
        montant_sal: mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: ctx.expl("LU_IMPOT",
            "Retenue d'impôt sur les salaires {annee}, classe d'impôt 1 (célibataire sans enfant).\n\n\
            Revenu imposable : {b} € − cotisations pension et maladie − frais d'obtention 540 € \
            − dépenses spéciales 480 € = {r} €/an\n\
            Barème 0 à 42 % → {i} € ; contribution au fonds pour l'emploi {fe} % → {ife} €\n\
            − crédit d'impôt salarié et crédit CO2 {cr} € = {a} €/an, {m} €/mois.\n\n\
            Source : ACD (tarif de base, CIS et CI-CO2).")
            .replace("{annee}", &annee.to_string())
            .replace("{b}", &format!("{:.2}", brut_an))
            .replace("{r}", &format!("{:.2}", revenu))
            .replace("{i}", &format!("{:.2}", impot_base))
            .replace("{fe}", &format!("{}", (taux_fe * dec!(100)).normalize()))
            .replace("{ife}", &format!("{:.2}", impot_fe))
            .replace("{cr}", &format!("{:.2}", credits))
            .replace("{a}", &format!("{:.2}", annuel))
            .replace("{m}", &format!("{:.2}", mensuel)),
        loi_ref:     Some(ctx.loi_ref("LIR (loi du 4 décembre 1967) art. 118, 139bis et 154quater")),
    })
}
