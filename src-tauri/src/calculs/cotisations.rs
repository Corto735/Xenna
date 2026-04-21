use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::models::{LigneCotisation, Statut};

// Plafond Mensuel Sécurité Sociale 2025
const PMSS: Decimal = dec!(3925.00);

pub fn ss_maladie(brut: Decimal) -> LigneCotisation {
    LigneCotisation {
        code:        "SS_MALADIE".into(),
        libelle:     "Assurance maladie, maternité, invalidité, décès".into(),
        base:        brut,
        taux_sal:    dec!(0.00),
        montant_sal: dec!(0.00),
        taux_pat:    dec!(0.1313),
        montant_pat: (brut * dec!(0.1313)).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: "La cotisation salariale maladie a été supprimée au 1er janvier 2018 \
            (LFSS 2018). En contrepartie, la CSG a été augmentée de 1,7 point. \
            Cette bascule visait à augmenter le salaire net sans accroître le coût employeur. \
            La part patronale (13,13%) finance la branche maladie de l'Assurance Maladie.".into(),
        loi_ref: Some("Loi n°2017-1836 du 30/12/2017 (LFSS 2018), art. 8".into()),
    }
}

pub fn ss_vieillesse_plafonnee(brut: Decimal) -> LigneCotisation {
    let base = brut.min(PMSS);
    LigneCotisation {
        code:        "SS_VIEILLESSE_PLAF".into(),
        libelle:     "Assurance vieillesse (plafonnée)".into(),
        base,
        taux_sal:    dec!(0.069),
        montant_sal: (base * dec!(0.069)).round_dp(2),
        taux_pat:    dec!(0.0855),
        montant_pat: (base * dec!(0.0855)).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: format!(
            "Cette cotisation retraite est limitée au Plafond Mensuel Sécurité Sociale \
            (PMSS = {PMSS} € en 2025). Au-delà, seule la cotisation déplafonnée s'applique. \
            Le système par répartition français, créé en 1945 par ordonnance du GPRF, \
            garantit une pension calculée sur les 25 meilleures années (salariés privés)."
        ),
        loi_ref: Some("Ordonnance n°45-2250 du 4/10/1945 — réformé par loi 2023-270 (réforme retraites)".into()),
    }
}

pub fn ss_vieillesse_deplafonnee(brut: Decimal) -> LigneCotisation {
    LigneCotisation {
        code:        "SS_VIEILLESSE_DEPLAF".into(),
        libelle:     "Assurance vieillesse (déplafonnée)".into(),
        base:        brut,
        taux_sal:    dec!(0.004),
        montant_sal: (brut * dec!(0.004)).round_dp(2),
        taux_pat:    dec!(0.0190),
        montant_pat: (brut * dec!(0.0190)).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: "S'applique sur la totalité du brut, sans plafond. \
            Cotisation solidaire : les hauts salaires contribuent proportionnellement \
            plus pour financer un système dont les pensions sont plafonnées. \
            Principe d'universalité de la Sécurité Sociale (Préambule de 1946).".into(),
        loi_ref: Some("CSS art. L241-3".into()),
    }
}

pub fn famille(brut: Decimal) -> LigneCotisation {
    LigneCotisation {
        code:        "FAMILLE".into(),
        libelle:     "Allocations familiales".into(),
        base:        brut,
        taux_sal:    dec!(0.00),
        montant_sal: dec!(0.00),
        taux_pat:    dec!(0.0345),
        montant_pat: (brut * dec!(0.0345)).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: "Financement des prestations familiales (allocations, crèches, aide à \
            la garde d'enfants). Depuis 2015, taux réduit à 3,45% pour les salaires \
            ≤ 3,5 SMIC (taux plein : 5,25%). Politique nataliste française datant de \
            l'entre-deux-guerres, institutionnalisée en 1945.".into(),
        loi_ref: Some("Décret 2015-390 du 3/04/2015 — CSS art. L241-6".into()),
    }
}

