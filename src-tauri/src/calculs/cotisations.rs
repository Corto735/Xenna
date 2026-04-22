use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{LigneCotisation, Statut};

// Abattement forfaitaire frais professionnels pour assiette CSG/CRDS — règle CSS art. L136-2
const ABATTEMENT_CSG: Decimal = dec!(0.9825);

pub fn ss_maladie(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("SS_MALADIE");
    let tp = ctx.taux_pat("SS_MALADIE");
    LigneCotisation {
        code:        "SS_MALADIE".into(),
        libelle:     "Assurance maladie, maternité, invalidité, décès".into(),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: "La cotisation salariale maladie a été supprimée au 1er janvier 2018 \
            (LFSS 2018). En contrepartie, la CSG a été augmentée de 1,7 point. \
            Cette bascule visait à augmenter le salaire net sans accroître le coût employeur. \
            La part patronale finance la branche maladie de l'Assurance Maladie.".into(),
        loi_ref: Some("Loi n°2017-1836 du 30/12/2017 (LFSS 2018), art. 8".into()),
    }
}

pub fn ss_vieillesse_plafonnee(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = brut.min(ctx.pmss);
    let ts = ctx.taux_sal("SS_VIEILLESSE_PLAF");
    let tp = ctx.taux_pat("SS_VIEILLESSE_PLAF");
    LigneCotisation {
        code:        "SS_VIEILLESSE_PLAF".into(),
        libelle:     "Assurance vieillesse (plafonnée)".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: format!(
            "Cette cotisation retraite est limitée au Plafond Mensuel Sécurité Sociale \
            (PMSS = {} € en {}). Au-delà, seule la cotisation déplafonnée s'applique. \
            Le système par répartition français, créé en 1945 par ordonnance du GPRF, \
            garantit une pension calculée sur les 25 meilleures années (salariés privés).",
            ctx.pmss, ctx.date_paie.format("%Y")
        ),
        loi_ref: Some("Ordonnance n°45-2250 du 4/10/1945 — réformé par loi 2023-270 (réforme retraites)".into()),
    }
}

pub fn ss_vieillesse_deplafonnee(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("SS_VIEILLESSE_DEPLAF");
    let tp = ctx.taux_pat("SS_VIEILLESSE_DEPLAF");
    LigneCotisation {
        code:        "SS_VIEILLESSE_DEPLAF".into(),
        libelle:     "Assurance vieillesse (déplafonnée)".into(),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: "S'applique sur la totalité du brut, sans plafond. \
            Cotisation solidaire : les hauts salaires contribuent proportionnellement \
            plus pour financer un système dont les pensions sont plafonnées. \
            Principe d'universalité de la Sécurité Sociale (Préambule de 1946).".into(),
        loi_ref: Some("CSS art. L241-3".into()),
    }
}

pub fn famille(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("FAMILLE");
    LigneCotisation {
        code:        "FAMILLE".into(),
        libelle:     "Allocations familiales".into(),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: "Financement des prestations familiales (allocations, crèches, aide à \
            la garde d'enfants). Taux réduit à 3,45% pour les salaires ≤ 3,5 SMIC (taux plein : 5,25%). \
            Politique nataliste française datant de l'entre-deux-guerres, institutionnalisée en 1945.".into(),
        loi_ref: Some("Décret 2015-390 du 3/04/2015 — CSS art. L241-6".into()),
    }
}

pub fn accident_travail(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("AT_MP");
    LigneCotisation {
        code:        "AT_MP".into(),
        libelle:     "Accidents du travail / Maladies professionnelles".into(),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: "Taux fixé par la CARSAT selon le code risque de l'entreprise \
            (secteur d'activité, sinistralité passée). Entièrement à la charge de \
            l'employeur : principe de responsabilité patronale instauré par la loi du \
            9 avril 1898, première loi sociale reconnaissant la responsabilité de \
            l'employeur sans faute prouvée.".into(),
        loi_ref: Some("Loi du 9/04/1898 — CSS art. L241-5".into()),
    }
}

