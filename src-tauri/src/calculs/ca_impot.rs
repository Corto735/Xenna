use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Impôt fédéral ─────────────────────────────────────────────────────────────

fn impot_fed_annuel(revenu: Decimal, annee: i32) -> Decimal {
    if annee <= 2019 {
        if revenu <= dec!(47630) { revenu * dec!(0.15) }
        else if revenu <= dec!(95259) { dec!(7144.50) + (revenu - dec!(47630)) * dec!(0.205) }
        else if revenu <= dec!(147667) { dec!(16904.45) + (revenu - dec!(95259)) * dec!(0.26) }
        else if revenu <= dec!(210371) { dec!(30531.53) + (revenu - dec!(147667)) * dec!(0.29) }
        else { dec!(48715.69) + (revenu - dec!(210371)) * dec!(0.33) }
    } else if annee == 2020 {
        if revenu <= dec!(48535) { revenu * dec!(0.15) }
        else if revenu <= dec!(97069) { dec!(7280.25) + (revenu - dec!(48535)) * dec!(0.205) }
        else if revenu <= dec!(150473) { dec!(17229.72) + (revenu - dec!(97069)) * dec!(0.26) }
        else if revenu <= dec!(214368) { dec!(31115.28) + (revenu - dec!(150473)) * dec!(0.29) }
        else { dec!(49644.83) + (revenu - dec!(214368)) * dec!(0.33) }
    } else if annee == 2021 {
        if revenu <= dec!(49020) { revenu * dec!(0.15) }
        else if revenu <= dec!(98040) { dec!(7353.00) + (revenu - dec!(49020)) * dec!(0.205) }
        else if revenu <= dec!(151978) { dec!(17401.10) + (revenu - dec!(98040)) * dec!(0.26) }
        else if revenu <= dec!(216511) { dec!(31425.98) + (revenu - dec!(151978)) * dec!(0.29) }
        else { dec!(50139.55) + (revenu - dec!(216511)) * dec!(0.33) }
    } else if annee == 2022 {
        if revenu <= dec!(50197) { revenu * dec!(0.15) }
        else if revenu <= dec!(100392) { dec!(7529.55) + (revenu - dec!(50197)) * dec!(0.205) }
        else if revenu <= dec!(155625) { dec!(17829.53) + (revenu - dec!(100392)) * dec!(0.26) }
        else if revenu <= dec!(221708) { dec!(32190.25) + (revenu - dec!(155625)) * dec!(0.29) }
        else { dec!(51352.32) + (revenu - dec!(221708)) * dec!(0.33) }
    } else if annee == 2023 {
        if revenu <= dec!(53359) { revenu * dec!(0.15) }
        else if revenu <= dec!(106717) { dec!(8003.85) + (revenu - dec!(53359)) * dec!(0.205) }
        else if revenu <= dec!(165430) { dec!(18942.24) + (revenu - dec!(106717)) * dec!(0.26) }
        else if revenu <= dec!(235675) { dec!(34207.26) + (revenu - dec!(165430)) * dec!(0.29) }
        else { dec!(54581.01) + (revenu - dec!(235675)) * dec!(0.33) }
    } else {
        // 2024+
        if revenu <= dec!(55867) { revenu * dec!(0.15) }
        else if revenu <= dec!(111733) { dec!(8380.05) + (revenu - dec!(55867)) * dec!(0.205) }
        else if revenu <= dec!(154906) { dec!(19832.58) + (revenu - dec!(111733)) * dec!(0.26) }
        else if revenu <= dec!(220000) { dec!(31057.56) + (revenu - dec!(154906)) * dec!(0.29) }
        else { dec!(49934.82) + (revenu - dec!(220000)) * dec!(0.33) }
    }
}

fn bpa_credit_fed(annee: i32) -> Decimal {
    // Montant personnel de base (MPB) × 15 % (taux de base fédéral)
    let bpa = match annee {
        i32::MIN..=2019 => dec!(12069),
        2020            => dec!(13229),
        2021            => dec!(13808),
        2022            => dec!(14398),
        2023            => dec!(15000),
        2024            => dec!(15705),
        _               => dec!(16129), // 2025+ estimation
    };
    (bpa * dec!(0.15)).round_dp(2)
}

