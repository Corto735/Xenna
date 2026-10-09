use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::qc_cotisations::*;
use super::ca_impot::*;

pub fn generer_bulletin_qc(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let mut cotisations = Vec::new();

    // ── Retraite (RRQ remplace RPC) ──────────────────────────
    cotisations.push(qc_rrq(brut, ctx));
    if let Some(rrq2) = qc_rrq2(brut, ctx) {
        cotisations.push(rrq2);
    }

    // ── AE (taux réduit) ─────────────────────────────────────
    cotisations.push(qc_ae(brut, ctx));

    // ── RQAP — assurance parentale québécoise ────────────────
    cotisations.push(qc_rqap(brut, ctx));

    // ── FSS + CNT (employeur uniquement) ─────────────────────
    cotisations.push(qc_fss(brut, ctx));
    cotisations.push(qc_cnt(brut, ctx));

    // ── Impôts à la source ───────────────────────────────────
    let sal = |code: &str| cotisations.iter().find(|c| c.code == code)
        .map(|c| c.montant_sal).unwrap_or(Decimal::ZERO);
    let r = RetenuesCa {
        rpc: sal("QC_RRQ"),
        taux_rpc: ctx.taux_sal("QC_RRQ"),
        rpc2: sal("QC_RRQ2"),
        ae: sal("QC_AE"),
        rqap: sal("QC_RQAP"),
    };
    cotisations.push(ca_impot_federal_qc(brut, &r, ctx));
    cotisations.push(qc_impot_provincial(brut, &r, ctx));

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
        absence: None,
        heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), pas: None, droits: None, alertes: Vec::new(), evolutions: Vec::new(),
        salarie,
    }
}
