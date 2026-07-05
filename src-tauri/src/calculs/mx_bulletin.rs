// Bulletin de paie mexicain : IMSS (cuotas obrero) + ISR + INFONAVIT/retiro.
//
// Salarié : IMSS obrero ~2,375 % + excédent 0,40 % sur la part > 3 UMA ; ISR
// (art. 96 LISR, méthode cuota fija + % sur l'excédent) diminué du subsidio al
// empleo. Employeur : INFONAVIT 5 % + retiro SAR 2 % (l'IMSS patronal complet —
// enfermedad, IV, guarderías, riesgos — n'est pas détaillé, lacune assumée).
// Devise MXN. Données : 2025 (2026 reconduit). Taux obrero/patronaux en base ;
// barème ISR, UMA et subsidio en Rust.
//
// Sources : Ley del Seguro Social art. 25-36 ; Ley del ISR art. 96 ;
// Ley del INFONAVIT art. 29 ; DOF 01/05/2024 (subsidio al empleo).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// UMA mensuelle (valeur journalière × 30,4) et subsidio, par année.
fn params_mx(annee: i32) -> (Decimal, Decimal, Decimal) {
    // (UMA mensuelle, seuil subsidio, montant subsidio)
    match annee {
        2025 | 2026 => (dec!(3439.46), dec!(9081.00), dec!(406.83)), // UMA 113,14 $/j × 30,4
        _           => (dec!(3439.46), dec!(9081.00), dec!(406.83)),
    }
}

/// Barème ISR mensuel (art. 96 LISR) : (limite inférieure, cuota fija, taux).
fn bareme_isr(annee: i32) -> [(Decimal, Decimal, Decimal); 11] {
    let _ = annee; // stable 2025-2026
    [
        (dec!(0.01),       dec!(0.00),       dec!(0.0192)),
        (dec!(746.05),     dec!(14.32),      dec!(0.0640)),
        (dec!(6332.06),    dec!(371.83),     dec!(0.1088)),
        (dec!(11128.02),   dec!(893.63),     dec!(0.1600)),
        (dec!(12935.83),   dec!(1182.88),    dec!(0.1792)),
        (dec!(15487.72),   dec!(1639.32),    dec!(0.2136)),
        (dec!(31236.50),   dec!(4005.46),    dec!(0.2352)),
        (dec!(49233.01),   dec!(8237.45),    dec!(0.3000)),
        (dec!(93993.91),   dec!(21665.72),   dec!(0.3200)),
        (dec!(125325.21),  dec!(31691.85),   dec!(0.3400)),
        (dec!(375975.62),  dec!(116912.87),  dec!(0.3500)),
    ]
}

/// ISR mensuel brut (avant subsidio) selon le barème cuota fija.
fn isr_brut(base: Decimal, annee: i32) -> (Decimal, Decimal, Decimal) {
    // Retourne (impôt, limite inférieure appliquée, taux appliqué) pour l'explication.
    let bareme = bareme_isr(annee);
    let mut choisi = bareme[0];
    for &t in bareme.iter() {
        if base >= t.0 { choisi = t; } else { break; }
    }
    let (li, cuota, taux) = choisi;
    let impot = (cuota + (base - li) * taux).max(Decimal::ZERO);
    (impot.round_dp(2), li, taux)
}

fn ligne_cotis(code: &str, libelle: &str, categorie: &str, base: Decimal, ctx: &ContextPaie, expl_key: &str, expl_fr: &str, loi: &str) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    let lib = ctx.libelle(code, libelle);
    let explication = ctx.expl(expl_key, expl_fr)
        .replace("{ts}", &format!("{:.3}", ts * dec!(100)))
        .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
        .replace("{base}", &format!("{:.2}", base))
        .replace("{ms}", &format!("{:.2}", (base * ts).round_dp(2)));
    LigneCotisation {
        code: code.into(), libelle: lib, base,
        taux_sal: ts, montant_sal: (base * ts).round_dp(2),
        taux_pat: tp, montant_pat: (base * tp).round_dp(2),
        categorie: categorie.into(), explication,
        loi_ref: Some(ctx.loi_ref(loi)),
    }
}

