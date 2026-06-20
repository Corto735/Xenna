// ── Pays-Bas — Loonheffing (box 1) + heffingskortingen ───────────────────────
//
// Salarié sous l'âge AOW, régime général. Le « net » néerlandais se calcule ainsi :
//
//   loonheffing = barème box 1 (loonbelasting + premies volksverzekeringen)
//               − algemene heffingskorting − arbeidskorting        (planché à 0)
//   net = brut − loonheffing
//
// Méthode : annualisation (brut × 12), application du barème et des crédits sur
// l'année, puis retenue mensuelle = / 12 (comme jp_impot). Les heffingskortingen
// ne sont pas remboursables par la paie → on plafonne la réduction à l'impôt dû.
//
// Sources : Wet IB 2001 ; Belastingplan 2026 ; tables officielles Belastingdienst
//   « tabel algemene heffingskorting 2026 » et « tabel arbeidskorting 2026 ».

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

/// Paramètres box 1 + heffingskortingen d'une année donnée (sous âge AOW).
/// `None` = année pas encore sourcée officiellement (lacune assumée, rien d'inventé).
pub struct NlParams {
    // Barème box 1 : tranche 1 [0 ; b1_plafond] au taux b1_taux (premies vv incluses),
    // tranche 2 ]b1_plafond ; b2_plafond] au taux b2_taux, tranche 3 au-delà à b3_taux.
    pub b1_plafond: Decimal,
    pub b1_taux:    Decimal,
    pub b2_plafond: Decimal,
    pub b2_taux:    Decimal,
    pub b3_taux:    Decimal,
    /// Part « premies volksverzekeringen » (AOW+Anw+Wlz) incluse dans le taux de tranche 1 — informatif.
    pub premie_vv:  Decimal,
    // Algemene heffingskorting : max, seuil de dégressivité, taux de dégressivité.
    pub ahk_max:    Decimal,
    pub ahk_seuil:  Decimal,
    pub ahk_afbouw: Decimal,
    // Arbeidskorting : 3 segments de montée (opbouw) puis dégressivité (afbouw).
    pub ak_s1_plafond: Decimal, pub ak_s1_taux: Decimal,
    pub ak_s2_plafond: Decimal, pub ak_s2_base: Decimal, pub ak_s2_taux: Decimal,
    pub ak_s3_plafond: Decimal, pub ak_s3_base: Decimal, pub ak_s3_taux: Decimal,
    pub ak_max:         Decimal,
    pub ak_afbouw_seuil: Decimal,
    pub ak_afbouw:      Decimal,
    /// Maximumpremieloon annuel (plafond d'assiette des cotisations patronales).
    pub max_premieloon: Decimal,
}

