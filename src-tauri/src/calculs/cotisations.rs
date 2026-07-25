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
        libelle:     ctx.libelle("SS_MALADIE", "Assurance maladie, maternité, invalidité, décès"),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: ctx.expl("SS_MALADIE", "La cotisation salariale maladie a été supprimée au 1er janvier 2018 \
            (LFSS 2018). En contrepartie, la CSG a été augmentée de 1,7 point. \
            Cette bascule visait à augmenter le salaire net sans accroître le coût employeur. \
            La part patronale finance la branche maladie de l'Assurance Maladie."),
        loi_ref: Some(ctx.loi_ref("Loi n°2017-1836 du 30/12/2017 (LFSS 2018), art. 8")),
    }
}

/// PMSS proratisé selon la quotité de travail (ETP), + note explicative (vide à temps plein).
/// PMSS_proraté = PMSS × ETP/100 (CSS art. L242-1 ; durée contractuelle / durée légale).
/// Même ratio que le SMIC proratisé du Fillon (§670 BOSS).
fn pmss_proratise(ctx: &ContextPaie, etp_pct: f64) -> (Decimal, String) {
    let ratio: Decimal = format!("{:.6}", (etp_pct / 100.0).clamp(0.0, 2.0))
        .parse()
        .unwrap_or(dec!(1));
    let pmss = (ctx.pmss * ratio).round_dp(2);
    let note = if (etp_pct - 100.0).abs() > 0.1 {
        ctx.expl("PMSS_ETP_NOTE",
            "\n⚠ Temps partiel {etp} % — PMSS proratisé : {pmss} € (plafond réduit, CSS art. L242-1)")
            .replace("{etp}", &format!("{:.0}", etp_pct))
            .replace("{pmss}", &pmss.to_string())
    } else {
        String::new()
    };
    (pmss, note)
}

pub fn ss_vieillesse_plafonnee(brut: Decimal, etp_pct: f64, ctx: &ContextPaie) -> LigneCotisation {
    let (pmss, note) = pmss_proratise(ctx, etp_pct);
    let base = brut.min(pmss);
    let ts = ctx.taux_sal("SS_VIEILLESSE_PLAF");
    let tp = ctx.taux_pat("SS_VIEILLESSE_PLAF");
    LigneCotisation {
        code:        "SS_VIEILLESSE_PLAF".into(),
        libelle:     ctx.libelle("SS_VIEILLESSE_PLAF", "Assurance vieillesse (plafonnée)"),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: ctx.expl("SS_VIEILLESSE_PLAF",
            "Cette cotisation retraite est limitée au Plafond Mensuel Sécurité Sociale \
            (PMSS = {pmss} € en {annee}). Au-delà, seule la cotisation déplafonnée s'applique. \
            Le système par répartition français, créé en 1945 par ordonnance du GPRF, \
            garantit une pension calculée sur les 25 meilleures années (salariés privés).{etp_info}")
            .replace("{etp_info}", &note)
            .replace("{pmss}", &pmss.to_string())
            .replace("{annee}", &ctx.date_paie.format("%Y").to_string()),
        loi_ref: Some(ctx.loi_ref("Ordonnance n°45-2250 du 4/10/1945 — réformé par loi 2023-270 (réforme retraites)")),
    }
}

pub fn ss_vieillesse_deplafonnee(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("SS_VIEILLESSE_DEPLAF");
    let tp = ctx.taux_pat("SS_VIEILLESSE_DEPLAF");
    LigneCotisation {
        code:        "SS_VIEILLESSE_DEPLAF".into(),
        libelle:     ctx.libelle("SS_VIEILLESSE_DEPLAF", "Assurance vieillesse (déplafonnée)"),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: ctx.expl("SS_VIEILLESSE_DEPLAF", "S'applique sur la totalité du brut, sans plafond. \
            Cotisation solidaire : les hauts salaires contribuent proportionnellement \
            plus pour financer un système dont les pensions sont plafonnées. \
            Principe d'universalité de la Sécurité Sociale (Préambule de 1946)."),
        loi_ref: Some(ctx.loi_ref("CSS art. L241-3")),
    }
}

