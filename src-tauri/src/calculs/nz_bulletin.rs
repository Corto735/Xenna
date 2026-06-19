// ── Nouvelle-Zélande — PAYE + ACC earner's levy + KiwiSaver ──────────────────
//
// Résident fiscal, secteur privé. Pas de sécurité sociale par cotisations.
// Le salarié supporte : l'impôt sur le revenu (PAYE, sans tranche exonérée) et
// l'ACC earner's levy (plafonné). KiwiSaver : employeur 3 % par défaut, en sus.
//
// Année calendaire Y → année fiscale (Y-1)/Y (clôture au 31 mars). Compromis documenté.
// Le seuil de 39 % (créé 1er avr. 2021) ne change le net que > 180 000 $. Les seuils ont été
// relevés au 1er avr. 2024 (réforme 2024). Sources : Income Tax Act 2007 ; Accident
// Compensation Act 2001 ; KiwiSaver Act 2006 (IRD/ACC).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

fn tax_from_bands(revenu: Decimal, bands: &[(Decimal, Decimal)], top: Decimal) -> Decimal {
    let mut tax = Decimal::ZERO;
    let mut prev = Decimal::ZERO;
    for &(upper, rate) in bands {
        if revenu > upper {
            tax += (upper - prev) * rate;
            prev = upper;
        } else {
            tax += (revenu - prev).max(Decimal::ZERO) * rate;
            return tax;
        }
    }
    tax + (revenu - prev).max(Decimal::ZERO) * top
}

/// Barème PAYE annuel par année calendaire. `None` = non couvert.
fn paye_annuel(revenu: Decimal, annee: i32) -> Option<Decimal> {
    let (bands, top): (&[(Decimal, Decimal)], Decimal) = match annee {
        // FY2014-15 → 2020-21 : 4 tranches, sans 39 %.
        2015..=2021 => (&[(dec!(14000), dec!(0.105)), (dec!(48000), dec!(0.175)),
                          (dec!(70000), dec!(0.30))], dec!(0.33)),
        // FY2021-22 → 2023-24 : ajout 39 % au-delà de 180 000.
        2022..=2024 => (&[(dec!(14000), dec!(0.105)), (dec!(48000), dec!(0.175)),
                          (dec!(70000), dec!(0.30)), (dec!(180000), dec!(0.33))], dec!(0.39)),
        // FY2024-25 / 2025-26 : seuils relevés (réforme 31 juil. 2024).
        2025 | 2026 => (&[(dec!(15600), dec!(0.105)), (dec!(53500), dec!(0.175)),
                          (dec!(78100), dec!(0.30)), (dec!(180000), dec!(0.33))], dec!(0.39)),
        _ => return None,
    };
    Some(tax_from_bands(revenu, bands, top))
}

/// ACC earner's levy : (taux, plafond annuel de gains soumis) par année calendaire.
fn acc_params(annee: i32) -> (Decimal, Decimal) {
    match annee {
        2015        => (dec!(0.0145), dec!(118191)),
        2016        => (dec!(0.0145), dec!(120070)),
        2017        => (dec!(0.0139), dec!(122063)),
        2018        => (dec!(0.0139), dec!(124053)),
        2019        => (dec!(0.0139), dec!(126286)),
        2020        => (dec!(0.0139), dec!(128470)),
        2021        => (dec!(0.0139), dec!(130911)),
        2022        => (dec!(0.0139), dec!(130911)),
        2023        => (dec!(0.0146), dec!(136544)),
        2024        => (dec!(0.0153), dec!(139384)),
        2025        => (dec!(0.0160), dec!(142283)),
        _           => (dec!(0.0167), dec!(152790)), // 2026
    }
}

pub fn generer_bulletin_nz(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();
    let rev_ann = brut * dec!(12);

    let Some(paye_an) = paye_annuel(rev_ann, annee) else {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "NZD",
            "Nouvelle-Zélande : données disponibles pour les années fiscales 2014-15 à 2025-26.",
        );
    };

    // PAYE (impôt sur le revenu)
    let paye_mens = (paye_an / dec!(12)).round_dp(2);
    let taux_paye = if brut > Decimal::ZERO { (paye_mens / brut).round_dp(4) } else { Decimal::ZERO };
    let ligne_paye = LigneCotisation {
        code: "NZ_PAYE".into(),
        libelle: "PAYE — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_paye, montant_sal: paye_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu (PAYE) — année fiscale {fy0}-{fy1}, sans tranche exonérée.\n\n\
            Revenu annuel estimé : {rev:.0} $ → {imp:.0} $/an / 12 = {mens:.2} $/mois.\n\n\
            Base légale : Income Tax Act 2007.",
            fy0 = annee - 1, fy1 = annee, rev = rev_ann, imp = paye_an, mens = paye_mens,
        ),
        loi_ref: Some("Income Tax Act 2007".into()),
    };

    // ACC earner's levy (taux + plafond datés)
    let (acc_taux, acc_cap) = acc_params(annee);
    let plafond_acc_mens = acc_cap / dec!(12);
    let base_acc = brut.min(plafond_acc_mens);
    let acc_mens = (base_acc * acc_taux).round_dp(2);
    let ligne_acc = LigneCotisation {
        code: "NZ_ACC".into(),
        libelle: "ACC earner's levy — Assurance accidents".into(),
        base: base_acc, taux_sal: acc_taux, montant_sal: acc_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "ACC earner's levy — couverture accidents, {t:.2} % du salaire brut (année {fy0}-{fy1}).\n\
            Assiette plafonnée à {cap:.0} $/an. Montant : {m:.2} $/mois.\n\n\
            Base légale : Accident Compensation Act 2001.",
            t = acc_taux * dec!(100), fy0 = annee - 1, fy1 = annee, cap = acc_cap, m = acc_mens,
        ),
        loi_ref: Some("Accident Compensation Act 2001".into()),
    };

    // KiwiSaver employeur (défaut 3 %, taux lu en base), en sus du salaire
    let tk = ctx.taux_pat("NZ_KIWISAVER_EMP");
    let ligne_ks = LigneCotisation {
        code: "NZ_KIWISAVER_EMP".into(),
        libelle: "KiwiSaver — Retraite (employeur, défaut 3 %)".into(),
        base: brut, taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: tk, montant_pat: (brut * tk).round_dp(2),
        categorie: "Cotisations patronales".into(),
        explication: format!(
            "KiwiSaver — épargne-retraite, cotisation employeur par défaut {t:.1} %, versée en sus.\n\
            Optionnelle selon adhésion du salarié.\nEmployeur : {mp:.2} $/mois.\n\n\
            Base légale : KiwiSaver Act 2006.",
            t = tk * dec!(100), mp = (brut * tk).round_dp(2),
        ),
        loi_ref: Some("KiwiSaver Act 2006".into()),
    };

    let cotisations = vec![ligne_paye, ligne_acc, ligne_ks];
    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "NZD".into(),
        absence: None,
        heures_sup: None,
        salarie,
    }
}