/// Renvoie les paramètres officiels de l'année, ou `None` si non encore sourcée.
pub fn nl_parametres(annee: i32) -> Option<NlParams> {
    match annee {
        // 2026 — tables officielles Belastingdienst (vérifiées).
        2026 => Some(NlParams {
            b1_plafond: dec!(38883), b1_taux: dec!(0.3575),
            b2_plafond: dec!(78426), b2_taux: dec!(0.3756),
            b3_taux:    dec!(0.4950),
            premie_vv:  dec!(0.2765), // AOW 17,90 + Anw 0,10 + Wlz 9,65
            ahk_max:    dec!(3115), ahk_seuil: dec!(29736), ahk_afbouw: dec!(0.06398),
            ak_s1_plafond: dec!(11965), ak_s1_taux: dec!(0.08324),
            ak_s2_plafond: dec!(25845), ak_s2_base: dec!(996),  ak_s2_taux: dec!(0.31009),
            ak_s3_plafond: dec!(45592), ak_s3_base: dec!(5300), ak_s3_taux: dec!(0.01950),
            ak_max:        dec!(5685), ak_afbouw_seuil: dec!(45592), ak_afbouw: dec!(0.06510),
            max_premieloon: dec!(79409),
        }),
        // 2025 — tables officielles Belastingdienst (vérifiées, cumuls recoupés).
        2025 => Some(NlParams {
            b1_plafond: dec!(38441), b1_taux: dec!(0.3582),
            b2_plafond: dec!(76817), b2_taux: dec!(0.3748),
            b3_taux:    dec!(0.4950),
            premie_vv:  dec!(0.2765),
            ahk_max:    dec!(3068), ahk_seuil: dec!(28406), ahk_afbouw: dec!(0.06337),
            ak_s1_plafond: dec!(12169), ak_s1_taux: dec!(0.08053),
            ak_s2_plafond: dec!(26288), ak_s2_base: dec!(980),  ak_s2_taux: dec!(0.30030),
            ak_s3_plafond: dec!(43071), ak_s3_base: dec!(5220), ak_s3_taux: dec!(0.02258),
            ak_max:        dec!(5599), ak_afbouw_seuil: dec!(43071), ak_afbouw: dec!(0.06510),
            max_premieloon: dec!(75864),
        }),
        // 2024 — tables officielles Belastingdienst (vérifiées, cumuls recoupés).
        // Tranches 1 et 2 au même taux 36,97 % ; premies volksverzekeringen jusqu'à 38 098 €.
        2024 => Some(NlParams {
            b1_plafond: dec!(38098), b1_taux: dec!(0.3697),
            b2_plafond: dec!(75518), b2_taux: dec!(0.3697),
            b3_taux:    dec!(0.4950),
            premie_vv:  dec!(0.2765),
            ahk_max:    dec!(3362), ahk_seuil: dec!(24812), ahk_afbouw: dec!(0.0663),
            ak_s1_plafond: dec!(11490), ak_s1_taux: dec!(0.08425),
            ak_s2_plafond: dec!(24820), ak_s2_base: dec!(968),  ak_s2_taux: dec!(0.31433),
            ak_s3_plafond: dec!(39957), ak_s3_base: dec!(5158), ak_s3_taux: dec!(0.02471),
            ak_max:        dec!(5532), ak_afbouw_seuil: dec!(39957), ak_afbouw: dec!(0.06510),
            max_premieloon: dec!(71628),
        }),
        // 2023 — système à 2 tranches (combiné 36,93 % jusqu'à 73 031 €). Tables officielles.
        2023 => Some(NlParams {
            b1_plafond: dec!(73031), b1_taux: dec!(0.3693),
            b2_plafond: dec!(73031), b2_taux: dec!(0.3693),
            b3_taux:    dec!(0.4950),
            premie_vv:  dec!(0.2765),
            ahk_max:    dec!(3070), ahk_seuil: dec!(22660), ahk_afbouw: dec!(0.06095),
            ak_s1_plafond: dec!(10741), ak_s1_taux: dec!(0.08231),
            ak_s2_plafond: dec!(23201), ak_s2_base: dec!(884),  ak_s2_taux: dec!(0.29861),
            ak_s3_plafond: dec!(37691), ak_s3_base: dec!(4605), ak_s3_taux: dec!(0.03085),
            ak_max:        dec!(5052), ak_afbouw_seuil: dec!(37691), ak_afbouw: dec!(0.06510),
            max_premieloon: dec!(66956),
        }),
        // 2022 — système à 2 tranches (combiné 37,07 % jusqu'à 69 398 €). Tables officielles
        // (arbeidskorting : cumuls recoupés à partir des bornes officielles).
        2022 => Some(NlParams {
            b1_plafond: dec!(69398), b1_taux: dec!(0.3707),
            b2_plafond: dec!(69398), b2_taux: dec!(0.3707),
            b3_taux:    dec!(0.4950),
            premie_vv:  dec!(0.2765),
            ahk_max:    dec!(2888), ahk_seuil: dec!(21317), ahk_afbouw: dec!(0.06007),
            ak_s1_plafond: dec!(10350), ak_s1_taux: dec!(0.04541),
            ak_s2_plafond: dec!(22356), ak_s2_base: dec!(470),  ak_s2_taux: dec!(0.28461),
            ak_s3_plafond: dec!(36649), ak_s3_base: dec!(3887), ak_s3_taux: dec!(0.02610),
            ak_max:        dec!(4260), ak_afbouw_seuil: dec!(36649), ak_afbouw: dec!(0.0586),
            max_premieloon: dec!(59706),
        }),
        // 2015-2021 : à sourcer année par année depuis les archives officielles.
        _ => None,
    }
}

