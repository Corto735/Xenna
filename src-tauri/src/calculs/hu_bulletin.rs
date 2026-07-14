// ── Hongrie — TB (cotisation sociale salarié) + SZJA (impôt proportionnel) ───────
//
// 2025 :
//   • Társadalombiztosítási járulék (TB) salarié 18,5 % (retraite 10 % + maladie 7 %
//     + chômage 1,5 %), prélevé en une cotisation unique.
//   • Szociális hozzájárulási adó (szocho) employeur 13 %.
//   • SZJA (impôt sur le revenu) : taux proportionnel unique 15 %.
//
// 2026 : taux inchangés (TB 18,5 % / szocho 13 % / SZJA 15 %, stables) → mêmes
// règles qu'en 2025.
// Simplifications : abattements familiaux (családi kedvezmény), exonération des
// moins de 25 ans et des mères de moins de 30 ans NON modélisés (net prudent).
// Source : NAV (SZJA 2025-2026) ; loi LXXX de 2019 (TB).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn generer_bulletin_hu(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "HUF", "HU",
            "Hongrie : données disponibles pour 2025 et 2026.", ctx);
    }

    let ts = ctx.taux_sal("HU_TB");
    let tp = ctx.taux_pat("HU_SZOCHO");
    let tb_sal = (brut * ts).round_dp(2);
    let mut cotisations = vec![
        LigneCotisation {
            code: "HU_TB".into(), libelle: ctx.libelle("HU_TB", "Társadalombiztosítás — Cotisation sociale"),
            base: brut, taux_sal: ts, montant_sal: tb_sal,
            taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
            categorie: "Sécurité sociale".into(),
            explication: ctx.expl("HU_TB",
                "TB — {ts} % salarié (retraite 10 % + maladie 7 % + chômage 1,5 %). Salarié : {ms} HUF.")
                .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
                .replace("{ms}", &format!("{:.2}", tb_sal)),
            loi_ref: Some(ctx.loi_ref("2019. évi CXXII. törvény (TB)")),
        },
        LigneCotisation {
            code: "HU_SZOCHO".into(), libelle: ctx.libelle("HU_SZOCHO", "Szociális hozzájárulási adó (employeur)"),
            base: brut, taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
            taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
            categorie: "Sécurité sociale".into(),
            explication: ctx.expl("HU_SZOCHO", "Szocho — {tp} % à la charge de l'employeur.")
                .replace("{tp}", &format!("{:.2}", tp * dec!(100))),
            loi_ref: Some(ctx.loi_ref("2018. évi LII. törvény (szocho)")),
        },
    ];

    // SZJA : 15 % proportionnel sur le brut.
    let impot = (brut * dec!(0.15)).round_dp(2);
    cotisations.push(LigneCotisation {
        code: "HU_SZJA".into(),
        libelle: ctx.libelle("HU_SZJA", "SZJA — Impôt sur le revenu (15 %)"),
        base: brut, taux_sal: dec!(0.15), montant_sal: impot,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("HU_SZJA",
            "Impôt sur le revenu {annee} : taux proportionnel unique 15 % → {im} HUF/mois.\n\n\
            Note : abattements familiaux et exonérations jeunes/mères non modélisés (net prudent).\n\
            Source : NAV.")
            .replace("{annee}", &annee.to_string())
            .replace("{im}", &format!("{:.2}", impot)),
        loi_ref: Some(ctx.loi_ref("1995. évi CXVII. törvény (SZJA)")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "HUF".into(), absence: None, heures_sup: None, conges: None, salarie,
    }
}