pub fn famille(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("FAMILLE");
    LigneCotisation {
        code:        "FAMILLE".into(),
        libelle:     ctx.libelle("FAMILLE", "Allocations familiales"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: ctx.expl("FAMILLE", "Financement des prestations familiales (allocations, crèches, aide à \
            la garde d'enfants). Taux réduit à 3,45% pour les salaires ≤ 3,5 SMIC (taux plein : 5,25%). \
            Politique nataliste française datant de l'entre-deux-guerres, institutionnalisée en 1945."),
        loi_ref: Some(ctx.loi_ref("Décret 2015-390 du 3/04/2015 — CSS art. L241-6")),
    }
}

pub fn accident_travail(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("AT_MP");
    LigneCotisation {
        code:        "AT_MP".into(),
        libelle:     ctx.libelle("AT_MP", "Accidents du travail / Maladies professionnelles"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité Sociale".into(),
        explication: ctx.expl("AT_MP", "Taux fixé par la CARSAT selon le code risque de l'entreprise \
            (secteur d'activité, sinistralité passée). Entièrement à la charge de \
            l'employeur : principe de responsabilité patronale instauré par la loi du \
            9 avril 1898, première loi sociale reconnaissant la responsabilité de \
            l'employeur sans faute prouvée."),
        loi_ref: Some(ctx.loi_ref("Loi du 9/04/1898 — CSS art. L241-5")),
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
            libelle:     ctx.libelle("CSG_DEDUCTIBLE", "CSG déductible"),
            base:        assiette,
            taux_sal:    ts_ded,
            montant_sal: (assiette * ts_ded).round_dp(2),
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "CSG/CRDS".into(),
            explication: ctx.expl("CSG_DEDUCTIBLE", "La CSG (Contribution Sociale Généralisée) a été créée en 1991 \
                par Michel Rocard pour diversifier le financement de la Sécurité Sociale \
                au-delà du travail salarié (revenus du capital inclus). La part déductible \
                est soustraite du revenu imposable à l'IR. \
                L'assiette est 98,25% du brut (abattement de 1,75% pour frais professionnels)."),
            loi_ref: Some(ctx.loi_ref("Loi n°90-1168 du 29/12/1990 — créée par Michel Rocard")),
        },
        LigneCotisation {
            code:        "CSG_NON_DEDUCTIBLE".into(),
            libelle:     ctx.libelle("CSG_NON_DEDUCTIBLE", "CSG non déductible"),
            base:        assiette,
            taux_sal:    ts_non_ded,
            montant_sal: (assiette * ts_non_ded).round_dp(2),
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "CSG/CRDS".into(),
            explication: ctx.expl("CSG_NON_DEDUCTIBLE", "Fraction de CSG non déductible du revenu imposable : constitue \
                un impôt sec sur le salaire. Augmentée de 1,7 point en 2018 (LFSS 2018) \
                en contrepartie de la suppression des cotisations salariales maladie et chômage."),
            loi_ref: Some(ctx.loi_ref("LFSS 2018 — Loi n°2017-1836")),
        },
        LigneCotisation {
            code:        "CRDS".into(),
            libelle:     ctx.libelle("CRDS", "CRDS"),
            base:        assiette,
            taux_sal:    ts_crds,
            montant_sal: (assiette * ts_crds).round_dp(2),
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "CSG/CRDS".into(),
            explication: ctx.expl("CRDS", "La CRDS (Contribution au Remboursement de la Dette Sociale, 0,5%) \
                a été créée en 1996 par Alain Juppé pour rembourser la dette de la Sécurité \
                Sociale via la CADES. Prévue pour durer 13 ans, elle existe toujours. \
                Non déductible de l'IR."),
            loi_ref: Some(ctx.loi_ref("Ordonnance n°96-50 du 24/01/1996 (plan Juppé)")),
        },
    ]
}

pub fn chomage(brut: Decimal, etp_pct: f64, ctx: &ContextPaie) -> LigneCotisation {
    let (pmss, note) = pmss_proratise(ctx, etp_pct);
    let base = brut.min(pmss * dec!(4));
    let ts = ctx.taux_sal("CHOMAGE");
    let tp = ctx.taux_pat("CHOMAGE");
    LigneCotisation {
        code:        "CHOMAGE".into(),
        libelle:     ctx.libelle("CHOMAGE", "Assurance chômage"),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Chômage".into(),
        explication: ctx.expl("CHOMAGE", "Depuis 2018, la cotisation salariale chômage a été supprimée \
            et compensée par la hausse de CSG. Seule la part patronale subsiste, \
            plafonnée à 4 PMSS. L'assurance chômage (UNEDIC) est gérée paritairement \
            depuis 1958.{etp_info}")
            .replace("{etp_info}", &note),
        loi_ref: Some(ctx.loi_ref("Convention UNEDIC — suppression cotisation sal. : LFSS 2018")),
    }
}

/// Calcule le coefficient Fillon pour un ratio SMIC/brut donné.
///
/// Accepte des valeurs mensuelles ou cumulées annualisées indifféremment,
/// car le ratio `seuil × smic / brut` est invariant à l'échelle (les mois se simplifient).
///
/// Deux formules selon la période (détectée via `ctx.fillon_puissance`) :
///
///   2019+ (puissance, formule officielle URSSAF) :
///     inner = (1/2) × (seuil × SMIC / brut − 1)
///     C = Tmin + (Tdelta × inner^P)   ∈ [0 ; Tmax]   arrondi à 4 décimales
///
///   2015-2018 (linéaire) :
///     C = (Tmax / 0,6) × (seuil × SMIC / brut − 1)   ∈ [0 ; Tmax]
///
/// Retourne Decimal::ZERO si le brut dépasse le seuil (aucune réduction applicable).
pub fn fillon_coeff(smic: Decimal, brut: Decimal, ctx: &ContextPaie) -> Decimal {
    let tmax  = match ctx.fillon_coeff_max  { Some(v) => v, None => return Decimal::ZERO };
    let seuil = ctx.fillon_seuil_smic.unwrap_or(dec!(1.6));

    if brut <= Decimal::ZERO || smic <= Decimal::ZERO {
        return Decimal::ZERO;
    }

    if let Some(p) = ctx.fillon_puissance {
        // ── Formule 2019+ (puissance) ────────────────────────────────────────
        let tmin   = ctx.fillon_tmin.unwrap_or(Decimal::ZERO);
        let tdelta = tmax - tmin;

        // inner = (1/2) × (seuil × SMIC / brut − 1)
        // Vaut 0 à brut = seuil × SMIC, vaut 1 à brut = SMIC (si seuil=3).
        let inner = (seuil * smic / brut - Decimal::ONE) / dec!(2);

        if inner <= Decimal::ZERO {
            return Decimal::ZERO; // brut ≥ seuil × SMIC → aucune réduction
        }

        // On borne inner à 1 (brut en dessous du SMIC → coefficient plafonné à Tmax)
        let inner_clamped = inner.min(Decimal::ONE);

        // L'élévation à la puissance nécessite f64 (rust_decimal n'a pas de powf).
        // L'erreur de représentation flottante est < 1e-14, absorbée par l'arrondi à 4dp.
        let inner_f64: f64 = inner_clamped.to_string().parse().unwrap_or(0.0);
        let p_f64:     f64 = p.to_string().parse().unwrap_or(1.75);
        let powered:   Decimal = format!("{:.10}", inner_f64.powf(p_f64))
            .parse()
            .unwrap_or(Decimal::ZERO);

        (tmin + tdelta * powered).clamp(tmin, tmax).round_dp(4)
    } else {
        // ── Formule 2015-2018 (linéaire) ─────────────────────────────────────
        let ratio = (seuil * smic / brut).round_dp(10);
        if ratio <= Decimal::ONE {
            return Decimal::ZERO;
        }
        ((tmax / dec!(0.6)) * (ratio - Decimal::ONE))
            .min(tmax)
            .max(Decimal::ZERO)
            .round_dp(4)
    }
}

/// Réduction générale des cotisations patronales (loi Fillon, CSS art. L241-13).
/// Retourne None si le salaire dépasse le seuil ou si les paramètres Fillon
/// ne sont pas en base pour cette date.
pub fn reduction_fillon(brut: Decimal, etp_pct: f64, absence_ratio: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    // §670 BOSS (CSS art. L241-13) : le SMIC est proratisé selon la durée contractuelle.
    // SMIC_proraté = SMIC_mensuel × (ETP / 100)
    // On utilise le SMIC de référence Fillon (gelé au 1er janvier), pas le SMIC
    // courant : une revalorisation en cours d'année ne modifie pas ce calcul.
    let ratio: Decimal = format!("{:.6}", (etp_pct / 100.0).clamp(0.0, 2.0))
        .parse()
        .unwrap_or(dec!(1));
    // Absence (CSS art. D241-7 IV) : la valeur du SMIC est corrigée selon le rapport
    // des revenus d'activité dus / dus si présent tout le mois. `absence_ratio` (borné
    // [0;1], calculé en amont : (base − retenue + maintien)/base) porte cette correction ;
    // maintien intégral → ratio = 1 (SMIC plein), absence non rémunérée → SMIC réduit.
    let smic = (ctx.smic_mensuel_fillon * ratio * absence_ratio).round_dp(2);

    let coeff = fillon_coeff(smic, brut, ctx);
    if coeff == Decimal::ZERO {
        return None;
    }

    let tmax      = ctx.fillon_coeff_max.unwrap_or(coeff);
    let seuil     = ctx.fillon_seuil_smic.unwrap_or(dec!(1.6));
    let tmin      = ctx.fillon_tmin.unwrap_or(Decimal::ZERO);
    let tdelta    = tmax - tmin;
    let montant   = (brut * coeff).round_dp(2);
    let seuil_eur = (seuil * smic).round_dp(2);

    let etp_info = if (etp_pct - 100.0).abs() > 0.1 {
        ctx.expl("REDUCTION_FILLON_ETP", "\n⚠ Temps partiel {etp} % — SMIC proratisé : {smic} € (§670 BOSS)")
            .replace("{etp}", &format!("{:.0}", etp_pct))
            .replace("{smic}", &smic.to_string())
    } else {
        String::new()
    };

    // Correction d'absence (CSS art. D241-7 IV) : affichée seulement quand elle joue.
    let abs_info = if absence_ratio < Decimal::ONE {
        ctx.expl("REDUCTION_FILLON_ABSENCE",
            "\n⚠ Absence : SMIC corrigé au prorata de la rémunération (× {ratio}) → {smic} € (CSS art. D241-7 IV)")
            .replace("{ratio}", &absence_ratio.round_dp(4).to_string())
            .replace("{smic}", &smic.to_string())
    } else {
        String::new()
    };

    // Précision réglementaire : le SMIC retenu pour la réduction générale est celui
    // en vigueur au 1er janvier de l'année (gelé), et non le SMIC courant. Le décret
    // n°2026-509 du 12/06/2026 a formalisé ce gel pour 2026 (neutralisation de la
    // revalorisation du SMIC au 1er juin) ; hors 2026 on ne cite que les articles.
    let annee = ctx.date_paie.format("%Y").to_string();
    let ref_smic = if annee == "2026" {
        "\nRéf. : décret n°2026-509 du 12/06/2026 (JO 14/06/2026) — CSS art. L241-13 et D241-7."
    } else {
        "\nRéf. : CSS art. L241-13 et D241-7."
    };

    // Gabarit traduit (ou français natif), puis substitution des placeholders.
    // L'ordre des replace : les clés les plus longues d'abord pour éviter qu'une
    // clé courte ne morde sur une plus longue (ici aucune ne se chevauche, mais
    // on reste prudent : {seuil_eur} avant {seuil}, {inner_disp} avant {inner}).
    let explication = if ctx.fillon_puissance.is_some() {
        let inner = (seuil * smic / brut - Decimal::ONE) / dec!(2);
        // Arrondi à 4 dp pour l'affichage — évite les longues suites décimales
        let inner_disp = inner.min(Decimal::ONE).max(Decimal::ZERO).round_dp(4);
        let p = ctx.fillon_puissance.unwrap_or(dec!(1.75));
        ctx.expl("REDUCTION_FILLON_PUISSANCE",
            "[ Calcul mensuel — CSS art. L241-13 ]\n\
            \n\
            Formule : C = Tmin + (Tdelta × D^P)\n\
            D = (1/2) × (seuil × SMIC mensuel / Salaire brut − 1)\n\
            \n\
            Paramètres : Tmin={tmin}  Tdelta={tdelta}  Tmax={tmax}  P={p}  Seuil={seuil}×SMIC\n\
            \n\
            D = (1/2) × ({seuil} × {smic} / {brut} − 1)\n\
              = {inner_disp}\n\
            \n\
            C = {tmin} + ({tdelta} × {inner_disp}^{p})\n\
              = {coeff}\n\
            \n\
            ── Réduction mensuelle ─────────────────────────────\n\
            Réduction = Salaire brut × C\n\
                      = {brut} × {coeff}\n\
                      = {montant} €\n\
            ────────────────────────────────────────────────────\n\
            \n\
            S'annule à {seuil} × SMIC = {seuil_eur} €/mois.{etp_info}{abs_info}\n\
            Smic au 01/01/{annee} retenu : {smic} € — valeur gelée toute l'année, la revalorisation du SMIC en cours d'année n'est pas répercutée sur la réduction générale.{ref_smic}\n\
            Loi Fillon du 17/01/2003 : allègement des charges patronales sur les bas salaires.")
            .replace("{etp_info}", &etp_info)
            .replace("{abs_info}", &abs_info)
            .replace("{ref_smic}", ref_smic)
            .replace("{annee}", &annee)
            .replace("{inner_disp}", &inner_disp.to_string())
            .replace("{seuil_eur}", &seuil_eur.to_string())
            .replace("{tmin}", &tmin.to_string())
            .replace("{tdelta}", &tdelta.to_string())
            .replace("{tmax}", &tmax.to_string())
            .replace("{seuil}", &seuil.to_string())
            .replace("{smic}", &smic.to_string())
            .replace("{brut}", &brut.to_string())
            .replace("{coeff}", &coeff.to_string())
            .replace("{montant}", &montant.to_string())
            .replace("{p}", &p.to_string())
    } else {
        ctx.expl("REDUCTION_FILLON_LINEAIRE",
            "[ Calcul mensuel — ancienne formule linéaire 2015-2018 ]\n\
            \n\
            Formule : C = (Tmax / 0,6) × (seuil × SMIC / brut − 1)\n\
              = ({tmax} / 0,6) × ({seuil} × {smic} / {brut} − 1)\n\
              = {coeff}\n\
            \n\
            ── Réduction mensuelle ─────────────────────────────\n\
            Réduction = Salaire brut × C\n\
                      = {brut} × {coeff}\n\
                      = {montant} €\n\
            ────────────────────────────────────────────────────\n\
            \n\
            S'annule à {seuil} × SMIC = {seuil_eur} €/mois.{etp_info}{abs_info}\n\
            Smic au 01/01/{annee} retenu : {smic} € — valeur gelée toute l'année, la revalorisation du SMIC en cours d'année n'est pas répercutée sur la réduction générale.{ref_smic}")
            .replace("{etp_info}", &etp_info)
            .replace("{abs_info}", &abs_info)
            .replace("{ref_smic}", ref_smic)
            .replace("{annee}", &annee)
            .replace("{seuil_eur}", &seuil_eur.to_string())
            .replace("{tmax}", &tmax.to_string())
            .replace("{seuil}", &seuil.to_string())
            .replace("{smic}", &smic.to_string())
            .replace("{brut}", &brut.to_string())
            .replace("{coeff}", &coeff.to_string())
            .replace("{montant}", &montant.to_string())
    };

    Some(LigneCotisation {
        code:        "REDUCTION_FILLON".into(),
        libelle:     ctx.libelle("REDUCTION_FILLON", "Réduction générale des cotisations patronales"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    -coeff,
        montant_pat: -montant,
        categorie:   "Allègement".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("Loi n°2003-47 du 17/01/2003 (Fillon) — CSS art. L241-13 et D241-7")),
    })
}

/// Cotisation maladie complémentaire du régime local d'Alsace-Moselle (droit local).
/// Uniquement salariale, assiette = salaire brut total.
/// Retourne None si le code n'est pas en base pour la date (avant 2015 ou absent).
pub fn maladie_alsace_moselle(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let ts = ctx.taux_sal("ALSACE_MOSELLE_MALADIE");
    if ts == Decimal::ZERO {
        return None;
    }
    Some(LigneCotisation {
        code:        "ALSACE_MOSELLE_MALADIE".into(),
        libelle:     ctx.libelle("ALSACE_MOSELLE_MALADIE", "Maladie complémentaire Alsace-Moselle (régime local)"),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Sécurité Sociale".into(),
        explication: ctx.expl("ALSACE_MOSELLE_MALADIE", "Le régime local d'Alsace-Moselle (droit local) offre une couverture \
            maladie complémentaire obligatoire aux salariés des départements du Bas-Rhin (67), \
            Haut-Rhin (68) et Moselle (57). Cette cotisation, uniquement salariale, est prélevée \
            en sus du régime général. Elle finance un remboursement à 90 % (contre 70 % en régime \
            général) des frais de santé, sans ticket modérateur pour les hospitalisations. \
            Ce régime est issu du droit bismarckien applicable depuis 1871, maintenu lors du \
            retour de l'Alsace-Lorraine à la France en 1919 (loi du 1er juin 1924). \
            Taux 1,50 % jusqu'au 30/06/2018, puis 1,30 % à compter du 01/07/2018 (LFSS 2018)."),
        loi_ref: Some(ctx.loi_ref("Loi locale du 1/06/1924 — CSS art. L325-1 et s. — Loi 2018-1203 du 22/12/2018")),
    })
}

pub fn retraite_complementaire(brut: Decimal, statut: &Statut, etp_pct: f64, ctx: &ContextPaie) -> Vec<LigneCotisation> {
    let (pmss, note) = pmss_proratise(ctx, etp_pct);
    let t1_base = brut.min(pmss);
    let t2_base = if brut > pmss {
        (brut - pmss).min(pmss * dec!(7))
    } else {
        Decimal::ZERO
    };

    let mut lignes = vec![
        LigneCotisation {
            code:        "AGIRC_ARRCO_T1".into(),
            libelle:     ctx.libelle("AGIRC_ARRCO_T1", "AGIRC-ARRCO Tranche 1"),
            base:        t1_base,
            taux_sal:    ctx.taux_sal("AGIRC_ARRCO_T1"),
            montant_sal: (t1_base * ctx.taux_sal("AGIRC_ARRCO_T1")).round_dp(2),
            taux_pat:    ctx.taux_pat("AGIRC_ARRCO_T1"),
            montant_pat: (t1_base * ctx.taux_pat("AGIRC_ARRCO_T1")).round_dp(2),
            categorie:   "Retraite complémentaire".into(),
            explication: ctx.expl("AGIRC_ARRCO_T1",
                "AGIRC-ARRCO : fusion en 2019 des régimes cadres (AGIRC, 1947) et non-cadres \
                (ARRCO, 1961). Système par points. \
                Tranche 1 = salaire jusqu'au PMSS ({pmss} €).{etp_info}")
                .replace("{etp_info}", &note)
                .replace("{pmss}", &pmss.to_string()),
            loi_ref: Some(ctx.loi_ref("Accord national interprofessionnel du 17/11/2017")),
        },
    ];

    if t2_base > Decimal::ZERO {
        lignes.push(LigneCotisation {
            code:        "AGIRC_ARRCO_T2".into(),
            libelle:     ctx.libelle("AGIRC_ARRCO_T2", "AGIRC-ARRCO Tranche 2"),
            base:        t2_base,
            taux_sal:    ctx.taux_sal("AGIRC_ARRCO_T2"),
            montant_sal: (t2_base * ctx.taux_sal("AGIRC_ARRCO_T2")).round_dp(2),
            taux_pat:    ctx.taux_pat("AGIRC_ARRCO_T2"),
            montant_pat: (t2_base * ctx.taux_pat("AGIRC_ARRCO_T2")).round_dp(2),
            categorie:   "Retraite complémentaire".into(),
            explication: ctx.expl("AGIRC_ARRCO_T2", "Tranche 2 : fraction du salaire entre 1 et 8 PMSS. \
                Taux plus élevé car vise les salaires intermédiaires à élevés. \
                Géré paritairement (syndicats + patronat).{etp_info}")
                .replace("{etp_info}", &note),
            loi_ref: Some(ctx.loi_ref("Accord national interprofessionnel du 17/11/2017")),
        });
    }

    // Contributions d'équilibre (CEG)
    let ceg_t1_ts = ctx.taux_sal("AGIRC_ARRCO_CEG_T1");
    let ceg_t1_tp = ctx.taux_pat("AGIRC_ARRCO_CEG_T1");
    if ceg_t1_ts + ceg_t1_tp > Decimal::ZERO {
        lignes.push(LigneCotisation {
            code:        "AGIRC_ARRCO_CEG_T1".into(),
            libelle:     ctx.libelle("AGIRC_ARRCO_CEG_T1", "Contribution d'Équilibre Général T1"),
            base:        t1_base,
            taux_sal:    ceg_t1_ts,
            montant_sal: (t1_base * ceg_t1_ts).round_dp(2),
            taux_pat:    ceg_t1_tp,
            montant_pat: (t1_base * ceg_t1_tp).round_dp(2),
            categorie:   "Retraite complémentaire".into(),
            explication: ctx.expl("AGIRC_ARRCO_CEG_T1", "Contribution non génératrice de points, destinée à l'équilibre \
                financier du régime AGIRC-ARRCO. Créée lors de la fusion 2019."),
            loi_ref: Some(ctx.loi_ref("ANI 17/11/2017")),
        });
    }

    // Prévoyance cadre minimale (art. 7 CCN 1947)
    if matches!(statut, Statut::Cadre) {
        let tp = ctx.taux_pat("PREVOYANCE_CADRE_MIN");
        lignes.push(LigneCotisation {
            code:        "PREVOYANCE_CADRE_MIN".into(),
            libelle:     ctx.libelle("PREVOYANCE_CADRE_MIN", "Prévoyance cadre minimale (art. 7 CCN 1947)"),
            base:        t1_base,
            taux_sal:    Decimal::ZERO,
            montant_sal: Decimal::ZERO,
            taux_pat:    tp,
            montant_pat: (t1_base * tp).round_dp(2),
            categorie:   "Prévoyance".into(),
            explication: ctx.expl("PREVOYANCE_CADRE_MIN", "La Convention Collective Nationale des Cadres (14/03/1947) \
                impose aux employeurs une cotisation minimale de 1,5% sur la tranche A \
                pour financer la prévoyance décès des cadres. Obligation employeur unique \
                en Europe, résultat de la négociation d'après-guerre."),
            loi_ref: Some(ctx.loi_ref("Convention du 14/03/1947 — Article 7")),
        });
    }

    lignes
}
