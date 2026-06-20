// ── Tchéquie — sociální + zdravotní pojištění + daň z příjmů ─────────────────────
//
// 2025 :
//   • Sociální pojištění salarié 7,1 % (retraite 6,5 % + maladie 0,6 %) / employeur 24,8 %.
//   • Zdravotní pojištění (santé) salarié 4,5 % / employeur 9 %.
//   • Daň z příjmů : 15 % jusqu'à 36× le salaire moyen (≈ 139 671 CZK/mois en 2025),
//     23 % au-delà. Sleva na poplatníka (crédit d'impôt) : 2 570 CZK/mois.
//
// 2026 : taux sociaux/santé et sleva (2 570 CZK/mois) inchangés ; seul le seuil de la
// tranche à 23 % évolue (36× salaire moyen 48 967 CZK → 146 901 CZK/mois, contre
// 139 671 en 2025).
// La « super-hrubá mzda » a été supprimée en 2021 : la base d'imposition est le brut.
// Source : ČSSZ (sociální) ; VZP (zdravotní) ; Finanční správa (daň 2025 et 2026).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

fn ligne_cot(code: &str, libelle: &str, base: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    let lib = ctx.libelle(code, libelle);
    let explication = ctx.expl("CZ_GENERIC", "{libelle}. Salarié {ts} % / employeur {tp} %.")
        .replace("{libelle}", &lib)
        .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
        .replace("{tp}", &format!("{:.2}", tp * dec!(100)));
    LigneCotisation {
        code: code.into(), libelle: lib, base,
        taux_sal: ts, montant_sal: (base * ts).round_dp(2),
        taux_pat: tp, montant_pat: (base * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("Zákony o pojistném (ČR)")),
    }
}

pub fn generer_bulletin_cz(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "CZK", "Tchéquie : données disponibles pour 2025 et 2026.");
    }

    let mut cotisations = vec![
        ligne_cot("CZ_SOCIAL",    "Sociální pojištění — Sécurité sociale", brut, ctx),
        ligne_cot("CZ_ZDRAVOTNI", "Zdravotní pojištění — Assurance maladie", brut, ctx),
    ];

    // Daň z příjmů : 15 % jusqu'au seuil, 23 % au-delà ; sleva 2 570 CZK.
    let seuil = if annee >= 2026 { dec!(146901) } else { dec!(139671) };
    let part_haute = (brut - seuil).max(Decimal::ZERO);
    let part_basse = brut - part_haute;
    let impot_brut = part_basse * dec!(0.15) + part_haute * dec!(0.23);
    let impot = (impot_brut - dec!(2570)).max(Decimal::ZERO).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "CZ_DAN".into(),
        libelle: ctx.libelle("CZ_DAN", "Daň z příjmů — Impôt sur le revenu"),
        base: brut, taux_sal: taux_imp, montant_sal: impot,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("CZ_DAN",
            "Impôt sur le revenu {annee}.\n\n\
            15 % jusqu'à {seuil} CZK/mois, 23 % au-delà = {ib} CZK\n\
            − sleva na poplatníka 2 570 CZK = {im} CZK/mois.\n\n\
            Source : Finanční správa.")
            .replace("{annee}", &annee.to_string())
            .replace("{seuil}", &format!("{:.0}", seuil))
            .replace("{ib}", &format!("{:.2}", impot_brut.round_dp(2)))
            .replace("{im}", &format!("{:.2}", impot)),
        loi_ref: Some(ctx.loi_ref("Zákon o daních z příjmů")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "CZK".into(), absence: None, heures_sup: None, salarie,
    }
}
