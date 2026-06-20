// ── Autriche — Sozialversicherung (plafonnée) + Lohnsteuer progressif ────────────
//
// 2025 :
//   • Sozialversicherung salarié 18,07 % (PV 10,25 + KV 3,87 + ALV 2,95 + AK 0,50
//     + WBF 0,50), employeur ≈ 21,03 %, sur l'assiette plafonnée à la
//     Höchstbeitragsgrundlage (6 450 €/mois en 2025).
//   • Lohnsteuer : barème progressif 2025 (0 / 20 / 30 / 40 / 48 / 50 / 55 %),
//     assiette = revenu après cotisations sociales salariales.
//
// 2026 :
//   • Höchstbeitragsgrundlage portée à 6 930 €/mois (contre 6 450 en 2025).
//   • Barème Lohnsteuer relevé des 2/3 de l'inflation (+1,733 %) sauf la tranche à 55 %
//     (seuils 13 539 / 21 992 / 36 458 / 70 365 / 104 859 / 1 000 000 €).
//   • Taux SV salarié inchangé (18,07 %, lu en base).
//
// Simplifications documentées (net prudent) : 13ᵉ/14ᵉ mois (Sonderzahlungen, imposés
// à 6 %) non modélisés ; crédits AVAB/AEAB et Verkehrsabsetzbetrag non modélisés.
// Sources : ÖGK (taux SV, Höchstbeitragsgrundlage 2025-2026) ; BMF (barème Lohnsteuer 2025-2026).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Lohnsteuer annuel (barème progressif, cumul par tranche) selon l'année.
fn lohnsteuer(t: Decimal, annee: i32) -> Decimal {
    if annee >= 2026 {
        return if t <= dec!(13539) {
            Decimal::ZERO
        } else if t <= dec!(21992) {
            (t - dec!(13539)) * dec!(0.20)
        } else if t <= dec!(36458) {
            dec!(1690.60) + (t - dec!(21992)) * dec!(0.30)
        } else if t <= dec!(70365) {
            dec!(6030.40) + (t - dec!(36458)) * dec!(0.40)
        } else if t <= dec!(104859) {
            dec!(19593.20) + (t - dec!(70365)) * dec!(0.48)
        } else if t <= dec!(1000000) {
            dec!(36150.32) + (t - dec!(104859)) * dec!(0.50)
        } else {
            dec!(483720.82) + (t - dec!(1000000)) * dec!(0.55)
        };
    }
    if t <= dec!(13308) {
        Decimal::ZERO
    } else if t <= dec!(21617) {
        (t - dec!(13308)) * dec!(0.20)
    } else if t <= dec!(35836) {
        dec!(1661.80) + (t - dec!(21617)) * dec!(0.30)
    } else if t <= dec!(69166) {
        dec!(5927.50) + (t - dec!(35836)) * dec!(0.40)
    } else if t <= dec!(103072) {
        dec!(19259.50) + (t - dec!(69166)) * dec!(0.48)
    } else if t <= dec!(1000000) {
        dec!(35534.38) + (t - dec!(103072)) * dec!(0.50)
    } else {
        dec!(483998.38) + (t - dec!(1000000)) * dec!(0.55)
    }
}

pub fn generer_bulletin_at(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Autriche : données disponibles pour 2025 et 2026.");
    }

    // Assiette SV plafonnée (Höchstbeitragsgrundlage : 6 450 €/mois en 2025, 6 930 en 2026).
    let hbgl = if annee >= 2026 { dec!(6930) } else { dec!(6450) };
    let assiette_sv = brut.min(hbgl);
    let ts = ctx.taux_sal("AT_SV");
    let tp = ctx.taux_pat("AT_SV");
    let sv_sal = (assiette_sv * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "AT_SV".into(),
        libelle: ctx.libelle("AT_SV", "Sozialversicherung — Cotisations sociales"),
        base: assiette_sv, taux_sal: ts, montant_sal: sv_sal,
        taux_pat: tp, montant_pat: (assiette_sv * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("AT_SV",
            "Sozialversicherung — salarié {ts} % / employeur {tp} % (retraite PV, \
            maladie KV, chômage ALV, AK, WBF). Assiette plafonnée à {hbgl} €/mois \
            (Höchstbeitragsgrundlage). Salarié : {ms} €.")
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{hbgl}", &format!("{:.0}", hbgl))
            .replace("{ms}", &format!("{:.2}", sv_sal)),
        loi_ref: Some(ctx.loi_ref("ASVG")),
    }];

    // Lohnsteuer : base annuelle = (brut − SV salarié) × 12.
    let base_an = ((brut - sv_sal).max(Decimal::ZERO)) * dec!(12);
    let impot_mens = (lohnsteuer(base_an, annee) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "AT_LOHNSTEUER".into(),
        libelle: ctx.libelle("AT_LOHNSTEUER", "Lohnsteuer — Impôt sur le revenu"),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("AT_LOHNSTEUER",
            "Impôt sur le revenu {annee} (annualisé).\n\n\
            Base = (brut − SV salarié) × 12 = {b} €\n\
            Barème 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n\
            (seuils {seuils})\n\
            → {im} €/mois.\n\n\
            Note : 13ᵉ/14ᵉ mois (Sonderzahlungen) et crédits non modélisés (net prudent).\n\
            Source : BMF.")
            .replace("{annee}", &annee.to_string())
            .replace("{b}", &format!("{:.0}", base_an))
            .replace("{im}", &format!("{:.2}", impot_mens))
            .replace("{seuils}", if annee >= 2026 {
                "13 539 / 21 992 / 36 458 / 70 365 / 104 859 / 1 000 000 €"
            } else {
                "13 308 / 21 617 / 35 836 / 69 166 / 103 072 / 1 000 000 €"
            }),
        loi_ref: Some(ctx.loi_ref("EStG 1988")),
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
