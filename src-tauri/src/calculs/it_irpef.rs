use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Barème IRPEF annuel ───────────────────────────────────────────────────────
//
// Calcule l'IRPEF annuelle brute sur un revenu imposable annuel.
// Source : DPR 917/1986 art. 11, modifié par chaque Legge di Bilancio.
pub fn irpef_annuel(revenu: Decimal, annee: i32) -> Decimal {
    if annee <= 2021 {
        // 5 tranches (DPR 917/1986)
        if revenu <= dec!(15000) {
            revenu * dec!(0.23)
        } else if revenu <= dec!(28000) {
            dec!(3450) + (revenu - dec!(15000)) * dec!(0.27)
        } else if revenu <= dec!(55000) {
            dec!(6960) + (revenu - dec!(28000)) * dec!(0.38)
        } else if revenu <= dec!(75000) {
            dec!(17220) + (revenu - dec!(55000)) * dec!(0.41)
        } else {
            dec!(25420) + (revenu - dec!(75000)) * dec!(0.43)
        }
    } else if annee <= 2023 {
        // 4 tranches (L. 234/2021 — Bilancio 2022)
        if revenu <= dec!(15000) {
            revenu * dec!(0.23)
        } else if revenu <= dec!(28000) {
            dec!(3450) + (revenu - dec!(15000)) * dec!(0.25)
        } else if revenu <= dec!(50000) {
            dec!(6700) + (revenu - dec!(28000)) * dec!(0.35)
        } else {
            dec!(14400) + (revenu - dec!(50000)) * dec!(0.43)
        }
    } else {
        // 3 tranches (L. 213/2023 — Bilancio 2024)
        if revenu <= dec!(28000) {
            revenu * dec!(0.23)
        } else if revenu <= dec!(50000) {
            dec!(6440) + (revenu - dec!(28000)) * dec!(0.35)
        } else {
            dec!(14140) + (revenu - dec!(50000)) * dec!(0.43)
        }
    }
}

// ── Détractions travail salarié (art. 13 TUIR) ───────────────────────────────
//
// Réduit l'IRPEF payable. Inversement proportionnel au revenu.
// Calcul sur revenu annuel estimé.
pub fn detrazione_lavdip(revenu: Decimal, annee: i32) -> Decimal {
    if annee <= 2021 {
        // Art. 13 TUIR original
        if revenu <= dec!(8000) {
            dec!(1880)
        } else if revenu <= dec!(28000) {
            dec!(902) + dec!(978) * (dec!(28000) - revenu) / dec!(20000)
        } else if revenu <= dec!(55000) {
            dec!(978) * (dec!(55000) - revenu) / dec!(27000)
        } else {
            Decimal::ZERO
        }
    } else if annee <= 2023 {
        // L. 234/2021 — relèvement seuils + montant plat 15–28 k
        if revenu <= dec!(15000) {
            dec!(1880)
        } else if revenu <= dec!(28000) {
            dec!(1910) // montant plat (allègement classe moyenne)
        } else if revenu <= dec!(50000) {
            dec!(1910) * (dec!(50000) - revenu) / dec!(22000)
        } else {
            Decimal::ZERO
        }
    } else {
        // L. 213/2023 — 1ère tranche portée à 1 955 €
        if revenu <= dec!(15000) {
            dec!(1955)
        } else if revenu <= dec!(28000) {
            dec!(1910)
        } else if revenu <= dec!(50000) {
            dec!(1910) * (dec!(50000) - revenu) / dec!(22000)
        } else {
            Decimal::ZERO
        }
    }
}

