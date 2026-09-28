// ── Cotisations Chine — 五险一金 (Pékin) ─────────────────────────────────
//
// Cinq assurances + fonds logement :
//   养老 (retraite), 医疗 (maladie), 失业 (chômage),
//   工伤 (AT), 生育 (maternité), 住房公积金 (fonds logement)
//
// Base clampée : min(max(brut, BASE_MIN), BASE_MAX)
// Plancher et plafond de Pékin par période de juillet à juin (voir `bornes`).
//
// Sources : 社会保险法 ; 住房公积金管理条例 ; avis de Pékin 2024, 2025 et 2026.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

/// (plancher, plafond) mensuels de l'assiette à Pékin, par période annuelle
/// commençant en juillet (avis conjoints RH / assurance maladie / fisc de Pékin) :
/// 07/2024-06/2025 : 6 891 / 35 283 ; 07/2025-06/2026 : 7 162 / 35 811 ;
/// dès 07/2026 : 7 270 / 36 348.
fn bornes(d: NaiveDate) -> (Decimal, Decimal) {
    let juillet = |a: i32| NaiveDate::from_ymd_opt(a, 7, 1).unwrap();
    if d >= juillet(2026) { (dec!(7270), dec!(36348)) }
    else if d >= juillet(2025) { (dec!(7162), dec!(35811)) }
    else { (dec!(6891), dec!(35283)) }
}

pub fn cn_base_clampee(brut: Decimal, d: NaiveDate) -> Decimal {
    let (min, max) = bornes(d);
    brut.max(min).min(max)
}

/// Bornes du fonds logement de Pékin (année de juillet à juin) : plafond aligné
/// sur la sécurité sociale, plancher égal au salaire minimum — 2 420 ¥, puis
/// 2 540 ¥ dès le 01/09/2025 (Comité du fonds logement de Pékin, avis 2025 et 2026).
fn bornes_gjj(d: NaiveDate) -> (Decimal, Decimal) {
    let min = if d >= NaiveDate::from_ymd_opt(2025, 9, 1).unwrap() { dec!(2540) } else { dec!(2420) };
    (min, bornes(d).1)
}

#[allow(clippy::too_many_arguments)]
fn ligne(
    ctx: &ContextPaie,
    code: &str, libelle: &str, base: Decimal, brut: Decimal,
    ts: Decimal, tp: Decimal,
    categorie: &str, explication: &str, loi_ref: &str,
    (min, max): (Decimal, Decimal),
) -> LigneCotisation {
    // Sous-phrase {expl} traduite (par code), puis injectée dans le gabarit générique.
    let expl_sub = ctx.expl(code, explication);
    let explication = ctx.expl("CN_GENERIC",
        "{expl}\nBase clampée : ¥{base} (brut ¥{brut}, min ¥{min}–max ¥{max})\n\
        Salarié : {ts_pct} % = ¥{ms} | Employeur : {tp_pct} % = ¥{mp}")
        .replace("{expl}", &expl_sub)
        .replace("{base}", &format!("{:.2}", base))
        .replace("{brut}", &format!("{:.2}", brut))
        .replace("{min}", &format!("{:.0}", min))
        .replace("{max}", &format!("{:.0}", max))
        .replace("{ts_pct}", &format!("{:.1}", ts * dec!(100)))
        .replace("{tp_pct}", &format!("{:.1}", tp * dec!(100)))
        .replace("{ms}", &format!("{:.2}", (base * ts).round_dp(2)))
        .replace("{mp}", &format!("{:.2}", (base * tp).round_dp(2)));
    LigneCotisation {
        code:        code.into(),
        libelle:     ctx.libelle(code, libelle),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   categorie.into(),
        explication,
        loi_ref: Some(ctx.loi_ref(loi_ref)),
    }
}



