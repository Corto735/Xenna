// ── Grèce — EFKA (plafonné) + impôt sur le revenu progressif ─────────────────────
//
// 2025 :
//   • EFKA salarié 13,87 % / employeur 22,29 %, sur l'assiette plafonnée à
//     7 572,62 €/mois (plafond 2025).
//   • Impôt sur le revenu : 9 / 22 / 28 / 36 / 44 % (seuils 10 000 / 20 000 /
//     30 000 / 40 000 €), assiette = revenu après cotisations salariales.
//   • Réduction d'impôt salarié : 777 € (sans enfant, simplifiée).
//
// 2026 (réforme Ν. 5246/2025) : baisse des taux intermédiaires de 2 points et nouveau
// palier 39 % de 40 000 à 60 000 € ; le 44 % ne s'applique plus qu'au-delà de 60 000 €.
// Barème 9 / 20 / 26 / 34 / 39 / 44 %. Taux EFKA réduits (salarié 13,37 % / employeur
// 21,79 %, lus en base) et plafond porté à 7 761,94 €/mois.
// Simplification (net prudent) : réduction d'impôt fixée à 777 € (majorations pour
// enfants, dégressivité et taux 0 % des moins de 25 ans non modélisés).
// Source : EFKA ; AADE (barème 2025 et 2026, Ν. 5246/2025).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Impôt sur le revenu annuel (barème progressif), avant réduction, selon l'année.
fn impot_brut(t: Decimal, annee: i32) -> Decimal {
    if annee >= 2026 {
        return if t <= dec!(10000) {
            t * dec!(0.09)
        } else if t <= dec!(20000) {
            dec!(900) + (t - dec!(10000)) * dec!(0.20)
        } else if t <= dec!(30000) {
            dec!(2900) + (t - dec!(20000)) * dec!(0.26)
        } else if t <= dec!(40000) {
            dec!(5500) + (t - dec!(30000)) * dec!(0.34)
        } else if t <= dec!(60000) {
            dec!(8900) + (t - dec!(40000)) * dec!(0.39)
        } else {
            dec!(16700) + (t - dec!(60000)) * dec!(0.44)
        };
    }
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

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "GR",
            "Grèce : données disponibles pour 2025 et 2026.", ctx);
    }

    // EFKA sur assiette plafonnée (7 572,62 €/mois en 2025, 7 761,94 € en 2026).
    let plafond = if annee >= 2026 { dec!(7761.94) } else { dec!(7572.62) };
    let assiette = brut.min(plafond);
    let ts = ctx.taux_sal("GR_EFKA");
    let tp = ctx.taux_pat("GR_EFKA");
    let efka_sal = (assiette * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "GR_EFKA".into(),
        libelle: ctx.libelle("GR_EFKA", "EFKA — Cotisations sociales"),
        base: assiette, taux_sal: ts, montant_sal: efka_sal,
        taux_pat: tp, montant_pat: (assiette * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("GR_EFKA",
            "EFKA — salarié {ts} % / employeur {tp} % (retraite, maladie, \
            complémentaire). Assiette plafonnée à {pl} €/mois. Salarié : {ms} €.")
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{pl}", &format!("{:.2}", plafond))
            .replace("{ms}", &format!("{:.2}", efka_sal)),
        loi_ref: Some(ctx.loi_ref("Ν. 4387/2016 (EFKA)")),
    }];

    // Impôt : base annuelle = (brut − EFKA) × 12 ; réduction 777 €.
    let base_an = ((brut - efka_sal).max(Decimal::ZERO)) * dec!(12);
    let impot_an = (impot_brut(base_an, annee) - dec!(777)).max(Decimal::ZERO);
    let impot_mens = (impot_an / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "GR_FOROS".into(),
        libelle: ctx.libelle("GR_FOROS", "Φόρος εισοδήματος — Impôt sur le revenu"),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("GR_FOROS",
            "Impôt sur le revenu {annee} (annualisé).\n\n\
            Base = (brut − EFKA) × 12 = {b} €\n\
            {bareme}\n\
            − réduction salarié 777 € → {im} €/mois.\n\n\
            Note : majorations pour enfants non modélisées (net prudent).\n\
            Source : AADE.")
            .replace("{annee}", &annee.to_string())
            .replace("{b}", &format!("{:.0}", base_an))
            .replace("{im}", &format!("{:.2}", impot_mens))
            .replace("{bareme}", if annee >= 2026 {
                "Barème 9 / 20 / 26 / 34 / 39 / 44 % (seuils 10 000 / 20 000 / 30 000 / 40 000 / 60 000 €)"
            } else {
                "Barème 9 / 22 / 28 / 36 / 44 % (seuils 10 000 / 20 000 / 30 000 / 40 000 €)"
            }),
        loi_ref: Some(ctx.loi_ref("Ν. 4172/2013 (Κ.Φ.Ε.)")),
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
