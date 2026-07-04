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
            Montée en charge 2012-2019 (décret n°2011-291) :\n\
            2016 : 10,29 % — 2017 : 10,56 % — 2018 : 10,83 % — 2019+ : 11,10 %\n\
            Taux collectivité stable : 30,65 % (vs ≈ 16 % total en privé)\n\
            \n\
            Taux appliqués : agent {ts_pct} % — collectivité {tp_pct} %.")
            .replace("{ts_pct}", &ts_pct.to_string())
            .replace("{tp_pct}", &tp_pct.to_string()),
        loi_ref: Some(ctx.loi_ref(
            "Décret n°2011-291 du 15/03/2011 — CGFP art. L712-3 et s. — \
            Loi n°83-634 du 13/07/1983 (statut général FP)"
        )),
    }
}
