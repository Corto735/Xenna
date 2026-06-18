use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{AbsenceInput, Bulletin, Pays, Salarie};
use super::cotisations::*;
use super::ch_bulletin::generer_bulletin_ch;
use super::lu_bulletin::generer_bulletin_lu;
use super::fpt_bulletin::generer_bulletin_fpt;
use super::it_bulletin::generer_bulletin_it;
use super::ca_bulletin::generer_bulletin_ca;
use super::qc_bulletin::generer_bulletin_qc;
use super::de_bulletin::generer_bulletin_de;
use super::es_bulletin::generer_bulletin_es;
use super::pt_bulletin::generer_bulletin_pt;
use super::be_bulletin::generer_bulletin_be;
use super::uk_bulletin::generer_bulletin_uk;
use super::jp_bulletin::generer_bulletin_jp;
use super::cn_bulletin::generer_bulletin_cn;
use super::nl_bulletin::generer_bulletin_nl;
use super::au_bulletin::generer_bulletin_au;
use super::nz_bulletin::generer_bulletin_nz;
use super::pl_bulletin::generer_bulletin_pl;
use super::kr_bulletin::generer_bulletin_kr;
use super::ad_bulletin::generer_bulletin_ad;
use super::mc_bulletin::generer_bulletin_mc;
use super::dk_bulletin::generer_bulletin_dk;
use super::fi_bulletin::generer_bulletin_fi;

pub fn generer_bulletin(salarie: Salarie, ctx: &ContextPaie, absence: Option<&AbsenceInput>) -> Bulletin {
    match salarie.pays {
        Pays::Suisse            => return generer_bulletin_ch(salarie, ctx),
        Pays::Luxembourg        => return generer_bulletin_lu(salarie, ctx),
        Pays::FonctionPublique  => return generer_bulletin_fpt(salarie, ctx),
        Pays::Italia            => return generer_bulletin_it(salarie, ctx),
        Pays::Canada            => return generer_bulletin_ca(salarie, ctx),
        Pays::Quebec            => return generer_bulletin_qc(salarie, ctx),
        Pays::Allemagne         => return generer_bulletin_de(salarie, ctx),
        Pays::Espagne           => return generer_bulletin_es(salarie, ctx),
        Pays::Portugal          => return generer_bulletin_pt(salarie, ctx),
        Pays::Belgique          => return generer_bulletin_be(salarie, ctx),
        Pays::Angleterre        => return generer_bulletin_uk(salarie, ctx),
        Pays::Japon             => return generer_bulletin_jp(salarie, ctx),
        Pays::Chine             => return generer_bulletin_cn(salarie, ctx),
        Pays::PaysBas           => return generer_bulletin_nl(salarie, ctx),
        Pays::Australie         => return generer_bulletin_au(salarie, ctx),
        Pays::NouvelleZelande   => return generer_bulletin_nz(salarie, ctx),
        Pays::Pologne           => return generer_bulletin_pl(salarie, ctx),
        Pays::CoreeDuSud        => return generer_bulletin_kr(salarie, ctx),
        Pays::Andorre           => return generer_bulletin_ad(salarie, ctx),
        Pays::Monaco            => return generer_bulletin_mc(salarie, ctx),
        Pays::Danemark          => return generer_bulletin_dk(salarie, ctx),
        Pays::Finlande          => return generer_bulletin_fi(salarie, ctx),
        Pays::France            => {}
    }

    // Brut mensuel plein = référence pour le SJB des IJSS et le per-day du maintien.
    let base_ref = salarie.salaire_brut;
    let absence_res = absence.and_then(|a| super::absence::compute_absence(base_ref, a, ctx));

    // Assiette de cotisations du mois : brut plein − retenue + maintien − IJSS brutes.
    // Toutes les cotisations (Fillon, tranches, plafonds) tournent sur cette assiette.
    let brut = match &absence_res {
        Some(r) => (base_ref - r.retenue + r.maintien - r.ijss_brut).max(Decimal::ZERO),
        None    => base_ref,
    };
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

    if let Some(fillon) = reduction_fillon(brut, salarie.etp, ctx) {
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
        devise: "EUR".into(),
        absence: absence_res,
        salarie,
    }
}
