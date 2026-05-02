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
