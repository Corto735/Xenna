// ── Slovaquie — zdravotné + sociálne poistenie + daň z príjmov ───────────────────
//
// 2025 :
//   • Zdravotné poistenie (santé) salarié 4 % / employeur 11 % — sans plafond.
//   • Sociálne poistenie salarié 9,4 % / employeur 25,2 %, sur l'assiette plafonnée
//     à 15 730 €/mois (7× le salaire moyen, 2025).
//   • Daň z príjmov : 19 % jusqu'à 4 036,79 €/mois, 25 % au-delà.
//   • Nezdaniteľná časť (part non imposable) : 479,48 €/mois (simplifiée — la
//     dégressivité pour hauts revenus n'est pas modélisée → net prudent).
//
// 2026 :
//   • Nezdaniteľná časť 497,23 €/mois (21× životné minimum 284,13 € de juillet 2025).
//   • Daň : 19 % jusqu'à 176,8× ŽM = 4 186,18 €/mois, 25 %, puis 30 % (212,4× ŽM)
//     et 35 % (264× ŽM).
//   • Santé salarié 5 % (hausse temporaire de consolidation, 2026-2027), employeur
//     11 % (lus en base) ; sociaux inchangés. Plafond social 16 764 €/mois
//     (11 × salaire moyen 2024 de 1 524 €, Sociálna poisťovňa).
// Source : Sociálna poisťovňa ; ÚDZS ; Finančná správa (daň 2025 et 2026 ; NČZD 2026).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn generer_bulletin_sk(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "SK",
            "Slovaquie : données disponibles pour 2025 et 2026.", ctx);
    }
    // Part non imposable mensuelle et seuil 25 % selon l'année.
    let nczd = if annee >= 2026 { dec!(497.23) } else { dec!(479.48) };
    let seuil = if annee >= 2026 { dec!(4186.18) } else { dec!(4036.79) };

    // Santé (non plafonnée).
    let ts_z = ctx.taux_sal("SK_ZDRAVOTNE");
    let tp_z = ctx.taux_pat("SK_ZDRAVOTNE");
    let z_sal = (brut * ts_z).round_dp(2);
    // Social (assiette plafonnée : 15 730 €/mois en 2025, 16 764 € en 2026).
    let plafond_soc = if annee >= 2026 { dec!(16764) } else { dec!(15730) };
    let assiette_soc = brut.min(plafond_soc);
    let ts_s = ctx.taux_sal("SK_SOCIALNE");
    let tp_s = ctx.taux_pat("SK_SOCIALNE");
    let s_sal = (assiette_soc * ts_s).round_dp(2);

    let mut cotisations = vec![
        LigneCotisation {
            code: "SK_ZDRAVOTNE".into(), libelle: ctx.libelle("SK_ZDRAVOTNE", "Zdravotné poistenie — Assurance maladie"),
            base: brut, taux_sal: ts_z, montant_sal: z_sal,
            taux_pat: tp_z, montant_pat: (brut * tp_z).round_dp(2),
            categorie: "Sécurité sociale".into(),
            explication: ctx.expl("SK_ZDRAVOTNE", "Assurance maladie — salarié {ts} % / employeur {tp} %.")
                .replace("{ts}", &format!("{:.2}", ts_z * dec!(100)))
                .replace("{tp}", &format!("{:.2}", tp_z * dec!(100))),
            loi_ref: Some(ctx.loi_ref("Zákon o zdravotnom poistení")),
        },
        LigneCotisation {
            code: "SK_SOCIALNE".into(), libelle: ctx.libelle("SK_SOCIALNE", "Sociálne poistenie — Sécurité sociale"),
            base: assiette_soc, taux_sal: ts_s, montant_sal: s_sal,
            taux_pat: tp_s, montant_pat: (assiette_soc * tp_s).round_dp(2),
            categorie: "Sécurité sociale".into(),
            explication: ctx.expl("SK_SOCIALNE",
                "Sécurité sociale — salarié {ts} % / employeur {tp} %. Assiette plafonnée à {plaf} €/mois.")
                .replace("{plaf}", &plafond_soc.to_string())
                .replace("{ts}", &format!("{:.2}", ts_s * dec!(100)))
                .replace("{tp}", &format!("{:.2}", tp_s * dec!(100))),
            loi_ref: Some(ctx.loi_ref("Zákon o sociálnom poistení")),
        },
    ];

    // Daň z príjmov : base = brut − cotisations salariales − part non imposable.
    let base = (brut - z_sal - s_sal - nczd).max(Decimal::ZERO);
    // Tranches mensuelles : 19 / 25 % jusqu'en 2025 ; dès 2026, 30 % au-delà de
    // 212,4 × ŽM (5 029,10 €) et 35 % au-delà de 264 × ŽM (6 250,86 €) — 3ᵉ paquet
    // de consolidation (loi du 24/09/2025), ŽM = 284,13 €.
    let tranches: Vec<(Decimal, Decimal)> = if annee >= 2026 {
        vec![(seuil, dec!(0.19)), (dec!(5029.10), dec!(0.25)), (dec!(6250.86), dec!(0.30)), (Decimal::MAX, dec!(0.35))]
    } else {
        vec![(seuil, dec!(0.19)), (Decimal::MAX, dec!(0.25))]
    };
    let mut impot = Decimal::ZERO;
    let mut bas = Decimal::ZERO;
    for (haut, t) in &tranches {
        if base > bas { impot += (base.min(*haut) - bas) * t; }
        bas = *haut;
    }
    let impot = impot.round_dp(2);
    let bareme = tranches.iter().map(|(haut, t)| {
        let pct = format!("{} %", (t * dec!(100)).normalize());
        if *haut == Decimal::MAX { pct } else { format!("{pct} ≤ {:.2} €", haut) }
    }).collect::<Vec<_>>().join(" < ");
    let taux_imp = if brut > Decimal::ZERO { (impot / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "SK_DAN".into(),
        libelle: ctx.libelle("SK_DAN", "Daň z príjmov — Impôt sur le revenu"),
        base: brut, taux_sal: taux_imp, montant_sal: impot,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("SK_DAN",
            "Impôt sur le revenu {annee}.\n\n\
            Base = brut − cotisations salariales − part non imposable {nczd} € = {b} €\n\
            Barème mensuel : {bareme} → {im} €/mois.\n\n\
            Note : dégressivité de la part non imposable non modélisée (net prudent).\n\
            Source : Finančná správa.")
            .replace("{annee}", &annee.to_string())
            .replace("{nczd}", &format!("{:.2}", nczd))
            .replace("{bareme}", &bareme)
            .replace("{b}", &format!("{:.2}", base))
            .replace("{im}", &format!("{:.2}", impot)),
        loi_ref: Some(ctx.loi_ref("Zákon o dani z príjmov")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(), absence: None, heures_sup: None, conges: None, salarie,
    }
}