/// Barème box 1 brut (impôt + premies volksverzekeringen) sur le revenu annuel.
fn box1_brut(revenu: Decimal, p: &NlParams) -> Decimal {
    let t1 = revenu.min(p.b1_plafond) * p.b1_taux;
    let t2 = (revenu.min(p.b2_plafond) - p.b1_plafond).max(Decimal::ZERO) * p.b2_taux;
    let t3 = (revenu - p.b2_plafond).max(Decimal::ZERO) * p.b3_taux;
    t1 + t2 + t3
}

/// Algemene heffingskorting (crédit d'impôt général) sur le revenu annuel.
fn algemene_heffingskorting(revenu: Decimal, p: &NlParams) -> Decimal {
    if revenu <= p.ahk_seuil {
        p.ahk_max
    } else {
        (p.ahk_max - p.ahk_afbouw * (revenu - p.ahk_seuil)).max(Decimal::ZERO)
    }
}

/// Arbeidskorting (crédit d'impôt sur les revenus du travail) sur le revenu annuel.
fn arbeidskorting(revenu: Decimal, p: &NlParams) -> Decimal {
    if revenu <= p.ak_s1_plafond {
        revenu * p.ak_s1_taux
    } else if revenu <= p.ak_s2_plafond {
        p.ak_s2_base + p.ak_s2_taux * (revenu - p.ak_s1_plafond)
    } else if revenu <= p.ak_s3_plafond {
        p.ak_s3_base + p.ak_s3_taux * (revenu - p.ak_s2_plafond)
    } else {
        (p.ak_max - p.ak_afbouw * (revenu - p.ak_afbouw_seuil)).max(Decimal::ZERO)
    }
}

/// Ligne « loonheffing » mensuelle (côté salarié). `None` si l'année n'est pas sourcée.
pub fn nl_loonheffing(brut: Decimal, annee: i32, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let p = nl_parametres(annee)?;
    let rev_ann = brut * dec!(12);

    let box1   = box1_brut(rev_ann, &p);
    let ahk    = algemene_heffingskorting(rev_ann, &p);
    let ak     = arbeidskorting(rev_ann, &p);
    let heffing_ann = (box1 - ahk - ak).max(Decimal::ZERO);
    let mensuel = (heffing_ann / dec!(12)).round_dp(2);
    let taux_eff = if brut > Decimal::ZERO { (mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    Some(LigneCotisation {
        code:        "NL_LOONHEFFING".into(),
        libelle:     "Loonheffing — Impôt sur le revenu + premies volksverzekeringen".into(),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: format!(
            "Loonheffing {annee} (salarié sous l'âge AOW) — retenue à la source mensuelle.\n\n\
            Revenu annuel estimé : {rev:.0} €\n\n\
            Barème box 1 :\n\
            • tranche 1 (0 → {b1p:.0} €) : {b1t:.2} % (dont premies volksverzekeringen {pvv:.2} %)\n\
            • tranche 2 ({b1p:.0} → {b2p:.0} €) : {b2t:.2} %\n\
            • tranche 3 (> {b2p:.0} €) : {b3t:.2} %\n\
            = impôt + premies bruts : {box1:.0} €/an\n\n\
            − algemene heffingskorting : {ahk:.0} €\n\
            − arbeidskorting : {ak:.0} €\n\
            = loonheffing : {ha:.0} €/an / 12 = {mens:.2} €/mois\n\
            Taux effectif : {teff:.2} %\n\n\
            Base légale : Wet IB 2001 ; Belastingplan {annee}.",
            rev = rev_ann,
            b1p = p.b1_plafond, b1t = p.b1_taux * dec!(100), pvv = p.premie_vv * dec!(100),
            b2p = p.b2_plafond, b2t = p.b2_taux * dec!(100), b3t = p.b3_taux * dec!(100),
            box1 = box1, ahk = ahk, ak = ak, ha = heffing_ann,
            mens = mensuel, teff = taux_eff * dec!(100),
        ),
        loi_ref: Some(ctx.loi_ref("Wet IB 2001 — Belastingplan 2026")),
    })
}
