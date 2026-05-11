// ── Cotisations Espagne — régime général, contrato indefinido ─────────────────
//
// Assiette : salaire mensuel réel borné entre ES_BASE_MIN et ES_BASE_MAX.
// Les taux sont lus depuis ContextPaie (cotisation_taux DB).
// Les plafonds sont codés ici (même pattern que lu_cotisaciones.rs) :
//   ES_BASE_MIN et ES_BASE_MAX varient par décret annuel (migrations 0032).
//
// Sources légales :
//   LGSS (RDL 8/2015) art. 143-147, 270, 33 ET, 7 DA19a
//   Ley 21/2021 (MEI) ; Ordenes de cotización annuelles MITES

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Plafonds de cotisation mensuels ──────────────────────────────────────────

fn es_base_min(ctx: &ContextPaie) -> Decimal {
    match ctx.date_paie.year() {
        i32::MIN..=2015 => dec!(648.60),
        2016            => dec!(655.20),
        2017            => dec!(707.70),
        2018            => dec!(735.90),
        2019            => dec!(900.00),
        2020            => dec!(950.00),
        2021 if ctx.date_paie.month() < 9 => dec!(950.00),
        2021            => dec!(965.00),
        2022            => dec!(1000.00),
        2023            => dec!(1080.00),
        2024            => dec!(1134.00),
        _               => dec!(1184.00), // 2025+
    }
}

fn es_base_max(ctx: &ContextPaie) -> Decimal {
    match ctx.date_paie.year() {
        i32::MIN..=2015 => dec!(3606.00),
        2016            => dec!(3642.00),
        2017            => dec!(3751.20),
        2018            => dec!(3803.70),
        2019..=2021     => dec!(4070.10),
        2022            => dec!(4139.40),
        2023            => dec!(4495.50),
        2024            => dec!(4720.50),
        _               => dec!(4909.50), // 2025+
    }
}

fn assiette(brut: Decimal, ctx: &ContextPaie) -> Decimal {
    brut.clamp(es_base_min(ctx), es_base_max(ctx))
}

// ── Cotisations ───────────────────────────────────────────────────────────────

pub fn contingencias_comunes(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = assiette(brut, ctx);
    let ts   = ctx.taux_sal("ES_CC");
    let tp   = ctx.taux_pat("ES_CC");
    LigneCotisation {
        code:        "ES_CC".into(),
        libelle:     "Contingencias Comunes — maladie, maternité, retraite".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Sécurité sociale".into(),
        explication: format!(
            "Cotisation principale du régime général de la Sécurité sociale espagnole. \
            Couvre : maladie commune (enfermedad común), maternité/paternité, \
            incapacité permanente, retraite (jubilación), décès et survie. \
            \n\n\
            Assiette : salaire mensuel réel borné entre {base_min:.2} € (ES_BASE_MIN) \
            et {base_max:.2} € (ES_BASE_MAX) en {annee}. \
            Assiette retenue : {base:.2} €.\n\
            Salarié : {ts_pct:.2} % × {base:.2} € = {ms:.2} €\n\
            Employeur : {tp_pct:.2} % × {base:.2} € = {mp:.2} €\n\
            Total : {total:.2} % — soit {tot:.2} €\n\
            \n\
            Base légale : LGSS (RDL 8/2015) art. 143 et 144. \
            Taux stables depuis 2015 : 4,70 % sal + 23,60 % pat = 28,30 % total.",
            base_min  = es_base_min(ctx),
            base_max  = es_base_max(ctx),
            annee     = ctx.date_paie.year(),
            ts_pct    = ts * dec!(100),
            tp_pct    = tp * dec!(100),
            ms        = (base * ts).round_dp(2),
            mp        = (base * tp).round_dp(2),
            total     = (ts + tp) * dec!(100),
            tot       = ((base * ts) + (base * tp)).round_dp(2),
        ),
        loi_ref: Some("LGSS (RDL 8/2015) art. 143-144 — Ordenes de cotización annuelles MITES".into()),
    }
}

pub fn desempleo(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = assiette(brut, ctx);
    let ts   = ctx.taux_sal("ES_DESEMPLEO");
    let tp   = ctx.taux_pat("ES_DESEMPLEO");
    LigneCotisation {
        code:        "ES_DESEMPLEO".into(),
        libelle:     "Desempleo — assurance chômage (contrato indefinido)".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Chômage".into(),
        explication: format!(
            "Cotisation chômage pour contrato indefinido (contrat à durée indéterminée). \
            Gérée par le SEPE (Servicio Público de Empleo Estatal). \
            Les taux pour contrato temporal sont différents (non couverts ici). \
            \n\n\
            Salarié : {ts_pct:.2} % × {base:.2} € = {ms:.2} €\n\
            Employeur : {tp_pct:.2} % × {base:.2} € = {mp:.2} €\n\
            Total : {total:.2} % — soit {tot:.2} €\n\
            \n\
            Base légale : LGSS (RDL 8/2015) art. 270 + Ordenes annuelles.",
            ts_pct = ts * dec!(100),
            tp_pct = tp * dec!(100),
            ms     = (base * ts).round_dp(2),
            mp     = (base * tp).round_dp(2),
            total  = (ts + tp) * dec!(100),
            tot    = ((base * ts) + (base * tp)).round_dp(2),
        ),
        loi_ref: Some("LGSS (RDL 8/2015) art. 270".into()),
    }
}

