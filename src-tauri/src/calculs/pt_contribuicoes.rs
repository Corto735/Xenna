// ── Cotisations Portugal — régime geral, secteur privé ───────────────────────
//
// Assiette : salaire brut réel sans plafond (regime geral).
// Taux lus depuis ContextPaie (cotisation_taux DB).
//
// Sources légales :
//   Lei 110/2009 (Código Contributivo) art. 53-54 — SS
//   Lei 98/2009 — Acidentes de Trabalho
//   DL 210/2015 art. 4-5 — FCT / FGCT

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

pub fn seguranca_social(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal("PT_SS");
    let tp = ctx.taux_pat("PT_SS");
    LigneCotisation {
        code:        "PT_SS".into(),
        libelle:     "Segurança Social — Taxa Social Única (TSU)".into(),
        base:        brut,
        taux_sal:    ts,
        montant_sal: (brut * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Sécurité sociale".into(),
        explication: ctx.expl("PT_SS",
            "Cotisation principale du régime geral de segurança social (TSU). \
            Couvre : doença (maladie/maternité), invalidez (invalidité), \
            velhice (retraite), sobrevivência (décès et survivants), \
            desemprego (chômage). \
            Assiette : rémunération brute intégrale, sans plafond.\n\n\
            Salarié : {ts_pct} % × {brut} € = {ms} €\n\
            Employeur : {tp_pct} % × {brut} € = {mp} €\n\
            Total : {total} % — soit {tot} €\n\
            \n\
            Taux stable depuis 2013 (retour au taux nominal après la hausse \
            temporaire à 11,5 % pendant la crise 2012-2013). \
            Base légale : Lei 110/2009 (Código Contributivo) art. 53 et 54.")
            .replace("{ts_pct}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp_pct}", &format!("{:.2}", tp * dec!(100)))
            .replace("{brut}", &format!("{:.2}", brut))
            .replace("{ms}", &format!("{:.2}", (brut * ts).round_dp(2)))
            .replace("{mp}", &format!("{:.2}", (brut * tp).round_dp(2)))
            .replace("{total}", &format!("{:.2}", (ts + tp) * dec!(100)))
            .replace("{tot}", &format!("{:.2}", ((brut * ts) + (brut * tp)).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("Lei 110/2009 (Código Contributivo) art. 53-54")),
    }
}

pub fn acidentes_trabalho(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("PT_AT_SEG");
    LigneCotisation {
        code:        "PT_AT_SEG".into(),
        libelle:     ctx.libelle("PT_AT_SEG", "Acidentes de Trabalho — assurance accidents du travail"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Accidents du travail".into(),
        explication: ctx.expl("PT_AT_SEG",
            "Assurance obligatoire couvrant les accidents du travail et les maladies \
            professionnelles. Exclusivement à la charge de l'employeur. \
            Le taux de {tp_pct} % est indicatif (secteur tertiaire, risque moyen) — \
            il varie de 0,5 % (travail de bureau) à 10 %+ (BTP, industrie lourde) \
            selon le contrat d'assurance avec la compagnie agréée.\n\
            Montant employeur : {mp} €.\n\
            \n\
            Base légale : Lei 98/2009 de 04/09/2009, art. 79.")
            .replace("{tp_pct}", &format!("{:.2}", tp * dec!(100)))
            .replace("{mp}", &format!("{:.2}", (brut * tp).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("Lei 98/2009 art. 79 — Regime de Reparação de Acidentes de Trabalho")),
    }
}

pub fn fct(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("PT_FCT");
    LigneCotisation {
        code:        "PT_FCT".into(),
        libelle:     "FCT — Fundo de Compensação do Trabalho".into(),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Garantie emploi".into(),
        explication: ctx.expl("PT_FCT",
            "Fonds de compensation du travail couvrant 50 % des indemnités de \
            licenciement en cas d'insolvabilité de l'employeur. \
            Applicable aux contrats à durée indéterminée (CDI) conclus après le \
            01/10/2013. Exclusivement patronal : {tp_pct} %.\n\
            Montant employeur : {mp} €.\n\
            \n\
            Base légale : DL 210/2015 art. 4.")
            .replace("{tp_pct}", &format!("{:.3}", tp * dec!(100)))
            .replace("{mp}", &format!("{:.2}", (brut * tp).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("DL 210/2015 art. 4")),
    }
}

pub fn fgct(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("PT_FGCT");
    LigneCotisation {
        code:        "PT_FGCT".into(),
        libelle:     "FGCT — Fundo de Garantia de Compensação do Trabalho".into(),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Garantie emploi".into(),
        explication: ctx.expl("PT_FGCT",
            "Fonds de garantie couvrant les 50 % restants des indemnités de \
            licenciement non couverts par le FCT. Exclusivement patronal : {tp_pct} %.\n\
            Montant employeur : {mp} €.\n\
            \n\
            Base légale : DL 210/2015 art. 5.")
            .replace("{tp_pct}", &format!("{:.3}", tp * dec!(100)))
            .replace("{mp}", &format!("{:.2}", (brut * tp).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("DL 210/2015 art. 5")),
    }
}
