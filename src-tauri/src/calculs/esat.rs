// ESAT (établissement ou service d'accompagnement par le travail) — rémunération
// garantie du travailleur handicapé.
//
// Le travailleur d'ESAT n'est pas salarié : il signe un contrat de soutien et
// d'aide par le travail (DSN : nature de contrat 70). Il perçoit une
// RÉMUNÉRATION GARANTIE (CASF art. L243-4 et R243-5 à R243-10) :
//
//   - comprise entre 55,7 % et 110,7 % du SMIC à temps plein, proratisée au
//     temps partiel (R243-5) ;
//   - composée d'une part financée par l'ESAT (au moins 5 % du SMIC) et d'une
//     AIDE AU POSTE versée par l'État (au plus 50,7 % du SMIC). Au-delà d'une
//     part ESAT de 20 % du SMIC, l'aide au poste baisse de 0,5 point par point
//     de part ESAT supplémentaire (R243-6) ;
//   - avant le décret 2018-194 (effet 01/01/2018) : aide au poste au plus 50 %,
//     rémunération garantie entre 55 % et 110 % du SMIC.
//
// Cotisations : la rémunération garantie entre dans l'assiette de la CSG et des
// cotisations de sécurité sociale (L243-5), mais pas de l'assurance chômage ni
// de l'AGS (pas de contrat de travail ; DSN : « salaire brut chômage » à 0).
// Faute d'affiliation à l'assurance chômage, pas de réduction générale.
// L'État compense à l'ESAT la totalité des cotisations patronales obligatoires
// afférentes à la part « aide au poste » (L243-6, R243-9, arrêté du 28/12/2006),
// retraite complémentaire Agirc-Arrco comprise : les travailleurs d'ESAT y sont
// affiliés.
//
// Sur le bulletin : la rémunération garantie est le brut ; l'aide au poste et la
// compensation des charges sont des lignes patronales négatives (recettes de
// l'ESAT), sans effet sur le net du travailleur — comme l'aide au poste EA.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

/// Barème de la rémunération garantie, en fraction du SMIC temps plein :
/// (aide au poste maximale, rémunération minimale, rémunération maximale).
fn bareme(date: NaiveDate) -> (Decimal, Decimal, Decimal) {
    if date < NaiveDate::from_ymd_opt(2018, 1, 1).unwrap() {
        (dec!(0.50), dec!(0.55), dec!(1.10))
    } else {
        (dec!(0.507), dec!(0.557), dec!(1.107))
    }
}

/// Cotisations patronales obligatoires compensées par l'État sur l'aide au poste
/// (R243-9 : CSS L242-1 et L921-1).
const CODES_COMPENSES: &[&str] = &[
    "SS_MALADIE", "SS_VIEILLESSE_PLAF", "SS_VIEILLESSE_DEPLAF", "FAMILLE", "AT_MP",
    "AGIRC_ARRCO_T1", "AGIRC_ARRCO_CEG_T1", "AGIRC_ARRCO_T2",
];

/// Part ESAT au-delà de laquelle l'aide au poste décroît (20 % du SMIC).
const SEUIL_PART_ESAT: Decimal = dec!(0.20);

fn ratio_etp(etp_pct: f64) -> Decimal {
    format!("{:.6}", (etp_pct / 100.0).clamp(0.0, 2.0))
        .parse()
        .unwrap_or(dec!(1))
}

/// Rémunération garantie minimale du mois : 55,7 % du SMIC mensuel × ETP.
pub fn remuneration_minimale(smic_mensuel: Decimal, etp_pct: f64, date: NaiveDate) -> Decimal {
    let (_, min, _) = bareme(date);
    (min * smic_mensuel * ratio_etp(etp_pct)).round_dp(2)
}

/// Décomposition d'une rémunération garantie : (part ESAT, aide au poste).
///
/// En fraction r du SMIC proratisé, avec a_max l'aide maximale :
///   r ≤ a_max + 20 %  → aide = a_max, part ESAT = r − a_max ;
///   au-delà           → r = p + a_max − 0,5 (p − 20 %)  ⇒  p = 2 (r − a_max − 10 %).
/// La rémunération est bornée au barème (une saisie hors bornes est ramenée
/// dedans pour la seule décomposition).
pub fn decomposer(remuneration: Decimal, smic_mensuel: Decimal, etp_pct: f64, date: NaiveDate) -> (Decimal, Decimal) {
    let (a_max, min, max) = bareme(date);
    let smic = smic_mensuel * ratio_etp(etp_pct);
    if smic <= Decimal::ZERO {
        return (remuneration, Decimal::ZERO);
    }
    let r = (remuneration / smic).clamp(min, max);
    let p = if r <= a_max + SEUIL_PART_ESAT {
        r - a_max
    } else {
        dec!(2) * (r - a_max - SEUIL_PART_ESAT / dec!(2))
    };
    let aide = ((r - p) * smic).round_dp(2).min(remuneration);
    ((remuneration - aide).round_dp(2), aide)
}

