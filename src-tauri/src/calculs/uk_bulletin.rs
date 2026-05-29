use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::uk_cotisations::{uk_ni_sal, uk_ni_pat, uk_income_tax};

pub fn generer_bulletin_uk(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;

    let cotisations = vec![
        uk_ni_sal(brut, ctx),
        uk_ni_pat(brut, ctx),
        uk_income_tax(brut, ctx),
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
        devise: "GBP".into(),
        salarie,
    }
}