pub fn accident_travail(brut: Decimal) -> LigneCotisation {
    // Taux variable selon le secteur — taux moyen approximatif pour la démo
    LigneCotisation {
        code:        "AT_MP".into(),
        libelle:     "Accidents du travail / Maladies professionnelles".into(),
        base:        brut,
        taux_sal:    dec!(0.00),
        montant_sal: dec!(0.00),
        taux_pat:    dec!(0.0235),
        montant_pat: (brut * dec!(0.0235)).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: "Taux fixé par la CARSAT selon le code risque de l'entreprise \
            (secteur d'activité, sinistralité passée). Entièrement à la charge de \
            l'employeur : principe de responsabilité patronale instauré par la loi du \
            9 avril 1898, première loi sociale reconnaissant la responsabilité de \
            l'employeur sans faute prouvée.".into(),
        loi_ref: Some("Loi du 9/04/1898 — CSS art. L241-5".into()),
    }
}

pub fn csg_crds(brut: Decimal) -> (LigneCotisation, LigneCotisation) {
    // Assiette CSG/CRDS = 98,25% du brut (abattement 1,75% frais pro)
    let assiette = (brut * dec!(0.9825)).round_dp(2);

    let csg_deductible = LigneCotisation {
        code:        "CSG_DEDUCTIBLE".into(),
        libelle:     "CSG déductible".into(),
        base:        assiette,
        taux_sal:    dec!(0.068),
        montant_sal: (assiette * dec!(0.068)).round_dp(2),
        taux_pat:    dec!(0.00),
        montant_pat: dec!(0.00),
        categorie:   "CSG/CRDS".into(),
        explication: "La CSG (Contribution Sociale Généralisée) a été créée en 1991 \
            par Michel Rocard pour diversifier le financement de la Sécurité Sociale \
            au-delà du travail salarié (revenus du capital inclus). La part déductible \
            (6,8%) est soustraite du revenu imposable à l'IR.".into(),
        loi_ref: Some("Loi n°90-1168 du 29/12/1990 — créée par Michel Rocard".into()),
    };

    let csg_non_deductible = LigneCotisation {
        code:        "CSG_NON_DED".into(),
        libelle:     "CSG non déductible + CRDS".into(),
        base:        assiette,
        taux_sal:    dec!(0.029),
        montant_sal: (assiette * dec!(0.029)).round_dp(2),
        taux_pat:    dec!(0.00),
        montant_pat: dec!(0.00),
        categorie:   "CSG/CRDS".into(),
        explication: "CSG non déductible (2,4%) + CRDS (0,5%). La CRDS a été créée en \
            1996 par Alain Juppé pour rembourser la dette de la Sécurité Sociale via \
            la CADES. Ces 2,9% ne sont pas déductibles de l'impôt sur le revenu : \
            ils constituent un impôt sec sur le salaire.".into(),
        loi_ref: Some("CRDS : Ordonnance n°96-50 du 24/01/1996 (plan Juppé)".into()),
    };

    (csg_deductible, csg_non_deductible)
}