pub fn ca_impot_federal(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee        = ctx.date_paie.year();
    let revenu_ann   = brut * dec!(12);
    let impot_brut   = impot_fed_annuel(revenu_ann, annee);
    let credit_bpa   = bpa_credit_fed(annee);
    let impot_net    = (impot_brut - credit_bpa).max(Decimal::ZERO);
    let impot_mens   = (impot_net / dec!(12)).round_dp(2);
    let taux_eff     = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code:        "CA_IMPOT_FED".into(),
        libelle:     format!("Impôt fédéral — retenue {annee}"),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: impot_mens,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt fédéral".into(),
        explication: format!(
            "Retenue mensuelle d'impôt fédéral sur le revenu. L'employeur est \
            sostituto d'imposta (retenues à la source — formulaire TD1). \
            \n\n\
            [ Calcul {} — barème fédéral ]\n\
            Revenu annuel estimé    : {:.2} CAD\n\
            Impôt brut annuel       : {:.2} CAD\n\
            Crédit personnel (MPB)  : − {:.2} CAD\n\
            Impôt net annuel        : {:.2} CAD\n\
            Retenue mensuelle       : {:.2} CAD (÷ 12)\n\
            Taux effectif           : {:.2} %\n\
            \n\
            Barème {} : 15/20,5/26/29/33 %. \
            Le Montant personnel de base ({} CAD) génère un crédit de 15 % = {:.2} CAD/an. \
            Régularisation en décembre ou déclaration T1 annuelle.",
            annee,
            revenu_ann, impot_brut, credit_bpa,
            impot_net, impot_mens,
            taux_eff * dec!(100),
            annee,
            match annee {
                i32::MIN..=2019 => "12 069", 2020 => "13 229", 2021 => "13 808",
                2022 => "14 398", 2023 => "15 000", 2024 => "15 705", _ => "16 129"
            },
            credit_bpa,
        ),
        loi_ref: Some("L.R.C. 1985, ch. 1 (5e suppl.), art. 117-117.1 — Formulaire TD1".into()),
    }
}

// ── Impôt provincial Ontario (référence hors Québec) ─────────────────────────

fn impot_on_annuel(revenu: Decimal) -> Decimal {
    // Barème Ontario 2024
    if revenu <= dec!(51446) { revenu * dec!(0.0505) }
    else if revenu <= dec!(102894) { dec!(2598.02) + (revenu - dec!(51446)) * dec!(0.0915) }
    else if revenu <= dec!(150000) { dec!(7305.51) + (revenu - dec!(102894)) * dec!(0.1116) }
    else if revenu <= dec!(220000) { dec!(12562.54) + (revenu - dec!(150000)) * dec!(0.1216) }
    else { dec!(21074.54) + (revenu - dec!(220000)) * dec!(0.1316) }
}

pub fn ca_impot_ontario(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee      = ctx.date_paie.year();
    let revenu_ann = brut * dec!(12);
    // MPB Ontario 2024 : 11 865 CAD × 5,05 % = 599,18 CAD
    let impot_brut = impot_on_annuel(revenu_ann);
    let credit_bpa = dec!(599.18);
    let impot_net  = (impot_brut - credit_bpa).max(Decimal::ZERO);
    let impot_mens = (impot_net / dec!(12)).round_dp(2);
    let taux_eff   = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code:        "ON_IMPOT_PROV".into(),
        libelle:     format!("Impôt provincial Ontario — retenue {annee}"),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: impot_mens,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt provincial".into(),
        explication: format!(
            "Retenue mensuelle d'impôt provincial de l'Ontario (province de référence hors Québec). \
            Barème 2024 : 5,05/9,15/11,16/12,16/13,16 %. \
            MPB Ontario 2024 : 11 865 CAD → crédit de 599,18 CAD/an. \
            \n\n\
            Revenu annuel estimé  : {:.2} CAD\n\
            Impôt brut annuel     : {:.2} CAD\n\
            Crédit MPB            : − 599,18 CAD\n\
            Impôt net annuel      : {:.2} CAD\n\
            Retenue mensuelle     : {:.2} CAD\n\
            Taux effectif         : {:.2} %\n\
            \n\
            Note : non applicable au Québec (province ayant son propre impôt séparé). \
            Les autres provinces (CB, AB, QC excl.) ont leurs propres barèmes — \
            utiliser Ontario comme approximation générale.",
            revenu_ann, impot_brut, impot_net, impot_mens,
            taux_eff * dec!(100),
        ),
        loi_ref: Some("L.O. 2007, ch. 11, ann. A — Formulaire TD1ON".into()),
    }
}

