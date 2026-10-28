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

// ── Barèmes ──────────────────────────────────────────────────────────────────
//
// Tous en base (migration 0150), datés comme les taux :
//   JP_KYUYO_KOJO (linéaire)  — déduction d'emploi 給与所得控除, 所得税法 art. 28,
//     avec son minimum JP_KYUYO_MIN (impôt) / JP_KYUYO_MIN_JUMIN (taxe locale :
//     sans l'exception de 50 000 ¥ de 2026) ;
//   JP_KISO_KOJO (linéaire)   — déduction de base 基礎控除 selon le revenu total ;
//   JP_SHOTOKU (progressif)   — barème 所得税, art. 89 ;
//   JP_FUKKO_TAUX, JP_JUMIN_KISO, JP_JUMIN_TAUX (plafond_reference).
// Un barème absent à la date rend le régime non couvert (jp_bulletin).

/// Barèmes et paramètres sans lesquels l'impôt japonais ne se calcule pas.
pub const BAREMES_REQUIS: [&str; 3] = ["JP_KYUYO_KOJO", "JP_KISO_KOJO", "JP_SHOTOKU"];

/// Déduction d'emploi : barème linéaire par tranche, porté à son minimum et
/// jamais au-delà du revenu.
pub fn kyuyo_shotoku_koyo(revenu_annuel: Decimal, pour_juminzei: bool, ctx: &ContextPaie) -> Decimal {
    let formule = ctx.bareme_lineaire("JP_KYUYO_KOJO", revenu_annuel).unwrap_or(Decimal::ZERO);
    let minimum = ctx.plafond(if pour_juminzei { "JP_KYUYO_MIN_JUMIN" } else { "JP_KYUYO_MIN" })
        .unwrap_or(Decimal::ZERO);
    formule.max(minimum).min(revenu_annuel)
}

// ── 所得税 + 復興特別所得税 (retenue mensuelle) ────────────────────────────────

/// `sociaux_mensuels` : cotisations sociales salariales du mois, intégralement
/// déductibles (社会保険料控除, 所得税法 art. 74).
pub fn jp_shotokuzei(brut: Decimal, sociaux_mensuels: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let rev_ann = brut * dec!(12);

    let deduction_emploi = kyuyo_shotoku_koyo(rev_ann, false, ctx);
    let sociaux          = sociaux_mensuels * dec!(12);
    let deduction_base   = ctx.bareme_lineaire("JP_KISO_KOJO", rev_ann - deduction_emploi)
        .unwrap_or(Decimal::ZERO);
    let revenu_imposable = (rev_ann - deduction_emploi - sociaux - deduction_base).max(Decimal::ZERO)
        .floor();

    let shotoku     = ctx.bareme_progressif("JP_SHOTOKU", revenu_imposable).unwrap_or(Decimal::ZERO);
    // Surtaxe de reconstruction (2,1 %, 2013-2037).
    let fukkoshuzei = (shotoku * ctx.plafond("JP_FUKKO_TAUX").unwrap_or(Decimal::ZERO)).round_dp(0);
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
            − 社会保険料控除 (cotisations sociales) : ¥{sh}\n\
            − 基礎控除 (déduction de base) : ¥{db}\n\
            = Revenu imposable : ¥{ri}\n\n\
            所得税 brute : ¥{sz}\n\
            + 復興特別所得税 (2,1 %) : ¥{fk}\n\
            = Total annuel : ¥{ta} / 12 = ¥{mens}/mois\n\
            Taux effectif : {teff} %\n\n\
            Base légale : 所得税法 art. 28, 74, 86, 89 ; 復興特別所得税法 (L. 02/12/2011).")
            .replace("{rev}", &format!("{:.0}", rev_ann))
            .replace("{de}", &format!("{:.0}", deduction_emploi))
            .replace("{sh}", &format!("{:.0}", sociaux))
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
// La 住民税 réelle est calculée l'année suivante sur N-1. Ici : estimation
// simplifiée sur base annualisée, au taux standard de 10 % (6 % municipal + 4 %
// préfectoral ; les villes désignées perçoivent 8 % et la préfecture 2 %).

pub fn jp_juminzei(brut: Decimal, sociaux_mensuels: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let rev_ann = brut * dec!(12);

    // Déduction d'emploi, cotisations sociales et déduction de base de la taxe
    // locale (JP_JUMIN_KISO, 43 万円, 地方税法 art. 34 et 314-2).
    let deduction_emploi = kyuyo_shotoku_koyo(rev_ann, true, ctx);
    let kiso_jumin       = ctx.plafond("JP_JUMIN_KISO").unwrap_or(Decimal::ZERO);
    let revenu_imposable = (rev_ann - deduction_emploi - sociaux_mensuels * dec!(12) - kiso_jumin)
        .max(Decimal::ZERO);

    let juminzei_ann = revenu_imposable * ctx.plafond("JP_JUMIN_TAUX").unwrap_or(Decimal::ZERO);
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
            Taux appliqué : 10 % flat (taux standard : 6 % municipal + 4 % préfectoral — 地方税法).\n\
            Assiette : revenu imposable estimé ¥{ri} (brut − déd. emploi − cotisations sociales − déd. de base 430 000 ¥)\n\
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
