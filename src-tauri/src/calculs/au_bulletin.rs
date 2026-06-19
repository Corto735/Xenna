// ── Australie — Impôt sur le revenu (PAYG) + Medicare levy + Superannuation ──
//
// Résident fiscal, secteur privé. Le salarié supporte :
//   • l'impôt sur le revenu (barème progressif des résidents) ;
//   • le Medicare levy (2 % depuis 2014-15 ; réductions bas revenus ignorées).
// L'employeur verse la Superannuation Guarantee (taux lu en base, échéancier 9,5 → 12 %).
//
// Année calendaire Y → exercice fiscal (Y-1)/Y (l'exercice court du 1er juil. au 30 juin ;
// le 1er semestre de l'année Y appartient à l'exercice (Y-1)/Y). Compromis documenté.
// Les crédits d'impôt (LITO, ancien LMITO) ne sont PAS modélisés → net prudent.
// Sources : Income Tax Assessment Act 1997 ; Medicare Levy Act 1986 ; SGAA 1992 (ATO).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Impôt à partir d'un barème : `bands` = [(borne haute, taux marginal)], `top` au-delà.
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

/// Barème résident par année calendaire (→ exercice (Y-1)/Y). `None` = non couvert.
fn impot_annuel(revenu: Decimal, annee: i32) -> Option<Decimal> {
    // Tranche exonérée 18 200 € constante depuis 2012-13.
    let bands: &[(Decimal, Decimal)] = match annee {
        // FY2014-15 / 2015-16 : 32,5 % jusqu'à 80 000.
        2015 | 2016 => &[(dec!(18200), dec!(0.0)), (dec!(37000), dec!(0.19)),
                         (dec!(80000), dec!(0.325)), (dec!(180000), dec!(0.37))],
        // FY2016-17 / 2017-18 : 32,5 % jusqu'à 87 000.
        2017 | 2018 => &[(dec!(18200), dec!(0.0)), (dec!(37000), dec!(0.19)),
                         (dec!(87000), dec!(0.325)), (dec!(180000), dec!(0.37))],
        // FY2018-19 / 2019-20 : 32,5 % jusqu'à 90 000.
        2019 | 2020 => &[(dec!(18200), dec!(0.0)), (dec!(37000), dec!(0.19)),
                         (dec!(90000), dec!(0.325)), (dec!(180000), dec!(0.37))],
        // FY2020-21 → 2023-24 (Stage 2) : 19 % jusqu'à 45 000, 32,5 % jusqu'à 120 000.
        2021..=2024 => &[(dec!(18200), dec!(0.0)), (dec!(45000), dec!(0.19)),
                         (dec!(120000), dec!(0.325)), (dec!(180000), dec!(0.37))],
        // FY2024-25 / 2025-26 (Stage 3) : 16 % jusqu'à 45 000, 30 % jusqu'à 135 000.
        2025 | 2026 => &[(dec!(18200), dec!(0.0)), (dec!(45000), dec!(0.16)),
                         (dec!(135000), dec!(0.30)), (dec!(190000), dec!(0.37))],
        _ => return None,
    };
    Some(tax_from_bands(revenu, bands, dec!(0.45)))
}

pub fn generer_bulletin_au(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    let Some(impot_an) = impot_annuel(brut * dec!(12), annee) else {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "AUD",
            "Australie : données disponibles pour les exercices 2014-15 à 2025-26.",
        );
    };

    let rev_ann = brut * dec!(12);
    let impot_mens = (impot_an / dec!(12)).round_dp(2);
    let taux_impot = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    let ligne_impot = LigneCotisation {
        code: "AU_INCOME_TAX".into(),
        libelle: "Income tax — Impôt sur le revenu (PAYG)".into(),
        base: brut, taux_sal: taux_impot, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu des résidents — barème de l'exercice {fy0}-{fy1}.\n\n\
            Revenu annuel estimé : {rev:.0} $ → impôt {imp:.0} $/an / 12 = {mens:.2} $/mois.\n\
            Tranche exonérée : 18 200 $. Crédits LITO/LMITO non modélisés (net prudent).\n\n\
            Base légale : Income Tax Assessment Act 1997.",
            fy0 = annee - 1, fy1 = annee, rev = rev_ann, imp = impot_an, mens = impot_mens,
        ),
        loi_ref: Some("Income Tax Assessment Act 1997".into()),
    };

    // Medicare levy 2 % (constant depuis 2014-15)
    let medicare_mens = (brut * dec!(0.02)).round_dp(2);
    let ligne_medicare = LigneCotisation {
        code: "AU_MEDICARE".into(),
        libelle: "Medicare levy — Contribution santé (2 %)".into(),
        base: brut, taux_sal: dec!(0.02), montant_sal: medicare_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "Medicare levy — 2 % du revenu imposable (financement de la santé publique).\n\
            Montant : {m:.2} $/mois. Réductions bas revenus et surtaxe (MLS) non modélisées.\n\n\
            Base légale : Medicare Levy Act 1986.",
            m = medicare_mens,
        ),
        loi_ref: Some("Medicare Levy Act 1986".into()),
    };

    // Superannuation Guarantee (patronale, taux daté lu en base ; échéancier 9,5 → 12 %)
    let plafond_super_mens = dec!(250000) / dec!(12); // maximum contribution base (≈ 2025-26)
    let base_super = brut.min(plafond_super_mens);
    let ts = ctx.taux_pat("AU_SUPER");
    let ligne_super = LigneCotisation {
        code: "AU_SUPER".into(),
        libelle: "Superannuation Guarantee — Retraite (employeur)".into(),
        base: base_super, taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: ts, montant_pat: (base_super * ts).round_dp(2),
        categorie: "Cotisations patronales".into(),
        explication: format!(
            "Superannuation Guarantee — retraite, 100 % patronale, versée en sus du salaire.\n\
            Taux de l'exercice : {t:.2} %. Assiette plafonnée à la maximum contribution base.\n\
            Employeur : {mp:.2} $/mois.\n\n\
            Base légale : SGAA 1992.",
            t = ts * dec!(100), mp = (base_super * ts).round_dp(2),
        ),
        loi_ref: Some("Superannuation Guarantee (Administration) Act 1992".into()),
    };

    let cotisations = vec![ligne_impot, ligne_medicare, ligne_super];
    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "AUD".into(),
        absence: None,
        heures_sup: None,
        salarie,
    }
}