// ── Impôt provincial Québec ───────────────────────────────────────────────────

fn impot_qc_annuel(revenu: Decimal, annee: i32) -> Decimal {
    if annee <= 2023 {
        // Barème 2019-2023 (taux stables, seuils légèrement indexés)
        // Utilisation du barème 2023 pour 2019-2023 (approximation raisonnable)
        if revenu <= dec!(51780) { revenu * dec!(0.14) }
        else if revenu <= dec!(103545) { dec!(7249.20) + (revenu - dec!(51780)) * dec!(0.19) }
        else if revenu <= dec!(126000) { dec!(17084.55) + (revenu - dec!(103545)) * dec!(0.24) }
        else { dec!(22473.75) + (revenu - dec!(126000)) * dec!(0.2575) }
    } else {
        // 2024+
        if revenu <= dec!(51780) { revenu * dec!(0.14) }
        else if revenu <= dec!(103545) { dec!(7249.20) + (revenu - dec!(51780)) * dec!(0.19) }
        else if revenu <= dec!(126000) { dec!(17084.55) + (revenu - dec!(103545)) * dec!(0.24) }
        else { dec!(22473.75) + (revenu - dec!(126000)) * dec!(0.2575) }
    }
}

fn bpa_credit_qc(annee: i32) -> Decimal {
    let bpa = match annee {
        i32::MIN..=2021 => dec!(15270),
        2022            => dec!(16143),
        2023            => dec!(16143),
        2024            => dec!(17183),
        _               => dec!(17600), // 2025+ estimation
    };
    (bpa * dec!(0.14)).round_dp(2)
}

pub fn qc_impot_provincial(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee      = ctx.date_paie.year();
    let revenu_ann = brut * dec!(12);
    let impot_brut = impot_qc_annuel(revenu_ann, annee);
    let credit_bpa = bpa_credit_qc(annee);
    let impot_net  = (impot_brut - credit_bpa).max(Decimal::ZERO);
    let impot_mens = (impot_net / dec!(12)).round_dp(2);
    let taux_eff   = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code:        "QC_IMPOT_PROV".into(),
        libelle:     format!("Impôt provincial Québec — retenue {annee}"),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: impot_mens,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt provincial".into(),
        explication: format!(
            "Le Québec perçoit son propre impôt provincial directement (unique au Canada) \
            via Revenu Québec, contrairement aux autres provinces où l'ARC perçoit \
            les deux impôts conjointement. \
            \n\n\
            Barème {} : 14/19/24/25,75 %. MPB Québec : {} CAD → crédit : {:.2} CAD/an.\n\
            \n\
            [ Calcul ]\n\
            Revenu annuel estimé    : {:.2} CAD\n\
            Impôt brut annuel       : {:.2} CAD\n\
            Crédit MPB              : − {:.2} CAD\n\
            Impôt net annuel        : {:.2} CAD\n\
            Retenue mensuelle       : {:.2} CAD\n\
            Taux effectif           : {:.2} %\n\
            \n\
            L'employeur produit le relevé 1 (RL-1) au lieu du T4. \
            Le salarié québécois produit deux déclarations : T1 (fédéral) + TP-1 (provincial).",
            annee,
            match annee {
                i32::MIN..=2021 => "15 270", 2022 | 2023 => "16 143",
                2024 => "17 183", _ => "17 600"
            },
            credit_bpa,
            revenu_ann, impot_brut, credit_bpa,
            impot_net, impot_mens,
            taux_eff * dec!(100),
        ),
        loi_ref: Some("RLRQ, ch. I-3, art. 750 — Formulaire TP-1015.3 — Relevé 1 (RL-1)".into()),
    }
}

