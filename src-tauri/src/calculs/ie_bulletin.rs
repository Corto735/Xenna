// ── Irlande — PRSI + USC + Income Tax (PAYE) ─────────────────────────────────────
//
// 2025 :
//   • PRSI (Class A) salarié 4,1 % / employeur 11,15 %.
//   • USC (Universal Social Charge) : 0,5 % / 2 % / 3 % / 8 % (seuils annuels
//     12 012 / 27 382 / 70 044 €).
//   • Income Tax : 20 % jusqu'à 44 000 €/an, 40 % au-delà ; crédits d'impôt
//     (personnel 2 000 € + PAYE 2 000 € = 4 000 €).
//
// 2026 (Budget 2026) : USC inchangé sauf la bande à 2 % dont le plafond passe de
// 27 382 € à 28 700 € ; Income Tax et crédits inchangés (tranche 44 000 €, crédits 4 000 €).
//
// PRSI Class A (taux en base, périodes du 1ᵉʳ octobre) — guide SW14 de janvier 2026 :
//   jusqu'au 30/09/2025 : salarié 4,1 %  / employeur 11,15 % (8,90 % sous le seuil)
//   01/10/2025-30/09/2026 : 4,2 %  / 11,25 % (9,00 % sous le seuil)
//   dès le 01/10/2026      : 4,35 % / 11,40 % (9,15 % sous le seuil)
// Le taux employeur réduit (sous-classes A0/AX/AL) vaut 2,25 points de moins que le
// taux plein, jusqu'à 527 €/semaine en 2025 et 552 €/semaine en 2026. Le salarié ne
// cotise pas jusqu'à 352 €/semaine (A0) ; entre 352,01 et 424 € (AX), un crédit PRSI de
// 12 €/semaine, diminué d'un sixième de l'excédent sur 352,01 €, s'impute.
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
            salarie, brut, "EUR", "IE",
            "Irlande : données disponibles pour 2025 et 2026.", ctx);
    }

    let g = brut * dec!(12);

    // PRSI (taux pleins lus en base), raisonné à la semaine comme le barème.
    let hebdo = brut * dec!(12) / dec!(52);
    let ts_plein = ctx.taux_sal("IE_PRSI");
    let tp_plein = ctx.taux_pat("IE_PRSI");
    let seuil_pat = if annee >= 2026 { dec!(552) } else { dec!(527) };
    let tp = if hebdo <= seuil_pat { tp_plein - dec!(0.0225) } else { tp_plein };
    let prsi_hebdo = if hebdo <= dec!(352) {
        Decimal::ZERO
    } else if hebdo <= dec!(424) {
        let credit = (dec!(12) - (hebdo - dec!(352.01)) / dec!(6)).max(Decimal::ZERO);
        (hebdo * ts_plein - credit).max(Decimal::ZERO)
    } else {
        hebdo * ts_plein
    };
    let prsi_sal = (prsi_hebdo * dec!(52) / dec!(12)).round_dp(2);
    let ts = if brut > Decimal::ZERO { (prsi_sal / brut).round_dp(4) } else { Decimal::ZERO };
    let mut cotisations = vec![LigneCotisation {
        code: "IE_PRSI".into(),
        libelle: ctx.libelle("IE_PRSI", "PRSI (Class A) — Cotisation sociale"),
        base: brut, taux_sal: ts, montant_sal: prsi_sal,
        taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("IE_PRSI",
            "PRSI Class A — salarié {ts} % / employeur {tp} %. Salarié : {ms} €.")
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{ms}", &format!("{:.2}", prsi_sal)),
        loi_ref: Some(ctx.loi_ref("Social Welfare Consolidation Act 2005")),
    }];

    // USC (annualisé).
    let usc_mens = (usc(g, annee) / dec!(12)).round_dp(2);
    let usc_taux = if brut > Decimal::ZERO { (usc_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "IE_USC".into(),
        libelle: ctx.libelle("IE_USC", "Universal Social Charge (USC)"),
        base: brut, taux_sal: usc_taux, montant_sal: usc_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("IE_USC",
            "USC {annee} : 0,5 % / 2 % / 3 % / 8 % (seuils {seuils}).\n\
            Revenu annuel {g} € → {im} €/mois.")
            .replace("{annee}", &annee.to_string())
            .replace("{seuils}", if annee >= 2026 { "12 012 / 28 700 / 70 044 €" } else { "12 012 / 27 382 / 70 044 €" })
            .replace("{g}", &format!("{:.0}", g))
            .replace("{im}", &format!("{:.2}", usc_mens)),
        loi_ref: Some(ctx.loi_ref("Finance Act (USC)")),
    });

    // Income Tax (PAYE), annualisé, après crédits.
    let it_mens = (income_tax(g) / dec!(12)).round_dp(2); // bandes/crédits inchangés en 2026
    let it_taux = if brut > Decimal::ZERO { (it_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "IE_PAYE".into(),
        libelle: ctx.libelle("IE_PAYE", "Income Tax (PAYE) — Impôt sur le revenu"),
        base: brut, taux_sal: it_taux, montant_sal: it_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("IE_PAYE",
            "Impôt sur le revenu {annee} (annualisé).\n\n\
            20 % jusqu'à 44 000 €/an, 40 % au-delà − crédits 4 000 € (personnel + PAYE)\n\
            Revenu annuel {g} € → {im} €/mois.\n\n\
            Note : crédits d'un salarié célibataire. Source : Revenue.")
            .replace("{annee}", &annee.to_string())
            .replace("{g}", &format!("{:.0}", g))
            .replace("{im}", &format!("{:.2}", it_mens)),
        loi_ref: Some(ctx.loi_ref("Taxes Consolidation Act 1997")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(), absence: None, heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), pas: None, droits: None, alertes: Vec::new(), evolutions: Vec::new(), salarie,
    }
}