/// Retourne 3 lignes : CSG déductible, CSG non déductible, CRDS
pub fn csg_contributions(brut: Decimal, ctx: &ContextPaie) -> Vec<LigneCotisation> {
    let assiette = (brut * ABATTEMENT_CSG).round_dp(2);

    let ts_ded     = ctx.taux_sal("CSG_DEDUCTIBLE");
    let ts_non_ded = ctx.taux_sal("CSG_NON_DEDUCTIBLE");
    let ts_crds    = ctx.taux_sal("CRDS");

    vec![
        LigneCotisation {
            code:        "CSG_DEDUCTIBLE".into(),
            libelle:     "CSG déductible".into(),
            base:        assiette,
            taux_sal:    ts_ded,
            montant_sal: (assiette * ts_ded).round_dp(2),
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "CSG/CRDS".into(),
            explication: "La CSG (Contribution Sociale Généralisée) a été créée en 1991 \
                par Michel Rocard pour diversifier le financement de la Sécurité Sociale \
                au-delà du travail salarié (revenus du capital inclus). La part déductible \
                est soustraite du revenu imposable à l'IR. \
                L'assiette est 98,25% du brut (abattement de 1,75% pour frais professionnels).".into(),
            loi_ref: Some("Loi n°90-1168 du 29/12/1990 — créée par Michel Rocard".into()),
        },
        LigneCotisation {
            code:        "CSG_NON_DEDUCTIBLE".into(),
            libelle:     "CSG non déductible".into(),
            base:        assiette,
            taux_sal:    ts_non_ded,
            montant_sal: (assiette * ts_non_ded).round_dp(2),
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "CSG/CRDS".into(),
            explication: "Fraction de CSG non déductible du revenu imposable : constitue \
                un impôt sec sur le salaire. Augmentée de 1,7 point en 2018 (LFSS 2018) \
                en contrepartie de la suppression des cotisations salariales maladie et chômage.".into(),
            loi_ref: Some("LFSS 2018 — Loi n°2017-1836".into()),
        },
        LigneCotisation {
            code:        "CRDS".into(),
            libelle:     "CRDS".into(),
            base:        assiette,
            taux_sal:    ts_crds,
            montant_sal: (assiette * ts_crds).round_dp(2),
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "CSG/CRDS".into(),
            explication: "La CRDS (Contribution au Remboursement de la Dette Sociale, 0,5%) \
                a été créée en 1996 par Alain Juppé pour rembourser la dette de la Sécurité \
                Sociale via la CADES. Prévue pour durer 13 ans, elle existe toujours. \
                Non déductible de l'IR.".into(),
            loi_ref: Some("Ordonnance n°96-50 du 24/01/1996 (plan Juppé)".into()),
        },
    ]
}

pub fn chomage(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = brut.min(ctx.pmss * dec!(4));
    let ts = ctx.taux_sal("CHOMAGE");
    let tp = ctx.taux_pat("CHOMAGE");
    LigneCotisation {
        code:        "CHOMAGE".into(),
        libelle:     "Assurance chômage".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Chômage".into(),
        explication: "Depuis 2018, la cotisation salariale chômage a été supprimée \
            et compensée par la hausse de CSG. Seule la part patronale subsiste, \
            plafonnée à 4 PMSS. L'assurance chômage (UNEDIC) est gérée paritairement \
            depuis 1958.".into(),
        loi_ref: Some("Convention UNEDIC — suppression cotisation sal. : LFSS 2018".into()),
    }
}

/// Réduction générale des cotisations patronales (loi Fillon, CSS art. L241-13).
/// Calcul mensuel simplifié — équivalent à l'annualisé pour un salaire constant sur 12 mois.
/// Retourne None si le salaire dépasse le seuil (1,6 SMIC par défaut).
pub fn reduction_fillon(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let coeff_max = ctx.fillon_coeff_max?;
    let seuil     = ctx.fillon_seuil_smic.unwrap_or(dec!(1.6));

    if brut <= Decimal::ZERO || ctx.smic_mensuel <= Decimal::ZERO {
        return None;
    }

    // ratio = seuil × SMIC / brut  (si < 1 → pas de réduction)
    let ratio = (seuil * ctx.smic_mensuel / brut).round_dp(10);
    if ratio <= Decimal::ONE {
        return None;
    }

    let coeff   = ((coeff_max / dec!(0.6)) * (ratio - Decimal::ONE))
        .min(coeff_max)
        .max(Decimal::ZERO)
        .round_dp(4);
    let montant = (brut * coeff).round_dp(2);
    let seuil_eur = (seuil * ctx.smic_mensuel).round_dp(2);

    Some(LigneCotisation {
        code:        "REDUCTION_FILLON".into(),
        libelle:     "Réduction générale des cotisations patronales".into(),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    -coeff,
        montant_pat: -montant,
        categorie:   "Allègement".into(),
        explication: format!(
            "Coefficient = ({coeff_max} / 0,6) × ({seuil} × {smic} / {brut} − 1) = {coeff}. \
            SMIC mensuel : {smic} €. \
            La réduction s'annule à {seuil} SMIC ({seuil_eur} €/mois). \
            Instaurée par la loi Fillon du 17/01/2003, elle remplace la ristourne dégressive Juppé (1995). \
            Objectif : réduire le coût patronal des bas salaires pour favoriser l'emploi peu qualifié. \
            ⚠ Calcul mensuel simplifié — l'annualisation avec régularisation sera intégrée ultérieurement.",
            smic = ctx.smic_mensuel
        ),
        loi_ref: Some("Loi n°2003-47 du 17/01/2003 (Fillon) — CSS art. L241-13".into()),
    })
}

