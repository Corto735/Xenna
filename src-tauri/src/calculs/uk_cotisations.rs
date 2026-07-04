// ── Cotisations Royaume-Uni — NI Class 1 + Income Tax PAYE ──────────────────
//
// Périmètre : salarié secteur privé anglais, année fiscale 2024/25.
// Taux NI lus depuis ContextPaie (DB). Seuils et barème IT hardcodés par année.
//
// Sources légales :
//   National Insurance Contributions Act 2014
//   Income Tax Act 2007 ; Finance Act 2024

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Seuils annuels → mensuels ─────────────────────────────────────────────────

struct UkSeuils {
    pt:  Decimal, // Primary Threshold mensuel (NI sal commence au-dessus)
    uel: Decimal, // Upper Earnings Limit mensuel (NI sal → 2 % au-dessus)
    st:  Decimal, // Secondary Threshold mensuel (NI pat commence au-dessus)
    pa:  Decimal, // Personal Allowance mensuelle (IT exonérée)
    br_max: Decimal, // Plafond mensuel Basic Rate (20 %)
    hr_max: Decimal, // Plafond mensuel Higher Rate (40 %)
}

fn seuils(annee: i32) -> UkSeuils {
    match annee {
        // 2024/25 — gelés depuis 2021/22 (Finance Act 2024)
        _ => UkSeuils {
            pt:     dec!(12570) / dec!(12), // £1 047,50/mois
            uel:    dec!(50270) / dec!(12), // £4 189,17/mois
            st:     dec!(9100)  / dec!(12), // £758,33/mois
            pa:     dec!(12570) / dec!(12), // £1 047,50/mois
            br_max: dec!(50270) / dec!(12), // £4 189,17/mois
            hr_max: dec!(125140)/ dec!(12), // £10 428,33/mois
        },
    }
}

// ── National Insurance — part salariale ───────────────────────────────────────
//
// Tranche [PT – UEL]  : 8 % (taux lu en DB)
// Tranche au-delà UEL : 2 % (taux secondaire fixe)

pub fn uk_ni_sal(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let s     = seuils(annee);
    let ts    = ctx.taux_sal("UK_NI_SAL"); // 0,08

    let tranche_principale = if brut > s.pt {
        (brut.min(s.uel) - s.pt).max(Decimal::ZERO)
    } else {
        Decimal::ZERO
    };
    let tranche_haute = if brut > s.uel {
        (brut - s.uel).max(Decimal::ZERO)
    } else {
        Decimal::ZERO
    };

    let montant = (tranche_principale * ts + tranche_haute * dec!(0.02)).round_dp(2);
    let taux_eff = if brut > Decimal::ZERO { (montant / brut).round_dp(4) } else { Decimal::ZERO };

    let fy = format!("{annee}/{}", annee + 1 - 2000);
    LigneCotisation {
        code:        "UK_NI_SAL".into(),
        libelle:     ctx.libelle("UK_NI_SAL", "National Insurance Class 1 — salarié {fy}")
            .replace("{fy}", &fy),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: montant,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Sécurité sociale".into(),
        explication: ctx.expl("UK_NI_SAL",
            "National Insurance Class 1 — part salariale.\n\n\
            Tranche [PT – UEL] ({ts_pct} %) : £{pt} – £{uel}/mois\n\
            → base {tp} × {ts_pct} % = £{m1}\n\
            Tranche haute (> UEL, 2 %) : £{uel}/mois\n\
            → base {th} × 2 % = £{m2}\n\n\
            Total NI salarié : £{tot}\n\
            Taux effectif : {teff} %\n\n\
            Base légale : NIA 2014 ; Finance Act 2024.")
            .replace("{ts_pct}", &format!("{:.0}", ts * dec!(100)))
            .replace("{pt}",  &format!("{:.2}", s.pt))
            .replace("{uel}", &format!("{:.2}", s.uel))
            .replace("{tp}", &format!("{:.2}", tranche_principale))
            .replace("{th}",  &format!("{:.2}", tranche_haute))
            .replace("{m1}",  &format!("{:.2}", (tranche_principale * ts).round_dp(2)))
            .replace("{m2}",  &format!("{:.2}", (tranche_haute * dec!(0.02)).round_dp(2)))
            .replace("{tot}", &format!("{:.2}", montant))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("National Insurance Contributions Act 2014 — Finance Act 2024")),
    }
}

// ── National Insurance — part patronale ───────────────────────────────────────
//
// 13,8 % sur le salaire excédant le Secondary Threshold (ST).
// Pas de plafond côté employeur.

