// ── Cotisations Japon — 社会保険 (régime général, 協会けんぽ Tokyo) ────────────
//
// Périmètre : salarié secteur privé, Tokyo, ≥ 40 ans (taux 2024 à 2026 en base).
// Taux, grilles de rémunération standard et seuil d'affiliation lus en base.
//
// Sources :
//   健康保険法 ; 厚生年金保険法 ; 雇用保険法 ; 労働者災害補償保険法

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── 標準報酬月額 — rémunération mensuelle standard ──────────────────────────────
//
// Maladie, dépendance, soutien à l'enfance et pension ne se calculent pas sur le
// salaire réel mais sur la rémunération mensuelle standard du palier où il
// tombe (健康保険法 art. 40 ; 厚生年金保険法 art. 20). Les grilles sont en base
// (bareme_tranche, migration 0150) : JP_HYOJUN_KOSEI, 32 paliers de 88 000 à
// 650 000 ¥ ; JP_HYOJUN_KENPO, paliers 4 à 50 de la maladie, de 88 000 à
// 1 390 000 ¥ (les paliers 1 à 3 ne servent pas, sous le seuil d'affiliation).
//
// Seuil d'affiliation du temps partiel : JP_SEUIL_AFFILIATION (88 000 ¥ par
// mois, 被用者保険の適用要件). En deçà, aucune cotisation maladie ni pension —
// sans quoi le palier minimal dépasserait le salaire.

/// Rémunération standard d'une grille, nulle sous le seuil d'affiliation.
/// Grille ou seuil absents à la date : zéro (jp_bulletin déclare alors le
/// régime non couvert avant d'en arriver là).
fn hyojun(brut: Decimal, grille: &str, ctx: &ContextPaie) -> Decimal {
    match ctx.plafond("JP_SEUIL_AFFILIATION") {
        Some(seuil) if brut >= seuil => ctx.bareme_palier(grille, brut).unwrap_or(Decimal::ZERO),
        _ => Decimal::ZERO,
    }
}

/// Plafond d'une grille : rémunération standard du dernier palier.
fn plafond(grille: &str, ctx: &ContextPaie) -> Decimal {
    ctx.bareme(grille).and_then(|t| t.last()).map(|t| t.fixe).unwrap_or(Decimal::ZERO)
}

fn hyojun_kenpo(brut: Decimal, ctx: &ContextPaie) -> Decimal { hyojun(brut, "JP_HYOJUN_KENPO", ctx) }
fn hyojun_kosei(brut: Decimal, ctx: &ContextPaie) -> Decimal { hyojun(brut, "JP_HYOJUN_KOSEI", ctx) }

/// Part retenue sur le salaire : arrondi à l'entier, 0,50 ¥ et moins tronqués
/// (通知 : 50銭以下切り捨て、50銭超切り上げ).
fn arrondi_salarie(x: Decimal) -> Decimal {
    x.round_dp_with_strategy(0, rust_decimal::RoundingStrategy::MidpointTowardZero)
}

// ── 健康保険 — Assurance maladie ──────────────────────────────────────────────

pub fn jp_kenpo(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee   = ctx.date_paie.year();
    let plafond = plafond("JP_HYOJUN_KENPO", ctx);
    let base    = hyojun_kenpo(brut, ctx);
    let ts      = ctx.taux_sal("JP_KENPO"); // 0,0499
    let tp      = ctx.taux_pat("JP_KENPO");

    LigneCotisation {
        code:        "JP_KENPO".into(),
        libelle:     ctx.libelle("JP_KENPO", "健康保険 — Assurance maladie (協会けんぽ Tokyo)"),
        base,
        taux_sal:    ts,
        montant_sal: arrondi_salarie(base * ts),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(0),
        categorie:   "Sécurité sociale".into(),
        explication: ctx.expl("JP_KENPO",
            "Assurance maladie salariés (健康保険) — Kyokai Kenpo Tokyo {an}.\n\n\
            Taux : {ts} % sal + {tp} % pat = {tot} % total\n\
            Plafond 標準報酬月額 : ¥{plaf} /mois\n\
            Base retenue : ¥{base} (rémunération standard du palier)\n\
            Salarié : ¥{ms} | Employeur : ¥{mp}\n\n\
            Base légale : 健康保険法.")
            .replace("{an}", &annee.to_string())
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{tot}", &format!("{:.2}", (ts + tp) * dec!(100)))
            .replace("{plaf}", &format!("{}", plafond))
            .replace("{base}", &format!("{}", base))
            .replace("{ms}", &format!("{}", arrondi_salarie(base * ts)))
            .replace("{mp}", &format!("{}", (base * tp).round_dp(0))),
        loi_ref: Some(ctx.loi_ref("健康保険法 — 協会けんぽ Tokyo 料率")),
    }
}

