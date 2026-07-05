// Bulletin de paie indien : EPF + ESI + Professional Tax + impôt sur le revenu.
//
// Salarié : EPF 12 % (assiette plafonnée au minimum légal 15 000 ₹) ; ESI 0,75 %
// si brut ≤ 21 000 ₹/mois ; Professional Tax (État du Karnataka : 200 ₹/mois
// au-delà de 25 000 ₹, variable selon l'État) ; impôt sur le revenu (TDS mensuel,
// ancien/nouveau régime). Employeur : EPF 12 % + ESI 3,25 % (si applicable).
// Devise INR. Données : FY 2025-26. Barème d'impôt en Rust (in_impot.rs).
//
// Sources : EPF & MP Act 1952 ; ESI Act 1948 ; Karnataka Tax on Professions Act 1976 ;
// Income-tax Act 1961 + Finance Act 2025.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};
use super::in_impot::calcul_impot;

/// Plafond légal de l'assiette EPF (₹/mois).
const EPF_CEILING: Decimal = dec!(15000);
/// Seuil d'assujettissement ESI (₹/mois).
const ESI_SEUIL: Decimal = dec!(21000);
/// Professional Tax Karnataka : 200 ₹/mois au-delà de ce salaire.
const PT_SEUIL: Decimal = dec!(25000);
const PT_MONTANT: Decimal = dec!(200);

pub fn generer_bulletin_in(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee < 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "INR", "IN",
            "Inde : données disponibles pour l'exercice fiscal 2025-26.", ctx);
    }

    let regime = salarie.inde_regime.clone().unwrap_or_else(|| "nouveau".into());
    let mut cotisations = Vec::new();

    // ── EPF (Employees' Provident Fund) — salarié 12 % + employeur 12 % ──
    let base_epf = brut.min(EPF_CEILING);
    let ts_epf = ctx.taux_sal("IN_EPF");
    let tp_epf = ctx.taux_pat("IN_EPF");
    cotisations.push(LigneCotisation {
        code: "IN_EPF".into(),
        libelle: ctx.libelle("IN_EPF", "EPF — Fonds de prévoyance"),
        base: base_epf,
        taux_sal: ts_epf, montant_sal: (base_epf * ts_epf).round_dp(2),
        taux_pat: tp_epf, montant_pat: (base_epf * tp_epf).round_dp(2),
        categorie: "Retraite".into(),
        explication: ctx.expl("IN_EPF",
            "EPF (Employees' Provident Fund) — salarié {ts} % + employeur {tp} % sur l'assiette \
            plafonnée au minimum légal de 15 000 ₹ (nombre d'employeurs cotisent au-delà). \
            Base {base} ₹. Base légale : EPF & MP Act 1952.")
            .replace("{ts}", &format!("{:.2}", ts_epf * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp_epf * dec!(100)))
            .replace("{base}", &format!("{:.2}", base_epf)),
        loi_ref: Some(ctx.loi_ref("EPF & MP Act 1952")),
    });

    // ── ESI — seulement si brut ≤ 21 000 ₹/mois ──────────────
    if brut <= ESI_SEUIL {
        let ts_esi = ctx.taux_sal("IN_ESI");
        let tp_esi = ctx.taux_pat("IN_ESI");
        cotisations.push(LigneCotisation {
            code: "IN_ESI".into(),
            libelle: ctx.libelle("IN_ESI", "ESI — Assurance maladie"),
            base: brut,
            taux_sal: ts_esi, montant_sal: (brut * ts_esi).round_dp(2),
            taux_pat: tp_esi, montant_pat: (brut * tp_esi).round_dp(2),
            categorie: "Sécurité sociale".into(),
            explication: ctx.expl("IN_ESI",
                "ESI (Employees' State Insurance) — assurance maladie/maternité, due si le brut \
                mensuel ≤ 21 000 ₹. Salarié {ts} % + employeur {tp} % × {base} ₹. \
                Base légale : ESI Act 1948.")
                .replace("{ts}", &format!("{:.2}", ts_esi * dec!(100)))
                .replace("{tp}", &format!("{:.2}", tp_esi * dec!(100)))
                .replace("{base}", &format!("{:.2}", brut)),
            loi_ref: Some(ctx.loi_ref("ESI Act 1948")),
        });
    }

    // ── Professional Tax (Karnataka) — 200 ₹/mois au-delà de 25 000 ₹ ──
    if brut >= PT_SEUIL {
        cotisations.push(LigneCotisation {
            code: "IN_PT".into(),
            libelle: ctx.libelle("IN_PT", "Professional Tax (Karnataka)"),
            base: brut,
            taux_sal: Decimal::ZERO, montant_sal: PT_MONTANT,
            taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
            categorie: "Impôt sur le revenu".into(),
            explication: ctx.expl("IN_PT",
                "Professional Tax — impôt d'État sur l'exercice d'une profession. Karnataka : \
                200 ₹/mois forfaitaires au-delà de 25 000 ₹ de salaire (montant et seuil variables \
                selon l'État). Base légale : Karnataka Tax on Professions Act 1976."),
            loi_ref: Some(ctx.loi_ref("Karnataka Tax on Professions Act 1976")),
        });
    }

    // ── Impôt sur le revenu (TDS mensuel, ancien/nouveau régime) ──
    let r = calcul_impot(brut, &regime, annee);
    let taux_eff = if brut > Decimal::ZERO { (r.mensuel / brut).round_dp(4) } else { Decimal::ZERO };
    let regime_lbl = if r.regime == "nouveau" { "nouveau régime (sec. 115BAC)" } else { "ancien régime" };
    cotisations.push(LigneCotisation {
        code: "IN_IMPOT".into(),
        libelle: ctx.libelle("IN_IMPOT", "Impôt sur le revenu (TDS)"),
        base: brut,
        taux_sal: taux_eff, montant_sal: r.mensuel,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("IN_IMPOT",
            "Impôt sur le revenu (TDS mensuel), {regime}.\nRevenu annualisé : {annuel} ₹\n\
            − déduction standard, revenu imposable {imposable} ₹\nTranche marginale {marginal} %\n\
            Impôt annuel (cess 4 % inclus) : {ann} ₹ → mensuel {mens} ₹\n\
            Rebate 87A appliqué sous le seuil. Base légale : Income-tax Act 1961.")
            .replace("{regime}", regime_lbl)
            .replace("{annuel}", &format!("{:.2}", brut * dec!(12)))
            .replace("{imposable}", &format!("{:.2}", r.imposable))
            .replace("{marginal}", &format!("{:.0}", r.marginal * dec!(100)))
            .replace("{ann}", &format!("{:.2}", r.annuel))
            .replace("{mens}", &format!("{:.2}", r.mensuel)),
        loi_ref: Some(ctx.loi_ref("Income-tax Act 1961 — Finance Act 2025")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "INR".into(), absence: None, heures_sup: None, salarie,
    }
}