pub fn uk_ni_pat(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = ctx.date_paie.year();
    let s     = seuils(annee);
    let tp    = ctx.taux_pat("UK_NI_PAT"); // 0,138

    let base    = (brut - s.st).max(Decimal::ZERO);
    let montant = (base * tp).round_dp(2);
    let taux_eff = if brut > Decimal::ZERO { (montant / brut).round_dp(4) } else { Decimal::ZERO };

    let fy = format!("{annee}/{}", annee + 1 - 2000);
    LigneCotisation {
        code:        "UK_NI_PAT".into(),
        libelle:     ctx.libelle("UK_NI_PAT", "National Insurance Class 1 — employeur {fy}")
            .replace("{fy}", &fy),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    taux_eff,
        montant_pat: montant,
        categorie:   "Sécurité sociale".into(),
        explication: ctx.expl("UK_NI_PAT",
            "National Insurance Class 1 — part employeur.\n\n\
            Taux : {tp_pct} % sur salaire > ST (£{st}/mois)\n\
            Base imposable : £{base} × {tp_pct} % = £{tot}\n\
            Pas de plafond supérieur côté employeur.\n\
            Taux effectif sur salaire brut : {teff} %\n\n\
            Base légale : NIA 2014 ; Finance Act 2024.")
            .replace("{tp_pct}", &format!("{:.1}", tp * dec!(100)))
            .replace("{st}",   &format!("{:.2}", s.st))
            .replace("{base}", &format!("{:.2}", base))
            .replace("{tot}",  &format!("{:.2}", montant))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("National Insurance Contributions Act 2014 — Finance Act 2024")),
    }
}

// ── Income Tax PAYE ───────────────────────────────────────────────────────────
//
// Calcul mensuel proratisé sur base annuelle estimée.
// Personal Allowance : £12 570/an
// Basic  Rate 20 % : jusqu'à £50 270/an
// Higher Rate 40 % : £50 270 – £125 140/an
// Additional Rate 45 % : au-delà de £125 140/an

fn income_tax_annuel(revenu_annuel: Decimal, annee: i32) -> Decimal {
    let s = seuils(annee);
    let pa_annuel = s.pa * dec!(12);
    let br_annuel = s.br_max * dec!(12);
    let hr_annuel = s.hr_max * dec!(12);

    if revenu_annuel <= pa_annuel {
        return Decimal::ZERO;
    }
    let imposable = revenu_annuel - pa_annuel;
    let br_tranche = (br_annuel - pa_annuel).max(Decimal::ZERO);
    let hr_tranche = (hr_annuel - br_annuel).max(Decimal::ZERO);

    let impot_basic = (imposable.min(br_tranche)) * dec!(0.20);
    let impot_higher = if imposable > br_tranche {
        (imposable - br_tranche).min(hr_tranche) * dec!(0.40)
    } else { Decimal::ZERO };
    let impot_additional = if imposable > br_tranche + hr_tranche {
        (imposable - br_tranche - hr_tranche) * dec!(0.45)
    } else { Decimal::ZERO };

    impot_basic + impot_higher + impot_additional
}

pub fn uk_income_tax(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee  = ctx.date_paie.year();
    let s      = seuils(annee);
    let revenu_annuel = brut * dec!(12);

    let impot_annuel   = income_tax_annuel(revenu_annuel, annee);
    let impot_mensuel  = (impot_annuel / dec!(12)).round_dp(2);
    let taux_eff       = if brut > Decimal::ZERO { (impot_mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    let tranche_libelle = if revenu_annuel <= s.pa * dec!(12) {
        ctx.expl("UK_TL_PA", "dans la Personal Allowance (0 %)")
    } else if revenu_annuel <= s.br_max * dec!(12) {
        "Basic Rate (20 %)".into() // nom officiel, identique dans les 6 langues
    } else if revenu_annuel <= s.hr_max * dec!(12) {
        ctx.expl("UK_TL_HIGHER_PARTIAL", "Higher Rate partielle (40 %)")
    } else {
        "Additional Rate (45 %)".into() // nom officiel, identique dans les 6 langues
    };

    let fy = format!("{annee}/{}", annee + 1 - 2000);
    LigneCotisation {
        code:        "UK_INCOME_TAX".into(),
        libelle:     ctx.libelle("UK_INCOME_TAX", "Income Tax PAYE — retenue {fy}")
            .replace("{fy}", &fy),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: impot_mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: ctx.expl("UK_INCOME_TAX",
            "Income Tax PAYE (retenue à la source mensuelle).\n\n\
            Revenu annuel estimé : £{rev} → tranche : {tl}\n\
            Personal Allowance : £{pa}/an (exonéré)\n\
            Basic Rate 20 % : jusqu'à £{br}/an\n\
            Higher Rate 40 % : £{br} – £{hr}/an\n\
            Additional Rate 45 % : au-delà de £{hr}/an\n\n\
            Impôt annuel estimé : £{ia} / 12 = £{im}/mois\n\
            Taux effectif mensuel : {teff} %\n\n\
            Base légale : Income Tax Act 2007 ; Finance Act 2024.")
            .replace("{rev}", &format!("{:.2}", revenu_annuel))
            .replace("{tl}",  &tranche_libelle)
            .replace("{pa}",  &format!("{:.0}", s.pa * dec!(12)))
            .replace("{br}",  &format!("{:.0}", s.br_max * dec!(12)))
            .replace("{hr}",  &format!("{:.0}", s.hr_max * dec!(12)))
            .replace("{ia}",  &format!("{:.2}", impot_annuel))
            .replace("{im}",  &format!("{:.2}", impot_mensuel))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("Income Tax Act 2007 — Finance Act 2024")),
    }
}
