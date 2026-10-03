use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::it_cotisations::*;
use super::it_irpef::*;

pub fn generer_bulletin_it(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let mut cotisations = Vec::new();

    // ── Cotisations INPS ─────────────────────────────────────
    cotisations.push(it_ivs(brut, ctx));
    cotisations.push(it_naspi(brut, ctx));
    if salarie.contratto_termine {
        cotisations.push(it_naspi_termine(brut, ctx));
    }
    cotisations.push(it_malattia(brut, ctx));
    cotisations.push(it_maternita(brut, ctx));
    cotisations.push(it_fondo_garanzia(brut, ctx));

    // ── INAIL ────────────────────────────────────────────────
    cotisations.push(it_inail(brut, ctx));

    // ── TFR (accrual mensuel — coût patronal indicatif) ──────
    cotisations.push(it_tfr(brut, ctx));

    // ── Esonero contributivo 2022–2024 (taglio cuneo salarié)
    if let Some(esonero) = esonero_contributivo(brut, ctx) {
        cotisations.push(esonero);
    }

    // Revenu imposable IRPEF = brut − cotisations salariales INPS (art. 51 TUIR).
    let imponibile = (brut - cotisations.iter()
        .filter(|c| est_inps_salarie(&c.code))
        .map(|c| c.montant_sal)
        .sum::<Decimal>()).round_dp(2);

    // ── IRPEF — retenue à la source ──────────────────────────
    cotisations.push(irpef_mensuel(brut, imponibile, ctx));

    // ── Taglio del cuneo fiscale (depuis 2025)
    if let Some(bonus) = bonus_cuneo_mensuel(brut, imponibile, ctx) {
        cotisations.push(bonus);
    }

    // ── Addizionale regionale (si région renseignée) ─────────
    if let Some(ref regione) = salarie.regione {
        if let Some(add_reg) = addizionale_regionale(imponibile, regione, ctx) {
            cotisations.push(add_reg);
        }
    }

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();

    let net_imposable        = imponibile;
    let net_a_payer          = (brut - total_sal).round_dp(2);

    // TFR est un coût patronal différé — on l'exclut du "coût employeur immédiat"
    let tfr_pat: Decimal = cotisations.iter()
        .filter(|c| c.code == "IT_TFR")
        .map(|c| c.montant_pat)
        .sum();

    Bulletin {
        cotisations,
        brut,
        net_imposable,
        net_a_payer,
        cout_total_employeur: (brut + total_pat - tfr_pat).round_dp(2),
        devise: "EUR".into(),
        absence: None,
        heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(),
        salarie,
    }
}

/// Cotisations salariales INPS (et leur exonération), déduites du revenu
/// imposable ; l'IRPEF et les addizionali ne le sont pas.
fn est_inps_salarie(code: &str) -> bool {
    matches!(code, "IT_IVS" | "IT_NASPI" | "IT_NASPI_TERMINE" | "IT_MALATTIA" | "IT_MATERNITA")
        || code.starts_with("IT_ESONERO_")
}
