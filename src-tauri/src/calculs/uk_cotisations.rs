// ── Cotisations Royaume-Uni — NI Class 1 + Income Tax PAYE ──────────────────
//
// Périmètre : salarié secteur privé anglais, exercices 2024/25 à 2026/27.
// L'exercice britannique court du 6 avril au 5 avril : `annee_fiscale`.
// Taux NI, seuils et barème IT lus en base, datés par exercice.
//
// Sources légales :
//   National Insurance Contributions Act 2014
//   Income Tax Act 2007 ; Finance Act 2024

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Seuils ───────────────────────────────────────────────────────────────────
//
// En base (migration 0151), datés au 6 avril, début de l'exercice :
//   UK_NI_PT, UK_NI_UEL, UK_NI_ST — seuils NI MENSUELS tels que publiés par HMRC
//     (arrondis à la livre) : ce sont eux, et non l'annuel ÷ 12, qu'appliquent
//     les logiciels de paie ; UK_NI_TAUX_HAUT — taux salarié au-delà de l'UEL ;
//   UK_IT (barème progressif annuel) — Personal Allowance (tranche à 0 %),
//     Basic, Higher et Additional Rate.

struct UkSeuils {
    pt:  Decimal, // Primary Threshold mensuel (NI sal commence au-dessus)
    uel: Decimal, // Upper Earnings Limit mensuel (NI sal → taux haut au-dessus)
    st:  Decimal, // Secondary Threshold mensuel (NI pat commence au-dessus)
    taux_haut: Decimal, // taux NI salarié au-delà de l'UEL (2 %)
    pa_an: Decimal, // Personal Allowance annuelle (IT exonérée)
    br_an: Decimal, // plafond annuel de la Basic Rate (20 %)
    hr_an: Decimal, // plafond annuel de la Higher Rate (40 %)
}

/// Exercice fiscal (tax year) d'une date de paie : l'exercice « 2025 » est le
/// 2025/26, du 06/04/2025 au 05/04/2026.
fn annee_fiscale(d: chrono::NaiveDate) -> i32 {
    if (d.month(), d.day()) >= (4, 6) { d.year() } else { d.year() - 1 }
}

/// Seuils de l'exercice, ou None s'ils ne sont pas en base à cette date.
fn seuils(ctx: &ContextPaie) -> Option<UkSeuils> {
    let it = ctx.bareme("UK_IT")?;
    let borne = |i: usize| it.get(i).and_then(|t| t.borne_sup);
    Some(UkSeuils {
        pt:        ctx.plafond("UK_NI_PT")?,
        uel:       ctx.plafond("UK_NI_UEL")?,
        st:        ctx.plafond("UK_NI_ST")?,
        taux_haut: ctx.plafond("UK_NI_TAUX_HAUT")?,
        pa_an:     borne(0)?,
        br_an:     borne(1)?,
        hr_an:     borne(2)?,
    })
}

/// Le régime est-il couvert à la date de paie ? (uk_bulletin)
pub fn couvert(ctx: &ContextPaie) -> bool {
    seuils(ctx).is_some()
}

// ── National Insurance — part salariale ───────────────────────────────────────
//
// Tranche [PT – UEL]  : 8 % (taux lu en DB)
// Tranche au-delà UEL : 2 % (UK_NI_TAUX_HAUT)

pub fn uk_ni_sal(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = annee_fiscale(ctx.date_paie);
    let s     = seuils(ctx).expect("seuils UK vérifiés par uk_bulletin");
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

    let montant = (tranche_principale * ts + tranche_haute * s.taux_haut).round_dp(2);
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
            .replace("{m2}",  &format!("{:.2}", (tranche_haute * s.taux_haut).round_dp(2)))
            .replace("{tot}", &format!("{:.2}", montant))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("National Insurance Contributions Act 2014 — Finance Act 2024")),
    }
}

// ── National Insurance — part patronale ───────────────────────────────────────
//
// 13,8 % (≤ 2024/25) puis 15 % (dès le 06/04/2025) sur le salaire excédant le Secondary Threshold (ST).
// Pas de plafond côté employeur.

pub fn uk_ni_pat(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee = annee_fiscale(ctx.date_paie);
    let s     = seuils(ctx).expect("seuils UK vérifiés par uk_bulletin");
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

pub fn uk_income_tax(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee  = annee_fiscale(ctx.date_paie);
    let s      = seuils(ctx).expect("seuils UK vérifiés par uk_bulletin");
    let revenu_annuel = brut * dec!(12);

    let impot_annuel   = ctx.bareme_progressif("UK_IT", revenu_annuel).unwrap_or(Decimal::ZERO);
    let impot_mensuel  = (impot_annuel / dec!(12)).round_dp(2);
    let taux_eff       = if brut > Decimal::ZERO { (impot_mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    let tranche_libelle = if revenu_annuel <= s.pa_an {
        ctx.expl("UK_TL_PA", "dans la Personal Allowance (0 %)")
    } else if revenu_annuel <= s.br_an {
        "Basic Rate (20 %)".into() // nom officiel, identique dans les 6 langues
    } else if revenu_annuel <= s.hr_an {
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
            .replace("{pa}",  &format!("{:.0}", s.pa_an))
            .replace("{br}",  &format!("{:.0}", s.br_an))
            .replace("{hr}",  &format!("{:.0}", s.hr_an))
            .replace("{ia}",  &format!("{:.2}", impot_annuel))
            .replace("{im}",  &format!("{:.2}", impot_mensuel))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("Income Tax Act 2007 — Finance Act 2024")),
    }
}
