use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Plafonds (BBG) ────────────────────────────────────────────────────────────

fn de_bbg_kv(ctx: &ContextPaie) -> Decimal {
    match ctx.date_paie.year() {
        i32::MIN..=2015 => dec!(4125.00),
        2016            => dec!(4237.50),
        2017            => dec!(4350.00),
        2018            => dec!(4425.00),
        2019            => dec!(4537.50),
        2020            => dec!(4687.50),
        2021 | 2022     => dec!(4837.50),
        2023            => dec!(4987.50),
        2024            => dec!(5175.00),
        2025            => dec!(5512.50),
        _               => dec!(5812.50),
    }
}

fn de_bbg_rv(ctx: &ContextPaie) -> Decimal {
    match ctx.date_paie.year() {
        i32::MIN..=2015 => dec!(6050.00),
        2016            => dec!(6200.00),
        2017            => dec!(6350.00),
        2018            => dec!(6500.00),
        2019            => dec!(6700.00),
        2020            => dec!(6900.00),
        2021            => dec!(7100.00),
        2022            => dec!(7050.00),
        2023            => dec!(7300.00),
        2024            => dec!(7550.00),
        2025            => dec!(8050.00),
        _               => dec!(8450.00),
    }
}

// ── KV — Krankenversicherung ──────────────────────────────────────────────────

pub fn de_krankenversicherung(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let bbg = de_bbg_kv(ctx);
    let base = brut.min(bbg);
    let ts = ctx.taux_sal("DE_KRANKENVERSICHERUNG");
    let tp = ctx.taux_pat("DE_KRANKENVERSICHERUNG");
    let annee = ctx.date_paie.year();
    let partage_depuis_2019 = annee >= 2019;
    LigneCotisation {
        code:        "DE_KRANKENVERSICHERUNG".into(),
        libelle:     "KV — Krankenversicherung".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance maladie".into(),
        explication: format!(
            "L'assurance maladie légale (GKV) est organisée par le GKV-Spitzenverband et régie par \
            le SGB V. Le taux général (Allgemeiner Beitragssatz) est de 14,6 % depuis 2015. \
            S'y ajoute un taux additionnel (Zusatzbeitragssatz) fixé chaque année par caisse : \
            {taux_add:.1} % en moyenne pour {annee}.\n\n\
            {partage} \
            Assiette plafonnée à la Beitragsbemessungsgrenze KV : {bbg:.2} €/mois en {annee}. \
            Au-delà, aucune cotisation supplémentaire KV n'est due — \
            les hauts revenus peuvent opter pour une assurance privée (PKV) \
            si leur salaire dépasse la Jahresarbeitsentgeltgrenze.",
            taux_add = (ts + tp - dec!(0.146)) * dec!(100),
            annee    = annee,
            bbg      = bbg,
            partage  = if partage_depuis_2019 {
                "Depuis la réforme GKV-VEG du 01/01/2019, le Zusatzbeitrag est partagé à parts \
                égales entre salarié et employeur. Avant 2019, il était intégralement à la charge \
                du salarié. "
            } else {
                "Avant la réforme GKV-VEG (2019), le Zusatzbeitrag était intégralement à la charge \
                du salarié — d'où l'asymétrie des taux ici. "
            },
        ),
        loi_ref: Some("SGB V §241-242 — GKV-VEG 2019".into()),
    }
}

// ── RV — Rentenversicherung ───────────────────────────────────────────────────

pub fn de_rentenversicherung(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let bbg = de_bbg_rv(ctx);
    let base = brut.min(bbg);
    let ts = ctx.taux_sal("DE_RENTENVERSICHERUNG");
    let tp = ctx.taux_pat("DE_RENTENVERSICHERUNG");
    let annee = ctx.date_paie.year();
    LigneCotisation {
        code:        "DE_RENTENVERSICHERUNG".into(),
        libelle:     "RV — Rentenversicherung".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Retraite".into(),
        explication: format!(
            "L'assurance retraite légale (GRV) est gérée par la Deutsche Rentenversicherung (DRV). \
            Elle fonctionne en répartition (Umlageverfahren) : les cotisations actuelles financent \
            les pensions en cours. Le taux est de {taux:.1} % ({ts:.2} % salarié + {tp:.2} % patronal).\n\n\
            Assiette plafonnée à la Beitragsbemessungsgrenze RV : {bbg:.2} €/mois en {annee}. \
            Depuis le 01/01/2025, cette limite est unifiée Est/Ouest (auparavant, \
            les Neue Länder avaient un plafond distinct plus bas).\n\n\
            La pension est calculée en Entgeltpunkte (points de revenu) : chaque année, \
            1 point = salaire moyen national. La valeur d'un point (Rentenwert) est revalorisée annuellement.",
            taux = (ts + tp) * dec!(100),
            ts   = ts * dec!(100),
            tp   = tp * dec!(100),
            bbg  = bbg,
            annee = annee,
        ),
        loi_ref: Some("SGB VI §158, §160 — RV-Stabilitäts-G 2025 (unification BBG)".into()),
    }
}

// ── AV — Arbeitslosenversicherung ─────────────────────────────────────────────

