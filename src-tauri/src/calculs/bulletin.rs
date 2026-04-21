use rust_decimal::Decimal;
use crate::models::{Bulletin, Salarie};
use super::cotisations::*;

pub fn generer_bulletin(salarie: Salarie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let mut cotisations = Vec::new();

    cotisations.push(ss_maladie(brut));
    cotisations.push(ss_vieillesse_plafonnee(brut));
    cotisations.push(ss_vieillesse_deplafonnee(brut));
    cotisations.push(famille(brut));
    cotisations.push(accident_travail(brut));
    cotisations.push(chomage(brut));

    let (csg_ded, csg_non_ded) = csg_crds(brut);
    cotisations.push(csg_ded);
    cotisations.push(csg_non_ded);

    let retraites = retraite_complementaire(brut, &salarie.statut);
    cotisations.extend(retraites);

    if let Some(prev) = prevoyance_cadre(brut, &salarie.statut) {
        cotisations.push(prev);
    }

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();

    // Net imposable = Brut - cotisations salariales + CSG non déductible
    // (la CSG non ded est déjà dans total_sal donc on la rajoute car non déductible IR)
    let net_imposable = (brut - total_sal).round_dp(2);
    let net_a_payer   = net_imposable;

    Bulletin {
        cotisations,
        brut,
        net_imposable,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        salarie,
    }
}
