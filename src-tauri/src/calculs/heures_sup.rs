// Heures supplémentaires et complémentaires (France) — majoration + exonérations.
//
// Trois dispositifs, tous en vigueur depuis le 01/01/2019 (loi du 24/12/2018) :
//   1. Majoration de la rémunération (Code du travail) : HS +25 % les 8 premières,
//      +50 % au-delà (seuil mensuel forfaitaire ici — le seuil légal est hebdomadaire) ;
//      HC +10 % dans la limite du dixième des heures contractuelles, +25 % au-delà.
//   2. Réduction de cotisations salariales (CSS L241-17) : somme des taux salariaux
//      d'assurance vieillesse (base + complémentaire), plafonnée à 11,31 %.
//   3. Déduction forfaitaire patronale (CSS L241-18) : forfait €/heure supp selon
//      l'effectif (HS uniquement).
// L'exonération d'impôt sur le revenu (CGI 81 quater) est calculée par bulletin.rs
// après le net imposable (voir net_imposable_exo).
//
// Le taux horaire est dérivé du salaire de base : base / (151,67 × ETP/100).

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal_macros::dec;
use chrono::NaiveDate;
use crate::db::ContextPaie;
use crate::models::{HeuresSupResult, LigneCotisation, Salarie};

/// Heures mensuelles pour un temps plein (35 h hebdo × 52/12).
const HEURES_TEMPS_PLEIN: Decimal = dec!(151.67);
/// Plafond légal de la réduction de cotisations salariales (CSS L241-17).
const REDUC_SAL_HS_CAP: Decimal = dec!(0.1131);

/// Conversion f64 → Decimal bornée et arrondie (évite les artefacts binaires).
fn dec_h(h: f64) -> Decimal {
    Decimal::from_f64(h.max(0.0)).unwrap_or(Decimal::ZERO).round_dp(4)
}

/// Détail HS/HC + lignes de cotisation associées, prêt à être branché au bulletin.
pub struct HeuresSup {
    pub result: HeuresSupResult,
    /// Réduction salariale et déduction patronale (le cas échéant).
    pub lignes: Vec<LigneCotisation>,
    /// Rémunération brute majorée totale (HS + HC) à ajouter au brut.
    pub gain_total: Decimal,
}

/// Tarif de la déduction forfaitaire patronale selon l'effectif et la date.
/// None si aucun barème applicable (ex. ≥250 salariés avant 2026).
fn tarif_dfp(effectif: &str, ctx: &ContextPaie) -> Option<Decimal> {
    let code = match effectif {
        "20_249" => "DFP_HS_20_249",
        "250p"   => "DFP_HS_250P",
        _        => "DFP_HS_MOINS20",
    };
    ctx.plafond(code)
}

