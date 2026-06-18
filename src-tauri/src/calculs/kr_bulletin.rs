// ── Corée du Sud — 4대보험 + 소득세 + 지방소득세 ────────────────────────────────
//
// Salarié secteur privé. Côté salarié :
//   • 국민연금 (pension 4,5 %, plafonnée à 6 370 000 ₩/mois) ;
//   • 건강보험 (santé 3,545 %) + 장기요양 (12,95 % de la prime santé) ;
//   • 고용보험 (emploi 0,9 %) ;
//   • 소득세 (impôt progressif, après 근로소득공제 + 기본공제, − 근로소득세액공제)
//     + 지방소득세 (10 % de l'impôt national).
//
// Taux sociaux lus en base. 장기요양 et impôt calculés ici. Devise KRW (0 décimale).
// Sources : 국민연금법 ; 국민건강보험법 ; 노인장기요양보험법 ; 고용보험법 ; 소득세법.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// 근로소득공제 — déduction sur le salaire annuel brut (plafonnée à 20 M₩).
fn geunro_deduction(g: Decimal) -> Decimal {
    let d = if g <= dec!(5000000) {
        g * dec!(0.70)
    } else if g <= dec!(15000000) {
        dec!(3500000) + (g - dec!(5000000)) * dec!(0.40)
    } else if g <= dec!(45000000) {
        dec!(7500000) + (g - dec!(15000000)) * dec!(0.15)
    } else if g <= dec!(100000000) {
        dec!(12000000) + (g - dec!(45000000)) * dec!(0.05)
    } else {
        dec!(14750000) + (g - dec!(100000000)) * dec!(0.02)
    };
    d.min(dec!(20000000))
}

/// Barème 소득세 (impôt national) sur le revenu imposable annuel.
fn impot_bareme(t: Decimal) -> Decimal {
    if t <= dec!(14000000) {
        t * dec!(0.06)
    } else if t <= dec!(50000000) {
        dec!(840000) + (t - dec!(14000000)) * dec!(0.15)
    } else if t <= dec!(88000000) {
        dec!(6240000) + (t - dec!(50000000)) * dec!(0.24)
    } else if t <= dec!(150000000) {
        dec!(15360000) + (t - dec!(88000000)) * dec!(0.35)
    } else if t <= dec!(300000000) {
        dec!(37060000) + (t - dec!(150000000)) * dec!(0.38)
    } else if t <= dec!(500000000) {
        dec!(94060000) + (t - dec!(300000000)) * dec!(0.40)
    } else if t <= dec!(1000000000) {
        dec!(174060000) + (t - dec!(500000000)) * dec!(0.42)
    } else {
        dec!(384060000) + (t - dec!(1000000000)) * dec!(0.45)
    }
}

/// 근로소득세액공제 — crédit d'impôt sur les revenus du travail (plafonné par 총급여).
fn credit_impot(tax: Decimal, g: Decimal) -> Decimal {
    let base = if tax <= dec!(1300000) {
        tax * dec!(0.55)
    } else {
        dec!(715000) + (tax - dec!(1300000)) * dec!(0.30)
    };
    let cap = if g <= dec!(33000000) {
        dec!(740000)
    } else if g <= dec!(70000000) {
        (dec!(740000) - (g - dec!(33000000)) * dec!(0.008)).max(dec!(660000))
    } else if g <= dec!(120000000) {
        (dec!(660000) - (g - dec!(70000000)) * dec!(0.5)).max(dec!(500000))
    } else {
        (dec!(500000) - (g - dec!(120000000)) * dec!(0.5)).max(dec!(200000))
    };
    base.min(cap)
}