pub fn retraite_complementaire(brut: Decimal, statut: &Statut, ctx: &ContextPaie) -> Vec<LigneCotisation> {
    let t1_base = brut.min(ctx.pmss);
    let t2_base = if brut > ctx.pmss {
        (brut - ctx.pmss).min(ctx.pmss * dec!(7))
    } else {
        Decimal::ZERO
    };

    let mut lignes = vec![
        LigneCotisation {
            code:        "AGIRC_ARRCO_T1".into(),
            libelle:     "AGIRC-ARRCO Tranche 1".into(),
            base:        t1_base,
            taux_sal:    ctx.taux_sal("AGIRC_ARRCO_T1"),
            montant_sal: (t1_base * ctx.taux_sal("AGIRC_ARRCO_T1")).round_dp(2),
            taux_pat:    ctx.taux_pat("AGIRC_ARRCO_T1"),
            montant_pat: (t1_base * ctx.taux_pat("AGIRC_ARRCO_T1")).round_dp(2),
            categorie:   "Retraite complémentaire".into(),
            explication: format!(
                "AGIRC-ARRCO : fusion en 2019 des régimes cadres (AGIRC, 1947) et non-cadres \
                (ARRCO, 1961). Système par points. \
                Tranche 1 = salaire jusqu'au PMSS ({} €).",
                ctx.pmss
            ),
            loi_ref: Some("Accord national interprofessionnel du 17/11/2017".into()),
        },
    ];

    if t2_base > Decimal::ZERO {
        lignes.push(LigneCotisation {
            code:        "AGIRC_ARRCO_T2".into(),
            libelle:     "AGIRC-ARRCO Tranche 2".into(),
            base:        t2_base,
            taux_sal:    ctx.taux_sal("AGIRC_ARRCO_T2"),
            montant_sal: (t2_base * ctx.taux_sal("AGIRC_ARRCO_T2")).round_dp(2),
            taux_pat:    ctx.taux_pat("AGIRC_ARRCO_T2"),
            montant_pat: (t2_base * ctx.taux_pat("AGIRC_ARRCO_T2")).round_dp(2),
            categorie:   "Retraite complémentaire".into(),
            explication: "Tranche 2 : fraction du salaire entre 1 et 8 PMSS. \
                Taux plus élevé car vise les salaires intermédiaires à élevés. \
                Géré paritairement (syndicats + patronat).".into(),
            loi_ref: Some("Accord national interprofessionnel du 17/11/2017".into()),
        });
    }

    // Contributions d'équilibre (CEG)
    let ceg_t1_ts = ctx.taux_sal("AGIRC_ARRCO_CEG_T1");
    let ceg_t1_tp = ctx.taux_pat("AGIRC_ARRCO_CEG_T1");
    if ceg_t1_ts + ceg_t1_tp > Decimal::ZERO {
        lignes.push(LigneCotisation {
            code:        "AGIRC_ARRCO_CEG_T1".into(),
            libelle:     "Contribution d'Équilibre Général T1".into(),
            base:        t1_base,
            taux_sal:    ceg_t1_ts,
            montant_sal: (t1_base * ceg_t1_ts).round_dp(2),
            taux_pat:    ceg_t1_tp,
            montant_pat: (t1_base * ceg_t1_tp).round_dp(2),
            categorie:   "Retraite complémentaire".into(),
            explication: "Contribution non génératrice de points, destinée à l'équilibre \
                financier du régime AGIRC-ARRCO. Créée lors de la fusion 2019.".into(),
            loi_ref: Some("ANI 17/11/2017".into()),
        });
    }

    // Prévoyance cadre minimale (art. 7 CCN 1947)
    if matches!(statut, Statut::Cadre) {
        let tp = ctx.taux_pat("PREVOYANCE_CADRE_MIN");
        lignes.push(LigneCotisation {
            code:        "PREVOYANCE_CADRE_MIN".into(),
            libelle:     "Prévoyance cadre minimale (art. 7 CCN 1947)".into(),
            base:        t1_base,
            taux_sal:    Decimal::ZERO,
            montant_sal: Decimal::ZERO,
            taux_pat:    tp,
            montant_pat: (t1_base * tp).round_dp(2),
            categorie:   "Prévoyance".into(),
            explication: "La Convention Collective Nationale des Cadres (14/03/1947) \
                impose aux employeurs une cotisation minimale de 1,5% sur la tranche A \
                pour financer la prévoyance décès des cadres. Obligation employeur unique \
                en Europe, résultat de la négociation d'après-guerre.".into(),
            loi_ref: Some("Convention du 14/03/1947 — Article 7".into()),
        });
    }

    lignes
}
