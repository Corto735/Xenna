use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Massimale contributivo IVS (mensuel) ─────────────────────────────────────
//
// Applicable uniquement aux salariés sans ancienneté INPS au 31/12/1995.
// Sources : Circolari INPS annuali. Valeurs en €/mois = montant annuel / 12.
// Salariés pré-1996 : aucun plafond (brut total soumis).
pub fn massimale_mensuel(ctx: &ContextPaie) -> Decimal {
    match ctx.date_paie.year() {
        i32::MIN..=2016 => dec!(8360.33),  // 100 324 € / 12 — circ. INPS 13/2015
        2017            => dec!(8360.33),
        2018            => dec!(8452.25),  // 101 427 € / 12 — approx.
        2019            => dec!(8545.25),  // 102 543 € / 12 — circ. INPS 12/2019
        2020            => dec!(8587.92),  // 103 055 € / 12
        2021            => dec!(8587.92),  // gelé
        2022            => dec!(8587.92),  // gelé
        2023            => dec!(8751.17),  // 105 014 € / 12 — circ. INPS 7/2023
        2024            => dec!(9970.83),  // 119 650 € / 12 — circ. INPS 3/2024
        2025            => dec!(10281.58), // 123 379 € / 12 — estimation ISTAT
        _               => dec!(10384.42), // 2026+ : estimation ~1 %
    }
}

// ── IVS — Invalidità, Vecchiaia, Superstiti ──────────────────────────────────

pub fn it_ivs(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let massimale = massimale_mensuel(ctx);
    let base = brut.min(massimale);
    let ts = ctx.taux_sal("IT_IVS");
    let tp = ctx.taux_pat("IT_IVS");
    let explication = ctx.expl("IT_IVS",
        "L'IVS (Invalidità, Vecchiaia, Superstiti) est la cotisation de retraite \
        obligatoire italienne, gérée par l'INPS et régie par la Legge 335/1995 (réforme Dini). \
        Elle fonctionne en répartition : les cotisations actuelles financent les pensions en cours. \
        \n\n\
        Taux total : 33 % = 9,19 % salarié + 23,81 % employeur. \
        Ces taux sont stables depuis les années 1990. \
        La pension est calculée selon le système contributivo (post-1996) : \
        les cotisations versées sont valorisées au PIB nominal annuel, \
        puis converties en rente à l'âge de la retraite via un coefficient de transformation. \
        \n\n\
        Massimale contributivo {annee} : {massimale} €/mois ({annuel} €/an). \
        Ce plafond s'applique uniquement aux travailleurs sans ancienneté INPS au 31/12/1995 \
        (regime contributivo puro). Les salariés pré-1996 cotisent sur la totalité du salaire. \
        \n\n\
        Âges de retraite 2025 : retraite de vieillesse à 67 ans + 20 ans de cotisations ; \
        retraite anticipata à 42 ans et 10 mois de cotisations (hommes) ou 41 ans 10 mois (femmes).")
        .replace("{annee}", &ctx.date_paie.year().to_string())
        .replace("{massimale}", &format!("{:.2}", massimale))
        .replace("{annuel}", &format!("{:.2}", massimale * dec!(12)));
    LigneCotisation {
        code:        "IT_IVS".into(),
        libelle:     ctx.libelle("IT_IVS", "IVS — Invalidità, Vecchiaia, Superstiti"),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Previdenza sociale".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("L. 335/1995 — L. 153/1969 art. 12 — Circ. INPS annuali")),
    }
}

// ── NASpI — Nuova Assicurazione Sociale per l'Impiego ────────────────────────

