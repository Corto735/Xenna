// ── Slovaquie — zdravotné + sociálne poistenie + daň z príjmov ───────────────────
//
// 2025 :
//   • Zdravotné poistenie (santé) salarié 4 % / employeur 11 % — sans plafond.
//   • Sociálne poistenie salarié 9,4 % / employeur 25,2 %, sur l'assiette plafonnée
//     à 15 730 €/mois (7× le salaire moyen, 2025).
//   • Daň z príjmov : 19 % jusqu'à 4 036,79 €/mois, 25 % au-delà.
//   • Nezdaniteľná časť (part non imposable) : 479,48 €/mois (simplifiée — la
//     dégressivité pour hauts revenus n'est pas modélisée → net prudent).
// Source : Sociálna poisťovňa ; ÚDZS ; Finančná správa (daň 2025).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn generer_bulletin_sk(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Slovaquie : données disponibles pour 2025.");
    }

    // Santé (non plafonnée).
    let ts_z = ctx.taux_sal("SK_ZDRAVOTNE");
    let tp_z = ctx.taux_pat("SK_ZDRAVOTNE");
    let z_sal = (brut * ts_z).round_dp(2);
    // Social (assiette plafonnée à 15 730 €/mois).
    let assiette_soc = brut.min(dec!(15730));
    let ts_s = ctx.taux_sal("SK_SOCIALNE");
    let tp_s = ctx.taux_pat("SK_SOCIALNE");
    let s_sal = (assiette_soc * ts_s).round_dp(2);

    let mut cotisations = vec![
        LigneCotisation {
            code: "SK_ZDRAVOTNE".into(), libelle: "Zdravotné poistenie — Assurance maladie".into(),
            base: brut, taux_sal: ts_z, montant_sal: z_sal,
            taux_pat: tp_z, montant_pat: (brut * tp_z).round_dp(2),
            categorie: "Sécurité sociale".into(),
            explication: format!("Assurance maladie — salarié {:.2} % / employeur {:.2} %.",
                ts_z * dec!(100), tp_z * dec!(100)),
            loi_ref: Some("Zákon o zdravotnom poistení".into()),
        },
        LigneCotisation {
            code: "SK_SOCIALNE".into(), libelle: "Sociálne poistenie — Sécurité sociale".into(),
            base: assiette_soc, taux_sal: ts_s, montant_sal: s_sal,
            taux_pat: tp_s, montant_pat: (assiette_soc * tp_s).round_dp(2),
            categorie: "Sécurité sociale".into(),
            explication: format!(
                "Sécurité sociale — salarié {:.2} % / employeur {:.2} %. Assiette plafonnée \
                à 15 730 €/mois.", ts_s * dec!(100), tp_s * dec!(100)),
            loi_ref: Some("Zákon o sociálnom poistení".into()),
        },
    ];

    // Daň z príjmov : base = brut − cotisations salariales − part non imposable.
    let base = (brut - z_sal - s_sal - dec!(479.48)).max(Decimal::ZERO);
    let seuil = dec!(4036.79);
    let part_haute = (base - seuil).max(Decimal::ZERO);
    let part_basse = base - part_haute;
    let impot = (part_basse * dec!(0.19) + part_haute * dec!(0.25)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "SK_DAN".into(),
        libelle: "Daň z príjmov — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025.\n\n\
            Base = brut − cotisations salariales − part non imposable 479,48 € = {b:.2} €\n\
            19 % jusqu'à 4 036,79 €/mois, 25 % au-delà → {im:.2} €/mois.\n\n\
            Note : dégressivité de la part non imposable non modélisée (net prudent).\n\
            Source : Finančná správa.",
            b = base, im = impot,
        ),
        loi_ref: Some("Zákon o dani z príjmov".into()),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(), absence: None, salarie,
    }
}
