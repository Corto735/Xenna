use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::lu_cotisations::*;

pub fn generer_bulletin_lu(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let cotisations = vec![
        lu_ap(brut, ctx),
        lu_am(brut, ctx),
        lu_ad(brut, ctx),
        lu_aa(brut, ctx),
        lu_me(brut, ctx),
    ];

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(),
        salarie,
    }
}