pub fn retraite_complementaire(brut: Decimal, statut: &Statut) -> Vec<LigneCotisation> {
    match statut {
        Statut::NonCadre => {
            let tranche1 = brut.min(PMSS);
            let tranche2 = if brut > PMSS { (brut - PMSS).min(PMSS * dec!(7.0)) } else { dec!(0) };
            vec![
                LigneCotisation {
                    code:        "AGIRC_ARRCO_T1".into(),
                    libelle:     "AGIRC-ARRCO Tranche 1".into(),
                    base:        tranche1,
                    taux_sal:    dec!(0.0315),
                    montant_sal: (tranche1 * dec!(0.0315)).round_dp(2),
                    taux_pat:    dec!(0.0472),
                    montant_pat: (tranche1 * dec!(0.0472)).round_dp(2),
                    categorie:   "Retraite complémentaire".into(),
                    explication: "AGIRC-ARRCO : fusion en 2019 des régimes cadres (AGIRC, 1947) \
                        et non-cadres (ARRCO, 1961). Système par points : chaque cotisation \
                        achète des points convertis en pension à la retraite. \
                        Tranche 1 = salaire jusqu'au PMSS.".into(),
                    loi_ref: Some("Accord national interprofessionnel du 17/11/2017".into()),
                },
                LigneCotisation {
                    code:        "AGIRC_ARRCO_T2".into(),
                    libelle:     "AGIRC-ARRCO Tranche 2".into(),
                    base:        tranche2,
                    taux_sal:    dec!(0.0864),
                    montant_sal: (tranche2 * dec!(0.0864)).round_dp(2),
                    taux_pat:    dec!(0.1296),
                    montant_pat: (tranche2 * dec!(0.1296)).round_dp(2),
                    categorie:   "Retraite complémentaire".into(),
                    explication: "Tranche 2 : fraction du salaire entre 1 et 8 PMSS. \
                        Taux plus élevé car vise les salaires intermédiaires à élevés. \
                        Le régime AGIRC-ARRCO est géré paritairement (syndicats + patronat).".into(),
                    loi_ref: Some("Accord national interprofessionnel du 17/11/2017".into()),
                },
            ]
        }
        Statut::Cadre => {
            vec![
                LigneCotisation {
                    code:        "AGIRC_ARRCO_T1_CADRE".into(),
                    libelle:     "AGIRC-ARRCO T1 (cadre)".into(),
                    base:        brut.min(PMSS),
                    taux_sal:    dec!(0.0315),
                    montant_sal: (brut.min(PMSS) * dec!(0.0315)).round_dp(2),
                    taux_pat:    dec!(0.0472),
                    montant_pat: (brut.min(PMSS) * dec!(0.0472)).round_dp(2),
                    categorie:   "Retraite complémentaire".into(),
                    explication: "Même taux T1 que les non-cadres depuis la fusion AGIRC-ARRCO \
                        de 2019. Avant 2019, les cadres avaient un régime AGIRC séparé \
                        avec des taux distincts.".into(),
                    loi_ref: Some("Accord AGIRC-ARRCO 17/11/2017 — fusion 01/01/2019".into()),
                },
            ]
        }
    }
}

pub fn chomage(brut: Decimal) -> LigneCotisation {
    let base = brut.min(PMSS * dec!(4.0));
    LigneCotisation {
        code:        "CHOMAGE".into(),
        libelle:     "Assurance chômage".into(),
        base,
        taux_sal:    dec!(0.00),
        montant_sal: dec!(0.00),
        taux_pat:    dec!(0.0405),
        montant_pat: (base * dec!(0.0405)).round_dp(2),
        categorie:   "Chômage".into(),
        explication: "Depuis 2018, la cotisation salariale chômage (2,4%) a été supprimée \
            et compensée par la hausse de CSG. Seule la part patronale (4,05%) subsiste, \
            plafonnée à 4 PMSS. L'assurance chômage (UNEDIC) est gérée paritairement \
            depuis 1958, financée par les cotisations des employeurs uniquement depuis 2018.".into(),
        loi_ref: Some("Convention UNEDIC — suppression cotisation sal. : LFSS 2018".into()),
    }
}

pub fn prevoyance_cadre(brut: Decimal, statut: &Statut) -> Option<LigneCotisation> {
    match statut {
        Statut::Cadre => {
            let base = brut.min(PMSS);
            Some(LigneCotisation {
                code:        "PREVOYANCE_CADRE".into(),
                libelle:     "Prévoyance cadre (TA)".into(),
                base,
                taux_sal:    dec!(0.0000),
                montant_sal: dec!(0.00),
                taux_pat:    dec!(0.015),
                montant_pat: (base * dec!(0.015)).round_dp(2),
                categorie:   "Prévoyance".into(),
                explication: "La Convention Collective Nationale des Cadres (14/03/1947) \
                    impose aux employeurs une cotisation minimale de 1,5% sur la tranche A \
                    pour financer la prévoyance décès des cadres. Obligation employeur unique \
                    en Europe, résultat de la négociation interprofessionnelle d'après-guerre.".into(),
                loi_ref: Some("Convention du 14/03/1947 — Article 7".into()),
            })
        }
        Statut::NonCadre => None,
    }
}
