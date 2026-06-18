// ── Tchéquie — sociální + zdravotní pojištění + daň z příjmů ─────────────────────
//
// 2025 :
//   • Sociální pojištění salarié 7,1 % (retraite 6,5 % + maladie 0,6 %) / employeur 24,8 %.
//   • Zdravotní pojištění (santé) salarié 4,5 % / employeur 9 %.
//   • Daň z příjmů : 15 % jusqu'à 36× le salaire moyen (≈ 139 671 CZK/mois en 2025),
//     23 % au-delà. Sleva na poplatníka (crédit d'impôt) : 2 570 CZK/mois.
//
// La « super-hrubá mzda » a été supprimée en 2021 : la base d'imposition est le brut.
// Source : ČSSZ (sociální) ; VZP (zdravotní) ; Finanční správa (daň 2025).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

fn ligne_cot(code: &str, libelle: &str, base: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    LigneCotisation {
        code: code.into(), libelle: libelle.into(), base,
        taux_sal: ts, montant_sal: (base * ts).round_dp(2),
        taux_pat: tp, montant_pat: (base * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "{libelle}. Salarié {ts:.2} % / employeur {tp:.2} %.",
            ts = ts * dec!(100), tp = tp * dec!(100),
        ),
        loi_ref: Some("Zákony o pojistném (ČR)".into()),
    }
}

pub fn generer_bulletin_cz(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "CZK", "Tchéquie : données disponibles pour 2025.");
    }

    let mut cotisations = vec![
        ligne_cot("CZ_SOCIAL",    "Sociální pojištění — Sécurité sociale", brut, ctx),
        ligne_cot("CZ_ZDRAVOTNI", "Zdravotní pojištění — Assurance maladie", brut, ctx),
    ];

    // Daň z příjmů : 15 % jusqu'à 139 671 CZK/mois, 23 % au-delà ; sleva 2 570 CZK.
    let seuil = dec!(139671);
    let part_haute = (brut - seuil).max(Decimal::ZERO);
    let part_basse = brut - part_haute;
    let impot_brut = part_basse * dec!(0.15) + part_haute * dec!(0.23);
    let impot = (impot_brut - dec!(2570)).max(Decimal::ZERO).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "CZ_DAN".into(),
        libelle: "Daň z příjmů — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025.\n\n\
            15 % jusqu'à 139 671 CZK/mois, 23 % au-delà = {ib:.2} CZK\n\
            − sleva na poplatníka 2 570 CZK = {im:.2} CZK/mois.\n\n\
            Source : Finanční správa.",
            ib = impot_brut.round_dp(2), im = impot,
        ),
        loi_ref: Some("Zákon o daních z příjmů".into()),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "CZK".into(), absence: None, salarie,
    }
}
