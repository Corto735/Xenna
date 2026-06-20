// ── Andorre — CASS + IRPF ────────────────────────────────────────────────────
//
// Salarié secteur privé. Côté salarié : CASS 6,5 % + IRPF (0 / 5 / 10 %).
// Taux CASS lu en base ; IRPF (annualisé) calculé ici. Devise EUR.
// Sources : Llei 17/2008 (seguretat social) ; Llei 5/2014 (IRPF).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// IRPF annuel : 0 % jusqu'à 24 000 €, 5 % de 24 001 à 40 000 €, 10 % au-delà.
fn irpf_annuel(revenu: Decimal) -> Decimal {
    if revenu <= dec!(24000) {
        Decimal::ZERO
    } else if revenu <= dec!(40000) {
        (revenu - dec!(24000)) * dec!(0.05)
    } else {
        dec!(800) + (revenu - dec!(40000)) * dec!(0.10)
    }
}

pub fn generer_bulletin_ad(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    // IRPF créé en 2015 ; CASS 6,5 % et barème IRPF 0/5/10 stables sur 2015-2026.
    // (Les bonifications transitoires des toutes premières années ne sont pas modélisées.)
    if !(2015..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR",
            "Andorre : données disponibles à partir de 2015 (création de l'IRPF).");
    }

    // CASS (taux en base)
    let cass_ts = ctx.taux_sal("AD_CASS");
    let cass_tp = ctx.taux_pat("AD_CASS");
    let cass = LigneCotisation {
        code: "AD_CASS".into(), libelle: ctx.libelle("AD_CASS", "CASS — Sécurité sociale"), base: brut,
        taux_sal: cass_ts, montant_sal: (brut * cass_ts).round_dp(2),
        taux_pat: cass_tp, montant_pat: (brut * cass_tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("AD_CASS",
            "CASS — sécurité sociale (branche générale + retraite).\n\
            Salarié {ts} % / employeur {tp} %. Salarié : {ms} €.\n\n\
            Base légale : Llei 17/2008.")
            .replace("{ts}", &format!("{:.2}", cass_ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", cass_tp * dec!(100)))
            .replace("{ms}", &format!("{:.2}", (brut * cass_ts).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("Llei 17/2008 de la seguretat social")),
    };

    // IRPF (annualisé)
    let irpf_ann = irpf_annuel(brut * dec!(12));
    let irpf_mens = (irpf_ann / dec!(12)).round_dp(2);
    let taux_irpf = if brut > Decimal::ZERO { (irpf_mens / brut).round_dp(4) } else { Decimal::ZERO };
    let irpf = LigneCotisation {
        code: "AD_IRPF".into(), libelle: ctx.libelle("AD_IRPF", "IRPF — Impôt sur le revenu"), base: brut,
        taux_sal: taux_irpf, montant_sal: irpf_mens, taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("AD_IRPF",
            "IRPF — impôt sur le revenu (annualisé).\n\n\
            Revenu annuel : {ra} €\n\
            • 0 % jusqu'à 24 000 €\n• 5 % de 24 001 à 40 000 €\n• 10 % au-delà de 40 000 €\n\
            = {ia} €/an / 12 = {im} €/mois.\n\n\
            Base légale : Llei 5/2014 (IRPF).")
            .replace("{ra}", &format!("{:.0}", brut * dec!(12)))
            .replace("{ia}", &format!("{:.0}", irpf_ann))
            .replace("{im}", &format!("{:.2}", irpf_mens)),
        loi_ref: Some(ctx.loi_ref("Llei 5/2014 (IRPF)")),
    };

    let cotisations = vec![cass, irpf];
    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(), absence: None, heures_sup: None, salarie,
    }
}
