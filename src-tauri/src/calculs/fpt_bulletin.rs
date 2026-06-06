use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::cotisations::{ss_maladie, famille, accident_travail, csg_contributions, maladie_alsace_moselle};
use super::fpt_cotisations::fpt_cnracl;

/// Bulletin pour les agents titulaires de la Fonction Publique Territoriale (FPT).
///
/// Différences vs secteur privé (France) :
///   - CNRACL remplace SS_VIEILLESSE + AGIRC-ARRCO
///   - Pas de cotisation chômage (titulaires : emploi garanti)
///   - Pas de réduction Fillon (employeurs publics exclus du dispositif)
///   - Maladie, famille, AT/MP, CSG/CRDS : mêmes règles
///   - Alsace-Moselle compatible (agents des 3 départements)
pub fn generer_bulletin_fpt(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let mut cotisations = Vec::new();

    cotisations.push(ss_maladie(brut, ctx));
    cotisations.push(fpt_cnracl(brut, ctx));
    cotisations.push(famille(brut, ctx));
    cotisations.push(accident_travail(brut, ctx));
    cotisations.extend(csg_contributions(brut, ctx));

    if salarie.alsace_moselle {
        if let Some(am) = maladie_alsace_moselle(brut, ctx) {
            cotisations.push(am);
        }
    }

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();

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
        devise: "EUR".into(),
        absence: None,
        salarie,
    }
}
