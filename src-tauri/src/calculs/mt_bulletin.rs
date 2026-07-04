// ── Malte — Social Security Contributions (plafonnées) + impôt sur le revenu ─────
//
// 2025 :
//   • Social Security Contributions (SSC, Klassi 1) salarié 10 % / employeur 10 %,
//     sur l'assiette plafonnée (base maximale ≈ 27 679 €/an, soit 2 306,58 €/mois,
//     pour les personnes nées après 1962).
//   • Impôt sur le revenu (barème « single » 2025) : 0 % jusqu'à 12 000 €, 15 %,
//     25 %, 35 %, calculé par taux × revenu − abattement.
//
// 2026 : barème d'impôt single inchangé ; SSC Klassi 1 inchangée (10 % / 10 %) mais
// plafond porté à 2 423,67 €/mois (base maximale 29 084 €/an, contribution max
// 55,93 €/semaine, personnes nées après 1962).
// Simplification : barème célibataire (single) ; SSC non déductible de l'impôt.
// Source : Department of Social Security ; Commissioner for Revenue (barème 2025-2026).

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

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "MT",
            "Malte : données disponibles pour 2025 et 2026.", ctx);
    }

    // SSC sur assiette plafonnée (2 306,58 €/mois en 2025, 2 423,67 € en 2026).
    let plafond = if annee >= 2026 { dec!(2423.67) } else { dec!(2306.58) };
    let assiette = brut.min(plafond);
    let ts = ctx.taux_sal("MT_SSC");
    let tp = ctx.taux_pat("MT_SSC");
    let ssc_sal = (assiette * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "MT_SSC".into(),
        libelle: ctx.libelle("MT_SSC", "Social Security Contributions (Klassi 1)"),
        base: assiette, taux_sal: ts, montant_sal: ssc_sal,
        taux_pat: tp, montant_pat: (assiette * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("MT_SSC",
            "SSC — salarié {ts} % / employeur {tp} %. Assiette plafonnée à \
            {pl} €/mois. Salarié : {ms} €.")
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{pl}", &format!("{:.2}", plafond))
            .replace("{ms}", &format!("{:.2}", ssc_sal)),
        loi_ref: Some(ctx.loi_ref("Social Security Act (Cap. 318)")),
    }];

    // Impôt : base annuelle = brut × 12 (SSC non déductible).
    let base_an = brut * dec!(12);
    let impot_mens = (impot(base_an).max(Decimal::ZERO) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "MT_TAX".into(),
        libelle: ctx.libelle("MT_TAX", "Income Tax — Impôt sur le revenu"),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("MT_TAX",
            "Impôt sur le revenu {annee} (barème single, annualisé).\n\n\
            Base = brut × 12 = {b} €\n\
            0 % jusqu'à 12 000 €, puis 15 % / 25 % / 35 % (abattements 1 800 / 3 400 / 9 400 €)\n\
            → {im} €/mois.\n\n\
            Source : Commissioner for Revenue.")
            .replace("{annee}", &annee.to_string())
            .replace("{b}", &format!("{:.0}", base_an))
            .replace("{im}", &format!("{:.2}", impot_mens)),
        loi_ref: Some(ctx.loi_ref("Income Tax Act (Cap. 123)")),
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
