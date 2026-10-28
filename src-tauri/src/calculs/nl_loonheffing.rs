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

/// Paramètres officiels à la date, en base (NL_<CHAMP>, migration 0153 : 2022 à
/// 2026), ou `None` si l'année n'est pas sourcée (lacune assumée, rien d'inventé).
pub fn nl_parametres(ctx: &ContextPaie) -> Option<NlParams> {
    let p = |c: &str| ctx.plafond(c);
    Some(NlParams {
        b1_plafond: p("NL_B1_PLAFOND")?,
        b1_taux: p("NL_B1_TAUX")?,
        b2_plafond: p("NL_B2_PLAFOND")?,
        b2_taux: p("NL_B2_TAUX")?,
        b3_taux: p("NL_B3_TAUX")?,
        premie_vv: p("NL_PREMIE_VV")?,
        ahk_max: p("NL_AHK_MAX")?,
        ahk_seuil: p("NL_AHK_SEUIL")?,
        ahk_afbouw: p("NL_AHK_AFBOUW")?,
        ak_s1_plafond: p("NL_AK_S1_PLAFOND")?,
        ak_s1_taux: p("NL_AK_S1_TAUX")?,
        ak_s2_plafond: p("NL_AK_S2_PLAFOND")?,
        ak_s2_base: p("NL_AK_S2_BASE")?,
        ak_s2_taux: p("NL_AK_S2_TAUX")?,
        ak_s3_plafond: p("NL_AK_S3_PLAFOND")?,
        ak_s3_base: p("NL_AK_S3_BASE")?,
        ak_s3_taux: p("NL_AK_S3_TAUX")?,
        ak_max: p("NL_AK_MAX")?,
        ak_afbouw_seuil: p("NL_AK_AFBOUW_SEUIL")?,
        ak_afbouw: p("NL_AK_AFBOUW")?,
        max_premieloon: p("NL_MAX_PREMIELOON")?,
    })
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
    let p = nl_parametres(ctx)?;
    let rev_ann = brut * dec!(12);

    let box1   = box1_brut(rev_ann, &p);
    let ahk    = algemene_heffingskorting(rev_ann, &p);
    let ak     = arbeidskorting(rev_ann, &p);
    let heffing_ann = (box1 - ahk - ak).max(Decimal::ZERO);
    let mensuel = (heffing_ann / dec!(12)).round_dp(2);
    let taux_eff = if brut > Decimal::ZERO { (mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    Some(LigneCotisation {
        code:        "NL_LOONHEFFING".into(),
        libelle:     ctx.libelle("NL_LOONHEFFING", "Loonheffing — Impôt sur le revenu + premies volksverzekeringen"),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: ctx.expl("NL_LOONHEFFING",
            "Loonheffing {annee} (salarié sous l'âge AOW) — retenue à la source mensuelle.\n\n\
            Revenu annuel estimé : {rev} €\n\n\
            Barème box 1 :\n\
            • tranche 1 (0 → {b1p} €) : {b1t} % (dont premies volksverzekeringen {pvv} %)\n\
            • tranche 2 ({b1p} → {b2p} €) : {b2t} %\n\
            • tranche 3 (> {b2p} €) : {b3t} %\n\
            = impôt + premies bruts : {box1} €/an\n\n\
            − algemene heffingskorting : {ahk} €\n\
            − arbeidskorting : {ak} €\n\
            = loonheffing : {ha} €/an / 12 = {mens} €/mois\n\
            Taux effectif : {teff} %\n\n\
            Base légale : Wet IB 2001 ; Belastingplan {annee}.")
            .replace("{annee}", &annee.to_string())
            .replace("{rev}",  &format!("{:.0}", rev_ann))
            .replace("{b1p}",  &format!("{:.0}", p.b1_plafond))
            .replace("{b1t}",  &format!("{:.2}", p.b1_taux * dec!(100)))
            .replace("{pvv}",  &format!("{:.2}", p.premie_vv * dec!(100)))
            .replace("{b2p}",  &format!("{:.0}", p.b2_plafond))
            .replace("{b2t}",  &format!("{:.2}", p.b2_taux * dec!(100)))
            .replace("{b3t}",  &format!("{:.2}", p.b3_taux * dec!(100)))
            .replace("{box1}", &format!("{:.0}", box1))
            .replace("{ahk}",  &format!("{:.0}", ahk))
            .replace("{ak}",   &format!("{:.0}", ak))
            .replace("{ha}",   &format!("{:.0}", heffing_ann))
            .replace("{mens}", &format!("{:.2}", mensuel))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("Wet IB 2001 — Belastingplan 2026")),
    })
}
