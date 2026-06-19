// ── Chypre — assurance sociale (plafonnée) + GESY + impôt sur le revenu ──────────
//
// 2025 :
//   • Assurance sociale (Κοινωνικές Ασφαλίσεις) salarié 8,8 % / employeur 8,8 %,
//     sur l'assiette plafonnée à 5 551 €/mois (max insurable earnings 2025).
//   • GESY (système national de santé) salarié 2,65 % / employeur 2,90 % (non plafonné
//     en pratique ici).
//   • Impôt sur le revenu : 0 / 20 / 25 / 30 / 35 % (seuils 19 500 / 28 000 / 36 300 /
//     60 000 €), assiette = revenu après cotisations salariales (déductibles).
// 2026 (réforme fiscale) : seuil exonéré porté à 22 000 € et nouveaux paliers
// (0 / 20 / 25 / 30 / 35 %, seuils 22 000 / 32 000 / 42 000 / 72 000 €). Taux d'assurance
// sociale (8,8 %) et GESY inchangés ; plafond d'assurance sociale porté à 5 742 €/mois.
// Source : Υπηρεσίες Κοινωνικών Ασφαλίσεων ; Τμήμα Φορολογίας (barème 2025 et 2026).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Impôt sur le revenu annuel (barème progressif) selon l'année.
fn impot(t: Decimal, annee: i32) -> Decimal {
    if annee >= 2026 {
        return if t <= dec!(22000) {
            Decimal::ZERO
        } else if t <= dec!(32000) {
            (t - dec!(22000)) * dec!(0.20)
        } else if t <= dec!(42000) {
            dec!(2000) + (t - dec!(32000)) * dec!(0.25)
        } else if t <= dec!(72000) {
            dec!(4500) + (t - dec!(42000)) * dec!(0.30)
        } else {
            dec!(13500) + (t - dec!(72000)) * dec!(0.35)
        };
    }
    if t <= dec!(19500) {
        Decimal::ZERO
    } else if t <= dec!(28000) {
        (t - dec!(19500)) * dec!(0.20)
    } else if t <= dec!(36300) {
        dec!(1700) + (t - dec!(28000)) * dec!(0.25)
    } else if t <= dec!(60000) {
        dec!(3775) + (t - dec!(36300)) * dec!(0.30)
    } else {
        dec!(10885) + (t - dec!(60000)) * dec!(0.35)
    }
}

pub fn generer_bulletin_cy(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Chypre : données disponibles pour 2025 et 2026.");
    }

    // Assurance sociale (plafonnée : 5 551 €/mois en 2025, 5 742 € en 2026) + GESY (non plafonné).
    let plafond_si = if annee >= 2026 { dec!(5742) } else { dec!(5551) };
    let assiette_si = brut.min(plafond_si);
    let ts_si = ctx.taux_sal("CY_SI");
    let tp_si = ctx.taux_pat("CY_SI");
    let si_sal = (assiette_si * ts_si).round_dp(2);
    let ts_g = ctx.taux_sal("CY_GESY");
    let tp_g = ctx.taux_pat("CY_GESY");
    let gesy_sal = (brut * ts_g).round_dp(2);

    let mut cotisations = vec![
        LigneCotisation {
            code: "CY_SI".into(), libelle: "Κοινωνικές Ασφαλίσεις — Assurance sociale".into(),
            base: assiette_si, taux_sal: ts_si, montant_sal: si_sal,
            taux_pat: tp_si, montant_pat: (assiette_si * tp_si).round_dp(2),
            categorie: "Sécurité sociale".into(),
            explication: format!(
                "Assurance sociale — salarié {:.2} % / employeur {:.2} %. Assiette plafonnée \
                à {:.0} €/mois.", ts_si * dec!(100), tp_si * dec!(100), plafond_si),
            loi_ref: Some("Περί Κοινωνικών Ασφαλίσεων Νόμος".into()),
        },
        LigneCotisation {
            code: "CY_GESY".into(), libelle: "ΓΕΣΥ — Système national de santé".into(),
            base: brut, taux_sal: ts_g, montant_sal: gesy_sal,
            taux_pat: tp_g, montant_pat: (brut * tp_g).round_dp(2),
            categorie: "Sécurité sociale".into(),
            explication: format!(
                "GESY (santé) — salarié {:.2} % / employeur {:.2} %.",
                ts_g * dec!(100), tp_g * dec!(100)),
            loi_ref: Some("Περί Γενικού Συστήματος Υγείας Νόμος".into()),
        },
    ];

    // Impôt : base annuelle = (brut − cotisations salariales) × 12.
    let base_an = ((brut - si_sal - gesy_sal).max(Decimal::ZERO)) * dec!(12);
    let impot_mens = (impot(base_an, annee) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "CY_FOROS".into(),
        libelle: "Φόρος εισοδήματος — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu {annee} (annualisé).\n\n\
            Base = (brut − cotisations) × 12 = {b:.0} €\n\
            {bareme}\n\
            → {im:.2} €/mois.\n\n\
            Source : Τμήμα Φορολογίας.",
            annee = annee, b = base_an, im = impot_mens,
            bareme = if annee >= 2026 {
                "Barème 0 / 20 / 25 / 30 / 35 % (seuils 22 000 / 32 000 / 42 000 / 72 000 €)"
            } else {
                "Barème 0 / 20 / 25 / 30 / 35 % (seuils 19 500 / 28 000 / 36 300 / 60 000 €)"
            },
        ),
        loi_ref: Some("Περί Φορολογίας του Εισοδήματος Νόμος".into()),
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
