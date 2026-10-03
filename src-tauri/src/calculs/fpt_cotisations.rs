use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

/// CNRACL — régime de retraite principal des agents titulaires de la FPT.
/// Remplace SS_VIEILLESSE (plafonnée + déplafonnée) ET AGIRC-ARRCO.
/// Assiette : salaire brut total, sans plafond.
/// Taux historiques (montée en charge 2016→2019) chargés depuis la DB.
pub fn fpt_cnracl(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("FPT_CNRACL");
    let tp = ctx.taux_pat("FPT_CNRACL");
    let ts_pct = (ts * dec!(100)).round_dp(2);
    let tp_pct = (tp * dec!(100)).round_dp(2);
    LigneCotisation {
        code:        "FPT_CNRACL".into(),
        libelle:     ctx.libelle("FPT_CNRACL", "CNRACL — Retraite principale"),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Retraite (CNRACL)".into(),
        explication: ctx.expl("FPT_CNRACL",
            "La CNRACL (Caisse Nationale de Retraite des Agents des Collectivités Locales) \
            est le régime obligatoire de retraite des fonctionnaires territoriaux titulaires. \
            Elle remplace à la fois l'assurance vieillesse du régime général (CARSAT) \
            et la retraite complémentaire AGIRC-ARRCO — le fonctionnaire ne cotise donc \
            qu'à une seule caisse pour sa retraite de base et complémentaire.\n\
            \n\
            Différences notables avec le régime privé :\n\
            • Pension calculée sur les 6 derniers mois (traitement indiciaire), \
            non sur les 25 meilleures années comme en privé\n\
            • Taux de remplacement cible : 75 % après 41 ans et 3 trimestres (2016)\n\
            • Pas de capitalisation : régime par répartition\n\
            \n\
            Montée en charge 2011-2020 (décret n°2011-291) :\n\
            2015 : 9,54 % — 2016 : 9,94 % — 2017 : 10,29 % — 2018 : 10,56 % — 2019 : 10,83 % — 2020+ : 11,10 %\n\
            Taux collectivité : 30,65 % jusqu'en 2024, puis relevé de 3 points par an \
            (décret n°2025-86) : 34,65 % en 2025, 37,65 % en 2026, 40,65 % en 2027, 43,65 % en 2028\n\
            \n\
            Taux appliqués : agent {ts_pct} % — collectivité {tp_pct} %.")
            .replace("{ts_pct}", &ts_pct.to_string())
            .replace("{tp_pct}", &tp_pct.to_string()),
        loi_ref: Some(ctx.loi_ref(
            "Décret n°2011-291 du 15/03/2011 — Décret n°2025-86 du 30/01/2025 — CGFP art. L712-3 et s. — \
            Loi n°83-634 du 13/07/1983 (statut général FP)"
        )),
    }
}

/// Ligne patronale d'un employeur territorial : base × taux en base pour `code`.
/// None si aucun taux n'est en base à la date.
fn fpt_patronale(
    code: &str,
    libelle: &str,
    base: Decimal,
    categorie: &str,
    explication: String,
    loi_ref: &str,
    ctx: &ContextPaie,
) -> Option<LigneCotisation> {
    let tp = ctx.taux_pat(code);
    if tp == Decimal::ZERO {
        return None;
    }
    Some(LigneCotisation {
        code:        code.into(),
        libelle:     ctx.libelle(code, libelle),
        base,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   categorie.into(),
        explication,
        loi_ref:     Some(ctx.loi_ref(loi_ref)),
    })
}

/// Maladie, maternité, invalidité, décès des fonctionnaires affiliés à la
/// CNRACL : régime spécial, taux employeur propre (11,50 % jusqu'en 2017,
/// 9,88 % depuis 2018), aucune part agent.
pub fn fpt_maladie(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    fpt_patronale("FPT_MALADIE", "Maladie, maternité, invalidité, décès (fonctionnaires)", brut,
        "Sécurité Sociale",
        ctx.expl("FPT_MALADIE", "Les fonctionnaires affiliés à la CNRACL relèvent d'un régime \
            spécial d'assurance maladie : la collectivité cotise à un taux propre, 11,50 % \
            jusqu'en 2017 puis 9,88 % depuis 2018 (baisse liée à la hausse de la CSG), sur le \
            traitement indiciaire brut et la NBI. L'agent ne cotise pas."),
        "CSS art. L712-9 et D712-38", ctx)
}

/// Allocation temporaire d'invalidité des agents des collectivités locales,
/// à la place de la cotisation accidents du travail du régime général.
pub fn fpt_atiacl(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    fpt_patronale("FPT_ATIACL", "ATIACL — allocation temporaire d'invalidité", brut,
        "Sécurité Sociale",
        ctx.expl("FPT_ATIACL", "Le fonctionnaire titulaire ne relève pas de la branche accidents \
            du travail du régime général : l'allocation temporaire d'invalidité (ATIACL), gérée \
            par la Caisse des dépôts, indemnise l'invalidité permanente consécutive à un accident \
            de service ou à une maladie professionnelle. Cotisation de la collectivité de 0,40 % \
            du traitement indiciaire brut, hors NBI."),
        "Décret n°2005-442 du 2/05/2005", ctx)
}

/// FNAL : 0,10 % dans la limite du plafond de la Sécurité sociale (moins de
/// 50 agents), 0,50 % sur la totalité au-delà. Effectif « 250p » seul traité
/// comme 50 agents et plus : la tranche « 20_249 » ne permet pas de trancher.
pub fn fpt_fnal(brut: Decimal, effectif: Option<&str>, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let grand = effectif == Some("250p");
    let code = if grand { "FPT_FNAL_50" } else { "FPT_FNAL" };
    let base = if grand { brut } else { brut.min(ctx.pmss) };
    fpt_patronale(code, "FNAL — aide au logement", base, "Sécurité Sociale",
        ctx.expl("FPT_FNAL", "Le Fonds national d'aide au logement finance les aides personnelles \
            au logement. Collectivités de moins de 50 agents : 0,10 % dans la limite du plafond de \
            la Sécurité sociale ; 50 agents et plus : 0,50 % sur la totalité du traitement."),
        "CSS art. L834-1", ctx)
}

/// Contribution solidarité autonomie : 0,30 %, finance la perte d'autonomie
/// (journée de solidarité).
pub fn fpt_csa(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    fpt_patronale("FPT_CSA", "Contribution solidarité autonomie", brut, "Sécurité Sociale",
        ctx.expl("FPT_CSA", "Contribution de 0,30 % à la charge de l'employeur, créée en 2004 avec \
            la journée de solidarité : elle finance la Caisse nationale de solidarité pour \
            l'autonomie (personnes âgées et handicapées)."),
        "Loi n°2004-626 du 30/06/2004, art. 11", ctx)
}

/// Cotisation au Centre national de la fonction publique territoriale
/// (formation des agents), apprentissage compris depuis 2022.
pub fn fpt_cnfpt(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    fpt_patronale("FPT_CNFPT", "CNFPT — formation des agents", brut, "Formation",
        ctx.expl("FPT_CNFPT", "Le Centre national de la fonction publique territoriale forme les \
            agents territoriaux. Cotisation obligatoire de la collectivité : 1 % jusqu'en 2015, \
            0,9 % de 2016 à 2021, puis une cotisation apprentissage s'y ajoute : 0,95 % en \
            2022, 1 % depuis 2023 (dont 0,10 % pour la formation des apprentis)."),
        "Loi n°84-594 du 12/07/1984, art. 12-2", ctx)
}
