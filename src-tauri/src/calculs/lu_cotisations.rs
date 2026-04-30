use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Plafond cotisable luxembourgeois au 01/01/2026 ───────────────────────────
//
// SSM (salaire social minimum non-qualifié) estimé au 01/01/2026 : €2 703,76/mois
// (valeur après indexation de janvier 2025 ; à vérifier contre la publication CCSS)
// Plafond = 5 × SSM = €13 518,80/mois
const LU_PLAFOND_MENSUEL: Decimal = dec!(13518.80);

pub fn lu_ap(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = brut.min(LU_PLAFOND_MENSUEL);
    let ts = ctx.taux_sal("LU_AP");
    let tp = ctx.taux_pat("LU_AP");
    LigneCotisation {
        code:        "LU_AP".into(),
        libelle:     "AP — Assurance pension".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance pension".into(),
        explication: format!(
            "L'assurance pension obligatoire est gérée par la CNAP (Caisse nationale d'assurance pension). \
            Fondée par le Code de la Sécurité Sociale (CSS LU, Livre II), entré en vigueur le 1er janvier 1988. \
            Le régime est par répartition : les cotisations actuelles financent les pensions en cours. \
            Taux : 16 % total, partagé à parts égales (8 % salarié, 8 % employeur). \
            L'État contribue également un tiers supplémentaire directement depuis le budget national. \
            Assiette : salaire brut plafonné à 5 × SSM (≈ {} €/mois). \
            La pension complète est acquise après 40 années de cotisation. \
            Âge légal de retraite : 65 ans (pension normale) ou 57 ans (pension anticipée, sous conditions).",
            LU_PLAFOND_MENSUEL
        ),
        loi_ref: Some("CSS LU Livre II — Loi du 27/07/1987 ; RGD du 29/09/2017".into()),
    }
}

pub fn lu_am(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = brut.min(LU_PLAFOND_MENSUEL);
    let ts = ctx.taux_sal("LU_AM");
    let tp = ctx.taux_pat("LU_AM");
    LigneCotisation {
        code:        "LU_AM".into(),
        libelle:     "AM — Assurance maladie-maternité (CNS)".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance maladie".into(),
        explication: format!(
            "L'assurance maladie-maternité est gérée par la CNS (Caisse nationale de santé). \
            Elle couvre les frais de soins de santé (médecins, hôpitaux, médicaments) \
            et les indemnités pécuniaires de maladie (maintien de revenu à 100 % du dernier salaire \
            pendant 52 semaines, puis 80 % jusqu'à 78 semaines). \
            La cotisation de 3,05 % se décompose en deux volets, à parts égales salarié/employeur : \
              • Soins de santé : 2,80 % (couverture médicale remboursée à 100 % en déplafonnement) \
              • Indemnités pécuniaires : 0,25 % (maintien de salaire en cas de maladie) \
            Assiette : salaire brut plafonné à 5 × SSM (≈ {} €/mois). \
            Le Luxembourg pratique le tiers payant généralisé depuis 2010.",
            LU_PLAFOND_MENSUEL
        ),
        loi_ref: Some("CSS LU art. 10 et s. (soins) et art. 24 et s. (indemnités) — Loi du 17/12/2010".into()),
    }
}

pub fn lu_ad(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = brut.min(LU_PLAFOND_MENSUEL);
    let ts = ctx.taux_sal("LU_AD");
    LigneCotisation {
        code:        "LU_AD".into(),
        libelle:     "AD — Assurance dépendance".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Assurance dépendance".into(),
        explication: format!(
            "L'assurance dépendance (Pflegeversicherung en allemand) a été instituée par la loi du \
            19 juin 1998, entrée en vigueur le 1er janvier 1999. Elle finance les prestations \
            en nature accordées aux personnes ne pouvant plus accomplir les actes essentiels \
            de la vie quotidienne de manière autonome. \
            Originalité luxembourgeoise : la cotisation est uniquement salariale (1,40 %), \
            sans participation patronale. Elle est prélevée sur la totalité du revenu \
            plafonné à 5 × SSM (≈ {} €/mois). \
            Les prestations incluent l'aide à domicile, les séjours en maisons de soins \
            et les congés d'appui proches aidants. \
            Gestion : CNS (Caisse nationale de santé).",
            LU_PLAFOND_MENSUEL
        ),
        loi_ref: Some("Loi du 19/06/1998 portant introduction de l'assurance dépendance (CSS LU Livre IV)".into()),
    }
}

pub fn lu_aa(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = brut.min(LU_PLAFOND_MENSUEL);
    let tp = ctx.taux_pat("LU_AA");
    LigneCotisation {
        code:        "LU_AA".into(),
        libelle:     "AA — Assurance accidents (AAA)".into(),
        base,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance accidents".into(),
        explication: format!(
            "L'assurance accidents obligatoire est gérée par l'AAA \
            (Association d'assurance accident). Elle couvre les accidents du travail \
            et les maladies professionnelles. Entièrement à la charge de l'employeur. \
            Le taux (ici {:.2} %) est déterminé par l'AAA selon la classification des risques \
            de l'entreprise (secteur d'activité, sinistralité passée). \
            Ce taux est indicatif pour le secteur tertiaire (bureau, commerce, services) — \
            il peut être 3 à 10 fois plus élevé dans les secteurs à risque \
            (construction, chimie, industrie lourde). \
            Assiette : salaire brut plafonné à 5 × SSM (≈ {} €/mois). \
            CSS LU Livre III.",
            tp * dec!(100),
            LU_PLAFOND_MENSUEL
        ),
        loi_ref: Some("CSS LU Livre III — Loi du 17/12/1925 (réformée) ; RGD du 29/12/1995".into()),
    }
}

pub fn lu_me(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = brut.min(LU_PLAFOND_MENSUEL);
    let tp = ctx.taux_pat("LU_ME");
    LigneCotisation {
        code:        "LU_ME".into(),
        libelle:     "ME — Mutualité des employeurs".into(),
        base,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Mutualité des employeurs".into(),
        explication: format!(
            "La Mutualité des employeurs (ME) est un mécanisme de solidarité entre employeurs, \
            géré par le CCSS (Centre commun de la sécurité sociale). \
            Les employeurs maintiennent le salaire complet du salarié malade \
            du 1er au 77e jour (11 semaines), la CNS prenant le relais à partir du 78e jour. \
            La ME rembourse ensuite aux employeurs les salaires avancés, \
            mutualisés entre toutes les entreprises affiliées. \
            Taux ({:.2} %) indicatif — taux moyen national CCSS pour le secteur tertiaire. \
            Le taux réel est personnalisé chaque année par le CCSS selon : \
              • l'historique de sinistralité maladie de l'entreprise \
              • la taille de l'entreprise (< 15 salariés → taux fixe, ≥ 15 → taux personnalisé) \
              • le secteur d'activité \
            Assiette : salaire brut plafonné à 5 × SSM (≈ {} €/mois).",
            tp * dec!(100),
            LU_PLAFOND_MENSUEL
        ),
        loi_ref: Some("CSS LU art. 3 et s. (Livre II) — Loi du 07/10/1960 (réformée 01/01/1999)".into()),
    }
}
