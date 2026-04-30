use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Plafonds suisses en vigueur au 01/01/2026 ────────────────────────────────
//
// AC / AANP / AAP : plafond légal LACI/LAA = CHF 148 200/an = CHF 12 350/mois
const CH_PLAFOND_MENSUEL: Decimal = dec!(12350);

// LPP — valeurs OPP 2 entrées en vigueur le 01/01/2025 (OFAS, revalorisées)
//   Seuil d'entrée  : CHF 22 680/an ÷ 12 = CHF 1 890/mois
//   Déduction coord : CHF 27 225/an ÷ 12 = CHF 2 268,75/mois
//   Salaire coord min : CHF 3 780/an ÷ 12 = CHF 315/mois
//   Salaire coord max : CHF 64 260/an ÷ 12 = CHF 5 355/mois
const CH_LPP_SEUIL_ENTREE:   Decimal = dec!(1890);
const CH_LPP_DEDUCTION_COORD: Decimal = dec!(2268.75);
const CH_LPP_COORD_MIN:       Decimal = dec!(315);
const CH_LPP_COORD_MAX:       Decimal = dec!(5355);

/// Calcule le salaire coordonné LPP mensuel.
/// Retourne 0 si le salaire est inférieur au seuil d'entrée.
fn lpp_salaire_coordonne(brut: Decimal) -> Decimal {
    if brut < CH_LPP_SEUIL_ENTREE {
        return Decimal::ZERO;
    }
    let coord_brut = brut - CH_LPP_DEDUCTION_COORD;
    coord_brut.max(CH_LPP_COORD_MIN).min(CH_LPP_COORD_MAX)
}

// ── Cotisations 1er pilier ────────────────────────────────────────────────────

pub fn ch_avs(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("CH_AVS");
    let tp = ctx.taux_pat("CH_AVS");
    LigneCotisation {
        code:        "CH_AVS".into(),
        libelle:     "AVS — Assurance-vieillesse et survivants".into(),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "1er pilier".into(),
        explication: "L'Assurance-vieillesse et survivants (AVS, AHV en allemand) est le 1er pilier \
            du système suisse de prévoyance, institué par la LAVS de 1946, entrée en vigueur le \
            1er janvier 1948. Financée par répartition, elle garantit une rente vieillesse à partir de \
            65 ans (hommes et femmes depuis la réforme AVS 21, en vigueur au 01/01/2024). \
            Le taux total est de 8,70 % depuis 2020 (introduction du financement additionnel via TVA \
            dans le cadre de la Réforme fiscale et du financement de l'AVS — RFFA). \
            Partagé à parts égales : 4,35 % salarié, 4,35 % employeur. \
            L'assiette est le salaire brut total, sans plafond, par solidarité entre les assurés. \
            Rente maximale 2025 : CHF 2 590/mois ; rente minimale : CHF 1 225/mois.".into(),
        loi_ref: Some("Art. 5 et 13 LAVS (RS 831.10) — Réforme AVS 21 (RO 2023 92)".into()),
    }
}

pub fn ch_ai(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("CH_AI");
    let tp = ctx.taux_pat("CH_AI");
    LigneCotisation {
        code:        "CH_AI".into(),
        libelle:     "AI — Assurance invalidité".into(),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "1er pilier".into(),
        explication: "L'Assurance invalidité (AI, IV en allemand), instituée en 1959 par la LAI, \
            complète le 1er pilier suisse. Elle verse des rentes et finance des mesures de réadaptation \
            professionnelle pour les personnes dont la capacité de gain est durablement réduite. \
            Taux stable à 1,40 % total (0,70 % chacun). Assiette : salaire brut total, sans plafond. \
            Financée conjointement par les cotisations, la TVA et la Confédération. \
            La révision AI (2022) a renforcé la priorité à la réinsertion professionnelle \
            ('la réadaptation prime sur la rente').".into(),
        loi_ref: Some("Art. 3 LAI (RS 831.20)".into()),
    }
}

