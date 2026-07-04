// Impôt sur le revenu américain : fédéral progressif (IRS) + impôt d'État.
//
// Convention du simulateur : célibataire, 0 personne à charge. On annualise le
// brut (× 12), on retranche la déduction standard, on applique le barème par
// tranches, puis on redivise par 12 pour la retenue mensuelle.
//
// Barèmes codés en dur par année (match annee) ; année courante 2025.
// Sources : 26 U.S.C. §1 (barème fédéral), §63 (déduction standard) ;
// codes fiscaux des États (Revenue and Taxation Code CA §17041, NY Tax Law
// §601, Illinois 35 ILCS 5/, Pennsylvania 72 P.S. §7302, Colorado C.R.S. §39-22-104).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

/// Impôt annuel par tranches : chaque part de revenu est taxée à son taux marginal.
/// `seuils` = plafonds de tranche croissants ; `taux` = taux de chaque tranche.
fn impot_tranches(revenu: Decimal, seuils: &[Decimal], taux: &[Decimal]) -> Decimal {
    let mut impot = Decimal::ZERO;
    let mut prev = Decimal::ZERO;
    for (i, &seuil) in seuils.iter().enumerate() {
        if revenu <= prev { break; }
        let taxable = revenu.min(seuil) - prev;
        impot += taxable * taux[i];
        prev = seuil;
        if revenu <= seuil { break; }
    }
    impot
}

/// Barème fédéral (célibataire) + déduction standard, par année.
fn federal_params(annee: i32) -> (Decimal, [Decimal; 7], [Decimal; 7]) {
    match annee {
        2025 | 2026 => ( // 2026 reconduit sur le barème 2025 (en attendant publication IRS)
            dec!(15750),
            [dec!(11925), dec!(48475), dec!(103350), dec!(197300), dec!(250525), dec!(626350), dec!(9999999999)],
            [dec!(0.10),  dec!(0.12),  dec!(0.22),   dec!(0.24),    dec!(0.32),    dec!(0.35),    dec!(0.37)],
        ),
        _ => ( // 2024 (repli)
            dec!(14600),
            [dec!(11600), dec!(47150), dec!(100525), dec!(191950), dec!(243725), dec!(609350), dec!(9999999999)],
            [dec!(0.10),  dec!(0.12),  dec!(0.22),   dec!(0.24),    dec!(0.32),    dec!(0.35),    dec!(0.37)],
        ),
    }
}

/// Impôt fédéral sur le revenu (retenue mensuelle).
pub fn us_impot_federal(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let (deduction, seuils, taux) = federal_params(annee);
    let revenu_annuel = brut * dec!(12);
    let imposable = (revenu_annuel - deduction).max(Decimal::ZERO);
    let impot_annuel = impot_tranches(imposable, &seuils, &taux);
    let impot_mensuel = (impot_annuel / dec!(12)).round_dp(2);
    let taux_eff = if brut > Decimal::ZERO { (impot_mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code: "US_IMPOT_FED".into(),
        libelle: ctx.libelle("US_IMPOT_FED", "Impôt fédéral sur le revenu — retenue {annee}")
            .replace("{annee}", &annee.to_string()),
        base: brut,
        taux_sal: taux_eff,
        montant_sal: impot_mensuel,
        taux_pat: Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie: "Impôt fédéral".into(),
        explication: ctx.expl("US_IMPOT_FED",
            "Impôt fédéral sur le revenu (federal income tax withholding), célibataire sans \
            personne à charge.\nRevenu annualisé : {rev} $\nDéduction standard : − {ded} $\n\
            Revenu imposable : {imp} $\nBarème {annee} : 10/12/22/24/32/35/37 %\n\
            Impôt annuel : {ia} $ / 12 = {im} $/mois\nTaux effectif : {teff} %\n\
            Base légale : 26 U.S.C. §1 et §63.")
            .replace("{rev}", &format!("{:.0}", revenu_annuel))
            .replace("{ded}", &format!("{:.0}", deduction))
            .replace("{imp}", &format!("{:.0}", imposable))
            .replace("{annee}", &annee.to_string())
            .replace("{ia}", &format!("{:.2}", impot_annuel))
            .replace("{im}", &format!("{:.2}", impot_mensuel))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("26 U.S.C. §1 (barème) et §63 (déduction standard) — Internal Revenue Code")),
    }
}

