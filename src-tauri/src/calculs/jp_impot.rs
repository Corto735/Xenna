// ── Impôt sur le revenu Japon — 所得税 + 住民税 ───────────────────────────────
//
// 所得税 : impôt national progressif + surtaxe reconstruction (2,1 %)
// 住民税  : taxe locale (10 % flat, simplifié)
//
// Méthode : calcul annuel sur revenu estimé (brut × 12), retenue mensuelle = / 12.
// Déduction emploi (給与所得控除) déduite avant application du barème.
//
// Sources : 所得税法 ; 復興特別所得税 (Loi du 02/12/2011) ; 地方税法.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;
use chrono::Datelike;

// ── Déduction emploi 給与所得控除 ─────────────────────────────────────────────
//
// Réduit le revenu brut avant calcul de l'impôt.
// Source : 所得税法 art. 28.

pub fn kyuyo_shotoku_koyo(revenu_annuel: Decimal) -> Decimal {
    if revenu_annuel <= dec!(1800000) {
        (revenu_annuel * dec!(0.40)).max(dec!(550000))
    } else if revenu_annuel <= dec!(3600000) {
        revenu_annuel * dec!(0.30) + dec!(80000)
    } else if revenu_annuel <= dec!(6600000) {
        revenu_annuel * dec!(0.20) + dec!(440000)
    } else if revenu_annuel <= dec!(8500000) {
        revenu_annuel * dec!(0.10) + dec!(1100000)
    } else {
        dec!(1950000)
    }
}

// ── Barème 所得税 (impôt national) ────────────────────────────────────────────

fn shotoku_zei_annuel(revenu_imposable: Decimal) -> Decimal {
    // 7 tranches — stables depuis 2015 (所得税法 art. 89)
    if revenu_imposable <= dec!(1950000) {
        revenu_imposable * dec!(0.05)
    } else if revenu_imposable <= dec!(3300000) {
        dec!(97500) + (revenu_imposable - dec!(1950000)) * dec!(0.10)
    } else if revenu_imposable <= dec!(6950000) {
        dec!(232500) + (revenu_imposable - dec!(3300000)) * dec!(0.20)
    } else if revenu_imposable <= dec!(9000000) {
        dec!(962500) + (revenu_imposable - dec!(6950000)) * dec!(0.23)
    } else if revenu_imposable <= dec!(18000000) {
        dec!(1434000) + (revenu_imposable - dec!(9000000)) * dec!(0.33)
    } else if revenu_imposable <= dec!(40000000) {
        dec!(4404000) + (revenu_imposable - dec!(18000000)) * dec!(0.40)
    } else {
        dec!(13204000) + (revenu_imposable - dec!(40000000)) * dec!(0.45)
    }
}

// ── 所得税 + 復興特別所得税 (retenue mensuelle) ────────────────────────────────

pub fn jp_shotokuzei(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let _annee  = ctx.date_paie.year();
    let rev_ann = brut * dec!(12);

    let deduction_emploi = kyuyo_shotoku_koyo(rev_ann);
    let deduction_base   = dec!(480000); // 基礎控除 2024
    let revenu_imposable = (rev_ann - deduction_emploi - deduction_base).max(Decimal::ZERO);

    let shotoku     = shotoku_zei_annuel(revenu_imposable);
    let fukkoshuzei = (shotoku * dec!(0.021)).round_dp(0); // surtaxe reconstruction 2,1 %
    let total_ann   = shotoku + fukkoshuzei;
    let mensuel     = (total_ann / dec!(12)).round_dp(0);
    let taux_eff    = if brut > Decimal::ZERO { (mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code:        "JP_SHOTOKUZEI".into(),
        libelle:     ctx.libelle("JP_SHOTOKUZEI", "所得税 — Impôt sur le revenu + surtaxe reconstruction"),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: ctx.expl("JP_SHOTOKUZEI",
            "所得税 — impôt national sur le revenu (retenue mensuelle 源泉徴収).\n\n\
            Revenu brut annuel estimé : ¥{rev}\n\
            − 給与所得控除 (déduction emploi) : ¥{de}\n\
            − 基礎控除 (déduction de base) : ¥{db}\n\
            = Revenu imposable : ¥{ri}\n\n\
            所得税 brute : ¥{sz}\n\
            + 復興特別所得税 (2,1 %) : ¥{fk}\n\
            = Total annuel : ¥{ta} / 12 = ¥{mens}/mois\n\
            Taux effectif : {teff} %\n\n\
            Base légale : 所得税法 art. 28, 89 ; 復興特別所得税法 (L. 02/12/2011).")
            .replace("{rev}", &format!("{:.0}", rev_ann))
            .replace("{de}", &format!("{:.0}", deduction_emploi))
            .replace("{db}", &format!("{:.0}", deduction_base))
            .replace("{ri}", &format!("{:.0}", revenu_imposable))
            .replace("{sz}", &format!("{:.0}", shotoku))
            .replace("{fk}", &format!("{:.0}", fukkoshuzei))
            .replace("{ta}", &format!("{:.0}", total_ann))
            .replace("{mens}", &format!("{:.0}", mensuel))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("所得税法 — 復興特別所得税法")),
    }
}

// ── 住民税 — Taxe locale (10 % flat, simplifié) ───────────────────────────────
//
// La住民税 réelle est calculée l'année suivante sur N-1. Ici : estimation simplifiée
// sur base annualisée (8 % préfectoral + 2 % municipal = 10 % flat).

pub fn jp_juminzei(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let _annee  = ctx.date_paie.year();
    let rev_ann = brut * dec!(12);

    let deduction_emploi = kyuyo_shotoku_koyo(rev_ann);
    let revenu_imposable = (rev_ann - deduction_emploi).max(Decimal::ZERO);

    let juminzei_ann = revenu_imposable * dec!(0.10); // 10 % flat
    let mensuel      = (juminzei_ann / dec!(12)).round_dp(0);
    let taux_eff     = if brut > Decimal::ZERO { (mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code:        "JP_JUMINZEI".into(),
        libelle:     ctx.libelle("JP_JUMINZEI", "住民税 — Taxe locale (estimation)"),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Taxe locale".into(),
        explication: ctx.expl("JP_JUMINZEI",
            "住民税 — taxe locale prélevée par la collectivité (estimation mensuelle).\n\n\
            Taux appliqué : 10 % flat (8 % préfectoral + 2 % municipal — 地方税法).\n\
            Assiette : revenu imposable estimé ¥{ri} (brut − déd. emploi)\n\
            = ¥{ta}/an / 12 = ¥{mens}/mois\n\
            Taux effectif : {teff} %\n\n\
            Note : en pratique, la住民税 est calculée en juin N+1 sur les revenus N. \
            Cette estimation mensuelle est indicative.\n\
            Base légale : 地方税法.")
            .replace("{ri}", &format!("{:.0}", revenu_imposable))
            .replace("{ta}", &format!("{:.0}", juminzei_ann))
            .replace("{mens}", &format!("{:.0}", mensuel))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("地方税法")),
    }
}
