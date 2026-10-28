// Impôt sur le revenu américain : fédéral progressif (IRS) + impôt d'État.
//
// Convention du simulateur : célibataire, 0 personne à charge. On annualise le
// brut (× 12), on retranche la déduction standard, on applique le barème par
// tranches, puis on redivise par 12 pour la retenue mensuelle.
//
// Barèmes, déductions et seuils en base (migration 0152), datés comme les taux.
// Sources : 26 U.S.C. §1 (barème fédéral), §63 (déduction standard) ;
// codes fiscaux des États (Revenue and Taxation Code CA §17041, NY Tax Law
// §601, Illinois 35 ILCS 5/, Pennsylvania 72 P.S. §7302, Colorado C.R.S. §39-22-104).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

/// Le régime est-il couvert à la date de paie ? (us_bulletin)
pub fn couvert(ctx: &ContextPaie) -> bool {
    ctx.bareme("US_FED").is_some() && ctx.plafond("US_FED_DEDUCTION").is_some()
}

/// Taux en pourcentage, à la française (0,0495 → « 4,95 »).
fn pct(t: Decimal) -> String {
    (t * dec!(100)).normalize().to_string().replace('.', ",")
}

/// Impôt fédéral sur le revenu (retenue mensuelle).
pub fn us_impot_federal(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let deduction = ctx.plafond("US_FED_DEDUCTION").unwrap_or(Decimal::ZERO);
    let revenu_annuel = brut * dec!(12);
    let imposable = (revenu_annuel - deduction).max(Decimal::ZERO);
    let impot_annuel = ctx.bareme_progressif("US_FED", imposable).unwrap_or(Decimal::ZERO);
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
/// `state` = code à 2 lettres.
pub fn us_impot_state(brut: Decimal, state: &str, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let annee = ctx.date_paie.year();
    let revenu_annuel = brut * dec!(12);
    // Nom, loi et précision de chaque État modélisé ; barème (US_ETAT_<XX>) et
    // déduction (US_ETAT_<XX>_DEDUCTION) en base. Une tranche = forfaitaire.
    let (nom, loi, precision): (&str, &str, &str) = match state {
        "TX" | "FL" | "WA" => return None, // pas d'impôt sur le revenu d'État
        "IL" => ("Illinois", "35 ILCS 5/ — Illinois Income Tax Act", ""),
        "PA" => ("Pennsylvanie", "72 P.S. §7302 — PA Tax Reform Code", ""),
        "CO" => ("Colorado", "C.R.S. §39-22-104", " (base imposable fédérale)"),
        "CA" => ("Californie", "California Revenue and Taxation Code §17041", " (+1 % > 1 M$)"),
        "NY" => ("New York", "New York Tax Law §601", ""),
        _ => return None, // État non modélisé
    };
    let code = format!("US_ETAT_{state}");
    let tranches = ctx.bareme(&code)?;
    let desc = match (tranches.first(), tranches.last()) {
        (Some(t), _) if tranches.len() == 1 => format!("forfaitaire {} %{precision}", pct(t.taux)),
        (Some(a), Some(z)) => format!("progressif {} à {} %{precision}", pct(a.taux), pct(z.taux)),
        _ => return None,
    };
    let deduction = ctx.plafond(&format!("{code}_DEDUCTION")).unwrap_or(Decimal::ZERO);
    let imposable = (revenu_annuel - deduction).max(Decimal::ZERO);
    let mut impot_annuel = ctx.bareme_progressif(&code, imposable).unwrap_or(Decimal::ZERO);
    // Californie : surtaxe santé mentale (1 %) sur la fraction au-delà du seuil.
    if let (Some(seuil), Some(t)) = (ctx.plafond(&format!("{code}_SURTAXE_SEUIL")), ctx.plafond(&format!("{code}_SURTAXE_TAUX"))) {
        if imposable > seuil {
            impot_annuel += (imposable - seuil) * t;
        }
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
            .replace("{desc}", &desc)
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
