use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{AbsenceInput, AbsenceResult, Bulletin, LigneCotisation, Pays, Salarie};
use super::cotisations::*;
use super::heures_sup::HeuresSup;
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
use super::se_bulletin::generer_bulletin_se;
use super::ee_bulletin::generer_bulletin_ee;
use super::lv_bulletin::generer_bulletin_lv;
use super::lt_bulletin::generer_bulletin_lt;
use super::at_bulletin::generer_bulletin_at;
use super::cz_bulletin::generer_bulletin_cz;
use super::sk_bulletin::generer_bulletin_sk;
use super::hu_bulletin::generer_bulletin_hu;
use super::si_bulletin::generer_bulletin_si;
use super::gr_bulletin::generer_bulletin_gr;
use super::cy_bulletin::generer_bulletin_cy;
use super::mt_bulletin::generer_bulletin_mt;
use super::hr_bulletin::generer_bulletin_hr;
use super::ie_bulletin::generer_bulletin_ie;
use super::ro_bulletin::generer_bulletin_ro;
use super::bg_bulletin::generer_bulletin_bg;
use super::us_bulletin::generer_bulletin_us;
use super::mx_bulletin::generer_bulletin_mx;
use super::br_bulletin::generer_bulletin_br;
use super::ae_bulletin::generer_bulletin_ae;
use super::in_bulletin::generer_bulletin_in;

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
        Pays::Suede             => return generer_bulletin_se(salarie, ctx),
        Pays::Estonie           => return generer_bulletin_ee(salarie, ctx),
        Pays::Lettonie          => return generer_bulletin_lv(salarie, ctx),
        Pays::Lituanie          => return generer_bulletin_lt(salarie, ctx),
        Pays::Autriche          => return generer_bulletin_at(salarie, ctx),
        Pays::Tchequie          => return generer_bulletin_cz(salarie, ctx),
        Pays::Slovaquie         => return generer_bulletin_sk(salarie, ctx),
        Pays::Hongrie           => return generer_bulletin_hu(salarie, ctx),
        Pays::Slovenie          => return generer_bulletin_si(salarie, ctx),
        Pays::Grece             => return generer_bulletin_gr(salarie, ctx),
        Pays::Chypre            => return generer_bulletin_cy(salarie, ctx),
        Pays::Malte             => return generer_bulletin_mt(salarie, ctx),
        Pays::Croatie           => return generer_bulletin_hr(salarie, ctx),
        Pays::Irlande           => return generer_bulletin_ie(salarie, ctx),
        Pays::Roumanie          => return generer_bulletin_ro(salarie, ctx),
        Pays::Bulgarie          => return generer_bulletin_bg(salarie, ctx),
        Pays::EtatsUnis         => return generer_bulletin_us(salarie, ctx),
        Pays::Mexique           => return generer_bulletin_mx(salarie, ctx),
        Pays::Bresil            => return generer_bulletin_br(salarie, ctx),
        Pays::Emirats           => return generer_bulletin_ae(salarie, ctx),
        Pays::Inde              => return generer_bulletin_in(salarie, ctx),
        Pays::France            => {}
    }

    // Brut mensuel plein = référence pour le SJB des IJSS et le per-day du maintien.
    let base_ref = salarie.salaire_brut;
    let anciennete = salarie.anciennete.unwrap_or(1).clamp(0, 100);
    // Dispatch par type d'arrêt : "conge" → congés payés (retenue + indemnité
    // max maintien/dixième, sans IJSS) ; sinon → compute_absence, qui gère
    // "maladie" (IJSS + maintien), "pro" (AT/MP : IJSS 60 %/80 % sans carence,
    // maintien sans carence) et "sans_solde" (retenue sèche).
    let est_conge = absence.map(|a| a.type_arret == "conge").unwrap_or(false);
    let conges_res = if est_conge {
        absence.and_then(|a| super::conges_payes::compute_conges(base_ref, a))
    } else {
        None
    };
    let mut absence_res = if est_conge { None } else {
        absence.and_then(|a| super::absence::compute_absence(base_ref, a, anciennete, salarie.alsace_moselle, ctx))
    };

    // Heures supplémentaires / complémentaires : majoration ajoutée au brut, puis
    // réduction salariale, déduction patronale et exonération d'impôt. Le gain majoré
    // s'ajoute par-dessus le brut (éventuellement déjà ajusté par l'absence).
    let hs = super::heures_sup::calculer(&salarie, ctx);
    let gain_hs_total = hs.as_ref().map(|h| h.gain_total).unwrap_or(Decimal::ZERO);

    // Assiette de RÉFÉRENCE : brut plein − retenue + maintien (+ HS), AVANT
    // déduction des IJSS. C'est le bulletin « neutre » qui fixe le net cible de
    // la garantie du net : la subrogation ne doit ni enrichir ni appauvrir.
    let assiette_ref = match (&absence_res, &conges_res) {
        (Some(r), _) => (base_ref - r.retenue + r.maintien).max(Decimal::ZERO) + gain_hs_total,
        // Congés payés : retenue − puis indemnité + (max maintien/dixième).
        // Pas d'IJSS → la garantie du net ci-dessous ne s'active pas.
        (None, Some(c)) => (base_ref - c.retenue + c.indemnite).max(Decimal::ZERO) + gain_hs_total,
        _ => base_ref + gain_hs_total,
    };
    let ijss_brut = absence_res.as_ref().map(|r| r.ijss_brut).unwrap_or(Decimal::ZERO);
    let ijss_net = absence_res.as_ref().map(|r| r.ijss_net).unwrap_or(Decimal::ZERO);

    // Ajustement du net (garantie du net) : les IJSS brutes déduites du brut
    // échappent aux cotisations salariales → sans correction, le salarié
    // percevrait un net supérieur au bulletin de référence. On retient donc en
    // haut de bulletin un « ajustement » tel que, une fois les IJSS NETTES
    // (brutes − CSG/CRDS 6,7 % précomptées par la CPAM) réintégrées en bas de
    // bulletin, net(assiette) + IJSS nettes = net(assiette_ref).
    // Résolution par dichotomie à absence FIGÉE (seule l'assiette varie) —
    // le net France est strictement croissant en l'assiette.
    let mut net_cible_gn = Decimal::ZERO; // exposé dans AbsenceResult (transparence f(x))
    let ajustement = if ijss_brut > Decimal::ZERO {
        let lignes_ref = lignes_france(assiette_ref, &salarie, ctx, &hs, &absence_res, base_ref);
        let net_cible = net_de(assiette_ref, &lignes_ref);
        net_cible_gn = net_cible.round_dp(2);
        let cible_hors_ijss = net_cible - ijss_net;

        let mut lo = cible_hors_ijss.max(Decimal::ZERO); // invariant net ≤ assiette
        let mut hi = (assiette_ref - ijss_brut).max(lo);
        for _ in 0..60 {
            if hi - lo <= dec!(0.005) {
                break;
            }
            let mid = ((lo + hi) / dec!(2)).round_dp(4);
            let lignes = lignes_france(mid, &salarie, ctx, &hs, &absence_res, base_ref);
            if net_de(mid, &lignes) < cible_hors_ijss {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (assiette_ref - ijss_brut - hi).round_dp(2).max(Decimal::ZERO)
    } else {
        Decimal::ZERO
    };
    if let Some(r) = absence_res.as_mut() {
        r.ajustement_net = ajustement;
        r.assiette_ref = assiette_ref.round_dp(2);
        r.net_cible = net_cible_gn;
    }

    // Bulletin de RÉFÉRENCE : le même salarié en mois plein SANS absence (même
    // brut, HS incluses à l'identique donc neutres dans l'écart). Sert à chiffrer
    // la perte de salaire du salarié (net avant impôt = net_a_payer, PAS non
    // soustrait ici) et le coût réel employeur de l'absence (Δ coût total).
    // Pas de récursion : le bulletin de référence n'a pas d'absence.
    if absence_res.is_some() {
        let reference = generer_bulletin(salarie.clone(), ctx, None);
        if let Some(r) = absence_res.as_mut() {
            r.net_reference = reference.net_a_payer;
            r.cout_reference = reference.cout_total_employeur;
        }
    }

    // Assiette de cotisations du mois : référence − IJSS brutes − ajustement.
    // Toutes les cotisations (Fillon, tranches, plafonds) tournent sur cette assiette.
    let brut = (assiette_ref - ijss_brut - ajustement).max(Decimal::ZERO);
    let cotisations = lignes_france(brut, &salarie, ctx, &hs, &absence_res, base_ref);

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();

    // Net imposable = brut - cotisations salariales
    // (la CSG non déductible et la CRDS sont dans total_sal mais ne réduisent pas le net imposable)
    let csg_non_ded_et_crds: Decimal = cotisations.iter()
        .filter(|c| c.code == "CSG_NON_DEDUCTIBLE" || c.code == "CRDS")
        .map(|c| c.montant_sal)
        .sum();

    // IJSS réintégrées : NETTES dans le net à payer (bas de bulletin — la CPAM
    // précompte CSG 6,2 % + CRDS 0,5 % avant de verser à l'employeur subrogé) ;
    // imposables (60 premiers jours d'arrêt) dans le net imposable (base PAS).
    let ijss_imposable = absence_res.as_ref().map(|r| r.ijss_imposable).unwrap_or(Decimal::ZERO);

    let mut net_imposable = (brut - total_sal + csg_non_ded_et_crds + ijss_imposable).round_dp(2);
    let net_a_payer   = (brut - total_sal + ijss_net).round_dp(2);

    // Exonération d'impôt sur le revenu des HS/HC : retranchée du net imposable
    // (base PAS), plafonnée à l'enveloppe annuelle (mois isolé = plafond plein).
    let heures_sup = hs.map(|h| {
        let mut r = h.result;
        let exo = super::heures_sup::net_imposable_exo(h.gain_total, net_imposable, brut, ctx)
            .min(r.exo_plafond);
        net_imposable = (net_imposable - exo).round_dp(2);
        r.exo_fiscale = exo;
        r
    });

    Bulletin {
        cotisations,
        brut,
        net_imposable,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(),
        absence: absence_res,
        heures_sup,
        conges: conges_res,
        salarie,
    }
}

/// Lignes de cotisations françaises pour une assiette donnée. Fonction pure de
/// l'assiette (absence et HS figées) : c'est elle que la dichotomie de la
/// garantie du net sonde à répétition.
fn lignes_france(
    assiette: Decimal,
    salarie: &Salarie,
    ctx: &ContextPaie,
    hs: &Option<HeuresSup>,
    absence_res: &Option<AbsenceResult>,
    base_ref: Decimal,
) -> Vec<LigneCotisation> {
    let mut cotisations = Vec::new();

    cotisations.push(ss_maladie(assiette, ctx));
    cotisations.push(ss_vieillesse_plafonnee(assiette, salarie.etp, ctx));
    cotisations.push(ss_vieillesse_deplafonnee(assiette, ctx));
    cotisations.push(famille(assiette, ctx));
    cotisations.push(accident_travail(assiette, ctx));
    cotisations.push(chomage(assiette, salarie.etp, ctx));
    cotisations.extend(csg_contributions(assiette, ctx));
    cotisations.extend(retraite_complementaire(assiette, &salarie.statut, salarie.etp, ctx));

    if salarie.alsace_moselle {
        if let Some(am) = maladie_alsace_moselle(assiette, ctx) {
            cotisations.push(am);
        }
    }

    if let Some(fillon) = reduction_fillon(assiette, salarie.etp, ctx) {
        cotisations.push(fillon);
    }

    // Entreprise adaptée : aide au poste (État/ASP) au titre d'un salarié RQTH.
    // Aide versée à l'employeur → ligne patronale négative, n'affecte pas le net.
    if salarie.entreprise_adaptee {
        let absent_fraction = match absence_res {
            Some(r) if base_ref > Decimal::ZERO => (r.retenue / base_ref).min(Decimal::ONE),
            _ => Decimal::ZERO,
        };
        if let Some(aide) = super::ea::aide_poste_ea(salarie, base_ref, absent_fraction, ctx) {
            cotisations.push(aide);
        }
    }

    // Lignes heures supp/compl : réduction salariale (négatif salarial) + déduction
    // forfaitaire patronale (négatif patronal). Poussées avant les totaux.
    if let Some(h) = hs {
        cotisations.extend(h.lignes.iter().cloned());
    }

    cotisations
}

/// Net à payer (hors IJSS réintégrées) d'une assiette : assiette − cotisations salariales.
fn net_de(assiette: Decimal, lignes: &[LigneCotisation]) -> Decimal {
    let total_sal: Decimal = lignes.iter().map(|c| c.montant_sal).sum();
    assiette - total_sal
}