pub fn cn_yanglao(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base  = cn_base_clampee(brut, ctx.date_paie);
    let ts    = ctx.taux_sal("CN_YANGLAO"); // 0,09
    let tp    = ctx.taux_pat("CN_YANGLAO"); // 0,16
    ligne(
        ctx, "CN_YANGLAO", "养老保险 — Assurance retraite",
        base, brut, ts, tp, "Retraite",
        "Cotisation retraite obligatoire. Sal 8 % + pat 16 % = 24 % total. 社会保险法 art. 12.",
        "社会保险法 art. 12 — 北京市公告",
        bornes(ctx.date_paie),
    )
}

pub fn cn_yiliao(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base  = cn_base_clampee(brut, ctx.date_paie);
    let ts    = ctx.taux_sal("CN_YILIAO"); // 0,02
    let tp    = ctx.taux_pat("CN_YILIAO"); // 0,09
    ligne(
        ctx, "CN_YILIAO", "医疗保险 — Assurance maladie",
        base, brut, ts, tp, "Sécurité sociale",
        "Assurance maladie. Salarié 2 % (+ 3 ¥/mois au fonds des grosses dépenses, non repris) ; employeur 9 %, dont 1 % au fonds mutuel des grosses dépenses médicales (大额医疗互助资金). Maternité (0,8 %) sur sa propre ligne. 社会保险法 art. 23.",
        "社会保险法 art. 23 — 北京市公告",
        bornes(ctx.date_paie),
    )
}

pub fn cn_shiye(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base  = cn_base_clampee(brut, ctx.date_paie);
    let ts    = ctx.taux_sal("CN_SHIYE"); // 0,005
    let tp    = ctx.taux_pat("CN_SHIYE"); // 0,005
    ligne(
        ctx, "CN_SHIYE", "失业保险 — Assurance chômage",
        base, brut, ts, tp, "Chômage",
        "Assurance chômage. Sal 0,5 % + pat 0,5 % = 1 % total. 社会保险法 art. 44.",
        "社会保险法 art. 44 — 北京市公告",
        bornes(ctx.date_paie),
    )
}

pub fn cn_gongshang(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base  = cn_base_clampee(brut, ctx.date_paie);
    let tp    = ctx.taux_pat("CN_GONGSHANG"); // 0,004
    ligne(
        ctx, "CN_GONGSHANG", "工伤保险 — Accidents du travail",
        base, brut, Decimal::ZERO, tp, "Sécurité sociale",
        "100 % patronale. Taux Pékin général 0,4 %. 社会保险法 art. 33.",
        "社会保险法 art. 33 — 北京市公告",
        bornes(ctx.date_paie),
    )
}

pub fn cn_shengyu(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base  = cn_base_clampee(brut, ctx.date_paie);
    let tp    = ctx.taux_pat("CN_SHENGYU"); // 0,008
    ligne(
        ctx, "CN_SHENGYU", "生育保险 — Assurance maternité",
        base, brut, Decimal::ZERO, tp, "Sécurité sociale",
        "100 % patronale. Taux Pékin 0,8 %. 社会保险法 art. 53.",
        "社会保险法 art. 53 — 北京市公告",
        bornes(ctx.date_paie),
    )
}

pub fn cn_gongjijin(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let (min, max) = bornes_gjj(ctx.date_paie);
    let base  = brut.max(min).min(max);
    let ts    = ctx.taux_sal("CN_GONGJIJIN"); // 0,12
    let tp    = ctx.taux_pat("CN_GONGJIJIN"); // 0,12
    ligne(
        ctx, "CN_GONGJIJIN", "住房公积金 — Fonds de logement obligatoire",
        base, brut, ts, tp, "Épargne logement",
        "Fonds logement : sal 12 % + pat 12 % = 24 % total (taux maximal, de 5 à 12 % au choix de l'employeur). \
        Épargne individuelle disponible pour achat/loyer. 住房公积金管理条例.",
        "住房公积金管理条例 (1999, rév. 2019) — 北京住房公积金公告 2024",
        bornes_gjj(ctx.date_paie),
    )
}
