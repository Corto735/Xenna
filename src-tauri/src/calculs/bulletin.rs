use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::cotisations::*;

pub fn generer_bulletin(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let mut cotisations = Vec::new();

    cotisations.push(ss_maladie(brut, ctx));
    cotisations.push(ss_vieillesse_plafonnee(brut, ctx));
    cotisations.push(ss_vieillesse_deplafonnee(brut, ctx));
    cotisations.push(famille(brut, ctx));
    cotisations.push(accident_travail(brut, ctx));
    cotisations.push(chomage(brut, ctx));
    cotisations.extend(csg_contributions(brut, ctx));
    cotisations.extend(retraite_complementaire(brut, &salarie.statut, ctx));

    if salarie.alsace_moselle {
        if let Some(am) = maladie_alsace_moselle(brut, ctx) {
            cotisations.push(am);
        }
    }

    if let Some(fillon) = reduction_fillon(brut, ctx) {
        cotisations.push(fillon);
    }

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();

    // Net imposable = brut - cotisations salariales
    // (la CSG non déductible et la CRDS sont dans total_sal mais ne réduisent pas le net imposable)
    let csg_non_ded_et_crds: Decimal = cotisations.iter()
        .filter(|c| c.code == "CSG_NON_DEDUCTIBLE" || c.code == "CRDS")
        .map(|c| c.montant_sal)
        .sum();

    let net_imposable = (brut - total_sal + csg_non_ded_et_crds).round_dp(2);
    let net_a_payer   = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        salarie,
    }
}
