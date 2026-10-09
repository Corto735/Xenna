use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::cotisations::{famille, csg_contributions, maladie_alsace_moselle};
use super::fpt_cotisations::{fpt_cnracl, fpt_maladie, fpt_atiacl, fpt_fnal, fpt_csa, fpt_cnfpt};

/// Bulletin pour les agents titulaires de la Fonction Publique Territoriale (FPT).
///
/// Différences vs secteur privé (France) :
///   - CNRACL remplace SS_VIEILLESSE + AGIRC-ARRCO
///   - Pas de cotisation chômage (titulaires : emploi garanti)
///   - Pas de réduction Fillon (employeurs publics exclus du dispositif)
///   - Maladie : régime spécial (9,88 % employeur depuis 2018, pas de part agent)
///   - ATIACL (0,40 %) à la place de la cotisation AT/MP du régime général
///   - Famille, CSG/CRDS : mêmes règles ; FNAL, contribution solidarité
///     autonomie et CNFPT en plus
///   - Non modélisés : RAFP (primes), centre de gestion, versement mobilité
///   - Alsace-Moselle compatible (agents des 3 départements)
pub fn generer_bulletin_fpt(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let mut cotisations = Vec::new();

    cotisations.extend(fpt_maladie(brut, ctx));
    cotisations.push(fpt_cnracl(brut, ctx));
    cotisations.push(famille(brut, ctx));
    cotisations.extend(fpt_atiacl(brut, ctx));
    cotisations.extend(fpt_fnal(brut, salarie.effectif.as_deref(), ctx));
    cotisations.extend(fpt_csa(brut, ctx));
    cotisations.extend(fpt_cnfpt(brut, ctx));
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
        heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), pas: None, droits: None, alertes: Vec::new(), evolutions: Vec::new(),
        salarie,
    }
}