pub fn de_arbeitslosenversicherung(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let bbg = de_bbg_rv(ctx); // AV partage la BBG RV
    let base = brut.min(bbg);
    let ts = ctx.taux_sal("DE_ARBEITSLOSENVERSICHERUNG");
    let tp = ctx.taux_pat("DE_ARBEITSLOSENVERSICHERUNG");
    let annee = ctx.date_paie.year();
    LigneCotisation {
        code:        "DE_ARBEITSLOSENVERSICHERUNG".into(),
        libelle:     "AV — Arbeitslosenversicherung".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance chômage".into(),
        explication: format!(
            "L'assurance chômage (SGB III) est gérée par la Bundesagentur für Arbeit (BA). \
            Taux {annee} : {taux:.1} % ({ts:.2} % chacun). \
            Historique des taux : 3,0 % (2015-2018) → 2,6 % (2019, Qualifizierungschancengesetz) \
            → 2,4 % (2020-2022, réduction temporaire) → 2,6 % (2023+). \
            Même assiette plafonnée que la RV : {bbg:.2} €/mois en {annee}.",
            annee = annee,
            taux  = (ts + tp) * dec!(100),
            ts    = ts * dec!(100),
            bbg   = bbg,
        ),
        loi_ref: Some("SGB III §341-342 — Qualifizierungschancengesetz 2019".into()),
    }
}

// ── PV — Pflegeversicherung ───────────────────────────────────────────────────

pub fn de_pflegeversicherung(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let bbg = de_bbg_kv(ctx); // PV partage la BBG KV
    let base = brut.min(bbg);
    let ts = ctx.taux_sal("DE_PFLEGEVERSICHERUNG");
    let tp = ctx.taux_pat("DE_PFLEGEVERSICHERUNG");
    let annee = ctx.date_paie.year();
    LigneCotisation {
        code:        "DE_PFLEGEVERSICHERUNG".into(),
        libelle:     "PV — Pflegeversicherung (avec enfants)".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Assurance dépendance".into(),
        explication: format!(
            "L'assurance dépendance (Pflegeversicherung, SGB XI) finance les soins aux personnes \
            dépendantes (Pflegegrade 1 à 5). Taux {annee} : {taux:.2} % ({ts:.3} % chacun). \
            Progression historique : 2,35 % (2015-2016) → 2,55 % (2017-2018) → 3,05 % (2019-06/2023) \
            → 3,40 % (07/2023, réforme PUEG) → 3,60 % (2025+). \
            Assiette plafonnée à la Beitragsbemessungsgrenze KV : {bbg:.2} €/mois. \
            Les personnes sans enfant paient un supplément (Kinderlosenzuschlag) — voir ligne dédiée.",
            annee = annee,
            taux  = (ts + tp) * dec!(100),
            ts    = ts * dec!(100),
            bbg   = bbg,
        ),
        loi_ref: Some("SGB XI §54-55 — PUEG 01/07/2023".into()),
    }
}

pub fn de_pv_kinderlos(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let bbg = de_bbg_kv(ctx);
    let base = brut.min(bbg);
    let ts = ctx.taux_sal("DE_PV_KINDERLOS");
    LigneCotisation {
        code:        "DE_PV_KINDERLOS".into(),
        libelle:     "PV — Kinderlosenzuschlag (supplément sans enfant)".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Assurance dépendance".into(),
        explication: format!(
            "Les personnes sans enfant de plus de 23 ans paient un supplément salarial \
            (Kinderlosenzuschlag) au titre de la Pflegeversicherung. \
            Taux actuel : {ts:.2} % salarié uniquement (employeur exonéré). \
            Historique : +0,25 % (2005-2021) → +0,35 % (2022-06/2023) → +0,60 % (07/2023+). \
            La réforme PUEG de juillet 2023 a également introduit des réductions progressives \
            pour les familles nombreuses (2 à 5 enfants) non simulées ici.",
            ts = ts * dec!(100),
        ),
        loi_ref: Some("SGB XI §55 al. 3 — PUEG 01/07/2023".into()),
    }
}

// ── UV — Unfallversicherung ───────────────────────────────────────────────────

pub fn de_unfallversicherung(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("DE_UNFALLVERSICHERUNG");
    LigneCotisation {
        code:        "DE_UNFALLVERSICHERUNG".into(),
        libelle:     "UV — Unfallversicherung (taux moyen)".into(),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Assurance accidents".into(),
        explication:
            "L'assurance accidents du travail et maladies professionnelles (SGB VII) est \
            exclusivement à la charge de l'employeur. Elle est gérée par des Berufsgenossenschaften \
            (BG) sectorielles, chacune fixant son propre taux selon la classe de risque de l'entreprise.\n\n\
            Fourchette : ~0,5 % (services/administration) à ~3,5 % (BTP, industrie lourde). \
            Le taux affiché ici est la moyenne nationale indicative DGUV (~1,3 %). \
            L'assiette est le salaire brut sans plafond (contrairement aux autres cotisations).".into(),
        loi_ref: Some("SGB VII §150-162 — DGUV".into()),
    }
}
