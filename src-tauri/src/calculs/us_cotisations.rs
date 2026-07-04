// Cotisations sociales fédérales américaines (FICA) + SDI Californie.
//
// Taux lus en base (cotisation_taux) ; plafonds de salaire (wage base) codés en
// dur par année (match annee), comme les plafonds canadiens (MGA).
//
// Sources : 26 U.S.C. §3101 (part salariale FICA), §3111 (part patronale),
// §3301 (FUTA) ; Social Security wage base annuelle (SSA) ; California
// Unemployment Insurance Code §984 (SDI).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

/// Plafonds annuels (wage base) par année → mensualisés au besoin.
/// (SS wage base, seuil Additional Medicare, FUTA wage base).
fn plafonds_annuels(annee: i32) -> (Decimal, Decimal, Decimal) {
    match annee {
        2025 | 2026 => (dec!(176100), dec!(200000), dec!(7000)), // 2026 reconduit sur 2025
        _           => (dec!(168600), dec!(200000), dec!(7000)), // 2024 (repli)
    }
}

/// Social Security (OASDI) : 6,2 % salarié + 6,2 % patronal, plafonné.
pub fn us_social_security(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let (ss_base_an, _, _) = plafonds_annuels(ctx.date_paie.year());
    let plafond_mensuel = (ss_base_an / dec!(12)).round_dp(2);
    let base = brut.min(plafond_mensuel);
    let ts = ctx.taux_sal("US_SS");
    let tp = ctx.taux_pat("US_SS");
    LigneCotisation {
        code: "US_SS".into(),
        libelle: ctx.libelle("US_SS", "Social Security (OASDI)"),
        base,
        taux_sal: ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat: tp,
        montant_pat: (base * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("US_SS",
            "Social Security (Old-Age, Survivors and Disability Insurance). \
            Taux 6,2 % salarié + 6,2 % employeur, sur la rémunération plafonnée à la \
            « wage base » annuelle ({plaf_an} $/an, soit {plaf_m} $/mois en {annee}). \
            Au-delà, plus de cotisation SS. Base légale : 26 U.S.C. §3101(a) / §3111(a).")
            .replace("{plaf_an}", &format!("{:.0}", ss_base_an))
            .replace("{plaf_m}", &format!("{:.2}", plafond_mensuel))
            .replace("{annee}", &ctx.date_paie.year().to_string()),
        loi_ref: Some(ctx.loi_ref("26 U.S.C. §3101(a) et §3111(a) — Social Security Act")),
    }
}

/// Medicare : 1,45 % salarié + 1,45 % patronal, sans plafond.
pub fn us_medicare(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("US_MEDICARE");
    let tp = ctx.taux_pat("US_MEDICARE");
    LigneCotisation {
        code: "US_MEDICARE".into(),
        libelle: ctx.libelle("US_MEDICARE", "Medicare (assurance maladie)"),
        base: brut,
        taux_sal: ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat: tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie: "Assurance maladie".into(),
        explication: ctx.expl("US_MEDICARE",
            "Medicare (assurance maladie des seniors). Taux 1,45 % salarié + 1,45 % \
            employeur, sans plafond de salaire. Base légale : 26 U.S.C. §3101(b) / §3111(b)."),
        loi_ref: Some(ctx.loi_ref("26 U.S.C. §3101(b) et §3111(b) — Medicare")),
    }
}

/// Additional Medicare : 0,9 % salarié sur la fraction > seuil (200 000 $/an).
/// None si la rémunération mensuelle est sous le seuil.
pub fn us_additional_medicare(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let (_, seuil_an, _) = plafonds_annuels(ctx.date_paie.year());
    let seuil_mensuel = (seuil_an / dec!(12)).round_dp(2);
    if brut <= seuil_mensuel {
        return None;
    }
    let base = brut - seuil_mensuel;
    let ts = ctx.taux_sal("US_ADD_MEDICARE");
    Some(LigneCotisation {
        code: "US_ADD_MEDICARE".into(),
        libelle: ctx.libelle("US_ADD_MEDICARE", "Additional Medicare (surtaxe)"),
        base,
        taux_sal: ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat: Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie: "Assurance maladie".into(),
        explication: ctx.expl("US_ADD_MEDICARE",
            "Surtaxe Medicare de 0,9 % à la charge du seul salarié, sur la fraction de \
            rémunération dépassant {seuil} $/an ({seuil_m} $/mois). L'employeur ne cotise pas. \
            Base légale : 26 U.S.C. §3101(b)(2).")
            .replace("{seuil}", &format!("{:.0}", seuil_an))
            .replace("{seuil_m}", &format!("{:.2}", seuil_mensuel)),
        loi_ref: Some(ctx.loi_ref("26 U.S.C. §3101(b)(2) — Additional Medicare Tax")),
    })
}

/// FUTA : chômage fédéral, 0,6 % effectif employeur sur les 7 000 $/an.
pub fn us_futa(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let (_, _, futa_base_an) = plafonds_annuels(ctx.date_paie.year());
    let plafond_mensuel = (futa_base_an / dec!(12)).round_dp(2);
    let base = brut.min(plafond_mensuel);
    let tp = ctx.taux_pat("US_FUTA");
    LigneCotisation {
        code: "US_FUTA".into(),
        libelle: ctx.libelle("US_FUTA", "FUTA — Chômage fédéral (employeur)"),
        base,
        taux_sal: Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat: tp,
        montant_pat: (base * tp).round_dp(2),
        categorie: "Chômage".into(),
        explication: ctx.expl("US_FUTA",
            "Federal Unemployment Tax Act : chômage fédéral, 100 % employeur. Taux \
            nominal 6,0 % ramené à 0,6 % effectif grâce au crédit d'État (5,4 %), sur les \
            7 000 premiers $/an de salaire. Le chômage d'État (SUTA), à taux variable selon \
            l'expérience de l'employeur, n'est pas modélisé. Base légale : 26 U.S.C. §3301.")
            .replace("{annee}", &ctx.date_paie.year().to_string()),
        loi_ref: Some(ctx.loi_ref("26 U.S.C. §3301 et s. — FUTA")),
    }
}

/// California SDI : 1,2 % salarié, sans plafond depuis 2024. None hors Californie.
pub fn us_ca_sdi(brut: Decimal, state: &str, ctx: &ContextPaie) -> Option<LigneCotisation> {
    if state != "CA" {
        return None;
    }
    let ts = ctx.taux_sal("US_CA_SDI");
    Some(LigneCotisation {
        code: "US_CA_SDI".into(),
        libelle: ctx.libelle("US_CA_SDI", "California SDI — Assurance invalidité"),
        base: brut,
        taux_sal: ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat: Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie: "Prévoyance".into(),
        explication: ctx.expl("US_CA_SDI",
            "State Disability Insurance de Californie : 1,2 % à la charge du salarié en 2025, \
            sans plafond de salaire depuis le 01/01/2024 (SB 951). Finance l'assurance \
            invalidité et le congé familial payé (PFL). Base légale : California Unemployment \
            Insurance Code §984."),
        loi_ref: Some(ctx.loi_ref("California Unemployment Insurance Code §984 — SB 951")),
    })
}