// ── 子ども・子育て支援金 — Contribution enfance (dès avril 2026) ─────────────
//
// Prélevée avec l'assurance maladie sur la même assiette, à parts égales :
// 0,23 % au total pour l'exercice 2026 (taux uniforme national, cotisations
// d'avril 2026 versées en mai). Loi sur la promotion des mesures pour l'enfance
// (こども・子育て支援法, révision de 2024). Taux lu en base : aucune ligne avant.

pub fn jp_kodomo(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let ts = ctx.taux_sal("JP_KODOMO");
    let tp = ctx.taux_pat("JP_KODOMO");
    if ts == Decimal::ZERO && tp == Decimal::ZERO {
        return None;
    }
    let base = hyojun_kenpo(brut, ctx);
    Some(LigneCotisation {
        code:        "JP_KODOMO".into(),
        libelle:     ctx.libelle("JP_KODOMO", "子ども・子育て支援金 — Contribution enfance et parentalité"),
        base,
        taux_sal:    ts,
        montant_sal: arrondi_salarie(base * ts),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(0),
        categorie:   "Sécurité sociale".into(),
        explication: ctx.expl("JP_KODOMO",
            "Contribution de soutien à l'enfance (子ども・子育て支援金), perçue avec \
            l'assurance maladie depuis avril 2026 pour financer les prestations familiales.\n\n\
            Taux : {ts} % sal + {tp} % pat = {tot} % total (taux national uniforme)\n\
            Base : ¥{base} — Salarié : ¥{ms} | Employeur : ¥{mp}\n\n\
            Base légale : 子ども・子育て支援法.")
            .replace("{ts}", &format!("{:.3}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.3}", tp * dec!(100)))
            .replace("{tot}", &format!("{:.2}", (ts + tp) * dec!(100)))
            .replace("{base}", &format!("{}", base))
            .replace("{ms}", &format!("{}", arrondi_salarie(base * ts)))
            .replace("{mp}", &format!("{}", (base * tp).round_dp(0))),
        loi_ref: Some(ctx.loi_ref("子ども・子育て支援法")),
    })
}

// ── 介護保険 — Soins longue durée (≥ 40 ans) ─────────────────────────────────

pub fn jp_kaigo(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee   = ctx.date_paie.year();
    let plafond = plafond("JP_HYOJUN_KENPO", ctx); // même plafond que 健康保険
    let base    = hyojun_kenpo(brut, ctx);
    let ts      = ctx.taux_sal("JP_KAIGO"); // 0,008
    let tp      = ctx.taux_pat("JP_KAIGO");

    LigneCotisation {
        code:        "JP_KAIGO".into(),
        libelle:     ctx.libelle("JP_KAIGO", "介護保険 — Soins longue durée (≥ 40 ans)"),
        base,
        taux_sal:    ts,
        montant_sal: arrondi_salarie(base * ts),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(0),
        categorie:   "Sécurité sociale".into(),
        explication: ctx.expl("JP_KAIGO",
            "Assurance soins longue durée (介護保険) — applicable aux 40-64 ans.\n\n\
            Taux national {an} : {ts} % sal + {tp} % pat = {tot} % total\n\
            Même plafond que 健康保険 : ¥{plaf}/mois\n\
            Base : ¥{base} | Salarié : ¥{ms} | Employeur : ¥{mp}\n\n\
            Base légale : 介護保険法.")
            .replace("{an}", &annee.to_string())
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{tot}", &format!("{:.2}", (ts + tp) * dec!(100)))
            .replace("{plaf}", &format!("{}", plafond))
            .replace("{base}", &format!("{}", base))
            .replace("{ms}", &format!("{}", arrondi_salarie(base * ts)))
            .replace("{mp}", &format!("{}", (base * tp).round_dp(0))),
        loi_ref: Some(ctx.loi_ref("介護保険法 — MHLW 料率")),
    }
}

