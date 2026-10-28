// Bulletin de paie américain : FICA fédéral + impôt fédéral + impôt d'État.
//
// L'impôt d'État dépend de `salarie.us_state` (comme l'impôt provincial canadien
// dépend de `province`). États modélisés : sans impôt (TX, FL, WA), forfaitaire
// (IL, CO, PA), progressif (CA, NY). California SDI en sus pour la Californie.
// Devise USD. Barèmes 2025 et 2026 en base.

use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::us_cotisations::*;
use super::us_impot::{us_impot_federal, us_impot_state};

pub fn generer_bulletin_us(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;

    // Barèmes datés en base (migration 0152) depuis 2025.
    if !super::us_impot::couvert(ctx) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "USD", "US", "États-Unis : barèmes intégrés à partir de 2025.", ctx);
    }

    let state = salarie.us_state.as_deref().unwrap_or("TX");

    let mut cotisations = Vec::new();
    // ── FICA fédéral ─────────────────────────────────────────
    cotisations.push(us_social_security(brut, ctx));
    cotisations.push(us_medicare(brut, ctx));
    if let Some(add) = us_additional_medicare(brut, ctx) {
        cotisations.push(add);
    }
    cotisations.push(us_futa(brut, ctx));
    // ── SDI Californie (le cas échéant) ──────────────────────
    if let Some(sdi) = us_ca_sdi(brut, state, ctx) {
        cotisations.push(sdi);
    }
    // ── Impôts sur le revenu ─────────────────────────────────
    cotisations.push(us_impot_federal(brut, ctx));
    if let Some(state_tax) = us_impot_state(brut, state, ctx) {
        cotisations.push(state_tax);
    }

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "USD".into(),
        absence: None,
        heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), pas: None, droits: None, alertes: Vec::new(), evolutions: Vec::new(),
        salarie,
    }
}
