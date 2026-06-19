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
// 2026 :
//   • Sodra inchangé (salarié 19,50 % / employeur 1,77 %).
//   • GPM à trois tranches : 20 % jusqu'à 36 VDU (83 237,40 €/an ≈ 6 936,45 €/mois),
//     25 % de 36 à 60 VDU (138 729 €/an ≈ 11 560,75 €/mois), 32 % au-delà.
//   • NPD : formule unifiée 747 − 0,49 × (brut − MMA), MMA 2026 = 1 153 €,
//     plafonné à 747 €, nul à partir de ≈ 2 677 €.
//
// Note : accumulation pension volontaire (+3 %) non modélisée.
// Source : VMI (GPM/NPD 2025 et 2026) ; Sodra (MMA 2026 = 1 153 €, VDU 2026).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// NPD mensuel (montant non imposable) selon l'année.
/// 2026 : formule unifiée 747 − 0,49 × (brut − MMA 1 153 €), plafond 747, nul ≈ 2 677 €.
/// 2025 : barème dégressif à trois segments.
fn npd(brut: Decimal, annee: i32) -> Decimal {
    if annee >= 2026 {
        if brut <= dec!(1153) {
            return dec!(747);
        }
        return (dec!(747) - dec!(0.49) * (brut - dec!(1153))).max(Decimal::ZERO);
    }
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

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Lituanie : données disponibles pour 2025 et 2026.");
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

    // GPM : base = brut − NPD. Barème par tranches mensuelles selon l'année.
    let npd_m = npd(brut, annee);
    let base = (brut - npd_m).max(Decimal::ZERO);
    let gpm = if annee >= 2026 {
        // 2026 : 20 % jusqu'à 36 VDU (6 936,45 €/mois), 25 % de 36 à 60 VDU
        // (11 560,75 €/mois), 32 % au-delà. Les seuils s'appliquent au brut.
        let s1 = dec!(6936.45);
        let s2 = dec!(11560.75);
        let t3 = (brut - s2).max(Decimal::ZERO);
        let t2 = (brut.min(s2) - s1).max(Decimal::ZERO);
        let t1 = (base - t2 - t3).max(Decimal::ZERO);
        (t1 * dec!(0.20) + t2 * dec!(0.25) + t3 * dec!(0.32)).round_dp(2)
    } else {
        // 2025 : 20 % jusqu'à ≈ 10 540 €/mois, 32 % au-delà.
        let part_haute = (brut - dec!(10540)).max(Decimal::ZERO);
        let part_basse = (base - part_haute).max(Decimal::ZERO);
        (part_basse * dec!(0.20) + part_haute * dec!(0.32)).round_dp(2)
    };
    let taux_imp = if brut > Decimal::ZERO { (gpm / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "LT_GPM".into(),
        libelle: "GPM — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: gpm,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu {annee} (GPM).\n\n\
            NPD (non imposable) : {npd:.2} €\n\
            Base = brut − NPD = {b:.2} €\n\
            {bareme}\n\
            → {gpm:.2} €/mois.\n\n\
            Source : VMI.",
            annee = annee, npd = npd_m, b = base, gpm = gpm,
            bareme = if annee >= 2026 {
                "Barème 20 % (≤ 6 936 €/mois) / 25 % (jusqu'à 11 561 €/mois) / 32 % au-delà"
            } else {
                "Taux 20 % (jusqu'à ≈ 10 540 €/mois) puis 32 % au-delà"
            },
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
        devise: "EUR".into(), absence: None, heures_sup: None, salarie,
    }
}