/// Calcule les gains majorés et les exonérations sociales. Retourne None si aucune
/// heure n'est saisie (ni HS ni HC).
pub fn calculer(salarie: &Salarie, ctx: &ContextPaie) -> Option<HeuresSup> {
    let heures_supp = salarie.heures_supp.max(0.0);
    let heures_comp = salarie.heures_comp.max(0.0);
    if heures_supp <= 0.0 && heures_comp <= 0.0 {
        return None;
    }

    // Heures contractuelles du mois = 151,67 × ETP/100 ; taux horaire = base / heures.
    let etp_ratio: Decimal = Decimal::from_f64((salarie.etp / 100.0).clamp(0.0, 2.0))
        .unwrap_or(Decimal::ONE).round_dp(6);
    let heures_contractuelles = HEURES_TEMPS_PLEIN * etp_ratio;

    let salaire_base: Decimal = salarie.salaire_base.as_deref()
        .and_then(|s| s.parse().ok())
        .filter(|v: &Decimal| *v > Decimal::ZERO)
        .unwrap_or(salarie.salaire_brut);
    let taux_horaire = if heures_contractuelles > Decimal::ZERO {
        (salaire_base / heures_contractuelles).round_dp(4)
    } else {
        Decimal::ZERO
    };

    // ── Majoration heures supplémentaires : 8 à +25 %, le reste à +50 % ──
    let h_supp_25 = heures_supp.min(8.0);
    let h_supp_50 = (heures_supp - 8.0).max(0.0);
    let gain_hs = (dec_h(h_supp_25) * taux_horaire * dec!(1.25)
        + dec_h(h_supp_50) * taux_horaire * dec!(1.50)).round_dp(2);

    // ── Majoration heures complémentaires : 1/10 contractuel à +10 %, reste à +25 % ──
    let seuil_10 = (heures_contractuelles / dec!(10)).to_string().parse::<f64>().unwrap_or(0.0);
    let h_comp_10 = heures_comp.min(seuil_10);
    let h_comp_25 = (heures_comp - seuil_10).max(0.0);
    let gain_hc = (dec_h(h_comp_10) * taux_horaire * dec!(1.10)
        + dec_h(h_comp_25) * taux_horaire * dec!(1.25)).round_dp(2);

    let gain_total = gain_hs + gain_hc;

    // Le régime d'exonération HS n'existe que depuis le 01/01/2019.
    let regime_actif = ctx.date_paie >= NaiveDate::from_ymd_opt(2019, 1, 1).unwrap();
    let exo_plafond = ctx.plafond("EXO_FISCALE_HS_ANNUEL").unwrap_or(Decimal::ZERO);

    let mut lignes = Vec::new();
    let mut reduction_salariale = Decimal::ZERO;
    let mut deduction_patronale = Decimal::ZERO;

    if regime_actif && gain_total > Decimal::ZERO {
        // ── Réduction de cotisations salariales (vieillesse base + compl., ≤ 11,31 %) ──
        let taux_vieillesse = ctx.taux_sal("SS_VIEILLESSE_PLAF")
            + ctx.taux_sal("SS_VIEILLESSE_DEPLAF")
            + ctx.taux_sal("AGIRC_ARRCO_T1")
            + ctx.taux_sal("AGIRC_ARRCO_CEG_T1");
        let taux_reduc = taux_vieillesse.min(REDUC_SAL_HS_CAP);
        reduction_salariale = (gain_total * taux_reduc).round_dp(2);

        if reduction_salariale > Decimal::ZERO {
            let detail = format!(
                "\u{1}{{\"gain_total\":\"{}\",\"taux_reduc\":\"{}\",\"plafond_taux\":\"{}\",\"reduction\":\"{}\"}}",
                gain_total, taux_reduc, REDUC_SAL_HS_CAP, reduction_salariale,
            );
            let expl = ctx.expl("REDUC_SAL_HS", "Réduction de cotisations salariales d'assurance \
                vieillesse (de base et complémentaire) sur la rémunération des heures supplémentaires \
                et complémentaires, dans la limite de 11,31 % (loi du 24/12/2018, CSS art. L241-17). \
                Elle vient en déduction des cotisations salariales : elle augmente le net à payer.");
            lignes.push(LigneCotisation {
                code:        "REDUC_SAL_HS".into(),
                libelle:     ctx.libelle("REDUC_SAL_HS", "Réduction salariale heures supp./compl."),
                base:        gain_total,
                taux_sal:    taux_reduc,
                montant_sal: -reduction_salariale, // négatif → diminue le total salarial, augmente le net
                taux_pat:    Decimal::ZERO,
                montant_pat: Decimal::ZERO,
                categorie:   "Heures supplémentaires".into(),
                explication: format!("{expl}{detail}"),
                loi_ref:     Some(ctx.loi_ref("CSS art. L241-17 — Loi n°2018-1213 du 24/12/2018")),
            });
        }

        // ── Déduction forfaitaire patronale (heures supplémentaires uniquement) ──
        if heures_supp > 0.0 {
            if let Some(tarif) = tarif_dfp(salarie.effectif.as_deref().unwrap_or("moins20"), ctx) {
                deduction_patronale = (dec_h(heures_supp) * tarif).round_dp(2);
                if deduction_patronale > Decimal::ZERO {
                    let detail = format!(
                        "\u{1}{{\"heures_supp\":{:.2},\"tarif\":\"{}\",\"deduction\":\"{}\"}}",
                        heures_supp, tarif, deduction_patronale,
                    );
                    let expl = ctx.expl("DFP_HS", "Déduction forfaitaire de cotisations patronales \
                        par heure supplémentaire (CSS art. L241-18) : 1,50 € dans les entreprises de \
                        moins de 20 salariés, 0,50 € à partir de 20 salariés. Elle réduit le coût \
                        réel supporté par l'employeur, sans effet sur le net du salarié.");
                    lignes.push(LigneCotisation {
                        code:        "DFP_HS".into(),
                        libelle:     ctx.libelle("DFP_HS", "Déduction forfaitaire patronale (heures supp.)"),
                        base:        gain_hs,
                        taux_sal:    Decimal::ZERO,
                        montant_sal: Decimal::ZERO,
                        taux_pat:    Decimal::ZERO,
                        montant_pat: -deduction_patronale, // négatif → réduit le coût employeur
                        categorie:   "Heures supplémentaires".into(),
                        explication: format!("{expl}{detail}"),
                        loi_ref:     Some(ctx.loi_ref("CSS art. L241-18")),
                    });
                }
            }
        }
    }

    let result = HeuresSupResult {
        taux_horaire,
        h_supp_25, h_supp_50, gain_hs,
        h_comp_10, h_comp_25, gain_hc,
        reduction_salariale,
        deduction_patronale,
        exo_fiscale: Decimal::ZERO, // renseigné par bulletin.rs après le net imposable
        exo_plafond,
    };

    Some(HeuresSup { result, lignes, gain_total })
}

/// Net imposable exonérable au titre des HS/HC pour le mois, avant plafond annuel.
/// Approximation documentée : part des gains majorés dans le net imposable, au
/// prorata `net_imposable / brut`. Le régime n'existe que depuis 2019.
pub fn net_imposable_exo(
    gain_total: Decimal,
    net_imposable: Decimal,
    brut: Decimal,
    ctx: &ContextPaie,
) -> Decimal {
    if brut <= Decimal::ZERO || gain_total <= Decimal::ZERO {
        return Decimal::ZERO;
    }
    if ctx.date_paie < NaiveDate::from_ymd_opt(2019, 1, 1).unwrap() {
        return Decimal::ZERO;
    }
    (gain_total * net_imposable / brut).max(Decimal::ZERO).round_dp(2)
}
