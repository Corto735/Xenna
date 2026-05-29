use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::jp_cotisations::{jp_kenpo, jp_kaigo, jp_kosei, jp_koyo, jp_rousai};
use super::jp_impot::{jp_shotokuzei, jp_juminzei};

pub fn generer_bulletin_jp(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;

    // Cotisations salariales (pour base IIT, on les calcule d'abord)
    let kenpo  = jp_kenpo(brut, ctx);
    let kaigo  = jp_kaigo(brut, ctx);
    let kosei  = jp_kosei(brut, ctx);
    let koyo   = jp_koyo(brut, ctx);
    let rousai = jp_rousai(brut, ctx);

    let shotoku  = jp_shotokuzei(brut, ctx);
    let juminzei = jp_juminzei(brut, ctx);

    let cotisations = vec![kenpo, kaigo, kosei, koyo, rousai, shotoku, juminzei];

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
        salarie,
    }
}
