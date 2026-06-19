// ── Malte — Social Security Contributions (plafonnées) + impôt sur le revenu ─────
//
// 2025 :
//   • Social Security Contributions (SSC, Klassi 1) salarié 10 % / employeur 10 %,
//     sur l'assiette plafonnée (base maximale ≈ 27 679 €/an, soit 2 306,58 €/mois,
//     pour les personnes nées après 1962).
//   • Impôt sur le revenu (barème « single » 2025) : 0 % jusqu'à 12 000 €, 15 %,
//     25 %, 35 %, calculé par taux × revenu − abattement.
//
// Simplification : barème célibataire (single) ; SSC non déductible de l'impôt.
// Source : Department of Social Security ; Commissioner for Revenue (barème 2025).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Impôt annuel 2025 (barème single : taux × revenu − abattement).
fn impot(t: Decimal) -> Decimal {
    if t <= dec!(12000) {
        Decimal::ZERO
    } else if t <= dec!(16000) {
        t * dec!(0.15) - dec!(1800)
    } else if t <= dec!(60000) {
        t * dec!(0.25) - dec!(3400)
    } else {
        t * dec!(0.35) - dec!(9400)
    }
}

pub fn generer_bulletin_mt(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Malte : données disponibles pour 2025.");
    }

    // SSC sur assiette plafonnée (2 306,58 €/mois).
    let assiette = brut.min(dec!(2306.58));
    let ts = ctx.taux_sal("MT_SSC");
    let tp = ctx.taux_pat("MT_SSC");
    let ssc_sal = (assiette * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "MT_SSC".into(),
        libelle: "Social Security Contributions (Klassi 1)".into(),
        base: assiette, taux_sal: ts, montant_sal: ssc_sal,
        taux_pat: tp, montant_pat: (assiette * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "SSC — salarié {ts:.2} % / employeur {tp:.2} %. Assiette plafonnée à \
            2 306,58 €/mois (≈ 27 679 €/an). Salarié : {ms:.2} €.",
            ts = ts * dec!(100), tp = tp * dec!(100), ms = ssc_sal,
        ),
        loi_ref: Some("Social Security Act (Cap. 318)".into()),
    }];

    // Impôt : base annuelle = brut × 12 (SSC non déductible).
    let base_an = brut * dec!(12);
    let impot_mens = (impot(base_an).max(Decimal::ZERO) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "MT_TAX".into(),
        libelle: "Income Tax — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025 (barème single, annualisé).\n\n\
            Base = brut × 12 = {b:.0} €\n\
            0 % jusqu'à 12 000 €, puis 15 % / 25 % / 35 % (abattements 1 800 / 3 400 / 9 400 €)\n\
            → {im:.2} €/mois.\n\n\
            Source : Commissioner for Revenue.",
            b = base_an, im = impot_mens,
        ),
        loi_ref: Some("Income Tax Act (Cap. 123)".into()),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(), absence: None, heures_sup: None, salarie,
    }
}
