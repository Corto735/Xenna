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
    let ms   = (base * ts).round_dp(2);
    let mp   = (base * tp).round_dp(2);
    LigneCotisation {
        code:        "ES_CC".into(),
        libelle:     ctx.libelle("ES_CC", "Contingencias Comunes — maladie, maternité, retraite"),
        base,
        taux_sal:    ts,
        montant_sal: ms,
        taux_pat:    tp,
        montant_pat: mp,
        categorie:   "Sécurité sociale".into(),
        explication: ctx.expl("ES_CC",
            "Cotisation principale du régime général de la Sécurité sociale espagnole. \
            Couvre : maladie commune (enfermedad común), maternité/paternité, \
            incapacité permanente, retraite (jubilación), décès et survie. \
            \n\n\
            Assiette : salaire mensuel réel borné entre {base_min} € (ES_BASE_MIN) \
            et {base_max} € (ES_BASE_MAX) en {annee}. \
            Assiette retenue : {base} €.\n\
            Salarié : {ts_pct} % × {base} € = {ms} €\n\
            Employeur : {tp_pct} % × {base} € = {mp} €\n\
            Total : {total} % — soit {tot} €\n\
            \n\
            Base légale : LGSS (RDL 8/2015) art. 143 et 144. \
            Taux stables depuis 2015 : 4,70 % sal + 23,60 % pat = 28,30 % total.")
            .replace("{base_min}", &format!("{:.2}", es_base_min(ctx)))
            .replace("{base_max}", &format!("{:.2}", es_base_max(ctx)))
            .replace("{annee}", &ctx.date_paie.year().to_string())
            .replace("{base}", &format!("{:.2}", base))
            .replace("{ts_pct}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp_pct}", &format!("{:.2}", tp * dec!(100)))
            .replace("{ms}", &format!("{:.2}", ms))
            .replace("{mp}", &format!("{:.2}", mp))
            .replace("{total}", &format!("{:.2}", (ts + tp) * dec!(100)))
            .replace("{tot}", &format!("{:.2}", (ms + mp).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("LGSS (RDL 8/2015) art. 143-144 — Ordenes de cotización annuelles MITES")),
    }
}

pub fn desempleo(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = assiette(brut, ctx);
    let ts   = ctx.taux_sal("ES_DESEMPLEO");
    let tp   = ctx.taux_pat("ES_DESEMPLEO");
    let ms   = (base * ts).round_dp(2);
    let mp   = (base * tp).round_dp(2);
    LigneCotisation {
        code:        "ES_DESEMPLEO".into(),
        libelle:     ctx.libelle("ES_DESEMPLEO", "Desempleo — assurance chômage (contrato indefinido)"),
        base,
        taux_sal:    ts,
        montant_sal: ms,
        taux_pat:    tp,
        montant_pat: mp,
        categorie:   "Chômage".into(),
        explication: ctx.expl("ES_DESEMPLEO",
            "Cotisation chômage pour contrato indefinido (contrat à durée indéterminée). \
            Gérée par le SEPE (Servicio Público de Empleo Estatal). \
            Les taux pour contrato temporal sont différents (non couverts ici). \
            \n\n\
            Salarié : {ts_pct} % × {base} € = {ms} €\n\
            Employeur : {tp_pct} % × {base} € = {mp} €\n\
            Total : {total} % — soit {tot} €\n\
            \n\
            Base légale : LGSS (RDL 8/2015) art. 270 + Ordenes annuelles.")
            .replace("{base}", &format!("{:.2}", base))
            .replace("{ts_pct}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp_pct}", &format!("{:.2}", tp * dec!(100)))
            .replace("{ms}", &format!("{:.2}", ms))
            .replace("{mp}", &format!("{:.2}", mp))
            .replace("{total}", &format!("{:.2}", (ts + tp) * dec!(100)))
            .replace("{tot}", &format!("{:.2}", (ms + mp).round_dp(2))),
        loi_ref: Some(ctx.loi_ref("LGSS (RDL 8/2015) art. 270")),
    }
}

