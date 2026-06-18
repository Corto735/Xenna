// ── Grèce — EFKA (plafonné) + impôt sur le revenu progressif ─────────────────────
//
// 2025 :
//   • EFKA salarié 13,87 % / employeur 22,29 %, sur l'assiette plafonnée à
//     7 572,62 €/mois (plafond 2025).
//   • Impôt sur le revenu : 9 / 22 / 28 / 36 / 44 % (seuils 10 000 / 20 000 /
//     30 000 / 40 000 €), assiette = revenu après cotisations salariales.
//   • Réduction d'impôt salarié : 777 € (sans enfant, simplifiée).
//
// Simplification (net prudent) : réduction d'impôt fixée à 777 € (majorations pour
// enfants et dégressivité non modélisées). Source : EFKA ; AADE (barème 2025).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Impôt sur le revenu annuel 2025 (barème progressif), avant réduction.
fn impot_brut(t: Decimal) -> Decimal {
    if t <= dec!(10000) {
        t * dec!(0.09)
    } else if t <= dec!(20000) {
        dec!(900) + (t - dec!(10000)) * dec!(0.22)
    } else if t <= dec!(30000) {
        dec!(3100) + (t - dec!(20000)) * dec!(0.28)
    } else if t <= dec!(40000) {
        dec!(5900) + (t - dec!(30000)) * dec!(0.36)
    } else {
        dec!(9500) + (t - dec!(40000)) * dec!(0.44)
    }
}

pub fn generer_bulletin_gr(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Grèce : données disponibles pour 2025.");
    }

    // EFKA sur assiette plafonnée (7 572,62 €/mois).
    let assiette = brut.min(dec!(7572.62));
    let ts = ctx.taux_sal("GR_EFKA");
    let tp = ctx.taux_pat("GR_EFKA");
    let efka_sal = (assiette * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "GR_EFKA".into(),
        libelle: "EFKA — Cotisations sociales".into(),
        base: assiette, taux_sal: ts, montant_sal: efka_sal,
        taux_pat: tp, montant_pat: (assiette * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "EFKA — salarié {ts:.2} % / employeur {tp:.2} % (retraite, maladie, \
            complémentaire). Assiette plafonnée à 7 572,62 €/mois. Salarié : {ms:.2} €.",
            ts = ts * dec!(100), tp = tp * dec!(100), ms = efka_sal,
        ),
        loi_ref: Some("Ν. 4387/2016 (EFKA)".into()),
    }];

    // Impôt : base annuelle = (brut − EFKA) × 12 ; réduction 777 €.
    let base_an = ((brut - efka_sal).max(Decimal::ZERO)) * dec!(12);
    let impot_an = (impot_brut(base_an) - dec!(777)).max(Decimal::ZERO);
    let impot_mens = (impot_an / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "GR_FOROS".into(),
        libelle: "Φόρος εισοδήματος — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025 (annualisé).\n\n\
            Base = (brut − EFKA) × 12 = {b:.0} €\n\
            Barème 9 / 22 / 28 / 36 / 44 % (seuils 10 000 / 20 000 / 30 000 / 40 000 €)\n\
            − réduction salarié 777 € → {im:.2} €/mois.\n\n\
            Note : majorations pour enfants non modélisées (net prudent).\n\
            Source : AADE.",
            b = base_an, im = impot_mens,
        ),
        loi_ref: Some("Ν. 4172/2013 (Κ.Φ.Ε.)".into()),
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
