// ── Hongrie — TB (cotisation sociale salarié) + SZJA (impôt proportionnel) ───────
//
// 2025 :
//   • Társadalombiztosítási járulék (TB) salarié 18,5 % (retraite 10 % + maladie 7 %
//     + chômage 1,5 %), prélevé en une cotisation unique.
//   • Szociális hozzájárulási adó (szocho) employeur 13 %.
//   • SZJA (impôt sur le revenu) : taux proportionnel unique 15 %.
//
// Simplifications : abattements familiaux (családi kedvezmény), exonération des
// moins de 25 ans et des mères de moins de 30 ans NON modélisés (net prudent).
// Source : NAV (SZJA 2025) ; loi LXXX de 2019 (TB).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn generer_bulletin_hu(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "HUF", "Hongrie : données disponibles pour 2025.");
    }

    let ts = ctx.taux_sal("HU_TB");
    let tp = ctx.taux_pat("HU_SZOCHO");
    let tb_sal = (brut * ts).round_dp(2);
    let mut cotisations = vec![
        LigneCotisation {
            code: "HU_TB".into(), libelle: "Társadalombiztosítás — Cotisation sociale".into(),
            base: brut, taux_sal: ts, montant_sal: tb_sal,
            taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
            categorie: "Sécurité sociale".into(),
            explication: format!(
                "TB — {:.2} % salarié (retraite 10 % + maladie 7 % + chômage 1,5 %). \
                Salarié : {:.2} HUF.", ts * dec!(100), tb_sal),
            loi_ref: Some("2019. évi CXXII. törvény (TB)".into()),
        },
        LigneCotisation {
            code: "HU_SZOCHO".into(), libelle: "Szociális hozzájárulási adó (employeur)".into(),
            base: brut, taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
            taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
            categorie: "Sécurité sociale".into(),
            explication: format!("Szocho — {:.2} % à la charge de l'employeur.", tp * dec!(100)),
            loi_ref: Some("2018. évi LII. törvény (szocho)".into()),
        },
    ];

    // SZJA : 15 % proportionnel sur le brut.
    let impot = (brut * dec!(0.15)).round_dp(2);
    cotisations.push(LigneCotisation {
        code: "HU_SZJA".into(),
        libelle: "SZJA — Impôt sur le revenu (15 %)".into(),
        base: brut, taux_sal: dec!(0.15), montant_sal: impot,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025 : taux proportionnel unique 15 % → {im:.2} HUF/mois.\n\n\
            Note : abattements familiaux et exonérations jeunes/mères non modélisés (net prudent).\n\
            Source : NAV.",
            im = impot,
        ),
        loi_ref: Some("1995. évi CXVII. törvény (SZJA)".into()),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "HUF".into(), absence: None, salarie,
    }
}
