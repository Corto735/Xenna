// ── Finlande — cotisations + impôt d'État + impôt communal ───────────────────
//
// Salarié secteur privé (17-68 ans). Côté salarié :
//   • TyEL 7,30 % + chômage 0,89 % + päivärahamaksu 0,88 % (si revenu ≥ 17 255 €)
//     → déductibles du revenu imposable ;
//   • sairaanhoitomaksu 1,10 % (non déductible) ;
//   • impôt d'État progressif (5 tranches) + impôt communal moyen 7,50 %.
//
// Simplification documentée : les crédits d'impôt sur les revenus du travail
// (työtulovähennys, perusvähennys, ansiotulovähennys) NE sont PAS modélisés → le net
// affiché est prudent (impôt légèrement surestimé). Taux de cotisation lus en base.
// Sources : Tuloverolaki ; TyEL ; Sairausvakuutuslaki ; Työttömyysetuuksien rahoituslaki (2026).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Impôt d'État annuel (barème 2026), cumul calculé à partir des taux marginaux confirmés.
fn impot_etat(t: Decimal) -> Decimal {
    if t <= dec!(21200) {
        t * dec!(0.1264)
    } else if t <= dec!(32600) {
        dec!(2679.68) + (t - dec!(21200)) * dec!(0.19)
    } else if t <= dec!(40100) {
        dec!(4845.68) + (t - dec!(32600)) * dec!(0.3025)
    } else if t <= dec!(52100) {
        dec!(7114.43) + (t - dec!(40100)) * dec!(0.3325)
    } else {
        dec!(11104.43) + (t - dec!(52100)) * dec!(0.375)
    }
}

fn ligne_cot(code: &str, libelle: &str, base: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    LigneCotisation {
        code: code.into(), libelle: libelle.into(), base,
        taux_sal: ts, montant_sal: (base * ts).round_dp(2),
        taux_pat: tp, montant_pat: (base * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "{libelle}. Salarié {ts:.2} % / employeur {tp:.2} %. Salarié : {ms:.2} €.",
            ts = ts * dec!(100), tp = tp * dec!(100), ms = (base * ts).round_dp(2),
        ),
        loi_ref: Some("Lainsäädäntö (Finlande)".into()),
    }
}

pub fn generer_bulletin_fi(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2026 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Finlande : données disponibles pour 2026.");
    }

    let g = brut * dec!(12);

    let mut cotisations = vec![
        ligne_cot("FI_TYEL",         "TyEL — Retraite",            brut, ctx),
        ligne_cot("FI_TYOTTOMYYS",   "Työttömyysvakuutus — Chômage", brut, ctx),
    ];

    // Päivärahamaksu : uniquement si revenu annuel ≥ 17 255 €
    let paiva_ts = ctx.taux_sal("FI_PAIVARAHA");
    let paiva_montant = if g >= dec!(17255) { (brut * paiva_ts).round_dp(2) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "FI_PAIVARAHA".into(), libelle: "Päivärahamaksu — Indemnités journalières".into(),
        base: brut, taux_sal: if g >= dec!(17255) { paiva_ts } else { Decimal::ZERO },
        montant_sal: paiva_montant, taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "Päivärahamaksu — 0,88 % (uniquement si revenu annuel ≥ 17 255 €). Déductible.\n\
            Revenu annuel : {g:.0} € → {m:.2} €/mois.", g = g, m = paiva_montant,
        ),
        loi_ref: Some("Sairausvakuutuslaki".into()),
    });

    cotisations.push(ligne_cot("FI_SAIRAANHOITO", "Sairaanhoitomaksu — Soins de santé", brut, ctx));
    cotisations.push(ligne_cot("FI_TYONANTAJA_SV", "Sairausvakuutus employeur", brut, ctx));

    // Cotisations déductibles du revenu imposable : TyEL + chômage + päiväraha (annuel)
    let ded_annuel = (ctx.taux_sal("FI_TYEL") + ctx.taux_sal("FI_TYOTTOMYYS")) * g
        + paiva_montant * dec!(12);
    let taxable = (g - ded_annuel).max(Decimal::ZERO);

    let etat = impot_etat(taxable);
    let communal = taxable * dec!(0.0750);
    let impot_mens = ((etat + communal) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "FI_TULOVERO".into(), libelle: "Tulovero — Impôt (État + communal)".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2026 (annualisé).\n\n\
            Revenu imposable : {g:.0} € − cotisations déductibles {ded:.0} € = {tx:.0} €\n\
            Barème d'État : 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n\
            (seuils 21 200 / 32 600 / 40 100 / 52 100 €) → {et:.0} €\n\
            Impôt communal moyen 7,50 % → {co:.0} €\n\
            = {im:.2} €/mois.\n\n\
            Note : crédits työtulovähennys / perusvähennys non modélisés (net prudent).\n\
            Base légale : Tuloverolaki.",
            g = g, ded = ded_annuel, tx = taxable, et = etat, co = communal, im = impot_mens,
        ),
        loi_ref: Some("Tuloverolaki".into()),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(), absence: None, salarie,
    }
}
