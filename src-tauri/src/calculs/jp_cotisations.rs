// ── Cotisations Japon — 社会保険 (régime général, 協会けんぽ Tokyo) ────────────
//
// Périmètre : salarié secteur privé, Tokyo 2024, ≥ 40 ans.
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

// ── 健康保険 — Assurance maladie ──────────────────────────────────────────────

pub fn jp_kenpo(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee   = ctx.date_paie.year();
    let plafond = plafond_kenpo(annee);
    let base    = brut.min(plafond);
    let ts      = ctx.taux_sal("JP_KENPO"); // 0,0499
    let tp      = ctx.taux_pat("JP_KENPO");

    LigneCotisation {
        code:        "JP_KENPO".into(),
        libelle:     "健康保険 — Assurance maladie (協会けんぽ Tokyo)".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(0),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(0),
        categorie:   "Sécurité sociale".into(),
        explication: format!(
            "Assurance maladie salariés (健康保険) — Kyokai Kenpo Tokyo {annee}.\n\n\
            Taux : {ts_pct:.2} % sal + {tp_pct:.2} % pat = {tot:.2} % total\n\
            Plafond 標準報酬月額 : ¥{plaf} /mois\n\
            Base retenue : ¥{base} (min(brut, plafond))\n\
            Salarié : ¥{ms} | Employeur : ¥{mp}\n\n\
            Base légale : 健康保険法.",
            ts_pct = ts * dec!(100), tp_pct = tp * dec!(100),
            tot  = (ts + tp) * dec!(100),
            plaf = plafond, base = base,
            ms   = (base * ts).round_dp(0),
            mp   = (base * tp).round_dp(0),
        ),
        loi_ref: Some("健康保険法 — 協会けんぽ Tokyo 料率 2024".into()),
    }
}

// ── 介護保険 — Soins longue durée (≥ 40 ans) ─────────────────────────────────

pub fn jp_kaigo(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee   = ctx.date_paie.year();
    let plafond = plafond_kenpo(annee); // même plafond que 健康保険
    let base    = brut.min(plafond);
    let ts      = ctx.taux_sal("JP_KAIGO"); // 0,008
    let tp      = ctx.taux_pat("JP_KAIGO");

    LigneCotisation {
        code:        "JP_KAIGO".into(),
        libelle:     "介護保険 — Soins longue durée (≥ 40 ans)".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(0),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(0),
        categorie:   "Sécurité sociale".into(),
        explication: format!(
            "Assurance soins longue durée (介護保険) — applicable aux 40-64 ans.\n\n\
            Taux national {annee} : {ts_pct:.2} % sal + {tp_pct:.2} % pat = {tot:.2} % total\n\
            Même plafond que 健康保険 : ¥{plaf}/mois\n\
            Base : ¥{base} | Salarié : ¥{ms} | Employeur : ¥{mp}\n\n\
            Base légale : 介護保険法.",
            ts_pct = ts * dec!(100), tp_pct = tp * dec!(100),
            tot  = (ts + tp) * dec!(100),
            plaf = plafond, base = base,
            ms   = (base * ts).round_dp(0),
            mp   = (base * tp).round_dp(0),
        ),
        loi_ref: Some("介護保険法 — MHLW 料率 2024".into()),
    }
}

// ── 厚生年金保険 — Retraite salariés ─────────────────────────────────────────

pub fn jp_kosei(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee   = ctx.date_paie.year();
    let plafond = plafond_kosei(annee);
    let base    = brut.min(plafond);
    let ts      = ctx.taux_sal("JP_KOSEI"); // 0,0915
    let tp      = ctx.taux_pat("JP_KOSEI");

    LigneCotisation {
        code:        "JP_KOSEI".into(),
        libelle:     "厚生年金保険 — Assurance retraite salariés".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(0),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(0),
        categorie:   "Retraite".into(),
        explication: format!(
            "Assurance retraite obligatoire des salariés (厚生年金保険).\n\n\
            Taux unique national (depuis oct. 2017) : {ts_pct:.2} % sal + {tp_pct:.2} % pat = {tot:.2} %\n\
            Plafond 標準報酬月額 : ¥{plaf}/mois (grade 32)\n\
            Base : ¥{base} | Salarié : ¥{ms} | Employeur : ¥{mp}\n\n\
            Base légale : 厚生年金保険法.",
            ts_pct = ts * dec!(100), tp_pct = tp * dec!(100),
            tot  = (ts + tp) * dec!(100),
            plaf = plafond, base = base,
            ms   = (base * ts).round_dp(0),
            mp   = (base * tp).round_dp(0),
        ),
        loi_ref: Some("厚生年金保険法 — MHLW 2024".into()),
    }
}

// ── 雇用保険 — Assurance emploi ───────────────────────────────────────────────

pub fn jp_koyo(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("JP_KOYO"); // 0,006
    let tp = ctx.taux_pat("JP_KOYO"); // 0,0095

    LigneCotisation {
        code:        "JP_KOYO".into(),
        libelle:     "雇用保険 — Assurance emploi (chômage)".into(),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(0),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(0),
        categorie:   "Chômage".into(),
        explication: format!(
            "Assurance emploi (雇用保険) — 一般の事業 (secteur général) 2024.\n\n\
            Taux : salarié {ts_pct:.2} % + employeur {tp_pct:.2} % = {tot:.2} % total\n\
            Assiette : salaire brut intégral, sans plafond.\n\
            Salarié : ¥{ms} | Employeur : ¥{mp}\n\n\
            Base légale : 雇用保険法.",
            ts_pct = ts * dec!(100), tp_pct = tp * dec!(100),
            tot = (ts + tp) * dec!(100),
            ms  = (brut * ts).round_dp(0),
            mp  = (brut * tp).round_dp(0),
        ),
        loi_ref: Some("雇用保険法 — MHLW 料率 2024".into()),
    }
}

// ── 労災保険 — Accidents du travail ──────────────────────────────────────────

pub fn jp_rousai(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("JP_ROUSAI"); // 0,003

    LigneCotisation {
        code:        "JP_ROUSAI".into(),
        libelle:     "労災保険 — Accidents du travail (bureau)".into(),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(0),
        categorie:   "Sécurité sociale".into(),
        explication: format!(
            "Assurance accidents du travail (労働者災害補償保険).\n\
            100 % à la charge de l'employeur. Taux bureau/services généraux 2024 : {tp_pct:.2} %.\n\
            Employeur : ¥{mp}\n\n\
            Base légale : 労働者災害補償保険法.",
            tp_pct = tp * dec!(100),
            mp     = (brut * tp).round_dp(0),
        ),
        loi_ref: Some("労働者災害補償保険法 — 労災保険料率表 2024".into()),
    }
}
