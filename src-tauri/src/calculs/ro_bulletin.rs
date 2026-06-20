// ── Roumanie — CAS + CASS (salarié) + impôt 10 % ; CAM (employeur) ───────────────
//
// Depuis 2018 (réforme « révolution fiscale », OUG 79/2017), taux figés :
//   • CAS (pension) salarié 25 % + CASS (santé) salarié 10 %.
//   • CAM (contribution assurantielle de travail) employeur 2,25 %.
//   • Impozit pe venit : 10 % proportionnel, sur le revenu après CAS et CASS.
//
// Couverture 2018-2026 (taux constants : CAS 25 % / CASS 10 % / CAM 2,25 % / impôt 10 %).
// Simplification : la déduction personnelle (deducere personală), réservée aux bas
// salaires et dégressive, n'est pas modélisée (net prudent).
// Source : ANAF ; Codul fiscal (Legea 227/2015).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

fn ligne_cot(code: &str, libelle: &str, base: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    let lib = ctx.libelle(code, libelle);
    let explication = ctx.expl("RO_GENERIC", "{libelle}. Salarié {ts} % / employeur {tp} %.")
        .replace("{libelle}", &lib)
        .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
        .replace("{tp}", &format!("{:.2}", tp * dec!(100)));
    LigneCotisation {
        code: code.into(), libelle: lib, base,
        taux_sal: ts, montant_sal: (base * ts).round_dp(2),
        taux_pat: tp, montant_pat: (base * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("Codul fiscal (Legea 227/2015)")),
    }
}

pub fn generer_bulletin_ro(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2018..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "RON", "Roumanie : données disponibles depuis 2018 (réforme OUG 79/2017).");
    }

    let mut cotisations = vec![
        ligne_cot("RO_CAS",  "CAS — Pension", brut, ctx),
        ligne_cot("RO_CASS", "CASS — Assurance santé", brut, ctx),
        ligne_cot("RO_CAM",  "CAM — Contribution de travail (employeur)", brut, ctx),
    ];

    // Impôt : base = brut − CAS − CASS ; 10 %.
    let cot_sal = ctx.taux_sal("RO_CAS") + ctx.taux_sal("RO_CASS");
    let base = (brut * (Decimal::ONE - cot_sal)).max(Decimal::ZERO);
    let impot = (base * dec!(0.10)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "RO_IMPOZIT".into(),
        libelle: ctx.libelle("RO_IMPOZIT", "Impozit pe venit — Impôt sur le revenu (10 %)"),
        base: brut, taux_sal: taux_imp, montant_sal: impot,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("RO_IMPOZIT",
            "Impôt sur le revenu {annee} : 10 % proportionnel (flat tax depuis 2018).\n\n\
            Base = brut − CAS 25 % − CASS 10 % = {b} RON → {im} RON/mois.\n\n\
            Note : déduction personnelle (bas salaires) non modélisée (net prudent).\n\
            Source : ANAF.")
            .replace("{annee}", &annee.to_string())
            .replace("{b}", &format!("{:.2}", base))
            .replace("{im}", &format!("{:.2}", impot)),
        loi_ref: Some(ctx.loi_ref("Codul fiscal (Legea 227/2015)")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "RON".into(), absence: None, heures_sup: None, salarie,
    }
}
