use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::jp_cotisations::{jp_kenpo, jp_kaigo, jp_kodomo, jp_kosei, jp_koyo, jp_rousai};
use super::jp_impot::{jp_shotokuzei, jp_juminzei};

pub fn generer_bulletin_jp(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;

    // Cotisations salariales (pour base IIT, on les calcule d'abord)
    let kenpo  = jp_kenpo(brut, ctx);
    let kaigo  = jp_kaigo(brut, ctx);
    let kosei  = jp_kosei(brut, ctx);
    let koyo   = jp_koyo(brut, ctx);
    let rousai = jp_rousai(brut, ctx);
    let kodomo = jp_kodomo(brut, ctx);

    let mut cotisations = vec![kenpo, kaigo];
    cotisations.extend(kodomo);
    cotisations.extend([kosei, koyo, rousai]);

    // Cotisations sociales salariales, déductibles de l'impôt et de la taxe locale.
    let sociaux: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    cotisations.push(jp_shotokuzei(brut, sociaux, ctx));
    cotisations.push(jp_juminzei(brut, sociaux, ctx));

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(0);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(0),
        devise: "JPY".into(),
        absence: None,
        heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), pas: None, droits: None, alertes: Vec::new(), evolutions: Vec::new(),
        salarie,
    }
}