fn pct(x: Decimal) -> String {
    format!("{} %", (x * dec!(100)).round_dp(2).normalize()).replace('.', ",")
}

/// Lignes ESAT : aide au poste (État) et compensation des cotisations patronales
/// afférentes à cette part. `lignes` = cotisations déjà calculées sur
/// l'assiette (on y lit les taux patronaux compensés).
pub fn lignes_esat(
    remuneration: Decimal,
    etp_pct: f64,
    lignes: &[LigneCotisation],
    ctx: &ContextPaie,
) -> Vec<LigneCotisation> {
    if remuneration <= Decimal::ZERO {
        return Vec::new();
    }
    let (a_max, min, max) = bareme(ctx.date_paie);
    let (part_esat, aide) = decomposer(remuneration, ctx.smic_mensuel, etp_pct, ctx.date_paie);
    let smic_prorata = (ctx.smic_mensuel * ratio_etp(etp_pct)).round_dp(2);

    let aide_ligne = LigneCotisation {
        code:        "ESAT_AIDE_POSTE".into(),
        libelle:     ctx.libelle("ESAT_AIDE_POSTE", "Aide au poste — ESAT (État)"),
        base:        remuneration,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    Decimal::ZERO,
        montant_pat: -aide,
        categorie:   "Aide à l'emploi".into(),
        explication: ctx.expl("ESAT_AIDE_POSTE",
            "Le travailleur d'ESAT perçoit une rémunération garantie comprise entre {min} et \
            {max} du SMIC ({smic} € pour ce temps de travail). Elle se compose d'une part \
            financée par l'ESAT, au moins 5 % du SMIC (ici {part} €), et d'une aide au poste \
            financée par l'État, au plus {amax} du SMIC (ici {aide} €), qui baisse de 0,5 point \
            par point de part ESAT au-delà de 20 % du SMIC. L'aide au poste est versée à l'ESAT \
            et doit figurer sur le bulletin : elle ne change pas le net du travailleur, elle \
            réduit le coût supporté par l'ESAT.")
            .replace("{min}", &pct(min))
            .replace("{max}", &pct(max))
            .replace("{amax}", &pct(a_max))
            .replace("{smic}", &smic_prorata.to_string())
            .replace("{part}", &part_esat.to_string())
            .replace("{aide}", &aide.to_string()),
        loi_ref: Some(ctx.loi_ref("CASF art. L243-4, R243-5 et R243-6 — Décret n°2018-194 du 21/03/2018")),
    };

    // Compensation (L243-6, R243-9, arrêté du 28/12/2006) : la totalité des
    // cotisations patronales OBLIGATOIRES dues sur la part « aide au poste » —
    // assurances sociales, AT, allocations familiales (CSS L242-1) et retraite
    // complémentaire (CSS L921-1). Hors périmètre, donc à la charge de l'ESAT :
    // FNAL, versement mobilité, médecine du travail, taxe sur les salaires
    // (circulaire DGAS/3B/2008-259, notice du bordereau).
    let taux_pat: Decimal = lignes.iter()
        .filter(|l| CODES_COMPENSES.contains(&l.code.as_str()))
        .map(|l| l.taux_pat)
        .sum();
    let compensation = (aide * taux_pat).round_dp(2);

    let compensation_ligne = LigneCotisation {
        code:        "ESAT_COMPENSATION".into(),
        libelle:     ctx.libelle("ESAT_COMPENSATION", "Compensation par l'État des charges sur l'aide au poste"),
        base:        aide,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    -taux_pat,
        montant_pat: -compensation,
        categorie:   "Aide à l'emploi".into(),
        explication: ctx.expl("ESAT_COMPENSATION",
            "L'État rembourse à l'ESAT la totalité des cotisations patronales obligatoires dues \
            sur la part de la rémunération garantie égale à l'aide au poste : assurance maladie, \
            vieillesse, allocations familiales, accidents du travail et retraite complémentaire. \
            Ici : {aide} € d'aide au poste × {taux} de taux patronaux = {comp} €. Le FNAL, le \
            versement mobilité et la médecine du travail restent à la charge de l'ESAT. Pas de \
            cotisation chômage ni AGS, ni de réduction générale : le travailleur d'ESAT n'a pas \
            de contrat de travail.")
            .replace("{aide}", &aide.to_string())
            .replace("{taux}", &pct(taux_pat))
            .replace("{comp}", &compensation.to_string()),
        loi_ref: Some(ctx.loi_ref("CASF art. L243-6 et R243-9 — Arrêté du 28/12/2006")),
    };

    vec![aide_ligne, compensation_ligne]
}