pub fn ch_apg(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("CH_APG");
    let tp = ctx.taux_pat("CH_APG");
    LigneCotisation {
        code:        "CH_APG".into(),
        libelle:     "APG — Allocations pour perte de gain".into(),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "1er pilier".into(),
        explication: "Les Allocations pour perte de gain (APG, EO en allemand) compensent \
            le manque à gagner lors du service militaire, civil ou de protection civile, \
            ainsi que lors de la maternité et, depuis 2021, lors du congé paternité \
            (2 semaines, introduit au 01/01/2021). \
            Taux stable à 0,50 % total (0,25 % chacun). Assiette : salaire brut total, sans plafond. \
            Souvent présentées groupées avec l'AVS et l'AI sous l'appellation \
            'cotisations AVS/AI/APG' = 10,60 % total (5,30 % chacun).".into(),
        loi_ref: Some("Art. 27 LAPG (RS 834.1) — Congé paternité : Loi du 25/09/2020".into()),
    }
}

// ── Assurance chômage ─────────────────────────────────────────────────────────

pub fn ch_ac(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = brut.min(CH_PLAFOND_MENSUEL);
    let ts = ctx.taux_sal("CH_AC");
    let tp = ctx.taux_pat("CH_AC");
    LigneCotisation {
        code:        "CH_AC".into(),
        libelle:     "AC — Assurance-chômage".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance chômage".into(),
        explication: format!(
            "L'Assurance-chômage obligatoire (AC, ALV en allemand), instituée par la LACI de 1982, \
            garantit une indemnité de chômage représentant 70 à 80 % du gain assuré. \
            Taux : 2,20 % total (1,10 % chacun), plafonné à CHF 148 200/an \
            (CHF {} /mois). Ce plafond correspond au gain assuré maximal. \
            La partie du salaire au-delà du plafond n'est pas soumise à la cotisation AC. \
            Administrée par le SECO, les cantons et les caisses de chômage (paritaires ou syndicales). \
            Révision LACI de 2011 : durcissement des conditions d'indemnisation ; \
            révision 2014 : stabilisation des finances (solde IDA remboursé).",
            CH_PLAFOND_MENSUEL
        ),
        loi_ref: Some("Art. 3 LACI (RS 837.0) — Ordonnance OACI (RS 837.02)".into()),
    }
}

// ── Assurance accidents (LAA) ─────────────────────────────────────────────────

pub fn ch_aanp(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = brut.min(CH_PLAFOND_MENSUEL);
    let ts = ctx.taux_sal("CH_AANP");
    LigneCotisation {
        code:        "CH_AANP".into(),
        libelle:     "AANP — Assurance accidents non professionnels".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Assurance accidents".into(),
        explication: format!(
            "La LAA (Loi fédérale sur l'assurance-accidents, RS 832.20) distingue deux couvertures : \
            les accidents professionnels (AAP, à charge de l'employeur) et non professionnels \
            (AANP, à charge du salarié). \
            L'AANP couvre les accidents survenant hors du travail (loisirs, sport, vie privée). \
            Cotisation prélevée sur le salarié uniquement. \
            Assiette : salaire brut plafonné à CHF 148 200/an (CHF {} /mois). \
            Le taux (ici {:.1} %) est fixé par l'assureur (SUVA ou assureur privé agréé) \
            selon la classe de risque professionnelle — ce taux est indicatif pour un emploi de bureau.",
            CH_PLAFOND_MENSUEL,
            ts * dec!(100)
        ),
        loi_ref: Some("Art. 92 et 93 LAA (RS 832.20) — OLAA (RS 832.202)".into()),
    }
}

pub fn ch_aap(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = brut.min(CH_PLAFOND_MENSUEL);
    let tp = ctx.taux_pat("CH_AAP");
    LigneCotisation {
        code:        "CH_AAP".into(),
        libelle:     "AAP — Assurance accidents professionnels".into(),
        base,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance accidents".into(),
        explication: format!(
            "L'assurance accidents professionnels (AAP) couvre les accidents et maladies professionnelles \
            survenus dans le cadre du travail. Entièrement à charge de l'employeur. \
            Assiette : salaire brut plafonné à CHF 148 200/an (CHF {} /mois). \
            Le taux (ici {:.1} %) est déterminé par la SUVA ou un assureur agréé \
            selon la classe de risque de l'entreprise (code NOGA). \
            Les entreprises à risque élevé (bâtiment, chimie…) paient un taux nettement plus fort \
            que les employeurs de bureau. Taux indicatif pour un secteur tertiaire standard.",
            CH_PLAFOND_MENSUEL,
            tp * dec!(100)
        ),
        loi_ref: Some("Art. 92 et 93 LAA (RS 832.20) — OLAA (RS 832.202)".into()),
    }
}