pub fn generer_bulletin_kr(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    // 2024 et 2025 : NHI 7,09 % (gelé), LTC 12,95 %, pension 9 %, EI 0,9 % identiques.
    if !(2024..=2025).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "KRW", "Corée du Sud : données disponibles pour 2024-2025.");
    }

    let mut cotisations = Vec::new();

    // 국민연금 (plafonnée — 상한액 juil. 2025-juin 2026)
    let base_nps = brut.min(dec!(6370000));
    let nps_ts = ctx.taux_sal("KR_NPS");
    let nps_tp = ctx.taux_pat("KR_NPS");
    cotisations.push(LigneCotisation {
        code: "KR_NPS".into(), libelle: "국민연금 — Pension nationale".into(), base: base_nps,
        taux_sal: nps_ts, montant_sal: (base_nps * nps_ts).round_dp(0),
        taux_pat: nps_tp, montant_pat: (base_nps * nps_tp).round_dp(0),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "국민연금 — pension. {ts:.2} % sal / {tp:.2} % pat.\n\
            Assiette plafonnée à 6 370 000 ₩/mois → {base:.0} ₩.\n\
            Salarié : {ms:.0} ₩.\n\nBase légale : 국민연금법.",
            ts = nps_ts * dec!(100), tp = nps_tp * dec!(100), base = base_nps,
            ms = (base_nps * nps_ts).round_dp(0),
        ),
        loi_ref: Some("국민연금법".into()),
    });

    // 건강보험
    let nhi_ts = ctx.taux_sal("KR_NHI");
    let nhi_tp = ctx.taux_pat("KR_NHI");
    let nhi_sal = (brut * nhi_ts).round_dp(0);
    let nhi_pat = (brut * nhi_tp).round_dp(0);
    cotisations.push(LigneCotisation {
        code: "KR_NHI".into(), libelle: "건강보험 — Assurance santé".into(), base: brut,
        taux_sal: nhi_ts, montant_sal: nhi_sal, taux_pat: nhi_tp, montant_pat: nhi_pat,
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "건강보험 — santé. {ts:.3} % chacun. Salarié : {ms:.0} ₩.\n\nBase légale : 국민건강보험법.",
            ts = nhi_ts * dec!(100), ms = nhi_sal,
        ),
        loi_ref: Some("국민건강보험법".into()),
    });

    // 장기요양 = 12,95 % de la prime santé
    let ltc_taux = dec!(0.1295);
    let ltc_sal = (nhi_sal * ltc_taux).round_dp(0);
    let ltc_pat = (nhi_pat * ltc_taux).round_dp(0);
    cotisations.push(LigneCotisation {
        code: "KR_LTC".into(), libelle: "장기요양보험 — Dépendance".into(), base: nhi_sal,
        taux_sal: ltc_taux, montant_sal: ltc_sal, taux_pat: ltc_taux, montant_pat: ltc_pat,
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "장기요양보험 — soins de longue durée. 12,95 % de la prime santé (2025).\n\
            Assiette : prime santé salariale {b:.0} ₩ → {ms:.0} ₩.\n\nBase légale : 노인장기요양보험법.",
            b = nhi_sal, ms = ltc_sal,
        ),
        loi_ref: Some("노인장기요양보험법".into()),
    });

    // 고용보험
    let ei_ts = ctx.taux_sal("KR_EI");
    let ei_tp = ctx.taux_pat("KR_EI");
    cotisations.push(LigneCotisation {
        code: "KR_EI".into(), libelle: "고용보험 — Assurance emploi".into(), base: brut,
        taux_sal: ei_ts, montant_sal: (brut * ei_ts).round_dp(0),
        taux_pat: ei_tp, montant_pat: (brut * ei_tp).round_dp(0),
        categorie: "Chômage".into(),
        explication: format!(
            "고용보험 — emploi. Salarié {ts:.2} % / employeur {tp:.2} %.\n\nBase légale : 고용보험법.",
            ts = ei_ts * dec!(100), tp = ei_tp * dec!(100),
        ),
        loi_ref: Some("고용보험법".into()),
    });

    // 산재보험 (employeur)
    let sj_tp = ctx.taux_pat("KR_SANJAE");
    cotisations.push(LigneCotisation {
        code: "KR_SANJAE".into(), libelle: "산재보험 — Accidents (employeur)".into(), base: brut,
        taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: sj_tp, montant_pat: (brut * sj_tp).round_dp(0),
        categorie: "Cotisations patronales".into(),
        explication: format!(
            "산재보험 — accidents du travail, 100 % patronal. ≈ {tp:.2} % (moyen).\n\nBase légale : 고용보험법/산재.",
            tp = sj_tp * dec!(100),
        ),
        loi_ref: Some("산업재해보상보험법".into()),
    });

    // 소득세 + 지방소득세 (annualisé)
    let g = brut * dec!(12);
    let taxable = (g - geunro_deduction(g) - dec!(1500000)).max(Decimal::ZERO);
    let tax_brut = impot_bareme(taxable);
    let credit = credit_impot(tax_brut, g);
    let national_ann = (tax_brut - credit).max(Decimal::ZERO);
    let national_mens = (national_ann / dec!(12)).round_dp(0);
    let local_mens = (national_mens * dec!(0.10)).round_dp(0);
    let taux_it = if brut > Decimal::ZERO { (national_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "KR_INCOME_TAX".into(), libelle: "소득세 — Impôt sur le revenu".into(), base: brut,
        taux_sal: taux_it, montant_sal: national_mens, taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "소득세 — impôt national (annualisé).\n\n\
            총급여 {g:.0} ₩ − 근로소득공제 {ded:.0} ₩ − 기본공제 1 500 000 ₩\n\
            = revenu imposable {tx:.0} ₩\n\
            Barème 6→45 % : {tb:.0} ₩ − 근로소득세액공제 {cr:.0} ₩\n\
            = {na:.0} ₩/an / 12 = {nm:.0} ₩/mois.\n\n\
            Base légale : 소득세법.",
            g = g, ded = geunro_deduction(g), tx = taxable, tb = tax_brut,
            cr = credit, na = national_ann, nm = national_mens,
        ),
        loi_ref: Some("소득세법".into()),
    });
    cotisations.push(LigneCotisation {
        code: "KR_LOCAL_TAX".into(), libelle: "지방소득세 — Impôt local (10 %)".into(), base: national_mens,
        taux_sal: dec!(0.10), montant_sal: local_mens, taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Taxe locale".into(),
        explication: format!(
            "지방소득세 — impôt local = 10 % de l'impôt national.\n\
            {n:.0} ₩ × 10 % = {l:.0} ₩/mois.\n\nBase légale : 지방세법.",
            n = national_mens, l = local_mens,
        ),
        loi_ref: Some("지방세법".into()),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(0);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(0),
        devise: "KRW".into(),
        absence: None,
        salarie,
    }
}