// ── Bonus cuneo (trattamento integrativo) 2024–2025 ──────────────────────────
//
// Converti depuis l'esonero contributivo 2022–2023 en avantage IRPEF.
// Montant annuel → divisé par 12 pour le mensuel.
// Retourne None hors période concernée ou si revenu > seuil.
pub fn bonus_cuneo_mensuel(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let annee          = ctx.date_paie.year();
    let reddito_annuo  = brut * dec!(12);

    let bonus_annuel = if annee == 2024 {
        // L. 213/2023 : 1 200 €/an si reddito ≤ 35 000 €
        if reddito_annuo <= dec!(35000) {
            dec!(1200)
        } else {
            return None;
        }
    } else if annee >= 2025 {
        // L. 207/2024 : deux formules
        if reddito_annuo <= dec!(20000) {
            // Bonus = 7,1 % × reddito, plafonné à 1 400 €
            (reddito_annuo * dec!(0.071)).min(dec!(1400))
        } else if reddito_annuo <= dec!(40000) {
            dec!(1000) // detrazione fissa
        } else {
            return None;
        }
    } else {
        return None;
    };

    let bonus_mensuel = (bonus_annuel / dec!(12)).round_dp(2);

    let desc = if annee == 2024 {
        ctx.expl("IT_BONUS_CUNEO_DESC_2024",
            "L. 213/2023 : bonus 1 200 €/an pour reddito ≤ 35 000 €.")
    } else {
        ctx.expl("IT_BONUS_CUNEO_DESC_2025",
            "L. 207/2024 : bonus 7,1 % × reddito (max 1 400 €) si reddito ≤ 20 000 € ; \
            detrazione fissa 1 000 € si reddito 20 001–40 000 €.")
    };
    let explication = ctx.expl("IT_BONUS_CUNEO",
        "Avantage fiscal mensuel versé par l'employeur (sostituto d'imposta) \
        au titre du taglio del cuneo fiscale. \
        \n\n\
        {annee} : {desc} \
        Reddito annuel estimé : {reddito} € → Bonus annuel : {bonus_a} € \
        → Bonus mensuel : {bonus_m} €. \
        \n\n\
        Ce bonus est versé directement sur la paie mensuelle par l'employeur \
        (avance sur crédit d'impôt) et régularisé lors de la déclaration annuelle \
        730 ou modèle Redditi. \
        Il est «recuperable» par l'État si le reddito réel dépasse les seuils.")
        .replace("{annee}", &annee.to_string())
        .replace("{desc}", &desc)
        .replace("{reddito}", &format!("{:.0}", reddito_annuo))
        .replace("{bonus_a}", &format!("{:.2}", bonus_annuel))
        .replace("{bonus_m}", &format!("{:.2}", bonus_mensuel));

    Some(LigneCotisation {
        code:        "IT_BONUS_CUNEO".into(),
        libelle:     ctx.libelle("IT_BONUS_CUNEO", "Bonus cuneo fiscale {annee} (trattamento integrativo)")
                        .replace("{annee}", &annee.to_string()),
        base:        brut,
        taux_sal:    -bonus_mensuel / brut, // taux effectif indicatif
        montant_sal: -bonus_mensuel,        // négatif = réduit les retenues (augmente le net)
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Bonus IRPEF".into(),
        explication,
        loi_ref: Some(ctx.loi_ref(if annee == 2024 {
            "L. 213/2023 art. 1 c. 2-9 (Bilancio 2024)"
        } else {
            "L. 207/2024 art. 1 c. 4-9 (Bilancio 2025)"
        })),
    })
}

// ── IRPEF mensuelle (retenue à la source) ────────────────────────────────────
//
// Méthode : estimation du revenu annuel = brut × 12, calcul de l'IRPEF annuelle,
// soustraction de la détraction travail salarié, division par 12.
// Cette méthode suit la pratique des sostituti d'imposta italiens pour les paies mensuelles.
pub fn irpef_mensuel(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee        = ctx.date_paie.year();
    let reddito_ann  = brut * dec!(12);

    let irpef_brute  = irpef_annuel(reddito_ann, annee);
    let detrazione   = detrazione_lavdip(reddito_ann, annee).max(Decimal::ZERO);
    let irpef_nette  = (irpef_brute - detrazione).max(Decimal::ZERO);
    let irpef_mensuelle = (irpef_nette / dec!(12)).round_dp(2);

    let taux_effectif = if brut > Decimal::ZERO {
        (irpef_mensuelle / brut).round_dp(4)
    } else {
        Decimal::ZERO
    };

    let nb_tranches = if annee <= 2021 { 5 } else if annee <= 2023 { 4 } else { 3 };
    let explication = ctx.expl("IT_IRPEF",
        "L'IRPEF (Imposta sul Reddito delle Persone Fisiche) est l'impôt progressif \
        sur le revenu des personnes physiques, régi par le TUIR (DPR 917/1986). \
        L'employeur (sostituto d'imposta, art. 23 TUIR) est tenu d'effectuer \
        une retenue mensuelle estimative sur le salaire, régularisée en décembre \
        ou lors de la présentation du modèle 730/Redditi. \
        \n\n\
        [ Calcul {annee} — barème {nb_tranches} tranches ]\n\
        Revenu annuel estimé  : {reddito} €\n\
        IRPEF brute annuelle  : {irpef_b} €\n\
        Détraction lavoro dip.: − {det} €\n\
        IRPEF nette annuelle  : {irpef_n} €\n\
        IRPEF mensuelle       : {irpef_m} € (÷ 12)\n\
        Taux effectif         : {teff} %\n\
        \n\
        Les addizionali régionale et communale sont calculées séparément \
        et retenues en cours d'année (généralement à partir de mars de l'année suivante \
        via modèle 730 ou retenue complémentaire de novembre–décembre).")
        .replace("{annee}", &annee.to_string())
        .replace("{nb_tranches}", &nb_tranches.to_string())
        .replace("{reddito}", &format!("{:.2}", reddito_ann))
        .replace("{irpef_b}", &format!("{:.2}", irpef_brute))
        .replace("{det}", &format!("{:.2}", detrazione))
        .replace("{irpef_n}", &format!("{:.2}", irpef_nette))
        .replace("{irpef_m}", &format!("{:.2}", irpef_mensuelle))
        .replace("{teff}", &format!("{:.2}", taux_effectif * dec!(100)));
    LigneCotisation {
        code:        "IT_IRPEF".into(),
        libelle:     ctx.libelle("IT_IRPEF", "IRPEF — Retenue à la source {annee}")
                        .replace("{annee}", &annee.to_string()),
        base:        brut,
        taux_sal:    taux_effectif,
        montant_sal: irpef_mensuelle,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Imposta".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("DPR 917/1986 art. 11 et 23 (TUIR) — L. 213/2023 (Bilancio 2024)")),
    }
}

