// ── Autriche — Sozialversicherung (plafonnée) + Lohnsteuer progressif ────────────
//
// 2025 :
//   • Sozialversicherung salarié 18,07 % (PV 10,25 + KV 3,87 + ALV 2,95 + AK 0,50
//     + WBF 0,50), employeur ≈ 21,03 %, sur l'assiette plafonnée à la
//     Höchstbeitragsgrundlage (6 450 €/mois en 2025).
//   • Lohnsteuer : barème progressif 2025 (0 / 20 / 30 / 40 / 48 / 50 / 55 %),
//     assiette = revenu après cotisations sociales salariales.
//
// Simplifications documentées (net prudent) : 13ᵉ/14ᵉ mois (Sonderzahlungen, imposés
// à 6 %) non modélisés ; crédits AVAB/AEAB et Verkehrsabsetzbetrag non modélisés.
// Sources : ÖGK (taux SV 2025, Höchstbeitragsgrundlage) ; BMF (barème Lohnsteuer 2025).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Lohnsteuer annuel 2025 (barème progressif, cumul par tranche).
fn lohnsteuer(t: Decimal) -> Decimal {
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

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Autriche : données disponibles pour 2025.");
    }

    // Assiette SV plafonnée (Höchstbeitragsgrundlage 6 450 €/mois en 2025).
    let assiette_sv = brut.min(dec!(6450));
    let ts = ctx.taux_sal("AT_SV");
    let tp = ctx.taux_pat("AT_SV");
    let sv_sal = (assiette_sv * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "AT_SV".into(),
        libelle: "Sozialversicherung — Cotisations sociales".into(),
        base: assiette_sv, taux_sal: ts, montant_sal: sv_sal,
        taux_pat: tp, montant_pat: (assiette_sv * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "Sozialversicherung — salarié {ts:.2} % / employeur {tp:.2} % (retraite PV, \
            maladie KV, chômage ALV, AK, WBF). Assiette plafonnée à 6 450 €/mois \
            (Höchstbeitragsgrundlage). Salarié : {ms:.2} €.",
            ts = ts * dec!(100), tp = tp * dec!(100), ms = sv_sal,
        ),
        loi_ref: Some("ASVG".into()),
    }];

    // Lohnsteuer : base annuelle = (brut − SV salarié) × 12.
    let base_an = ((brut - sv_sal).max(Decimal::ZERO)) * dec!(12);
    let impot_mens = (lohnsteuer(base_an) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "AT_LOHNSTEUER".into(),
        libelle: "Lohnsteuer — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025 (annualisé).\n\n\
            Base = (brut − SV salarié) × 12 = {b:.0} €\n\
            Barème 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n\
            (seuils 13 308 / 21 617 / 35 836 / 69 166 / 103 072 / 1 000 000 €)\n\
            → {im:.2} €/mois.\n\n\
            Note : 13ᵉ/14ᵉ mois (Sonderzahlungen) et crédits non modélisés (net prudent).\n\
            Source : BMF.",
            b = base_an, im = impot_mens,
        ),
        loi_ref: Some("EStG 1988".into()),
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
