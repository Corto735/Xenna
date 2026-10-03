use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Plafond cotisable luxembourgeois (5 × SSM non-qualifié) ──────────────────
//
// Le SSM est indexé automatiquement (échelle mobile, tranche ±2,5 %)
// et peut faire l'objet de revalorisations discrétionnaires (accords tripartites).
// Source : CCSS — Journal officiel luxembourgeois.
// Valeurs approximatives pour les années 2015-2024 (à vérifier contre CCSS).
// Depuis 2025, au mois près : SSM non qualifié 2 637,79 € (janv.-avr. 2025),
// 2 703,74 € (indice 968,04, mai 2025-mai 2026), 2 771,33 € (indice 992,24,
// dès juin 2026) — FEDIL, paramètres sociaux au 01/01/2026 et au 01/06/2026.
fn lu_plafond_mensuel(ctx: &ContextPaie) -> Decimal {
    let d = ctx.date_paie;
    match d.year() {
        i32::MIN..=2016 => dec!(9615.00),   // ~5 × 1 923 EUR — SSM pré-indexation 2017
        2017 | 2018     => dec!(9995.00),   // ~5 × 1 999 EUR — indexation jan. 2017
        2019            => dec!(10245.00),  // ~5 × 2 049 EUR — indexation 2019
        2020            => dec!(10710.00),  // ~5 × 2 142 EUR
        2021            => dec!(11010.00),  // ~5 × 2 202 EUR
        2022            => dec!(11985.00),  // ~5 × 2 397 EUR — forte hausse post-COVID
        2023            => dec!(12855.00),  // ~5 × 2 571 EUR
        2024            => dec!(13185.00),  // ~5 × 2 637 EUR
        2025 if d.month() < 5 => dec!(13188.95), // 5 × 2 637,79 EUR
        2025            => dec!(13518.68),  // 5 × 2 703,74 EUR (indice 968,04)
        2026 if d.month() < 6 => dec!(13518.68),
        _               => dec!(13856.63),  // 5 × 2 771,33 EUR (indice 992,24, juin 2026)
    }
}

/// Salaire social minimum non qualifié (18 ans), cinquième du plafond.
fn lu_ssm(ctx: &ContextPaie) -> Decimal {
    (lu_plafond_mensuel(ctx) / dec!(5)).round_dp(2)
}

pub fn lu_ap(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let plafond = lu_plafond_mensuel(ctx);
    let base = brut.min(plafond);
    let ts = ctx.taux_sal("LU_AP");
    let tp = ctx.taux_pat("LU_AP");
    LigneCotisation {
        code:        "LU_AP".into(),
        libelle:     ctx.libelle("LU_AP", "AP — Assurance pension"),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance pension".into(),
        explication: ctx.expl("LU_AP",
            "L'assurance pension obligatoire est gérée par la CNAP (Caisse nationale d'assurance pension). \
            Fondée par le Code de la Sécurité Sociale (CSS LU, Livre II), entré en vigueur le 1er janvier 1988. \
            Le régime est par répartition : les cotisations actuelles financent les pensions en cours. \
            Taux partagé à parts égales : 8 % salarié et 8 % employeur jusqu'en 2025, 8,5 % chacun \
            depuis le 1er janvier 2026 (réforme des pensions, loi du 18/12/2025). \
            L'État contribue également un tiers supplémentaire directement depuis le budget national. \
            Assiette : salaire brut plafonné à 5 × SSM (≈ {plafond} €/mois en {annee}). \
            La pension complète est acquise après 40 années de cotisation. \
            Âge légal de retraite : 65 ans (pension normale) ou 57 ans (pension anticipée, sous conditions).")
            .replace("{plafond}", &format!("{:.2}", plafond))
            .replace("{annee}", &ctx.date_paie.year().to_string()),
        loi_ref: Some(ctx.loi_ref("CSS LU Livre II — Loi du 27/07/1987 ; RGD du 29/09/2017")),
    }
}

pub fn lu_am(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let plafond = lu_plafond_mensuel(ctx);
    let base = brut.min(plafond);
    let ts = ctx.taux_sal("LU_AM");
    let tp = ctx.taux_pat("LU_AM");
    LigneCotisation {
        code:        "LU_AM".into(),
        libelle:     ctx.libelle("LU_AM", "AM — Assurance maladie-maternité (CNS)"),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance maladie".into(),
        explication: ctx.expl("LU_AM",
            "L'assurance maladie-maternité est gérée par la CNS (Caisse nationale de santé). \
            Elle couvre les frais de soins de santé (médecins, hôpitaux, médicaments) \
            et les indemnités pécuniaires de maladie (maintien de revenu à 100 % du dernier salaire \
            pendant 52 semaines, puis 80 % jusqu'à 78 semaines). \
            La cotisation de 3,05 % se décompose : soins de santé 2,80 % + indemnités pécuniaires 0,25 %. \
            Assiette : salaire brut plafonné à 5 × SSM (≈ {plafond} €/mois en {annee}). \
            Le Luxembourg pratique le tiers payant généralisé depuis 2010.")
            .replace("{plafond}", &format!("{:.2}", plafond))
            .replace("{annee}", &ctx.date_paie.year().to_string()),
        loi_ref: Some(ctx.loi_ref("CSS LU art. 10 et s. (soins) et art. 24 et s. (indemnités) — Loi du 17/12/2010")),
    }
}