pub fn fogasa(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = assiette(brut, ctx);
    let tp   = ctx.taux_pat("ES_FOGASA");
    LigneCotisation {
        code:        "ES_FOGASA".into(),
        libelle:     "FOGASA — Fondo de Garantía Salarial".into(),
        base,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Garantie salariale".into(),
        explication: format!(
            "Fonds de garantie des salaires impayés en cas d''insolvabilité ou faillite \
            de l''employeur. Protège les travailleurs pour leurs salaires, congés payés \
            et indemnités (dans des limites légales). \
            Exclusivement à la charge de l''employeur : {tp_pct:.2} %.\n\
            Montant employeur : {mp:.2} €.\n\
            \n\
            Base légale : art. 33 Estatuto de los Trabajadores (RDL 2/2015) ; \
            LGSS art. 33. Taux stable à 0,20 % depuis de nombreuses années.",
            tp_pct = tp * dec!(100),
            mp     = (base * tp).round_dp(2),
        ),
        loi_ref: Some("ET (RDL 2/2015) art. 33 — LGSS (RDL 8/2015)".into()),
    }
}

pub fn formacion_profesional(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = assiette(brut, ctx);
    let ts   = ctx.taux_sal("ES_FP");
    let tp   = ctx.taux_pat("ES_FP");
    LigneCotisation {
        code:        "ES_FP".into(),
        libelle:     "Formación Profesional — formation professionnelle continue".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Formation professionnelle".into(),
        explication: format!(
            "Finance la formation professionnelle continue des salariés (FUNDAE). \
            La cotisation ouvre des droits à des crédits de formation annuels. \
            Salarié : {ts_pct:.2} % — Employeur : {tp_pct:.2} %. \
            Total : {total:.2} %.\n\
            Salarié : {ms:.2} € — Employeur : {mp:.2} €.\n\
            \n\
            Base légale : LGSS art. 7 et DA 19a.",
            ts_pct = ts * dec!(100),
            tp_pct = tp * dec!(100),
            total  = (ts + tp) * dec!(100),
            ms     = (base * ts).round_dp(2),
            mp     = (base * tp).round_dp(2),
        ),
        loi_ref: Some("LGSS (RDL 8/2015) art. 7 et DA 19a".into()),
    }
}

pub fn mei(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let ts = ctx.taux_sal("ES_MEI");
    let tp = ctx.taux_pat("ES_MEI");
    // Avant 2023, ES_MEI est absent de la DB → ContextPaie retourne ZERO.
    // On ne génère pas de ligne si les deux taux sont nuls.
    if ts == Decimal::ZERO && tp == Decimal::ZERO {
        return None;
    }
    let base = assiette(brut, ctx);
    let annee = ctx.date_paie.year();
    Some(LigneCotisation {
        code:        "ES_MEI".into(),
        libelle:     format!("MEI — Mecanismo de Equidad Intergeneracional {annee}"),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Réserve retraite".into(),
        explication: format!(
            "Cotisation additionnelle instaurée par la Ley 21/2021 pour alimenter \
            le Fondo de Reserva de la Seguridad Social (Fonds de réserve des retraites). \
            Objectif : couvrir le surcroît de retraites des générations baby-boom. \
            Le taux progresse annuellement jusqu''en 2032.\n\n\
            {annee} : salarié {ts_pct:.2} % + employeur {tp_pct:.2} % = {total:.2} % total.\n\
            Salarié : {ms:.2} € — Employeur : {mp:.2} €.\n\
            \n\
            En vigueur depuis le 01/01/2023. \
            Base légale : Ley 21/2021 art. 2 ; Ordenes de cotización annuelles.",
            annee   = annee,
            ts_pct  = ts * dec!(100),
            tp_pct  = tp * dec!(100),
            total   = (ts + tp) * dec!(100),
            ms      = (base * ts).round_dp(2),
            mp      = (base * tp).round_dp(2),
        ),
        loi_ref: Some("Ley 21/2021 art. 2 — Mecanismo de Equidad Intergeneracional".into()),
    })
}
