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

/// Base minimale des groupes 4 à 7 (régime général) : SMI mensuel majoré d'un
/// sixième (prorata des deux paies extraordinaires). Ordres de cotisation annuels ;
/// 2026 : Orden PJC/297/2026, art. 3 (1 424,40 €).
fn es_base_min(ctx: &ContextPaie) -> Decimal {
    match ctx.date_paie.year() {
        i32::MIN..=2015 => dec!(756.60),
        2016            => dec!(764.40),
        2017            => dec!(825.60),
        2018            => dec!(858.60),
        2019            => dec!(1050.00),
        2020            => dec!(1108.33),
        2021 if ctx.date_paie.month() < 9 => dec!(1108.33),
        2021            => dec!(1125.83),
        2022            => dec!(1166.70),
        2023            => dec!(1260.00),
        2024            => dec!(1323.00),
        2025            => dec!(1381.20),
        _               => dec!(1424.40), // 2026
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
        2025            => dec!(4909.50),
        _               => dec!(5101.20), // 2026 — Orden PJC/297/2026
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

// ── Cotización adicional de solidaridad (depuis 2025) ─────────────────────────
//
// Sur la part du salaire qui excède la base maximale, en trois tranches (jusqu'à
// 10 % au-dessus, de 10 à 50 %, au-delà), réparties comme les contingences
// communes. RDL 2/2023 ; 2025 : Orden TRM/42/2025 ; 2026 : Orden PJC/297/2026 art. 17.
// Taux (salarié, employeur) par tranche.
fn solidaridad_taux(annee: i32) -> Option<[(Decimal, Decimal); 3]> {
    match annee {
        i32::MIN..=2024 => None,
        2025 => Some([(dec!(0.0015), dec!(0.0077)), (dec!(0.0017), dec!(0.0083)), (dec!(0.0019), dec!(0.0098))]),
        _    => Some([(dec!(0.0019), dec!(0.0096)), (dec!(0.0021), dec!(0.0104)), (dec!(0.0024), dec!(0.0122))]),
    }
}

pub fn solidaridad(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let annee = ctx.date_paie.year();
    let taux = solidaridad_taux(annee)?;
    let bmax = es_base_max(ctx);
    if brut <= bmax {
        return None;
    }
    let bornes = [bmax, bmax * dec!(1.10), bmax * dec!(1.50), Decimal::MAX];
    let (mut ms, mut mp) = (Decimal::ZERO, Decimal::ZERO);
    for i in 0..3 {
        let tranche = (brut.min(bornes[i + 1]) - bornes[i]).max(Decimal::ZERO);
        ms += tranche * taux[i].0;
        mp += tranche * taux[i].1;
    }
    let (ms, mp) = (ms.round_dp(2), mp.round_dp(2));
    let exces = brut - bmax;
    Some(LigneCotisation {
        code:        "ES_SOLIDARIDAD".into(),
        libelle:     ctx.libelle("ES_SOLIDARIDAD", "Cotización de solidaridad {annee}")
                        .replace("{annee}", &annee.to_string()),
        base:        exces,
        taux_sal:    (ms / exces).round_dp(4),
        montant_sal: ms,
        taux_pat:    (mp / exces).round_dp(4),
        montant_pat: mp,
        categorie:   "Réserve retraite".into(),
        explication: ctx.expl("ES_SOLIDARIDAD",
            "Cotisation additionnelle de solidarité sur la part du salaire qui dépasse la \
            base maximale ({base_max} €) : {t1} % jusqu'à 10 % au-dessus, {t2} % de 10 à 50 %, \
            {t3} % au-delà, répartis comme les contingences communes.\n\
            Excédent : {base} € — salarié {ms} € — employeur {mp} €.\n\
            Instaurée en 2025 (RDL 2/2023), taux croissants jusqu'en 2045.")
            .replace("{base_max}", &format!("{:.2}", bmax))
            .replace("{t1}", &format!("{:.2}", (taux[0].0 + taux[0].1) * dec!(100)))
            .replace("{t2}", &format!("{:.2}", (taux[1].0 + taux[1].1) * dec!(100)))
            .replace("{t3}", &format!("{:.2}", (taux[2].0 + taux[2].1) * dec!(100)))
            .replace("{base}", &format!("{:.2}", exces))
            .replace("{ms}", &format!("{:.2}", ms))
            .replace("{mp}", &format!("{:.2}", mp)),
        loi_ref: Some(ctx.loi_ref("RDL 2/2023 — LGSS art. 19 bis — Orden PJC/297/2026 art. 17")),
    })
}
