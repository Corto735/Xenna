// ── Cotisations Chine — 五险一金 (Pékin 2024) ─────────────────────────────────
//
// Cinq assurances + fonds logement :
//   养老 (retraite), 医疗 (maladie), 失业 (chômage),
//   工伤 (AT), 生育 (maternité), 住房公积金 (fonds logement)
//
// Base clampée : min(max(brut, BASE_MIN), BASE_MAX)
// Plafonds Pékin 2024 : MIN ¥6 891 / MAX ¥35 283
//
// Sources : 社会保险法 ; 住房公积金管理条例 ; 北京市公告 2024.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

fn base_min(annee: i32) -> Decimal {
    match annee {
        _ => dec!(6891),   // Pékin 2024
    }
}

fn base_max(annee: i32) -> Decimal {
    match annee {
        _ => dec!(35283),  // Pékin 2024 = 3 × salaire moyen
    }
}

pub fn cn_base_clampee(brut: Decimal, annee: i32) -> Decimal {
    let min = base_min(annee);
    let max = base_max(annee);
    brut.max(min).min(max)
}

#[allow(clippy::too_many_arguments)]
fn ligne(
    ctx: &ContextPaie,
    code: &str, libelle: &str, base: Decimal, brut: Decimal,
    ts: Decimal, tp: Decimal,
    categorie: &str, explication: &str, loi_ref: &str,
) -> LigneCotisation {
    // Sous-phrase {expl} traduite (par code), puis injectée dans le gabarit générique.
    let expl_sub = ctx.expl(code, explication);
    let explication = ctx.expl("CN_GENERIC",
        "{expl}\nBase clampée : ¥{base} (brut ¥{brut}, min ¥{min}–max ¥{max})\n\
        Salarié : {ts_pct} % = ¥{ms} | Employeur : {tp_pct} % = ¥{mp}")
        .replace("{expl}", &expl_sub)
        .replace("{base}", &format!("{:.2}", base))
        .replace("{brut}", &format!("{:.2}", brut))
        .replace("{min}", &format!("{:.0}", base_min(2024)))
        .replace("{max}", &format!("{:.0}", base_max(2024)))
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

use chrono::Datelike;

pub fn cn_yanglao(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let ts    = ctx.taux_sal("CN_YANGLAO"); // 0,08
    let tp    = ctx.taux_pat("CN_YANGLAO"); // 0,16
    ligne(
        ctx, "CN_YANGLAO", "养老保险 — Assurance retraite",
        base, brut, ts, tp, "Retraite",
        "Cotisation retraite obligatoire. Sal 8 % + pat 16 % = 24 % total. 社会保险法 art. 12.",
        "社会保险法 art. 12 — 北京市公告 2024",
    )
}

pub fn cn_yiliao(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let ts    = ctx.taux_sal("CN_YILIAO"); // 0,02
    let tp    = ctx.taux_pat("CN_YILIAO"); // 0,08
    ligne(
        ctx, "CN_YILIAO", "医疗保险 — Assurance maladie",
        base, brut, ts, tp, "Sécurité sociale",
        "Assurance maladie. Sal 2 % + pat 8 % = 10 % total. 社会保险法 art. 23.",
        "社会保险法 art. 23 — 北京市公告 2024",
    )
}

pub fn cn_shiye(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let ts    = ctx.taux_sal("CN_SHIYE"); // 0,005
    let tp    = ctx.taux_pat("CN_SHIYE"); // 0,005
    ligne(
        ctx, "CN_SHIYE", "失业保险 — Assurance chômage",
        base, brut, ts, tp, "Chômage",
        "Assurance chômage. Sal 0,5 % + pat 0,5 % = 1 % total. 社会保险法 art. 44.",
        "社会保险法 art. 44 — 北京市公告 2024",
    )
}

pub fn cn_gongshang(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let tp    = ctx.taux_pat("CN_GONGSHANG"); // 0,004
    ligne(
        ctx, "CN_GONGSHANG", "工伤保险 — Accidents du travail",
        base, brut, Decimal::ZERO, tp, "Sécurité sociale",
        "100 % patronale. Taux Pékin général 0,4 %. 社会保险法 art. 33.",
        "社会保险法 art. 33 — 北京市公告 2024",
    )
}

pub fn cn_shengyu(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let tp    = ctx.taux_pat("CN_SHENGYU"); // 0,008
    ligne(
        ctx, "CN_SHENGYU", "生育保险 — Assurance maternité",
        base, brut, Decimal::ZERO, tp, "Sécurité sociale",
        "100 % patronale. Taux Pékin 0,8 %. 社会保险法 art. 53.",
        "社会保险法 art. 53 — 北京市公告 2024",
    )
}

pub fn cn_gongjijin(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let ts    = ctx.taux_sal("CN_GONGJIJIN"); // 0,12
    let tp    = ctx.taux_pat("CN_GONGJIJIN"); // 0,12
    ligne(
        ctx, "CN_GONGJIJIN", "住房公积金 — Fonds de logement obligatoire",
        base, brut, ts, tp, "Épargne logement",
        "Fonds logement : sal 12 % + pat 12 % = 24 % total. Pékin 2024. \
        Épargne individuelle disponible pour achat/loyer. 住房公积金管理条例.",
        "住房公积金管理条例 (1999, rév. 2019) — 北京住房公积金公告 2024",
    )
}
