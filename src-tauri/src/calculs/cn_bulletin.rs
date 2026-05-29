use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::cn_cotisations::{cn_yanglao, cn_yiliao, cn_shiye, cn_gongshang, cn_shengyu, cn_gongjijin};
use super::cn_impot::cn_iit;

pub fn generer_bulletin_cn(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;

    // Cinq assurances + fonds logement
    let yanglao   = cn_yanglao(brut, ctx);
    let yiliao    = cn_yiliao(brut, ctx);
    let shiye     = cn_shiye(brut, ctx);
    let gongshang = cn_gongshang(brut, ctx);
    let shengyu   = cn_shengyu(brut, ctx);
    let gongjijin = cn_gongjijin(brut, ctx);

    // Cotisations salariales totales pour le calcul de l'assiette IIT
    let sal_avant_iit: Decimal = [
        &yanglao, &yiliao, &shiye, &gongjijin,
    ].iter().map(|c| c.montant_sal).sum();

    let iit = cn_iit(brut, sal_avant_iit, ctx);

    let cotisations = vec![yanglao, yiliao, shiye, gongshang, shengyu, gongjijin, iit];

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "CNY".into(),
        salarie,
    }
}
