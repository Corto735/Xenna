// ── Croatie — mirovinsko (retraite, salarié) + porez na dohodak ──────────────────
//
// 2025 :
//   • Mirovinsko osiguranje (retraite) salarié 20 % (1er pilier 15 % + 2ᵉ pilier 5 %).
//   • Zdravstveno osiguranje (santé) employeur 16,5 %.
//   • Porez na dohodak : barème 2025 (réforme 2024, suppression de la prirez/surtaxe
//     communale, taux fixés par commune). Représentatif : 20 % jusqu'à 5 000 €/mois,
//     30 % au-delà. Abattement personnel (osobni odbitak) 600 €/mois.
//   • Assiette de l'impôt = revenu après cotisation retraite et abattement.
//
// Simplification : taux communaux représentatifs (20 % / 30 %). Source : HZMO ;
// Porezna uprava (barème 2025).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn generer_bulletin_hr(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Croatie : données disponibles pour 2025.");
    }

    // Retraite salarié 20 % ; santé 16,5 % employeur.
    let ts = ctx.taux_sal("HR_MIROVINSKO");
    let retraite = (brut * ts).round_dp(2);
    let tp = ctx.taux_pat("HR_ZDRAVSTVENO");
    let mut cotisations = vec![
        LigneCotisation {
            code: "HR_MIROVINSKO".into(), libelle: "Mirovinsko osiguranje — Retraite".into(),
            base: brut, taux_sal: ts, montant_sal: retraite,
            taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
            categorie: "Sécurité sociale".into(),
            explication: format!(
                "Retraite — {:.2} % salarié (1er pilier 15 % + 2ᵉ pilier 5 %). Salarié : {:.2} €.",
                ts * dec!(100), retraite),
            loi_ref: Some("Zakon o mirovinskom osiguranju".into()),
        },
        LigneCotisation {
            code: "HR_ZDRAVSTVENO".into(), libelle: "Zdravstveno osiguranje — Santé (employeur)".into(),
            base: brut, taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
            taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
            categorie: "Sécurité sociale".into(),
            explication: format!("Assurance maladie — {:.2} % à la charge de l'employeur.",
                tp * dec!(100)),
            loi_ref: Some("Zakon o obveznom zdravstvenom osiguranju".into()),
        },
    ];

    // Porez na dohodak : base = brut − retraite − abattement 600 € ; 20 % / 30 %.
    let base = (brut - retraite - dec!(600)).max(Decimal::ZERO);
    let part_haute = (base - dec!(5000)).max(Decimal::ZERO);
    let part_basse = base - part_haute;
    let impot = (part_basse * dec!(0.20) + part_haute * dec!(0.30)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "HR_POREZ".into(),
        libelle: "Porez na dohodak — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025.\n\n\
            Base = brut − retraite − abattement 600 € = {b:.2} €\n\
            20 % jusqu'à 5 000 €/mois, 30 % au-delà → {im:.2} €/mois.\n\n\
            Note : taux communaux représentatifs. Source : Porezna uprava.",
            b = base, im = impot,
        ),
        loi_ref: Some("Zakon o porezu na dohodak".into()),
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
