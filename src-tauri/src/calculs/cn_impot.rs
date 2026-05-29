// ── IIT Chine — 个人所得税 (réforme 2018) ─────────────────────────────────────
//
// Assiette : brut − cotisations salariales − déduction personnelle (¥5 000/mois)
// Tranches annuelles (depuis 01/01/2019) :
//   3 %  : 0 – ¥36 000
//   10 % : ¥36 000 – ¥144 000
//   20 % : ¥144 000 – ¥300 000
//   25 % : ¥300 000 – ¥420 000
//   30 % : ¥420 000 – ¥660 000
//   35 % : ¥660 000 – ¥960 000
//   45 % : > ¥960 000
//
// Méthode : annualisation du revenu mensuel, retenue = impôt annuel / 12.
//
// Sources : 个人所得税法 (2018) ; 国税发〔2018〕164号.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

fn iit_annuel(revenu_imposable_annuel: Decimal) -> Decimal {
    if revenu_imposable_annuel <= Decimal::ZERO {
        return Decimal::ZERO;
    }
    if revenu_imposable_annuel <= dec!(36000) {
        revenu_imposable_annuel * dec!(0.03)
    } else if revenu_imposable_annuel <= dec!(144000) {
        dec!(1080) + (revenu_imposable_annuel - dec!(36000)) * dec!(0.10)
    } else if revenu_imposable_annuel <= dec!(300000) {
        dec!(11880) + (revenu_imposable_annuel - dec!(144000)) * dec!(0.20)
    } else if revenu_imposable_annuel <= dec!(420000) {
        dec!(43080) + (revenu_imposable_annuel - dec!(300000)) * dec!(0.25)
    } else if revenu_imposable_annuel <= dec!(660000) {
        dec!(73080) + (revenu_imposable_annuel - dec!(420000)) * dec!(0.30)
    } else if revenu_imposable_annuel <= dec!(960000) {
        dec!(145080) + (revenu_imposable_annuel - dec!(660000)) * dec!(0.35)
    } else {
        dec!(250080) + (revenu_imposable_annuel - dec!(960000)) * dec!(0.45)
    }
}

pub fn cn_iit(brut: Decimal, cotisations_sal: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let _annee            = ctx.date_paie.year();
    let deduction_perso   = dec!(5000); // ¥5 000/mois (déduction personnelle depuis 2018)
    let base_mensuelle    = (brut - cotisations_sal - deduction_perso).max(Decimal::ZERO);
    let base_annuelle     = base_mensuelle * dec!(12);

    let iit_ann  = iit_annuel(base_annuelle);
    let mensuel  = (iit_ann / dec!(12)).round_dp(2);
    let taux_eff = if brut > Decimal::ZERO { (mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code:        "CN_IIT".into(),
        libelle:     "个人所得税 — Impôt sur le revenu (IIT)".into(),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: format!(
            "个人所得税 — impôt sur le revenu (réforme 2018).\n\n\
            Brut mensuel : ¥{brut:.2}\n\
            − Cotisations sociales sal : ¥{cot:.2}\n\
            − Déduction personnelle : ¥{dp:.0}/mois\n\
            = Base mensuelle imposable : ¥{bm:.2}\n\
            × 12 = Base annuelle : ¥{ba:.2}\n\n\
            IIT annuel (tranches 3/10/20/25/30/35/45 %) : ¥{ia:.2}\n\
            Retenue mensuelle : ¥{ia:.2} / 12 = ¥{mens:.2}\n\
            Taux effectif mensuel : {teff:.2} %\n\n\
            Base légale : 个人所得税法 (L. 31/08/2018) ; 国税发〔2018〕164号.",
            brut = brut, cot = cotisations_sal, dp = deduction_perso,
            bm   = base_mensuelle, ba = base_annuelle,
            ia   = iit_ann, mens = mensuel, teff = taux_eff * dec!(100),
        ),
        loi_ref: Some("个人所得税法 (2018) — 国税发〔2018〕164号".into()),
    }
}
