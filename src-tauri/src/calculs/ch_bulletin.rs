use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::ch_cotisations::*;

pub fn generer_bulletin_ch(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let cotisations = vec![
        ch_avs(brut, ctx),
        ch_ai(brut, ctx),
        ch_apg(brut, ctx),
        ch_ac(brut, ctx),
        ch_aanp(brut, ctx),
        ch_aap(brut, ctx),
        ch_ijm(brut, ctx),
        ch_lpp(brut, ctx),
    ];

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "CHF".into(),
        salarie,
    }
}
