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

    // ── Esonero contributivo 2022–2023 (taglio cuneo salarié)
    if let Some(esonero) = esonero_contributivo(brut, ctx) {
        cotisations.push(esonero);
    }

    // ── IRPEF — retenue à la source ──────────────────────────
    cotisations.push(irpef_mensuel(brut, ctx));

    // ── Bonus cuneo fiscale 2024–2025 (trattamento integrativo)
    if let Some(bonus) = bonus_cuneo_mensuel(brut, ctx) {
        cotisations.push(bonus);
    }

    // ── Addizionale regionale (si région renseignée) ─────────
    if let Some(ref regione) = salarie.regione {
        if let Some(add_reg) = addizionale_regionale(brut, regione, ctx) {
            cotisations.push(add_reg);
        }
    }

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();

    // Net imposable IRPEF = brut − cotisations INPS salarié uniquement
    // (IRPEF et addizionale ne réduisent pas l'imponibile IRPEF)
    let inps_sal: Decimal = cotisations.iter()
        .filter(|c| {
            matches!(
                c.code.as_str(),
                "IT_IVS" | "IT_NASPI" | "IT_MALATTIA" | "IT_MATERNITA"
                | "IT_ESONERO_2022" | "IT_ESONERO_2023"
            )
        })
        .map(|c| c.montant_sal)
        .sum();

    let net_imposable        = (brut - inps_sal).round_dp(2);
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
        heures_sup: None,
        salarie,
    }
}