pub fn generer_bulletin_mx(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "MXN", "MX",
            "Mexique : données disponibles pour 2025 (2026 reconduit).", ctx);
    }

    let (uma_mensuelle, seuil_subsidio, subsidio) = params_mx(annee);
    let mut cotisations = Vec::new();

    // ── IMSS obrero (base) ───────────────────────────────────
    cotisations.push(ligne_cotis("MX_IMSS", "IMSS — Cuotas obrero", "Sécurité sociale", brut, ctx,
        "MX_IMSS",
        "IMSS (Instituto Mexicano del Seguro Social) — part salarié agrégée : enfermedad y \
        maternidad (prestaciones en dinero + gastos médicos pensionados), invalidez y vida, \
        cesantía y vejez. Taux {ts} % × {base} $ = {ms} $. Base légale : Ley del Seguro Social art. 25-36.",
        "Ley del Seguro Social art. 25-36"));

    // ── IMSS excédent > 3 UMA ────────────────────────────────
    let trois_uma = uma_mensuelle * dec!(3);
    if brut > trois_uma {
        let base_exc = brut - trois_uma;
        cotisations.push(ligne_cotis("MX_IMSS_EXC", "IMSS — Excédente (> 3 UMA)", "Sécurité sociale", base_exc, ctx,
            "MX_EXC",
            "Cuota obrero sur l'excédent de salaire au-delà de 3 UMA ({base} $ = brut − 3 × {uma} $). \
            Taux {ts} % = {ms} $. Base légale : Ley del Seguro Social art. 106.",
            "Ley del Seguro Social art. 106")
            .clone_with_uma(uma_mensuelle));
    }

    // ── ISR (impôt sur le revenu) − subsidio al empleo ───────
    let (isr, li, taux_isr) = isr_brut(brut, annee);
    let sub = if brut <= seuil_subsidio { subsidio.min(isr) } else { Decimal::ZERO };
    let isr_net = (isr - sub).max(Decimal::ZERO).round_dp(2);
    let taux_eff = if brut > Decimal::ZERO { (isr_net / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "MX_ISR".into(),
        libelle: ctx.libelle("MX_ISR", "ISR — Impôt sur le revenu (retenue {annee})")
            .replace("{annee}", &annee.to_string()),
        base: brut,
        taux_sal: taux_eff,
        montant_sal: isr_net,
        taux_pat: Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("MX_ISR",
            "Impôt sur le revenu (retención mensual, art. 96 LISR).\nBase : {base} $\n\
            Tranche : limite inférieure {li} $, taux marginal {taux} %\nISR brut : {isr} $\n\
            − subsidio al empleo : {sub} $ (jusqu'à 406,83 $ pour un revenu ≤ 9 081 $)\n\
            ISR net : {isrnet} $\nBase légale : Ley del ISR art. 96 ; DOF 01/05/2024 (subsidio).")
            .replace("{base}", &format!("{:.2}", brut))
            .replace("{li}", &format!("{:.2}", li))
            .replace("{taux}", &format!("{:.2}", taux_isr * dec!(100)))
            .replace("{isr}", &format!("{:.2}", isr))
            .replace("{sub}", &format!("{:.2}", sub))
            .replace("{isrnet}", &format!("{:.2}", isr_net)),
        loi_ref: Some(ctx.loi_ref("Ley del ISR art. 96 — DOF 01/05/2024 (subsidio al empleo)")),
    });

    // ── Employeur : INFONAVIT + retiro ───────────────────────
    cotisations.push(ligne_cotis("MX_INFONAVIT", "INFONAVIT — Logement (employeur)", "Sécurité sociale", brut, ctx,
        "MX_INFONAVIT",
        "INFONAVIT (Instituto del Fondo Nacional de la Vivienda) — 5 % employeur sur le salaire, \
        finance le logement des travailleurs. Taux {tp} % × {base} $. Base légale : Ley del INFONAVIT art. 29.",
        "Ley del INFONAVIT art. 29"));
    cotisations.push(ligne_cotis("MX_RETIRO", "Retiro (SAR) — Retraite (employeur)", "Retraite", brut, ctx,
        "MX_RETIRO",
        "Retiro (Sistema de Ahorro para el Retiro) — 2 % employeur sur le salaire, vers l'Afore. \
        Taux {tp} % × {base} $. Base légale : Ley del Seguro Social art. 168.",
        "Ley del Seguro Social art. 168 — SAR"));

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "MXN".into(), absence: None, heures_sup: None, salarie,
    }
}

// Petit helper pour injecter {uma} dans l'explication de l'excédent.
trait CloneWithUma { fn clone_with_uma(self, uma: Decimal) -> Self; }
impl CloneWithUma for LigneCotisation {
    fn clone_with_uma(mut self, uma: Decimal) -> Self {
        self.explication = self.explication.replace("{uma}", &format!("{:.2}", uma));
        self
    }
}
