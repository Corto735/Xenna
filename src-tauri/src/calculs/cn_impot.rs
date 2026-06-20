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
        libelle:     ctx.libelle("CN_IIT", "个人所得税 — Impôt sur le revenu (IIT)"),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: ctx.expl("CN_IIT",
            "个人所得税 — impôt sur le revenu (réforme 2018).\n\n\
            Brut mensuel : ¥{brut}\n\
            − Cotisations sociales sal : ¥{cot}\n\
            − Déduction personnelle : ¥{dp}/mois\n\
            = Base mensuelle imposable : ¥{bm}\n\
            × 12 = Base annuelle : ¥{ba}\n\n\
            IIT annuel (tranches 3/10/20/25/30/35/45 %) : ¥{ia}\n\
            Retenue mensuelle : ¥{ia} / 12 = ¥{mens}\n\
            Taux effectif mensuel : {teff} %\n\n\
            Base légale : 个人所得税法 (L. 31/08/2018) ; 国税发〔2018〕164号.")
            .replace("{brut}", &format!("{:.2}", brut))
            .replace("{cot}", &format!("{:.2}", cotisations_sal))
            .replace("{dp}", &format!("{:.0}", deduction_perso))
            .replace("{bm}", &format!("{:.2}", base_mensuelle))
            .replace("{ba}", &format!("{:.2}", base_annuelle))
            .replace("{ia}", &format!("{:.2}", iit_ann))
            .replace("{mens}", &format!("{:.2}", mensuel))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("个人所得税法 (2018) — 国税发〔2018〕164号")),
    }
}
