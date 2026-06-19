use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::be_cotisations::*;
use super::be_pp::precompte_professionnel;

pub fn generer_bulletin_be(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut   = salarie.salaire_brut;
    let region = salarie.region_be.as_deref().unwrap_or("bruxelles");

    let mut cotisations = vec![
        onss_salarial(brut, ctx),
        onss_patronal(brut, ctx),
        precompte_professionnel(brut, region, ctx),
    ];

    if let Some(be) = bonus_emploi(brut, ctx)           { cotisations.push(be); }
    if let Some(rs) = reduction_structurelle(brut, ctx) { cotisations.push(rs); }

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();

    // net_a_payer = brut - (ONSS_SAL + PP - bonus_emploi)
    // Le bonus emploi est négatif dans montant_sal → se déduit automatiquement
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(),
        absence: None,
        heures_sup: None,
        salarie,
    }
}