pub fn it_naspi(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("IT_NASPI");
    LigneCotisation {
        code:        "IT_NASPI".into(),
        libelle:     ctx.libelle("IT_NASPI", "NASpI — Assicurazione chômage (cotisation ordinaire)"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Disoccupazione".into(),
        explication: ctx.expl("IT_NASPI",
            "La NASpI (Nuova Assicurazione Sociale per l'Impiego), instituée par \
            le D.Lgs. 22/2015 (Jobs Act), remplace l'ASpI depuis le 1er mai 2015. \
            Elle indemnise les salariés licenciés avec une allocation proportionnelle \
            au salaire moyen des 4 dernières années. \
            Durée d'indemnisation = moitié des semaines cotisées dans les 4 dernières années \
            (maximum 24 mois). Le montant est de 75 % du salaire moyen pour la part ≤ 1 310 € \
            et 25 % de la différence au-delà, avec un plafond mensuel (environ 1 550 €/mois en 2025). \
            L'allocation se réduit de 3 % par mois à partir du 6e mois. \
            \n\n\
            La cotisation salarié (0,30 %) a été supprimée par la L. 228/2012 à compter du \
            1er janvier 2013 : seule subsiste la cotisation ordinaire patronale de 1,61 %. \
            Aucun plafond n'est appliqué — assiette = rémunération brute totale."),
        loi_ref: Some(ctx.loi_ref("D.Lgs. 22/2015 — L. 92/2012 — L. 228/2012")),
    }
}

// ── NASpI CDD — Contributo addizionale contrats à durée déterminée ────────────

pub fn it_naspi_termine(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("IT_NASPI_TERMINE");
    LigneCotisation {
        code:        "IT_NASPI_TERMINE".into(),
        libelle:     ctx.libelle("IT_NASPI_TERMINE", "NASpI — Contributo addizionale CDD (+1,40 % pat.)"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Disoccupazione".into(),
        explication: ctx.expl("IT_NASPI_TERMINE",
            "Majoration patronale de 1,40 % applicable uniquement aux contrats \
            à durée déterminée (contratti a tempo determinato). \
            Instituée par la L. 92/2012 art. 2 c. 28-29 pour décourager le recours excessif \
            aux CDD et financer les prestations chômage plus fréquemment versées aux salariés \
            en contrats courts. \
            \n\n\
            Remboursée en cas de transformation du CDD en CDI dans les 6 mois \
            suivant la fin du contrat. \
            Non applicable aux CDD de remplacement (maladie, maternité), aux travailleurs \
            saisonniers (secteurs codifiés CCNL), aux apprentis, aux salariés en \
            contratto di apprendistato, ni aux intermittents."),
        loi_ref: Some(ctx.loi_ref("L. 92/2012 art. 2 c. 28-29 — D.Lgs. 81/2015")),
    }
}

// ── Malattia ─────────────────────────────────────────────────────────────────

pub fn it_malattia(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("IT_MALATTIA");
    LigneCotisation {
        code:        "IT_MALATTIA".into(),
        libelle:     ctx.libelle("IT_MALATTIA", "Malattia — Indemnités journalières (INPS)"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Previdenza sociale".into(),
        explication: ctx.expl("IT_MALATTIA",
            "La cotisation malattia finance le versement des indemnités journalières \
            (indennità di malattia) par l'INPS à partir du 4e jour d'arrêt de travail. \
            \n\n\
            Les 3 premiers jours (periodo di carenza) sont à la charge de l'employeur ou \
            pris en charge selon les dispositions du CCNL applicable \
            (certains CCNL suppriment la carenza). \
            L'indemnité INPS est de 50 % du salaire journalier du 4e au 20e jour, \
            66,66 % du 21e au 180e jour. Au-delà, selon les dispositions spécifiques. \
            \n\n\
            Le taux affiché (2,22 %) est indicatif pour le secteur commercio/industria. \
            Le taux réel dépend du CCNL applicable et peut être intégré dans \
            l'aliquota contributiva globale transmise à l'INPS."),
        loi_ref: Some(ctx.loi_ref("L. 153/1969 — DPR 1026/1976 — CCNL applicable")),
    }
}

// ── Maternità e Paternità ────────────────────────────────────────────────────

pub fn it_maternita(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("IT_MATERNITA");
    LigneCotisation {
        code:        "IT_MATERNITA".into(),
        libelle:     ctx.libelle("IT_MATERNITA", "Maternità / Paternità — Congés parentaux (INPS)"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Previdenza sociale".into(),
        explication: ctx.expl("IT_MATERNITA",
            "Finance les congés parentaux obligatoires versés par l'INPS : \
            • Congé maternité (congedo di maternità) : 5 mois, 80 % du salaire. \
            • Congé paternité obligatoire : 10 jours calendaires, 80 % du salaire \
              (étendu à 10 jours par la L. 160/2019 et confirmé depuis). \
            • Congé parental (congedo parentale) : jusqu'à 6 mois par parent, \
              indemnité progressivement étendue (L. 207/2024 : 80 % pour le 1er mois, \
              60 % pour le 2e mois). \
            Cotisation entièrement patronale (0,46 %), stable depuis plusieurs décennies."),
        loi_ref: Some(ctx.loi_ref("D.Lgs. 151/2001 (TU maternità) — L. 160/2019 — L. 207/2024")),
    }
}

// ── Fondo di Garanzia TFR ────────────────────────────────────────────────────

pub fn it_fondo_garanzia(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("IT_FONDO_GARANZIA");
    LigneCotisation {
        code:        "IT_FONDO_GARANZIA".into(),
        libelle:     ctx.libelle("IT_FONDO_GARANZIA", "Fondo di Garanzia TFR — INPS (L. 297/1982)"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Previdenza sociale".into(),
        explication: ctx.expl("IT_FONDO_GARANZIA",
            "Le Fondo di Garanzia, géré par l'INPS, garantit le paiement du TFR \
            (Trattamento Fine Rapporto) aux salariés dont l'employeur est insolvable \
            (faillite, concordato preventivo, liquidazione coatta). \
            Institué par la Legge 297/1982 art. 2. \
            Cotisation : 0,20 % exclusivement patronal, versée mensuellement à l'INPS \
            via F24 (code tributo GFFT). \
            Stable depuis sa création. \
            Distinct du versement direct du TFR au Fondo Tesoreria INPS, obligatoire pour \
            les aziende > 50 salariés depuis le 01/01/2007 (L. 296/2006 art. 1 c. 755)."),
        loi_ref: Some(ctx.loi_ref("L. 297/1982 art. 2 — L. 296/2006 art. 1 c. 755")),
    }
}

// ── INAIL ─────────────────────────────────────────────────────────────────────

pub fn it_inail(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("IT_INAIL");
    let explication = ctx.expl("IT_INAIL",
        "L'assurance accidents du travail et maladies professionnelles, gérée par l'INAIL, \
        est obligatoire pour tous les employeurs (DPR 1124/1965 — Testo Unico INAIL). \
        Entièrement à la charge de l'employeur. \
        \n\n\
        Le taux ({taux} % affiché) est indicatif pour un emploi de bureau / terziario \
        (voce di tariffa ATECO 63.11 ou 70.22). Le taux réel de l'entreprise est fixé \
        par l'INAIL selon : \
          • La voce di tariffa (classification ATECO de l'établissement) \
          • L'historique de sinistralité (oscillazione del tasso : ±28 %) \
          • Les mesures de prévention mises en œuvre (bonus malus) \
        \n\n\
        Secteurs à risque élevé : BTP (2–6 %), chimie (1–4 %), métallurgie (1–3 %). \
        Auto-liquidazione annuelle au 16 février : l'employeur déclare et paie \
        sur les rémunérations effectives de l'année écoulée.")
        .replace("{taux}", &format!("{:.2}", tp * dec!(100)));
    LigneCotisation {
        code:        "IT_INAIL".into(),
        libelle:     ctx.libelle("IT_INAIL", "INAIL — Assicurazione Infortuni e Malattie Professionali"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Assicurazione infortuni".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("DPR 1124/1965 — DPR 1124/1965 art. 268 — DM tariffe INAIL")),
    }
}

// ── TFR — Accrual mensuel ────────────────────────────────────────────────────

pub fn it_tfr(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("IT_TFR");
    let montant = (brut * tp).round_dp(2);
    let explication = ctx.expl("IT_TFR",
        "Le TFR (Trattamento Fine Rapporto) est une forme de rémunération différée, \
        instituée par la Legge 297/1982. \
        Chaque mois, l'employeur constitue une provision de {montant} € \
        (6,91 % = 1/13,5 de la rémunération annuelle brute). \
        \n\n\
        Destination du TFR selon la taille de l'entreprise : \
        • Aziende ≤ 50 sal. : reste chez l'employeur (accrual comptable). \
        • Aziende > 50 sal. (obligatoire depuis 01/01/2007) : choix du salarié entre \
          (a) Fondo Tesoreria INPS (versement mensuel via F24, code TRFS) \
          (b) Fonds de pension complémentaire. \
        \n\n\
        Paiement : à la fin du contrat (licenciement, démission, retraite). \
        Revalorisation annuelle : 75 % ISTAT + 1,5 % fixe. \
        Fiscalité : tassazione separata (taux IRPEF moyen des 5 dernières années, \
        inférieur au taux marginal courant).")
        .replace("{montant}", &format!("{:.2}", montant));
    LigneCotisation {
        code:        "IT_TFR".into(),
        libelle:     ctx.libelle("IT_TFR", "TFR — Trattamento Fine Rapporto (accrual mensuel)"),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: montant,
        categorie:   "Fine rapporto".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("L. 297/1982 — L. 296/2006 art. 1 c. 755 — DPR 917/1986 art. 17")),
    }
}

// ── Esonero contributivo (taglio cuneo) 2022–2023 ────────────────────────────
//
// Retourne None hors période ou si le revenu dépasse les seuils.
// Le montant est NÉGATIF : réduit les cotisations salariales (augmente le net).
pub fn esonero_contributivo(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let annee = ctx.date_paie.year();
    let mois  = ctx.date_paie.month();

    // Estimation revenu annuel (brut × 12 — simplifié, exclut 13e mois)
    let reddito_annuo = brut * dec!(12);

    if annee == 2022 && mois >= 7 {
        // DL 115/2022 : −0,8 % si reddito ≤ 35 000 €
        if reddito_annuo > dec!(35000) {
            return None;
        }
        let taux   = dec!(0.0080);
        let montant = -(brut * taux).round_dp(2);
        return Some(LigneCotisation {
            code:        "IT_ESONERO_2022".into(),
            libelle:     ctx.libelle("IT_ESONERO_2022", "Esonero contributivo H2 2022 (−0,80 % IVS)"),
            base:        brut,
            taux_sal:    -taux,
            montant_sal: montant,
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "Allegement".into(),
            explication: ctx.expl("IT_ESONERO_2022",
                "Réduction temporaire de la cotisation IVS salarié de 0,80 point \
                de pourcentage, applicable de juillet à décembre 2022 pour les salariés \
                dont le reddito annuel estimé n'excède pas 35 000 €. \
                DL 115/2022 art. 20 (Decreto Aiuti-bis), conv. L. 142/2022. \
                Le montant négatif augmente le net à payer du salarié."),
            loi_ref: Some(ctx.loi_ref("DL 115/2022 art. 20 — L. 142/2022")),
        });
    }

    if annee == 2023 {
        let taux = if reddito_annuo <= dec!(25000) {
            dec!(0.03)
        } else if reddito_annuo <= dec!(35000) {
            dec!(0.02)
        } else {
            return None;
        };
        let taux_pp    = taux * dec!(100);
        let taux_pp_s  = format!("{:.0}", taux_pp);
        let montant = -(brut * taux).round_dp(2);
        return Some(LigneCotisation {
            code:        "IT_ESONERO_2023".into(),
            libelle:     ctx.libelle("IT_ESONERO_2023", "Esonero contributivo 2023 (−{taux_pp} % IVS)")
                            .replace("{taux_pp}", &taux_pp_s),
            base:        brut,
            taux_sal:    -taux,
            montant_sal: montant,
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "Allegement".into(),
            explication: ctx.expl("IT_ESONERO_2023",
                "Réduction de {taux_pp} points de pourcentage sur la cotisation IVS salarié. \
                Applicable en 2023 selon le reddito annuel estimé (brut × 12) : \
                −3 pp si reddito ≤ 25 000 € ; −2 pp si reddito 25 001–35 000 €. \
                Ici reddito estimé : {reddito} €/an → taux appliqué : {taux_pp} pp. \
                Legge 197/2022 art. 1 c. 281-286 (Legge di Bilancio 2023).")
                .replace("{taux_pp}", &taux_pp_s)
                .replace("{reddito}", &format!("{:.0}", reddito_annuo)),
            loi_ref: Some(ctx.loi_ref("L. 197/2022 art. 1 c. 281-286")),
        });
    }

    if annee == 2024 {
        let taux = if reddito_annuo <= dec!(25000) {
            dec!(0.07)
        } else if reddito_annuo <= dec!(35000) {
            dec!(0.06)
        } else {
            return None;
        };
        let taux_pp   = taux * dec!(100);
        let taux_pp_s = format!("{:.0}", taux_pp);
        let montant = -(brut * taux).round_dp(2);
        return Some(LigneCotisation {
            code:        "IT_ESONERO_2024".into(),
            libelle:     ctx.libelle("IT_ESONERO_2024", "Esonero contributivo 2024 (−{taux_pp} % IVS)")
                            .replace("{taux_pp}", &taux_pp_s),
            base:        brut,
            taux_sal:    -taux,
            montant_sal: montant,
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "Allegement".into(),
            explication: ctx.expl("IT_ESONERO_2024",
                "Réduction de {taux_pp} points de pourcentage sur la cotisation IVS salarié. \
                Applicable en 2024 selon le reddito annuel estimé (brut × 12) : \
                −7 pp si reddito ≤ 25 000 € ; −6 pp si reddito 25 001–35 000 €. \
                Ici reddito estimé : {reddito} €/an → taux appliqué : {taux_pp} pp. \
                L.213/2023 art. 1 cc. 15-17 (Legge di Bilancio 2024).")
                .replace("{taux_pp}", &taux_pp_s)
                .replace("{reddito}", &format!("{:.0}", reddito_annuo)),
            loi_ref: Some(ctx.loi_ref("L. 213/2023 art. 1 cc. 15-17")),
        });
    }

    None
}
