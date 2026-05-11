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
        libelle:     "ONSS — cotisation salariale personnelle".into(),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Sécurité sociale".into(),
        explication: format!(
            "Cotisation personnelle ONSS de 13,07 % sur le salaire brut. \
            Couvre : maladie-invalidité, pension, chômage, accidents du travail, \
            allocations familiales. Assiette : salaire brut intégral, sans plafond.\n\n\
            Salarié : {ts_pct:.2} % × {brut:.2} € = {ms:.2} €\n\
            Taux stable depuis 2003. \
            Base légale : Loi du 27/06/1969 ; AR ONSS annuels.",
            ts_pct = ts * dec!(100),
            ms     = (brut * ts).round_dp(2),
        ),
        loi_ref: Some("Loi 27/06/1969 — AR ONSS annuels".into()),
    }
}

// ── ONSS patronal ─────────────────────────────────────────────────────────────

pub fn onss_patronal(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("BE_ONSS_PAT");
    LigneCotisation {
        code:        "BE_ONSS_PAT".into(),
        libelle:     "ONSS — cotisation patronale (taux global)".into(),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité sociale".into(),
        explication: format!(
            "Cotisation patronale globale ONSS ({tp_pct:.2} % du brut). \
            Regroupe : pension ({:.2} %), maladie-invalidité ({:.2} %), \
            chômage ({:.2} %), allocations familiales ({:.2} %), divers. \
            La réduction structurelle (BE_RED_STRUCT) est appliquée séparément. \
            Assiette : salaire brut intégral, sans plafond.\n\n\
            Employeur : {tp_pct:.2} % × {brut:.2} € = {mp:.2} €\n\
            Base légale : Loi 27/06/1969 ; AR ONSS annuels.",
            dec!(8.86), dec!(3.55) + dec!(2.35), dec!(1.46), dec!(5.25),
            tp_pct = tp * dec!(100),
            mp     = (brut * tp).round_dp(2),
        ),
        loi_ref: Some("Loi 27/06/1969 — AR ONSS annuels".into()),
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
        libelle:     format!("Bonus emploi — réduction cotisations ONSS {annee}"),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: -montant,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Réduction salariale".into(),
        explication: format!(
            "Réduction mensuelle des cotisations personnelles ONSS (13,07 %) \
            pour les travailleurs à bas salaire. \
            Le montant est dégressif entre le seuil bas et le seuil haut.\n\n\
            {annee} : seuil bas {sb:.0} €/an — seuil haut {sh:.0} €/an — max {mm:.2} €/mois\n\
            Salaire annuel estimé : {ann:.2} € → réduction mensuelle : {m:.2} €\n\
            Taux effectif indicatif : {teff:.2} %\n\
            \n\
            La réduction est déduite de la cotisation ONSS due par le travailleur. \
            Base légale : Loi 20/12/1999 ; AR annuels ONSS.",
            annee = annee,
            sb    = seuil_bas,
            sh    = seuil_haut,
            mm    = max_mensuel,
            ann   = annuel,
            m     = montant,
            teff  = taux_eff * dec!(100),
        ),
        loi_ref: Some("Loi 20/12/1999 — AR ONSS annuels (bonus emploi)".into()),
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
        libelle:     format!("Réduction structurelle patronale {annee}"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    taux_eff,
        montant_pat: -montant,
        categorie:   "Réduction patronale".into(),
        explication: format!(
            "Réduction mensuelle des cotisations patronales ONSS (AR 16/05/2003). \
            Montant forfaitaire si salaire ≤ seuil, dégressif jusqu''à 1,5 × seuil.\n\n\
            {annee} : montant plein {mp:.2} €/mois — seuil {seuil:.0} €/an\n\
            Salaire annuel estimé : {ann:.2} € → réduction mensuelle : {m:.2} €\n\
            \n\
            La réduction est décomptée du total des cotisations patronales ONSS. \
            Base légale : AR 16/05/2003 (réd. structurelle) + AR annuels ONSS.",
            annee = annee,
            mp    = montant_plein,
            seuil = seuil_annuel,
            ann   = annuel,
            m     = montant,
        ),
        loi_ref: Some("AR 16/05/2003 (réd. structurelle patronale) — AR ONSS annuels".into()),
    })
}
