// ── Espagne — retenue d'IRPF sur les revenus du travail ──────────────────────
//
// Algorithme officiel de l'AEAT, « Algoritmo de cálculo del tipo de retención a
// cuenta del IRPF para los rendimientos del trabajo personal » 2026 (identique
// en 2025 : art. 20 LIRPF selon RDL 4/2024, art. 81 et 85.3 RIRPF selon
// RD 142/2024). Hypothèses : contrat général, situation familiale 3 (sans
// conjoint à charge), sans descendant, moins de 65 ans, sans handicap, 12 paies
// (RETRIB = brut × 12).
//   1. RNT = RETRIB − cotisations salariales.
//   2. Réduction pour revenus du travail : 7 302 € jusqu'à 14 852 €, dégressive
//      jusqu'à 19 747,50 €.
//   3. BASE = RNT − autres frais (2 000 €) − réduction.
//   4. CUOTA = barème de retenue (BASE) − barème (minimum personnel 5 550 €),
//      plafonnée à 43 % de (RETRIB − 15 876) quand RETRIB ≤ 35 200 € ; aucune
//      retenue sous la limite d'exonération de 15 876 €.
//   5. TIPO = CUOTA / RETRIB × 100, tronqué à deux décimales ; retenue mensuelle
//      = brut × TIPO.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

/// Barème de retenue (tabla 2 de l'algorithme).
fn escala(base: Decimal) -> Decimal {
    let tranches = [
        (dec!(12450), dec!(0.19)),
        (dec!(20200), dec!(0.24)),
        (dec!(35200), dec!(0.30)),
        (dec!(60000), dec!(0.37)),
        (dec!(300000), dec!(0.45)),
        (Decimal::MAX, dec!(0.47)),
    ];
    let mut cuota = Decimal::ZERO;
    let mut bas = Decimal::ZERO;
    for (haut, taux) in tranches {
        if base <= bas { break; }
        cuota += (base.min(haut) - bas) * taux;
        bas = haut;
    }
    cuota
}

/// Réduction pour obtention de revenus du travail (art. 20 LIRPF).
fn reduccion(rnt: Decimal) -> Decimal {
    let r = if rnt <= dec!(14852) {
        dec!(7302)
    } else if rnt <= dec!(17673.52) {
        dec!(7302) - dec!(1.75) * (rnt - dec!(14852))
    } else if rnt < dec!(19747.50) {
        dec!(2364.34) - dec!(1.14) * (rnt - dec!(17673.52))
    } else {
        Decimal::ZERO
    };
    r.round_dp(2)
}

/// Ligne de retenue IRPF, ou None avant 2025 (algorithme non intégré).
/// `cotisations_mensuelles` = total des cotisations salariales du mois.
pub fn irpf(brut: Decimal, cotisations_mensuelles: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let annee = ctx.date_paie.year();
    if annee < 2025 {
        return None;
    }
    let retrib = brut * dec!(12);
    let cotis = cotisations_mensuelles * dec!(12);
    let rnt = (retrib - cotis).max(Decimal::ZERO);
    let otros = dec!(2000).min(rnt);
    let red = reduccion(rnt);
    let base = (rnt - otros - red).max(Decimal::ZERO);
    let minimo = dec!(5550);
    let limite_exento = dec!(15876);

    let cuota = if retrib <= limite_exento {
        Decimal::ZERO
    } else {
        let c = (escala(base) - escala(minimo)).max(Decimal::ZERO);
        if retrib <= dec!(35200) {
            c.min((retrib - limite_exento) * dec!(0.43))
        } else {
            c
        }
    };
    let tipo = if retrib > Decimal::ZERO {
        (cuota / retrib * dec!(100)).trunc_with_scale(2)
    } else {
        Decimal::ZERO
    };
    let retencion = (brut * tipo / dec!(100)).round_dp(2);

    Some(LigneCotisation {
        code:        "ES_IRPF".into(),
        libelle:     ctx.libelle("ES_IRPF", "Retención IRPF — Impôt sur le revenu"),
        base:        brut,
        taux_sal:    tipo / dec!(100),
        montant_sal: retencion,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: ctx.expl("ES_IRPF",
            "Retenue d'IRPF selon l'algorithme officiel de l'AEAT ({annee}), pour un salarié \
            sans conjoint à charge ni enfant, de moins de 65 ans, payé 12 fois par an.\n\n\
            Rémunération annuelle : {r} € − cotisations {c} € = {rnt} €\n\
            − autres frais 2 000 € − réduction pour revenus du travail {red} € = base {b} €\n\
            Barème de retenue 19 / 24 / 30 / 37 / 45 / 47 % sur la base, moins le même barème \
            sur le minimum personnel de 5 550 € → {q} €/an\n\
            Taux de retenue : {t} % (tronqué à deux décimales) → {m} €/mois.\n\n\
            Source : AEAT, Algoritmo de cálculo del tipo de retención {annee}.")
            .replace("{annee}", &annee.to_string())
            .replace("{r}", &format!("{:.2}", retrib))
            .replace("{c}", &format!("{:.2}", cotis))
            .replace("{rnt}", &format!("{:.2}", rnt))
            .replace("{red}", &format!("{:.2}", red))
            .replace("{b}", &format!("{:.2}", base))
            .replace("{q}", &format!("{:.2}", cuota))
            .replace("{t}", &format!("{:.2}", tipo))
            .replace("{m}", &format!("{:.2}", retencion)),
        loi_ref:     Some(ctx.loi_ref("Ley 35/2006 (LIRPF) art. 20 y 101 — RD 439/2007 (RIRPF) art. 80-87")),
    })
}
