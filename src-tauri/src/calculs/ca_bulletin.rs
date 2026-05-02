use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::ca_cotisations::*;
use super::ca_impot::*;

pub fn generer_bulletin_ca(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let mut cotisations = Vec::new();

    // ── Retraite ─────────────────────────────────────────────
    cotisations.push(ca_rpc(brut, ctx));
    if let Some(rpc2) = ca_rpc2(brut, ctx) {
        cotisations.push(rpc2);
    }

    // ── Assurance-emploi ─────────────────────────────────────
    cotisations.push(ca_ae(brut, ctx));

    // ── Impôts à la source ───────────────────────────────────
    let province = salarie.province.as_deref().unwrap_or("ON");
    cotisations.push(ca_impot_federal(brut, ctx));
    cotisations.push(ca_impot_provincial(brut, province, ctx));

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "CAD".into(),
        salarie,
    }
}