// ── Impôt provincial — toutes provinces/territoires (hors Québec) ─────────────
//
// Taux 2024 (Lois provinciales sur l'impôt sur le revenu).
// Indexés annuellement — valeurs à vérifier chaque année.
// Source : ARC T1 Guide, feuillets T4 et formulaires TD1 provinciaux.

fn impot_prov_brackets(revenu: Decimal, seuils: &[Decimal], taux: &[Decimal]) -> Decimal {
    let mut impot = Decimal::ZERO;
    let mut prev  = Decimal::ZERO;
    for (i, &seuil) in seuils.iter().enumerate() {
        if revenu <= prev { break; }
        let taxable = revenu.min(seuil) - prev;
        impot += taxable * taux[i];
        prev = seuil;
        if revenu <= seuil { break; }
    }
    impot
}

pub fn ca_impot_provincial(brut: Decimal, province: &str, ctx: &ContextPaie) -> LigneCotisation {
    if province == "ON" {
        return ca_impot_ontario(brut, ctx);
    }

    let annee      = ctx.date_paie.year();
    let revenu_ann = brut * dec!(12);

    let (impot_brut, bpa_credit, nom, loi, tranches_desc) = match province {

        "AB" => {
            let s = [dec!(148269), dec!(177922), dec!(237230), dec!(355845), dec!(9999999)];
            let t = [dec!(0.10),   dec!(0.12),   dec!(0.13),   dec!(0.14),   dec!(0.15)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(21003) * dec!(0.10)).round_dp(2),
             "Alberta", "Alberta Personal Income Tax Act, SA 1999 c A-33.5",
             "10/12/13/14/15 %. MPB 21 003 CAD.")
        },
        "BC" => {
            let s = [dec!(45654), dec!(91310), dec!(104835), dec!(127299), dec!(172602), dec!(240716), dec!(9999999)];
            let t = [dec!(0.0506), dec!(0.077), dec!(0.105),  dec!(0.1229), dec!(0.147),  dec!(0.168),  dec!(0.205)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(11981) * dec!(0.0506)).round_dp(2),
             "Colombie-Britannique", "Income Tax Act (B.C.), RSBC 1996 c 215",
             "5,06/7,70/10,50/12,29/14,70/16,80/20,50 %. MPB 11 981 CAD.")
        },
        "MB" => {
            let s = [dec!(36842), dec!(79625), dec!(9999999)];
            let t = [dec!(0.108),  dec!(0.1275), dec!(0.174)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(15780) * dec!(0.108)).round_dp(2),
             "Manitoba", "Income Tax Act (Manitoba), CCSM c I10",
             "10,80/12,75/17,40 %. MPB 15 780 CAD.")
        },
        "NB" => {
            let s = [dec!(49958), dec!(99916), dec!(185064), dec!(9999999)];
            let t = [dec!(0.094),  dec!(0.1482), dec!(0.1652), dec!(0.1784)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(12458) * dec!(0.094)).round_dp(2),
             "Nouveau-Brunswick", "Loi de l'impôt sur le revenu (N.-B.), LRN-B 2000 c I-2.2",
             "9,40/14,82/16,52/17,84 %. MPB 12 458 CAD.")
        },
        "NL" => {
            let s = [dec!(43198), dec!(86395), dec!(154244), dec!(215943), dec!(275870), dec!(551739), dec!(9999999)];
            let t = [dec!(0.087), dec!(0.145), dec!(0.158), dec!(0.178), dec!(0.198), dec!(0.208), dec!(0.213)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(10818) * dec!(0.087)).round_dp(2),
             "Terre-Neuve-et-Labrador", "Income Tax Act, 2000 (N.L.), SNL2000 c I-1.1",
             "8,70/14,50/15,80/17,80/19,80/20,80/21,30 %. MPB 10 818 CAD.")
        },
        "NS" => {
            let s = [dec!(29590), dec!(59180), dec!(93000), dec!(150000), dec!(9999999)];
            let t = [dec!(0.0879), dec!(0.1495), dec!(0.1667), dec!(0.175), dec!(0.21)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(8481) * dec!(0.0879)).round_dp(2),
             "Nouvelle-Écosse", "Income Tax Act (Nova Scotia), RSNS 1989 c 217",
             "8,79/14,95/16,67/17,50/21,00 %. MPB 8 481 CAD.")
        },
        "NT" => {
            let s = [dec!(50597), dec!(101198), dec!(164525), dec!(9999999)];
            let t = [dec!(0.059),  dec!(0.086),  dec!(0.122),  dec!(0.1405)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(16593) * dec!(0.059)).round_dp(2),
             "Territoires du Nord-Ouest", "Income Tax Act (Northwest Territories), RSNWT 1988 c I-3",
             "5,90/8,60/12,20/14,05 %. MPB 16 593 CAD.")
        },
        "NU" => {
            let s = [dec!(53268), dec!(106537), dec!(173205), dec!(9999999)];
            let t = [dec!(0.04),  dec!(0.07),   dec!(0.09),   dec!(0.115)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(17925) * dec!(0.04)).round_dp(2),
             "Nunavut", "Income Tax Act (Nunavut), RSNWT 1988 c I-3 (adapté)",
             "4,00/7,00/9,00/11,50 %. MPB 17 925 CAD. Taux les plus bas au Canada.")
        },
        "PE" => {
            let s = [dec!(32656), dec!(64313), dec!(105000), dec!(140000), dec!(9999999)];
            let t = [dec!(0.0965), dec!(0.1363), dec!(0.1665), dec!(0.18), dec!(0.1875)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(12000) * dec!(0.0965)).round_dp(2),
             "Île-du-Prince-Édouard", "Income Tax Act (P.E.I.), RSPEI 1988 c I-1",
             "9,65/13,63/16,65/18,00/18,75 %. MPB 12 000 CAD.")
        },
        "SK" => {
            let s = [dec!(49720), dec!(142058), dec!(9999999)];
            let t = [dec!(0.105),  dec!(0.125),  dec!(0.145)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(17661) * dec!(0.105)).round_dp(2),
             "Saskatchewan", "The Income Tax Act, 2000 (Saskatchewan), SS 2000 c I-2.01",
             "10,50/12,50/14,50 %. MPB 17 661 CAD.")
        },
        "YT" => {
            let s = [dec!(55867), dec!(111733), dec!(154906), dec!(500000), dec!(9999999)];
            let t = [dec!(0.064), dec!(0.09),  dec!(0.109), dec!(0.128), dec!(0.15)];
            (impot_prov_brackets(revenu_ann, &s, &t),
             (dec!(15705) * dec!(0.064)).round_dp(2),
             "Yukon", "Income Tax Act (Yukon), RSY 2002 c 118",
             "6,40/9,00/10,90/12,80/15,00 %. MPB 15 705 CAD (= fédéral).")
        },
        _ => return ca_impot_ontario(brut, ctx),  // fallback Ontario
    };

    let impot_net  = (impot_brut - bpa_credit).max(Decimal::ZERO);
    let impot_mens = (impot_net / dec!(12)).round_dp(2);
    let taux_eff   = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code:        format!("{province}_IMPOT_PROV"),
        libelle:     format!("Impôt provincial {nom} — retenue {annee}"),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: impot_mens,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt provincial".into(),
        explication: format!(
            "Retenue mensuelle d'impôt provincial — {nom}.\n\
            Barème {annee} : {tranches_desc}\n\
            \n\
            [ Calcul ]\n\
            Revenu annuel estimé  : {revenu:.2} CAD\n\
            Impôt brut annuel     : {ib:.2} CAD\n\
            Crédit MPB            : − {bpa:.2} CAD\n\
            Impôt net annuel      : {inet:.2} CAD\n\
            Retenue mensuelle     : {mens:.2} CAD (÷ 12)\n\
            Taux effectif         : {teff:.2} %\n\
            \n\
            Retenu conjointement avec l'impôt fédéral par l'employeur (sostituto d'imposta). \
            Régularisation via déclaration T1 annuelle (ARC) et, si applicable, \
            déclaration provinciale complémentaire.",
            nom      = nom,
            annee    = annee,
            tranches_desc = tranches_desc,
            revenu   = revenu_ann,
            ib       = impot_brut,
            bpa      = bpa_credit,
            inet     = impot_net,
            mens     = impot_mens,
            teff     = taux_eff * dec!(100),
        ),
        loi_ref: Some(loi.into()),
    }
}
