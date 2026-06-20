// ── Monaco — CAR + CCSS + chômage (pas d'impôt sur le revenu) ────────────────
//
// Salarié secteur privé. Pas d'IR pour les résidents (sauf nationaux français,
// imposés par la France — convention 1963). Côté salarié : CAR 6,85 % + chômage 2,4 %.
// CCSS (maladie/famille) : 100 % patronale. Taux lus en base. Devise EUR.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

fn ligne(code: &str, libelle: &str, categorie: &str, brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    let lib = ctx.libelle(code, libelle);
    let explication = ctx.expl("MC_GENERIC",
        "{libelle} — Caisses Sociales de Monaco.\n\
        Salarié {ts} % / employeur {tp} %. Salarié : {ms} €.\n\n\
        Note : Monaco ne prélève pas d'impôt sur le revenu des résidents \
        (sauf nationaux français — convention fiscale 1963).")
        .replace("{libelle}", &lib)
        .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
        .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
        .replace("{ms}", &format!("{:.2}", (brut * ts).round_dp(2)));
    LigneCotisation {
        code: code.into(), libelle: lib, base: brut,
        taux_sal: ts, montant_sal: (brut * ts).round_dp(2),
        taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
        categorie: categorie.into(),
        explication,
        loi_ref: Some(ctx.loi_ref("Caisses Sociales de Monaco")),
    }
}

pub fn generer_bulletin_mc(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    // CAR 6,85 % et chômage 2,4 % (salariés) stables depuis ~2019 ; pas d'IR → net constant.
    if !(2020..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Monaco : données disponibles pour 2020-2026.");
    }

    let cotisations = vec![
        ligne("MC_CAR",  "CAR — Retraite",         "Retraite",          brut, ctx),
        ligne("MC_CCSS", "CCSS — Maladie/famille", "Sécurité sociale",  brut, ctx),
        ligne("MC_CHOM", "Chômage",                "Chômage",           brut, ctx),
    ];
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
