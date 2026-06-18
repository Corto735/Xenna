// ── Lituanie — Sodra (sécurité sociale salarié) + GPM (impôt sur le revenu) ──────
//
// 2025 (réforme 2019 ayant transféré les cotisations au salarié) :
//   • Sodra salarié 19,50 % (retraite 8,72 % + maladie/PSD 6,98 % + maternité, etc.) ;
//     part employeur 1,77 % (régime général, hors majoration CDD).
//   • GPM (gyventojų pajamų mokestis) : 20 % jusqu'à 60 VDU/an (≈ 10 540 €/mois),
//     32 % au-delà, sur la base brut − NPD.
//   • NPD (montant non imposable) dégressif 2025 :
//       brut ≤ 1 038 € → 747 € ;
//       1 038 < brut ≤ 2 387,29 € → 747 − 0,49 × (brut − 1 038) ;
//       brut > 2 387,29 € → 400 − 0,18 × (brut − 642), plancher 0.
//
// Note : accumulation pension volontaire (+3 %) non modélisée.
// Source : VMI (GPM 2025, NPD 2025) ; Sodra.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// NPD mensuel 2025 (montant non imposable, dégressif).
fn npd(brut: Decimal) -> Decimal {
    if brut <= dec!(1038) {
        dec!(747)
    } else if brut <= dec!(2387.29) {
        (dec!(747) - dec!(0.49) * (brut - dec!(1038))).max(Decimal::ZERO)
    } else {
        (dec!(400) - dec!(0.18) * (brut - dec!(642))).max(Decimal::ZERO)
    }
}

pub fn generer_bulletin_lt(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Lituanie : données disponibles pour 2025.");
    }

    // Sodra salarié + patronal (taux lus en base).
    let ts = ctx.taux_sal("LT_SODRA");
    let tp = ctx.taux_pat("LT_SODRA");
    let sodra_sal = (brut * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "LT_SODRA".into(),
        libelle: "Sodra — Cotisations sociales".into(),
        base: brut, taux_sal: ts, montant_sal: sodra_sal,
        taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "Sodra — salarié {ts:.2} % (retraite, maladie/PSD, maternité) / employeur {tp:.2} %. \
            Salarié : {ms:.2} €.",
            ts = ts * dec!(100), tp = tp * dec!(100), ms = sodra_sal,
        ),
        loi_ref: Some("Valstybinio socialinio draudimo įstatymas".into()),
    }];

    // GPM : base = brut − NPD ; 20 % jusqu'à ≈ 10 540 €/mois, 32 % au-delà.
    let npd_m = npd(brut);
    let base = (brut - npd_m).max(Decimal::ZERO);
    let part_haute = (brut - dec!(10540)).max(Decimal::ZERO);
    let part_basse = (base - part_haute).max(Decimal::ZERO);
    let gpm = (part_basse * dec!(0.20) + part_haute * dec!(0.32)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (gpm / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "LT_GPM".into(),
        libelle: "GPM — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: gpm,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025 (GPM).\n\n\
            NPD (non imposable) dégressif : {npd:.2} €\n\
            Base = brut − NPD = {b:.2} €\n\
            Taux 20 % (jusqu'à ≈ 10 540 €/mois) puis 32 % au-delà → {gpm:.2} €/mois.\n\n\
            Source : VMI.",
            npd = npd_m, b = base, gpm = gpm,
        ),
        loi_ref: Some("Gyventojų pajamų mokesčio įstatymas".into()),
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
