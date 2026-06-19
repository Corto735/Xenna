// ── Irlande — PRSI + USC + Income Tax (PAYE) ─────────────────────────────────────
//
// 2025 :
//   • PRSI (Class A) salarié 4,1 % / employeur 11,15 %.
//   • USC (Universal Social Charge) : 0,5 % / 2 % / 3 % / 8 % (seuils annuels
//     12 012 / 27 382 / 70 044 €).
//   • Income Tax : 20 % jusqu'à 44 000 €/an, 40 % au-delà ; crédits d'impôt
//     (personnel 2 000 € + PAYE 2 000 € = 4 000 €).
//
// 2026 (Budget 2026) : PRSI Class A salarié 4,2 % (lu en base) ; USC inchangé sauf la
// bande à 2 % dont le plafond passe de 27 382 € à 28 700 € ; Income Tax et crédits
// inchangés (tranche standard 44 000 €, crédits 4 000 €).
// Note : la hausse PRSI de +0,15 % au 1ᵉʳ octobre 2026 n'est pas modélisée (taux annuel
// retenu = 4,2 %, net prudent).
// Simplification : assiette = brut (PRSI non déductible) ; crédits standard d'un
// salarié célibataire. Source : Revenue (barème 2025-2026) ; Department of Social Protection.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// USC annuel (bandes progressives) selon l'année.
fn usc(t: Decimal, annee: i32) -> Decimal {
    if annee >= 2026 {
        // Plafond de la bande à 2 % porté à 28 700 € (Budget 2026).
        return if t <= dec!(12012) {
            t * dec!(0.005)
        } else if t <= dec!(28700) {
            dec!(60.06) + (t - dec!(12012)) * dec!(0.02)
        } else if t <= dec!(70044) {
            dec!(393.82) + (t - dec!(28700)) * dec!(0.03)
        } else {
            dec!(1634.14) + (t - dec!(70044)) * dec!(0.08)
        };
    }
    if t <= dec!(12012) {
        t * dec!(0.005)
    } else if t <= dec!(27382) {
        dec!(60.06) + (t - dec!(12012)) * dec!(0.02)
    } else if t <= dec!(70044) {
        dec!(367.46) + (t - dec!(27382)) * dec!(0.03)
    } else {
        dec!(1647.32) + (t - dec!(70044)) * dec!(0.08)
    }
}

/// Income Tax annuel 2025 (20 % / 40 %) après crédits.
fn income_tax(t: Decimal) -> Decimal {
    let brut = if t <= dec!(44000) {
        t * dec!(0.20)
    } else {
        dec!(8800) + (t - dec!(44000)) * dec!(0.40)
    };
    (brut - dec!(4000)).max(Decimal::ZERO)
}

pub fn generer_bulletin_ie(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Irlande : données disponibles pour 2025 et 2026.");
    }

    let g = brut * dec!(12);

    // PRSI (taux lus en base).
    let ts = ctx.taux_sal("IE_PRSI");
    let tp = ctx.taux_pat("IE_PRSI");
    let prsi_sal = (brut * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "IE_PRSI".into(),
        libelle: "PRSI (Class A) — Cotisation sociale".into(),
        base: brut, taux_sal: ts, montant_sal: prsi_sal,
        taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "PRSI Class A — salarié {ts:.2} % / employeur {tp:.2} %. Salarié : {ms:.2} €.",
            ts = ts * dec!(100), tp = tp * dec!(100), ms = prsi_sal,
        ),
        loi_ref: Some("Social Welfare Consolidation Act 2005".into()),
    }];

    // USC (annualisé).
    let usc_mens = (usc(g, annee) / dec!(12)).round_dp(2);
    let usc_taux = if brut > Decimal::ZERO { (usc_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "IE_USC".into(),
        libelle: "Universal Social Charge (USC)".into(),
        base: brut, taux_sal: usc_taux, montant_sal: usc_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "USC {annee} : 0,5 % / 2 % / 3 % / 8 % (seuils {seuils}).\n\
            Revenu annuel {g:.0} € → {im:.2} €/mois.",
            annee = annee, g = g, im = usc_mens,
            seuils = if annee >= 2026 { "12 012 / 28 700 / 70 044 €" } else { "12 012 / 27 382 / 70 044 €" },
        ),
        loi_ref: Some("Finance Act (USC)".into()),
    });

    // Income Tax (PAYE), annualisé, après crédits.
    let it_mens = (income_tax(g) / dec!(12)).round_dp(2); // bandes/crédits inchangés en 2026
    let it_taux = if brut > Decimal::ZERO { (it_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "IE_PAYE".into(),
        libelle: "Income Tax (PAYE) — Impôt sur le revenu".into(),
        base: brut, taux_sal: it_taux, montant_sal: it_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu {annee} (annualisé).\n\n\
            20 % jusqu'à 44 000 €/an, 40 % au-delà − crédits 4 000 € (personnel + PAYE)\n\
            Revenu annuel {g:.0} € → {im:.2} €/mois.\n\n\
            Note : crédits d'un salarié célibataire. Source : Revenue.",
            annee = annee, g = g, im = it_mens,
        ),
        loi_ref: Some("Taxes Consolidation Act 1997".into()),
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