// ── Indemnités journalières maladie ──────────────────────────────────────────

pub fn ch_ijm(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("CH_IJM");
    let tp = ctx.taux_pat("CH_IJM");
    LigneCotisation {
        code:        "CH_IJM".into(),
        libelle:     "IJM — Indemnités journalières maladie (plan collectif)".into(),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Prévoyance maladie".into(),
        explication: "Contrairement à la France, la Suisse n'a pas d'assurance maladie indemnités \
            journalières obligatoire à charge partagée (seule la LAMal de base est obligatoire, \
            mais elle ne couvre pas le maintien du salaire). La plupart des employeurs souscrivent \
            un plan collectif IJM (sous la LCA ou la LAMal art. 67-77) garantissant \
            généralement 80 % du salaire pendant 720 ou 730 jours. \
            Le financement est conventionnel : taux et répartition salarié/employeur fixés \
            par accord d'entreprise ou convention collective. \
            Le taux affiché (1,50 % total, 0,75 % chacun) est indicatif — il varie \
            selon l'assureur, le taux de sinistralité et les garanties choisies.".into(),
        loi_ref: Some("Art. 67-77 LAMal (RS 832.10) — ou LCA (RS 221.229.1) selon contrat".into()),
    }
}

// ── Prévoyance professionnelle (LPP) ─────────────────────────────────────────

pub fn ch_lpp(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let coord = lpp_salaire_coordonne(brut);
    let ts = ctx.taux_sal("CH_LPP");
    let tp = ctx.taux_pat("CH_LPP");
    LigneCotisation {
        code:        "CH_LPP".into(),
        libelle:     "LPP — Prévoyance professionnelle (2ème pilier)".into(),
        base:        coord,
        taux_sal:    ts,
        montant_sal: (coord * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (coord * tp).round_dp(2),
        categorie:   "Prévoyance (LPP)".into(),
        explication: format!(
            "La LPP (Loi sur la prévoyance professionnelle, RS 831.40), en vigueur depuis 1985, \
            constitue le 2ème pilier du système suisse. Elle complète l'AVS pour maintenir \
            le niveau de vie à la retraite. \
            Obligatoire pour les salariés dont le salaire annuel dépasse CHF 22 680 \
            (seuil d'entrée 2025, OPP 2 art. 2). \
            \n\n\
            [ Calcul du salaire coordonné ]\n\
            Salaire coordonné = max(CHF {coord_min}, brut − déduction de coordination)\n\
              = max(CHF {coord_min}, {brut} − CHF {coord_ded})\n\
              = CHF {coord}\n\
            Plafonné à CHF {coord_max}/mois (CHF 64 260/an).\n\
            \n\
            Taux affiché : minimum légal pour la tranche d'âge 35-44 ans (art. 16 LPP) :\n\
              25-34 ans →  7 % total (3,50 % chacun)\n\
              35-44 ans → 10 % total (5,00 % chacun)  ← taux utilisé ici\n\
              45-54 ans → 15 % total (7,50 % chacun)\n\
              55-65 ans → 18 % total (9,00 % chacun)\n\
            \n\
            Les cotisations LPP sont déductibles du revenu imposable (employé et employeur). \
            Les fonds sont gérés en capitalisation individuelle, pas en répartition.",
            coord_min  = CH_LPP_COORD_MIN,
            coord_ded  = CH_LPP_DEDUCTION_COORD,
            brut       = brut,
            coord      = coord,
            coord_max  = CH_LPP_COORD_MAX,
        ),
        loi_ref: Some("Art. 7 et 16 LPP (RS 831.40) — OPP 2 (RS 831.441.1)".into()),
    }
}
