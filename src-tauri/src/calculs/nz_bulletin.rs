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

// Barème PAYE (NZ_PAYE), taux et plafond de l'ACC earner's levy (NZ_ACC_TAUX,
// NZ_ACC_PLAFOND) en base, par exercice (migration 0153).

pub fn generer_bulletin_nz(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    // Exercice néo-zélandais : 1er avril → 31 mars, désigné par son année de fin.
    let annee = if ctx.date_paie.month() >= 4 { ctx.date_paie.year() + 1 } else { ctx.date_paie.year() };
    let rev_ann = brut * dec!(12);

    let (Some(paye_an), Some(acc_taux), Some(acc_cap)) = (
        ctx.bareme_progressif("NZ_PAYE", rev_ann),
        ctx.plafond("NZ_ACC_TAUX"),
        ctx.plafond("NZ_ACC_PLAFOND"),
    ) else {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "NZD", "NZ",
            "Nouvelle-Zélande : données disponibles pour les années fiscales 2014-15 à 2026-27.", ctx);
    };

    // PAYE (impôt sur le revenu)
    let paye_mens = (paye_an / dec!(12)).round_dp(2);
    let taux_paye = if brut > Decimal::ZERO { (paye_mens / brut).round_dp(4) } else { Decimal::ZERO };
    let ligne_paye = LigneCotisation {
        code: "NZ_PAYE".into(),
        libelle: ctx.libelle("NZ_PAYE", "PAYE — Impôt sur le revenu"),
        base: brut, taux_sal: taux_paye, montant_sal: paye_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("NZ_PAYE",
            "Impôt sur le revenu (PAYE) — année fiscale {fy0}-{fy1}, sans tranche exonérée.\n\n\
            Revenu annuel estimé : {rev} $ → {imp} $/an / 12 = {mens} $/mois.\n\n\
            Base légale : Income Tax Act 2007.")
            .replace("{fy0}", &(annee - 1).to_string())
            .replace("{fy1}", &annee.to_string())
            .replace("{rev}", &format!("{:.0}", rev_ann))
            .replace("{imp}", &format!("{:.0}", paye_an))
            .replace("{mens}", &format!("{:.2}", paye_mens)),
        loi_ref: Some(ctx.loi_ref("Income Tax Act 2007")),
    };

    // ACC earner's levy (taux + plafond datés)
    let plafond_acc_mens = acc_cap / dec!(12);
    let base_acc = brut.min(plafond_acc_mens);
    let acc_mens = (base_acc * acc_taux).round_dp(2);
    let ligne_acc = LigneCotisation {
        code: "NZ_ACC".into(),
        libelle: ctx.libelle("NZ_ACC", "ACC earner's levy — Assurance accidents"),
        base: base_acc, taux_sal: acc_taux, montant_sal: acc_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("NZ_ACC",
            "ACC earner's levy — couverture accidents, {t} % du salaire brut (année {fy0}-{fy1}).\n\
            Assiette plafonnée à {cap} $/an. Montant : {m} $/mois.\n\n\
            Base légale : Accident Compensation Act 2001.")
            .replace("{t}", &format!("{:.2}", acc_taux * dec!(100)))
            .replace("{fy0}", &(annee - 1).to_string())
            .replace("{fy1}", &annee.to_string())
            .replace("{cap}", &format!("{:.0}", acc_cap))
            .replace("{m}", &format!("{:.2}", acc_mens)),
        loi_ref: Some(ctx.loi_ref("Accident Compensation Act 2001")),
    };

    // KiwiSaver employeur (défaut 3 %, taux lu en base), en sus du salaire
    let tk = ctx.taux_pat("NZ_KIWISAVER_EMP");
    let ligne_ks = LigneCotisation {
        code: "NZ_KIWISAVER_EMP".into(),
        libelle: ctx.libelle("NZ_KIWISAVER_EMP", "KiwiSaver — Retraite (employeur, défaut {t} %)")
            .replace("{t}", &format!("{:.1}", tk * dec!(100)).replace(".0", "")),
        base: brut, taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: tk, montant_pat: (brut * tk).round_dp(2),
        categorie: "Cotisations patronales".into(),
        explication: ctx.expl("NZ_KIWISAVER_EMP",
            "KiwiSaver — épargne-retraite, cotisation employeur par défaut {t} %, versée en sus.\n\
            Optionnelle selon adhésion du salarié.\nEmployeur : {mp} $/mois.\n\n\
            Base légale : KiwiSaver Act 2006.")
            .replace("{t}", &format!("{:.1}", tk * dec!(100)))
            .replace("{mp}", &format!("{:.2}", (brut * tk).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("KiwiSaver Act 2006")),
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
        heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), pas: None, droits: None, alertes: Vec::new(), evolutions: Vec::new(),
        salarie,
    }
}
