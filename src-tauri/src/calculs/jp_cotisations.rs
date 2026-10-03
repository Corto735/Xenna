// ── Cotisations Japon — 社会保険 (régime général, 協会けんぽ Tokyo) ────────────
//
// Périmètre : salarié secteur privé, Tokyo, ≥ 40 ans (taux 2024 à 2026 en base).
// Taux lus depuis ContextPaie (DB). Plafonds hardcodés par année.
//
// Sources :
//   健康保険法 ; 厚生年金保険法 ; 雇用保険法 ; 労働者災害補償保険法

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

fn plafond_kenpo(annee: i32) -> Decimal {
    match annee {
        _ => dec!(1390000), // ¥1 390 000/mois — grade 50 (2024+)
    }
}

fn plafond_kosei(annee: i32) -> Decimal {
    match annee {
        _ => dec!(650000), // ¥650 000/mois — grade 32 (2024+)
    }
}

// ── 標準報酬月額 — rémunération mensuelle standard ──────────────────────────────
//
// Maladie, dépendance, soutien à l'enfance et pension ne se calculent pas sur le
// salaire réel mais sur la rémunération mensuelle standard du palier où il
// tombe (健康保険法 art. 40 ; 厚生年金保険法 art. 20). Pension : 32 paliers de
// 88 000 à 650 000 ¥ ; maladie : 50 paliers de 58 000 à 1 390 000 ¥ (les
// paliers 4 à 35 de la maladie coïncident avec ceux de la pension).
// (borne haute exclue du palier, rémunération standard)
const PALIERS_KOSEI: [(u32, u32); 31] = [
    (93_000, 88_000), (101_000, 98_000), (107_000, 104_000), (114_000, 110_000),
    (122_000, 118_000), (130_000, 126_000), (138_000, 134_000), (146_000, 142_000),
    (155_000, 150_000), (165_000, 160_000), (175_000, 170_000), (185_000, 180_000),
    (195_000, 190_000), (210_000, 200_000), (230_000, 220_000), (250_000, 240_000),
    (270_000, 260_000), (290_000, 280_000), (310_000, 300_000), (330_000, 320_000),
    (350_000, 340_000), (370_000, 360_000), (395_000, 380_000), (425_000, 410_000),
    (455_000, 440_000), (485_000, 470_000), (515_000, 500_000), (545_000, 530_000),
    (575_000, 560_000), (605_000, 590_000), (635_000, 620_000),
];
const PALIERS_KENPO_HAUTS: [(u32, u32); 15] = [
    (665_000, 650_000), (695_000, 680_000), (730_000, 710_000), (770_000, 750_000),
    (810_000, 790_000), (855_000, 830_000), (905_000, 880_000), (955_000, 930_000),
    (1_005_000, 980_000), (1_055_000, 1_030_000), (1_115_000, 1_090_000),
    (1_175_000, 1_150_000), (1_235_000, 1_210_000), (1_295_000, 1_270_000),
    (1_355_000, 1_330_000),
];

fn palier(brut: Decimal, paliers: &[(u32, u32)], dernier: u32) -> Decimal {
    paliers.iter()
        .find(|(borne, _)| brut < Decimal::from(*borne))
        .map(|(_, std)| Decimal::from(*std))
        .unwrap_or(Decimal::from(dernier))
}

/// Seuil d'affiliation du salarié à temps partiel : 88 000 ¥ par mois
/// (被用者保険の適用要件). En deçà, Xenna ne retient aucune cotisation maladie
/// ni pension — sans quoi le palier minimal dépasserait le salaire.
const SEUIL_AFFILIATION: Decimal = dec!(88000);

/// Rémunération standard de la pension (厚生年金), nulle sous le seuil.
fn hyojun_kosei(brut: Decimal) -> Decimal {
    if brut < SEUIL_AFFILIATION { return Decimal::ZERO; }
    palier(brut, &PALIERS_KOSEI, 650_000)
}

/// Rémunération standard de la maladie (健康保険), aussi base de la dépendance
/// et du soutien à l'enfance ; nulle sous le seuil d'affiliation (les paliers
/// 1 à 3, de 58 000 à 78 000 ¥, ne servent donc pas).
fn hyojun_kenpo(brut: Decimal) -> Decimal {
    if brut < SEUIL_AFFILIATION { return Decimal::ZERO; }
    if brut < dec!(93000) { return dec!(88000); }
    if brut < dec!(635000) { return hyojun_kosei(brut); }
    palier(brut, &PALIERS_KENPO_HAUTS, 1_390_000)
}

/// Part retenue sur le salaire : arrondi à l'entier, 0,50 ¥ et moins tronqués
/// (通知 : 50銭以下切り捨て、50銭超切り上げ).
fn arrondi_salarie(x: Decimal) -> Decimal {
    x.round_dp_with_strategy(0, rust_decimal::RoundingStrategy::MidpointTowardZero)
}

// ── 健康保険 — Assurance maladie ──────────────────────────────────────────────

pub fn jp_kenpo(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee   = ctx.date_paie.year();
    let plafond = plafond_kenpo(annee);
    let base    = hyojun_kenpo(brut);
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
    let base = hyojun_kenpo(brut);
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
    let plafond = plafond_kenpo(annee); // même plafond que 健康保険
    let base    = hyojun_kenpo(brut);
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
    let annee   = ctx.date_paie.year();
    let plafond = plafond_kosei(annee);
    let base    = hyojun_kosei(brut);
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