pub fn fogasa(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = assiette(brut, ctx);
    let tp   = ctx.taux_pat("ES_FOGASA");
    let mp   = (base * tp).round_dp(2);
    LigneCotisation {
        code:        "ES_FOGASA".into(),
        libelle:     ctx.libelle("ES_FOGASA", "FOGASA — Fondo de Garantía Salarial"),
        base,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: mp,
        categorie:   "Garantie salariale".into(),
        explication: ctx.expl("ES_FOGASA",
            "Fonds de garantie des salaires impayés en cas d'insolvabilité ou faillite \
            de l'employeur. Protège les travailleurs pour leurs salaires, congés payés \
            et indemnités (dans des limites légales). \
            Exclusivement à la charge de l'employeur : {tp_pct} %.\n\
            Montant employeur : {mp} €.\n\
            \n\
            Base légale : art. 33 Estatuto de los Trabajadores (RDL 2/2015) ; \
            LGSS art. 33. Taux stable à 0,20 % depuis de nombreuses années.")
            .replace("{tp_pct}", &format!("{:.2}", tp * dec!(100)))
            .replace("{mp}", &format!("{:.2}", mp)),
        loi_ref: Some(ctx.loi_ref("ET (RDL 2/2015) art. 33 — LGSS (RDL 8/2015)")),
    }
}

pub fn formacion_profesional(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let base = assiette(brut, ctx);
    let ts   = ctx.taux_sal("ES_FP");
    let tp   = ctx.taux_pat("ES_FP");
    let ms   = (base * ts).round_dp(2);
    let mp   = (base * tp).round_dp(2);
    LigneCotisation {
        code:        "ES_FP".into(),
        libelle:     ctx.libelle("ES_FP", "Formación Profesional — formation professionnelle continue"),
        base,
        taux_sal:    ts,
        montant_sal: ms,
        taux_pat:    tp,
        montant_pat: mp,
        categorie:   "Formation professionnelle".into(),
        explication: ctx.expl("ES_FP",
            "Finance la formation professionnelle continue des salariés (FUNDAE). \
            La cotisation ouvre des droits à des crédits de formation annuels. \
            Salarié : {ts_pct} % — Employeur : {tp_pct} %. \
            Total : {total} %.\n\
            Salarié : {ms} € — Employeur : {mp} €.\n\
            \n\
            Base légale : LGSS art. 7 et DA 19a.")
            .replace("{ts_pct}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp_pct}", &format!("{:.2}", tp * dec!(100)))
            .replace("{total}", &format!("{:.2}", (ts + tp) * dec!(100)))
            .replace("{ms}", &format!("{:.2}", ms))
            .replace("{mp}", &format!("{:.2}", mp)),
        loi_ref: Some(ctx.loi_ref("LGSS (RDL 8/2015) art. 7 et DA 19a")),
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
    let ms   = (base * ts).round_dp(2);
    let mp   = (base * tp).round_dp(2);
    Some(LigneCotisation {
        code:        "ES_MEI".into(),
        libelle:     ctx.libelle("ES_MEI", "MEI — Mecanismo de Equidad Intergeneracional {annee}")
                        .replace("{annee}", &annee.to_string()),
        base,
        taux_sal:    ts,
        montant_sal: ms,
        taux_pat:    tp,
        montant_pat: mp,
        categorie:   "Réserve retraite".into(),
        explication: ctx.expl("ES_MEI",
            "Cotisation additionnelle instaurée par la Ley 21/2021 pour alimenter \
            le Fondo de Reserva de la Seguridad Social (Fonds de réserve des retraites). \
            Objectif : couvrir le surcroît de retraites des générations baby-boom. \
            Le taux progresse annuellement jusqu'en 2032.\n\n\
            {annee} : salarié {ts_pct} % + employeur {tp_pct} % = {total} % total.\n\
            Salarié : {ms} € — Employeur : {mp} €.\n\
            \n\
            En vigueur depuis le 01/01/2023. \
            Base légale : Ley 21/2021 art. 2 ; Ordenes de cotización annuelles.")
            .replace("{annee}", &annee.to_string())
            .replace("{ts_pct}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp_pct}", &format!("{:.2}", tp * dec!(100)))
            .replace("{total}", &format!("{:.2}", (ts + tp) * dec!(100)))
            .replace("{ms}", &format!("{:.2}", ms))
            .replace("{mp}", &format!("{:.2}", mp)),
        loi_ref: Some(ctx.loi_ref("Ley 21/2021 art. 2 — Mecanismo de Equidad Intergeneracional")),
    })
}