// ── Addizionale regionale ────────────────────────────────────────────────────
//
// Taux de base 2024 par code région (voir migration 0017_it_plafonds_irpef.sql).
// Retourne None si la région n'est pas reconnue.
pub fn addizionale_regionale(brut: Decimal, regione: &str, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let taux: Decimal = match regione {
        "AB" => dec!(0.0173),
        "BS" => dec!(0.0090),
        "BZ" => dec!(0.0090),
        "CL" => dec!(0.0203),
        "CP" => dec!(0.0203),
        "ER" => dec!(0.0090),
        "FV" => dec!(0.0070),
        "LA" => dec!(0.0173),
        "LG" => dec!(0.0090),
        "LO" => dec!(0.0123),
        "MR" => dec!(0.0153),
        "ML" => dec!(0.0203),
        "PM" => dec!(0.0095),
        "PG" => dec!(0.0197),
        "SR" => dec!(0.0049),
        "SC" => dec!(0.0100),
        "TS" => dec!(0.0142),
        "TN" => dec!(0.0060),
        "UM" => dec!(0.0135),
        "VA" => dec!(0.0090),
        "VE" => dec!(0.0090),
        _    => return None,
    };

    // L'addizionale est calculée sur le revenu annuel et retenue en 11 mensualités
    // (de mars à novembre de l'année N+1 via modèle 730, ou en novembre-décembre
    // via retenue complémentaire de l'employeur). Ici simplifiée en mensuel direct.
    let reddito_ann     = brut * dec!(12);
    let addiz_annuelle  = (reddito_ann * taux).round_dp(2);
    let addiz_mensuelle = (addiz_annuelle / dec!(12)).round_dp(2);

    let libelle_region = nom_region(regione);

    let explication = ctx.expl("IT_ADD_REG",
        "L'addizionale regionale IRPEF est un impôt prélevé par la Région \
        en sus de l'IRPEF nationale (art. 50 D.Lgs. 446/1997). \
        Chaque région fixe son taux annuellement par délibération. \
        \n\n\
        Région : {libelle} (code {code})\n\
        Taux de base 2024 : {taux_pct} %\n\
        Reddito annuel estimé : {reddito} €\n\
        Addizionale annuelle : {addiz_a} €\n\
        Mensualité indicative : {addiz_m} €\n\
        \n\
        Note : en pratique, l'addizionale régionale est retenue par l'employeur \
        en 11 mensualités de mars à novembre de l'année N+1 \
        (après déclaration 730), ou déduite sur les bulletins de novembre–décembre \
        par solde si l'employeur procède à l'assistenza fiscale. \
        Certaines régions ont des tranches supplémentaires pour hauts revenus \
        non reproduites ici.")
        .replace("{libelle}", libelle_region)
        .replace("{code}", regione)
        .replace("{taux_pct}", &format!("{:.2}", taux * dec!(100)))
        .replace("{reddito}", &format!("{:.2}", reddito_ann))
        .replace("{addiz_a}", &format!("{:.2}", addiz_annuelle))
        .replace("{addiz_m}", &format!("{:.2}", addiz_mensuelle));

    Some(LigneCotisation {
        code:        format!("IT_ADD_REG_{regione}"),
        libelle:     ctx.libelle("IT_ADD_REG", "Addizionale regionale IRPEF — {libelle_region}")
                        .replace("{libelle_region}", libelle_region),
        base:        brut,
        taux_sal:    taux,
        montant_sal: addiz_mensuelle,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Imposta regionale".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("D.Lgs. 446/1997 art. 50 — Delibere regionali annuali")),
    })
}

fn nom_region(code: &str) -> &'static str {
    match code {
        "AB" => "Abruzzo",
        "BS" => "Basilicata",
        "BZ" => "Bolzano / Südtirol",
        "CL" => "Calabria",
        "CP" => "Campania",
        "ER" => "Emilia-Romagna",
        "FV" => "Friuli-Venezia Giulia",
        "LA" => "Lazio",
        "LG" => "Liguria",
        "LO" => "Lombardia",
        "MR" => "Marche",
        "ML" => "Molise",
        "PM" => "Piemonte",
        "PG" => "Puglia",
        "SR" => "Sardegna",
        "SC" => "Sicilia",
        "TS" => "Toscana",
        "TN" => "Trento",
        "UM" => "Umbria",
        "VA" => "Valle d'Aosta",
        "VE" => "Veneto",
        _    => "Région inconnue",
    }
}