/// Assurance dépendance : 1,40 % sur TOUT le brut (pas de plafond), après un
/// abattement d'un quart du SSM non qualifié (CSS LU art. 375).
pub fn lu_ad(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let abattement = (lu_ssm(ctx) / dec!(4)).round_dp(2);
    let base = (brut - abattement).max(Decimal::ZERO);
    let ts = ctx.taux_sal("LU_AD");
    LigneCotisation {
        code:        "LU_AD".into(),
        libelle:     ctx.libelle("LU_AD", "AD — Assurance dépendance"),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Assurance dépendance".into(),
        explication: ctx.expl("LU_AD",
            "L'assurance dépendance (Pflegeversicherung en allemand) a été instituée par la loi du \
            19 juin 1998, entrée en vigueur le 1er janvier 1999. Elle finance les prestations \
            en nature accordées aux personnes ne pouvant plus accomplir les actes essentiels \
            de la vie quotidienne de manière autonome. \
            Originalité luxembourgeoise : la cotisation est uniquement salariale (1,40 %), \
            sans participation patronale. Pas de plafond : assiette = brut moins un abattement \
            d'un quart du salaire social minimum ({abattement} €/mois en {annee}). \
            Les prestations incluent l'aide à domicile, les séjours en maisons de soins \
            et les congés d'appui proches aidants. Gestion : CNS.")
            .replace("{abattement}", &format!("{:.2}", abattement))
            .replace("{annee}", &ctx.date_paie.year().to_string()),
        loi_ref: Some(ctx.loi_ref("Loi du 19/06/1998 portant introduction de l'assurance dépendance (CSS LU Livre IV)")),
    }
}

pub fn lu_aa(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let plafond = lu_plafond_mensuel(ctx);
    let base = brut.min(plafond);
    let tp = ctx.taux_pat("LU_AA");
    LigneCotisation {
        code:        "LU_AA".into(),
        libelle:     ctx.libelle("LU_AA", "AA — Assurance accidents (AAA)"),
        base,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance accidents".into(),
        explication: ctx.expl("LU_AA",
            "L'assurance accidents obligatoire est gérée par l'AAA (Association d'assurance accident). \
            Elle couvre les accidents du travail et les maladies professionnelles. \
            Entièrement à la charge de l'employeur. \
            Le taux ({taux} %) est indicatif pour le secteur tertiaire — \
            il peut être 3 à 10 fois plus élevé dans les secteurs à risque (BTP, chimie). \
            Assiette : salaire brut plafonné à 5 × SSM (≈ {plafond} €/mois en {annee}). CSS LU Livre III.")
            .replace("{taux}", &format!("{:.2}", tp * dec!(100)))
            .replace("{plafond}", &format!("{:.2}", plafond))
            .replace("{annee}", &ctx.date_paie.year().to_string()),
        loi_ref: Some(ctx.loi_ref("CSS LU Livre III — Loi du 17/12/1925 (réformée) ; RGD du 29/12/1995")),
    }
}

pub fn lu_me(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let plafond = lu_plafond_mensuel(ctx);
    let base = brut.min(plafond);
    let tp = ctx.taux_pat("LU_ME");
    LigneCotisation {
        code:        "LU_ME".into(),
        libelle:     ctx.libelle("LU_ME", "ME — Mutualité des employeurs"),
        base,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Mutualité des employeurs".into(),
        explication: ctx.expl("LU_ME",
            "La Mutualité des employeurs (ME) est un mécanisme de solidarité entre employeurs, \
            géré par le CCSS. Les employeurs maintiennent le salaire complet du salarié malade \
            du 1er au 77e jour (11 semaines), la CNS prenant le relais à partir du 78e jour. \
            La ME rembourse ensuite aux employeurs les salaires avancés, mutualisés entre toutes les entreprises. \
            Taux ({taux} %) indicatif — taux moyen national CCSS pour le secteur tertiaire. \
            Assiette : salaire brut plafonné à 5 × SSM (≈ {plafond} €/mois en {annee}).")
            .replace("{taux}", &format!("{:.2}", tp * dec!(100)))
            .replace("{plafond}", &format!("{:.2}", plafond))
            .replace("{annee}", &ctx.date_paie.year().to_string()),
        loi_ref: Some(ctx.loi_ref("CSS LU art. 3 et s. (Livre II) — Loi du 07/10/1960 (réformée 01/01/1999)")),
    }
}
