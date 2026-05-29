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

fn ligne(
    code: &str, libelle: &str, base: Decimal, brut: Decimal,
    ts: Decimal, tp: Decimal,
    categorie: &str, explication: String, loi_ref: &str,
) -> LigneCotisation {
    LigneCotisation {
        code:        code.into(),
        libelle:     libelle.into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   categorie.into(),
        explication: format!(
            "{expl}\nBase clampée : ¥{base:.2} (brut ¥{brut:.2}, min ¥{min:.0}–max ¥{max:.0})\n\
            Salarié : {ts_pct:.1} % = ¥{ms:.2} | Employeur : {tp_pct:.1} % = ¥{mp:.2}",
            expl    = explication,
            base    = base, brut = brut,
            min     = base_min(2024), max = base_max(2024),
            ts_pct  = ts * dec!(100), tp_pct = tp * dec!(100),
            ms      = (base * ts).round_dp(2),
            mp      = (base * tp).round_dp(2),
        ),
        loi_ref: Some(loi_ref.into()),
    }
}

use chrono::Datelike;

pub fn cn_yanglao(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let ts    = ctx.taux_sal("CN_YANGLAO"); // 0,08
    let tp    = ctx.taux_pat("CN_YANGLAO"); // 0,16
    ligne(
        "CN_YANGLAO", "养老保险 — Assurance retraite",
        base, brut, ts, tp, "Retraite",
        "Cotisation retraite obligatoire. Sal 8 % + pat 16 % = 24 % total. 社会保险法 art. 12.".into(),
        "社会保险法 art. 12 — 北京市公告 2024",
    )
}

pub fn cn_yiliao(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let ts    = ctx.taux_sal("CN_YILIAO"); // 0,02
    let tp    = ctx.taux_pat("CN_YILIAO"); // 0,08
    ligne(
        "CN_YILIAO", "医疗保险 — Assurance maladie",
        base, brut, ts, tp, "Sécurité sociale",
        "Assurance maladie. Sal 2 % + pat 8 % = 10 % total. 社会保险法 art. 23.".into(),
        "社会保险法 art. 23 — 北京市公告 2024",
    )
}

pub fn cn_shiye(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let ts    = ctx.taux_sal("CN_SHIYE"); // 0,005
    let tp    = ctx.taux_pat("CN_SHIYE"); // 0,005
    ligne(
        "CN_SHIYE", "失业保险 — Assurance chômage",
        base, brut, ts, tp, "Chômage",
        "Assurance chômage. Sal 0,5 % + pat 0,5 % = 1 % total. 社会保险法 art. 44.".into(),
        "社会保险法 art. 44 — 北京市公告 2024",
    )
}

pub fn cn_gongshang(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let tp    = ctx.taux_pat("CN_GONGSHANG"); // 0,004
    ligne(
        "CN_GONGSHANG", "工伤保险 — Accidents du travail",
        base, brut, Decimal::ZERO, tp, "Sécurité sociale",
        "100 % patronale. Taux Pékin général 0,4 %. 社会保险法 art. 33.".into(),
        "社会保险法 art. 33 — 北京市公告 2024",
    )
}

pub fn cn_shengyu(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let tp    = ctx.taux_pat("CN_SHENGYU"); // 0,008
    ligne(
        "CN_SHENGYU", "生育保险 — Assurance maternité",
        base, brut, Decimal::ZERO, tp, "Sécurité sociale",
        "100 % patronale. Taux Pékin 0,8 %. 社会保险法 art. 53.".into(),
        "社会保险法 art. 53 — 北京市公告 2024",
    )
}

pub fn cn_gongjijin(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let base  = cn_base_clampee(brut, annee);
    let ts    = ctx.taux_sal("CN_GONGJIJIN"); // 0,12
    let tp    = ctx.taux_pat("CN_GONGJIJIN"); // 0,12
    ligne(
        "CN_GONGJIJIN", "住房公积金 — Fonds de logement obligatoire",
        base, brut, ts, tp, "Épargne logement",
        "Fonds logement : sal 12 % + pat 12 % = 24 % total. Pékin 2024. \
        Épargne individuelle disponible pour achat/loyer. 住房公积金管理条例.".into(),
        "住房公积金管理条例 (1999, rév. 2019) — 北京住房公积金公告 2024",
    )
}
