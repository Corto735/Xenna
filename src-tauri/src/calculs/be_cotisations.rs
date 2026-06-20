// ── Cotisations Belgique — régime général ONSS, secteur privé ────────────────
//
// Assiette ONSS : salaire brut réel, sans plafond (régime général).
// Taux lus depuis ContextPaie (DB). Bonus emploi et réduction structurelle
// entièrement calculés en Rust (comme IT_IRPEF / PT_IRS).
//
// Sources légales :
//   Loi 27/06/1969 (ONSS) ; AR annuels ONSS
//   Loi 20/12/1999 (bonus emploi) ; AR 16/05/2003 (réd. structurelle)

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── ONSS salarial ─────────────────────────────────────────────────────────────

pub fn onss_salarial(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("BE_ONSS_SAL");
    LigneCotisation {
        code:        "BE_ONSS_SAL".into(),
        libelle:     ctx.libelle("BE_ONSS_SAL", "ONSS — cotisation salariale personnelle"),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Sécurité sociale".into(),
        explication: ctx.expl("BE_ONSS_SAL",
            "Cotisation personnelle ONSS de 13,07 % sur le salaire brut. \
            Couvre : maladie-invalidité, pension, chômage, accidents du travail, \
            allocations familiales. Assiette : salaire brut intégral, sans plafond.\n\n\
            Salarié : {ts_pct} % × {brut} € = {ms} €\n\
            Taux stable depuis 2003. \
            Base légale : Loi du 27/06/1969 ; AR ONSS annuels.")
            .replace("{ts_pct}", &format!("{:.2}", ts * dec!(100)))
            .replace("{brut}", &format!("{:.2}", brut))
            .replace("{ms}", &format!("{:.2}", (brut * ts).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("Loi 27/06/1969 — AR ONSS annuels")),
    }
}

// ── ONSS patronal ─────────────────────────────────────────────────────────────

pub fn onss_patronal(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("BE_ONSS_PAT");
    LigneCotisation {
        code:        "BE_ONSS_PAT".into(),
        libelle:     ctx.libelle("BE_ONSS_PAT", "ONSS — cotisation patronale (taux global)"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité sociale".into(),
        explication: ctx.expl("BE_ONSS_PAT",
            "Cotisation patronale globale ONSS ({tp_pct} % du brut). \
            Regroupe : pension (8,86 %), maladie-invalidité (5,90 %), \
            chômage (1,46 %), allocations familiales (5,25 %), divers. \
            La réduction structurelle (BE_RED_STRUCT) est appliquée séparément. \
            Assiette : salaire brut intégral, sans plafond.\n\n\
            Employeur : {tp_pct} % × {brut} € = {mp} €\n\
            Base légale : Loi 27/06/1969 ; AR ONSS annuels.")
            .replace("{tp_pct}", &format!("{:.2}", tp * dec!(100)))
            .replace("{brut}", &format!("{:.2}", brut))
            .replace("{mp}", &format!("{:.2}", (brut * tp).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("Loi 27/06/1969 — AR ONSS annuels")),
    }
}

// ── Bonus emploi ─────────────────────────────────────────────────────────────
//
// Réduction des cotisations personnelles ONSS pour les bas salaires.
// Montant mensuel dégressif entre seuil_bas et seuil_haut.
// Retourne None si le salaire dépasse le seuil haut.

fn bonus_emploi_params(annee: i32) -> (Decimal, Decimal, Decimal) {
    // (max_mensuel, seuil_bas_annuel, seuil_haut_annuel)
    match annee {
        i32::MIN..=2018 => (dec!(15.48), dec!(17800), dec!(27082)),
        2019             => (dec!(16.03), dec!(20832), dec!(29736)),
        2020 | 2021      => (dec!(16.44), dec!(20832), dec!(29736)),
        2022 | 2023      => (dec!(16.91), dec!(20832), dec!(29736)),
        _                => (dec!(17.81), dec!(21060), dec!(30120)), // 2024+
    }
}

pub fn bonus_emploi(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let annee = ctx.date_paie.year();
    let (max_mensuel, seuil_bas, seuil_haut) = bonus_emploi_params(annee);
    let annuel = brut * dec!(12);
    if annuel > seuil_haut {
        return None;
    }
    let montant = if annuel <= seuil_bas {
        max_mensuel
    } else {
        (max_mensuel * (seuil_haut - annuel) / (seuil_haut - seuil_bas)).round_dp(2)
    };
    let montant = montant.max(Decimal::ZERO);
    if montant == Decimal::ZERO {
        return None;
    }
    let taux_eff = if brut > Decimal::ZERO { -(montant / brut).round_dp(4) } else { Decimal::ZERO };
    Some(LigneCotisation {
        code:        "BE_BONUS_EMPLOI".into(),
        libelle:     ctx.libelle("BE_BONUS_EMPLOI", "Bonus emploi — réduction cotisations ONSS {annee}")
            .replace("{annee}", &annee.to_string()),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: -montant,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Réduction salariale".into(),
        explication: ctx.expl("BE_BONUS_EMPLOI",
            "Réduction mensuelle des cotisations personnelles ONSS (13,07 %) \
            pour les travailleurs à bas salaire. \
            Le montant est dégressif entre le seuil bas et le seuil haut.\n\n\
            {annee} : seuil bas {sb} €/an — seuil haut {sh} €/an — max {mm} €/mois\n\
            Salaire annuel estimé : {ann} € → réduction mensuelle : {m} €\n\
            Taux effectif indicatif : {teff} %\n\
            \n\
            La réduction est déduite de la cotisation ONSS due par le travailleur. \
            Base légale : Loi 20/12/1999 ; AR annuels ONSS.")
            .replace("{annee}", &annee.to_string())
            .replace("{sb}", &format!("{:.0}", seuil_bas))
            .replace("{sh}", &format!("{:.0}", seuil_haut))
            .replace("{mm}", &format!("{:.2}", max_mensuel))
            .replace("{ann}", &format!("{:.2}", annuel))
            .replace("{m}", &format!("{:.2}", montant))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("Loi 20/12/1999 — AR ONSS annuels (bonus emploi)")),
    })
}

// ── Réduction structurelle patronale ─────────────────────────────────────────
//
// Réduction forfaitaire mensuelle des cotisations patronales ONSS.
// Dégressive si salaire dépasse le seuil (→ 0 à seuil × 1,5).
// Retourne None si salaire trop élevé.

fn red_struct_params(annee: i32) -> (Decimal, Decimal) {
    // (montant_plein_mensuel, seuil_annuel)
    match annee {
        i32::MIN..=2018 => (dec!(100.00), dec!(24000)),
        2019..=2021      => (dec!(103.60), dec!(24012)),
        2022 | 2023      => (dec!(107.64), dec!(24012)),
        _                => (dec!(109.54), dec!(24012)), // 2024+
    }
}

pub fn reduction_structurelle(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let annee = ctx.date_paie.year();
    let (montant_plein, seuil_annuel) = red_struct_params(annee);
    let annuel = brut * dec!(12);
    let seuil_haut = seuil_annuel * dec!(1.5);
    let montant = if annuel <= seuil_annuel {
        montant_plein
    } else if annuel < seuil_haut {
        (montant_plein * (seuil_haut - annuel) / (seuil_haut - seuil_annuel)).round_dp(2)
    } else {
        Decimal::ZERO
    };
    if montant == Decimal::ZERO {
        return None;
    }
    let taux_eff = if brut > Decimal::ZERO { -(montant / brut).round_dp(4) } else { Decimal::ZERO };
    Some(LigneCotisation {
        code:        "BE_RED_STRUCT".into(),
        libelle:     ctx.libelle("BE_RED_STRUCT", "Réduction structurelle patronale {annee}")
            .replace("{annee}", &annee.to_string()),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    taux_eff,
        montant_pat: -montant,
        categorie:   "Réduction patronale".into(),
        explication: ctx.expl("BE_RED_STRUCT",
            "Réduction mensuelle des cotisations patronales ONSS (AR 16/05/2003). \
            Montant forfaitaire si salaire ≤ seuil, dégressif jusqu'à 1,5 × seuil.\n\n\
            {annee} : montant plein {mp} €/mois — seuil {seuil} €/an\n\
            Salaire annuel estimé : {ann} € → réduction mensuelle : {m} €\n\
            \n\
            La réduction est décomptée du total des cotisations patronales ONSS. \
            Base légale : AR 16/05/2003 (réd. structurelle) + AR annuels ONSS.")
            .replace("{annee}", &annee.to_string())
            .replace("{mp}", &format!("{:.2}", montant_plein))
            .replace("{seuil}", &format!("{:.0}", seuil_annuel))
            .replace("{ann}", &format!("{:.2}", annuel))
            .replace("{m}", &format!("{:.2}", montant)),
        loi_ref: Some(ctx.loi_ref("AR 16/05/2003 (réd. structurelle patronale) — AR ONSS annuels")),
    })
}
