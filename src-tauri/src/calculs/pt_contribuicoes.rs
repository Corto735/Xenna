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
        explication: format!(
            "Cotisation principale du régime geral de segurança social (TSU). \
            Couvre : doença (maladie/maternité), invalidez (invalidité), \
            velhice (retraite), sobrevivência (décès et survivants), \
            desemprego (chômage). \
            Assiette : rémunération brute intégrale, sans plafond.\n\n\
            Salarié : {ts_pct:.2} % × {brut:.2} € = {ms:.2} €\n\
            Employeur : {tp_pct:.2} % × {brut:.2} € = {mp:.2} €\n\
            Total : {total:.2} % — soit {tot:.2} €\n\
            \n\
            Taux stable depuis 2013 (retour au taux nominal après la hausse \
            temporaire à 11,5 % pendant la crise 2012-2013). \
            Base légale : Lei 110/2009 (Código Contributivo) art. 53 et 54.",
            ts_pct = ts * dec!(100),
            tp_pct = tp * dec!(100),
            ms     = (brut * ts).round_dp(2),
            mp     = (brut * tp).round_dp(2),
            total  = (ts + tp) * dec!(100),
            tot    = ((brut * ts) + (brut * tp)).round_dp(2),
        ),
        loi_ref: Some("Lei 110/2009 (Código Contributivo) art. 53-54".into()),
    }
}

pub fn acidentes_trabalho(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("PT_AT_SEG");
    LigneCotisation {
        code:        "PT_AT_SEG".into(),
        libelle:     "Acidentes de Trabalho — assurance accidents du travail".into(),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Accidents du travail".into(),
        explication: format!(
            "Assurance obligatoire couvrant les accidents du travail et les maladies \
            professionnelles. Exclusivement à la charge de l''employeur. \
            Le taux de {tp_pct:.2} % est indicatif (secteur tertiaire, risque moyen) — \
            il varie de 0,5 % (travail de bureau) à 10 %+ (BTP, industrie lourde) \
            selon le contrat d''assurance avec la compagnie agréée.\n\
            Montant employeur : {mp:.2} €.\n\
            \n\
            Base légale : Lei 98/2009 de 04/09/2009, art. 79.",
            tp_pct = tp * dec!(100),
            mp     = (brut * tp).round_dp(2),
        ),
        loi_ref: Some("Lei 98/2009 art. 79 — Regime de Reparação de Acidentes de Trabalho".into()),
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
        explication: format!(
            "Fonds de compensation du travail couvrant 50 % des indemnités de \
            licenciement en cas d''insolvabilité de l''employeur. \
            Applicable aux contrats à durée indéterminée (CDI) conclus après le \
            01/10/2013. Exclusivement patronal : {tp_pct:.3} %.\n\
            Montant employeur : {mp:.2} €.\n\
            \n\
            Base légale : DL 210/2015 art. 4.",
            tp_pct = tp * dec!(100),
            mp     = (brut * tp).round_dp(2),
        ),
        loi_ref: Some("DL 210/2015 art. 4".into()),
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
        explication: format!(
            "Fonds de garantie couvrant les 50 % restants des indemnités de \
            licenciement non couverts par le FCT. Exclusivement patronal : {tp_pct:.3} %.\n\
            Montant employeur : {mp:.2} €.\n\
            \n\
            Base légale : DL 210/2015 art. 5.",
            tp_pct = tp * dec!(100),
            mp     = (brut * tp).round_dp(2),
        ),
        loi_ref: Some("DL 210/2015 art. 5".into()),
    }
}
