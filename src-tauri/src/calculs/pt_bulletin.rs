use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::pt_contribuicoes::*;
use super::pt_irs::irs_retencao;

pub fn generer_bulletin_pt(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;

    let cotisations = vec![
        seguranca_social(brut, ctx),
        acidentes_trabalho(brut, ctx),
        fct(brut, ctx),
        fgct(brut, ctx),
        irs_retencao(brut, ctx),
    ];

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();

    // Net imposable ≈ net à payer (l'IRS retenu est une avance, pas une charge définitive ;
    // pour la simulation on l'inclut dans le calcul du net mensuel perçu).
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
