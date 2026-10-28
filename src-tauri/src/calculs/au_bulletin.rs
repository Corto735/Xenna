// ── Australie — Impôt sur le revenu (PAYG) + Medicare levy + Superannuation ──
//
// Résident fiscal, secteur privé. Le salarié supporte :
//   • l'impôt sur le revenu (barème progressif des résidents) ;
//   • le Medicare levy (2 % depuis 2014-15 ; réductions bas revenus ignorées).
// L'employeur verse la Superannuation Guarantee (taux lu en base, échéancier 9,5 → 12 %).
//
// Exercice fiscal : 1er juillet → 30 juin, désigné par son année de fin (une paie de
// septembre 2026 relève de l'exercice 2026-27, noté 2027).
// Les crédits d'impôt (LITO, ancien LMITO) ne sont PAS modélisés → net prudent.
// Sources : Income Tax Assessment Act 1997 ; Medicare Levy Act 1986 ; SGAA 1992 (ATO).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

// Barème des résidents par exercice (AU_IT) et taux du Medicare levy
// (AU_MEDICARE_TAUX) en base (migration 0153). Pas de barème à la date : non couvert.

pub fn generer_bulletin_au(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = if ctx.date_paie.month() >= 7 { ctx.date_paie.year() + 1 } else { ctx.date_paie.year() };

    let (Some(impot_an), Some(t_medicare)) =
        (ctx.bareme_progressif("AU_IT", brut * dec!(12)), ctx.plafond("AU_MEDICARE_TAUX")) else {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "AUD", "AU",
            "Australie : données disponibles pour les exercices 2014-15 à 2026-27.", ctx);
    };

    let rev_ann = brut * dec!(12);
    let impot_mens = (impot_an / dec!(12)).round_dp(2);
    let taux_impot = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    let ligne_impot = LigneCotisation {
        code: "AU_INCOME_TAX".into(),
        libelle: ctx.libelle("AU_INCOME_TAX", "Income tax — Impôt sur le revenu (PAYG)"),
        base: brut, taux_sal: taux_impot, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("AU_INCOME_TAX",
            "Impôt sur le revenu des résidents — barème de l'exercice {fy0}-{fy1}.\n\n\
            Revenu annuel estimé : {rev} $ → impôt {imp} $/an / 12 = {mens} $/mois.\n\
            Tranche exonérée : 18 200 $. Crédits LITO/LMITO non modélisés (net prudent).\n\n\
            Base légale : Income Tax Assessment Act 1997.")
            .replace("{fy0}", &(annee - 1).to_string())
            .replace("{fy1}", &annee.to_string())
            .replace("{rev}", &format!("{:.0}", rev_ann))
            .replace("{imp}", &format!("{:.0}", impot_an))
            .replace("{mens}", &format!("{:.2}", impot_mens)),
        loi_ref: Some(ctx.loi_ref("Income Tax Assessment Act 1997")),
    };

    // Medicare levy (2 % depuis 2014-15)
    let medicare_mens = (brut * t_medicare).round_dp(2);
    let ligne_medicare = LigneCotisation {
        code: "AU_MEDICARE".into(),
        libelle: ctx.libelle("AU_MEDICARE", "Medicare levy — Contribution santé (2 %)"),
        base: brut, taux_sal: t_medicare, montant_sal: medicare_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("AU_MEDICARE",
            "Medicare levy — 2 % du revenu imposable (financement de la santé publique).\n\
            Montant : {m} $/mois. Réductions bas revenus et surtaxe (MLS) non modélisées.\n\n\
            Base légale : Medicare Levy Act 1986.")
            .replace("{m}", &format!("{:.2}", medicare_mens)),
        loi_ref: Some(ctx.loi_ref("Medicare Levy Act 1986")),
    };

    // Superannuation Guarantee (patronale, taux daté lu en base ; échéancier 9,5 → 12 %)
    // Maximum contribution base : 62 500 $/trimestre en 2025-26 (250 000 $/an) ; devenue
    // annuelle avec le Payday Super, 270 830 $ en 2026-27 (32 500 $ de SG à 12 %).
    let plafond_super_mens = ctx.plafond("AU_SUPER_PLAFOND").unwrap_or(Decimal::MAX) / dec!(12);
    let base_super = brut.min(plafond_super_mens);
    let ts = ctx.taux_pat("AU_SUPER");
    let ligne_super = LigneCotisation {
        code: "AU_SUPER".into(),
        libelle: ctx.libelle("AU_SUPER", "Superannuation Guarantee — Retraite (employeur)"),
        base: base_super, taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: ts, montant_pat: (base_super * ts).round_dp(2),
        categorie: "Cotisations patronales".into(),
        explication: ctx.expl("AU_SUPER",
            "Superannuation Guarantee — retraite, 100 % patronale, versée en sus du salaire.\n\
            Taux de l'exercice : {t} %. Assiette plafonnée à la maximum contribution base.\n\
            Employeur : {mp} $/mois.\n\n\
            Base légale : SGAA 1992.")
            .replace("{t}", &format!("{:.2}", ts * dec!(100)))
            .replace("{mp}", &format!("{:.2}", (base_super * ts).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("Superannuation Guarantee (Administration) Act 1992")),
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
        heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), pas: None, droits: None, alertes: Vec::new(), evolutions: Vec::new(),
        salarie,
    }
}