// ── 厚生年金保険 — Retraite salariés ─────────────────────────────────────────

pub fn jp_kosei(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let plafond = plafond("JP_HYOJUN_KOSEI", ctx);
    let base    = hyojun_kosei(brut, ctx);
    let ts      = ctx.taux_sal("JP_KOSEI"); // 0,0915
    let tp      = ctx.taux_pat("JP_KOSEI");

    LigneCotisation {
        code:        "JP_KOSEI".into(),
        libelle:     ctx.libelle("JP_KOSEI", "厚生年金保険 — Assurance retraite salariés"),
        base,
        taux_sal:    ts,
        montant_sal: arrondi_salarie(base * ts),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(0),
        categorie:   "Retraite".into(),
        explication: ctx.expl("JP_KOSEI",
            "Assurance retraite obligatoire des salariés (厚生年金保険).\n\n\
            Taux unique national (depuis oct. 2017) : {ts} % sal + {tp} % pat = {tot} %\n\
            Plafond 標準報酬月額 : ¥{plaf}/mois (grade 32)\n\
            Base : ¥{base} | Salarié : ¥{ms} | Employeur : ¥{mp}\n\n\
            Base légale : 厚生年金保険法.")
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{tot}", &format!("{:.2}", (ts + tp) * dec!(100)))
            .replace("{plaf}", &format!("{}", plafond))
            .replace("{base}", &format!("{}", base))
            .replace("{ms}", &format!("{}", arrondi_salarie(base * ts)))
            .replace("{mp}", &format!("{}", (base * tp).round_dp(0))),
        loi_ref: Some(ctx.loi_ref("厚生年金保険法")),
    }
}

// ── 雇用保険 — Assurance emploi ───────────────────────────────────────────────

pub fn jp_koyo(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("JP_KOYO"); // 0,006
    let tp = ctx.taux_pat("JP_KOYO"); // 0,0095

    LigneCotisation {
        code:        "JP_KOYO".into(),
        libelle:     ctx.libelle("JP_KOYO", "雇用保険 — Assurance emploi (chômage)"),
        base:        brut,
        taux_sal:    ts,
        montant_sal: arrondi_salarie(brut * ts),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(0),
        categorie:   "Chômage".into(),
        explication: ctx.expl("JP_KOYO",
            "Assurance emploi (雇用保険) — 一般の事業 (secteur général) {annee}.\n\n\
            Taux : salarié {ts} % + employeur {tp} % = {tot} % total\n\
            Assiette : salaire brut intégral, sans plafond.\n\
            Salarié : ¥{ms} | Employeur : ¥{mp}\n\n\
            Base légale : 雇用保険法.")
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{tot}", &format!("{:.2}", (ts + tp) * dec!(100)))
            .replace("{ms}", &format!("{}", arrondi_salarie(brut * ts)))
            .replace("{mp}", &format!("{}", (brut * tp).round_dp(0)))
            .replace("{annee}", &ctx.date_paie.year().to_string()),
        loi_ref: Some(ctx.loi_ref("雇用保険法 — MHLW 料率")),
    }
}

// ── 労災保険 — Accidents du travail ──────────────────────────────────────────

pub fn jp_rousai(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("JP_ROUSAI"); // 0,003

    LigneCotisation {
        code:        "JP_ROUSAI".into(),
        libelle:     ctx.libelle("JP_ROUSAI", "労災保険 — Accidents du travail (bureau)"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(0),
        categorie:   "Sécurité sociale".into(),
        explication: ctx.expl("JP_ROUSAI",
            "Assurance accidents du travail (労働者災害補償保険).\n\
            100 % à la charge de l'employeur. Taux bureau/services généraux {annee} : {tp} %.\n\
            Employeur : ¥{mp}\n\n\
            Base légale : 労働者災害補償保険法.")
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{mp}", &format!("{}", (brut * tp).round_dp(0)))
            .replace("{annee}", &ctx.date_paie.year().to_string()),
        loi_ref: Some(ctx.loi_ref("労働者災害補償保険法 — 労災保険料率表")),
    }
}