/// Impôt d'État. None pour les États sans impôt sur le revenu (TX, FL, WA…).
/// `state` = code à 2 lettres. Barèmes 2025.
pub fn us_impot_state(brut: Decimal, state: &str, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let annee = ctx.date_paie.year();
    let revenu_annuel = brut * dec!(12);
    // (déduction, seuils, taux, nom, loi, description barème). Flat = 1 tranche.
    let (deduction, seuils, taux, nom, loi, desc): (Decimal, Vec<Decimal>, Vec<Decimal>, &str, &str, &str) = match state {
        "TX" | "FL" | "WA" => return None, // pas d'impôt sur le revenu d'État
        "IL" => (dec!(2850), vec![dec!(9999999999)], vec![dec!(0.0495)],
            "Illinois", "35 ILCS 5/ — Illinois Income Tax Act", "forfaitaire 4,95 %"),
        "PA" => (Decimal::ZERO, vec![dec!(9999999999)], vec![dec!(0.0307)],
            "Pennsylvanie", "72 P.S. §7302 — PA Tax Reform Code", "forfaitaire 3,07 %"),
        "CO" => (dec!(15750), vec![dec!(9999999999)], vec![dec!(0.0440)],
            "Colorado", "C.R.S. §39-22-104", "forfaitaire 4,40 % (base imposable fédérale)"),
        "CA" => (dec!(5540),
            vec![dec!(11079), dec!(26264), dec!(41452), dec!(57542), dec!(72724), dec!(371479), dec!(445771), dec!(742953), dec!(9999999999)],
            vec![dec!(0.01),  dec!(0.02),  dec!(0.04),  dec!(0.06),  dec!(0.08),  dec!(0.093),  dec!(0.103),  dec!(0.113),  dec!(0.123)],
            "Californie", "California Revenue and Taxation Code §17041", "progressif 1 à 12,3 % (+1 % > 1 M$)"),
        "NY" => (dec!(8000),
            vec![dec!(8500), dec!(11700), dec!(13900), dec!(80650), dec!(215400), dec!(1077550), dec!(5000000), dec!(25000000), dec!(9999999999)],
            vec![dec!(0.04), dec!(0.045), dec!(0.0525), dec!(0.055), dec!(0.06),  dec!(0.0685),  dec!(0.0965),  dec!(0.103),    dec!(0.109)],
            "New York", "New York Tax Law §601", "progressif 4 à 10,9 %"),
        _ => return None, // État non modélisé
    };

    let imposable = (revenu_annuel - deduction).max(Decimal::ZERO);
    let mut impot_annuel = impot_tranches(imposable, &seuils, &taux);
    // Californie : surtaxe santé mentale 1 % sur la fraction > 1 000 000 $.
    if state == "CA" && imposable > dec!(1000000) {
        impot_annuel += (imposable - dec!(1000000)) * dec!(0.01);
    }
    let impot_mensuel = (impot_annuel / dec!(12)).round_dp(2);
    let taux_eff = if brut > Decimal::ZERO { (impot_mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    Some(LigneCotisation {
        code: "US_IMPOT_STATE".into(),
        libelle: ctx.libelle("US_IMPOT_STATE", "Impôt d'État {nom} — retenue {annee}")
            .replace("{nom}", nom)
            .replace("{annee}", &annee.to_string()),
        base: brut,
        taux_sal: taux_eff,
        montant_sal: impot_mensuel,
        taux_pat: Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie: "Impôt d'État".into(),
        explication: ctx.expl("US_IMPOT_STATE",
            "Impôt d'État sur le revenu — {nom} ({desc}), célibataire sans personne à charge.\n\
            Revenu annualisé : {rev} $\nRevenu imposable : {imp} $ (après déduction {ded} $)\n\
            Impôt annuel : {ia} $ / 12 = {im} $/mois\nTaux effectif : {teff} %\n\
            Base légale : {loi}.")
            .replace("{nom}", nom)
            .replace("{desc}", desc)
            .replace("{rev}", &format!("{:.0}", revenu_annuel))
            .replace("{imp}", &format!("{:.0}", imposable))
            .replace("{ded}", &format!("{:.0}", deduction))
            .replace("{ia}", &format!("{:.2}", impot_annuel))
            .replace("{im}", &format!("{:.2}", impot_mensuel))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100)))
            .replace("{loi}", loi),
        loi_ref: Some(ctx.loi_ref(loi)),
    })
}
