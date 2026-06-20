// ── Danemark — AM-bidrag + ATP + impôt (bund/kommune/topskat) ────────────────
//
// Salarié secteur privé. Côté salarié :
//   • AM-bidrag 8 % (prélevé en premier) ;
//   • ATP (forfait mensuel) ;
//   • impôt = (bundskat 12,01 % + kommuneskat moyen 25,1 %) sur le revenu après
//     AM-bidrag, ATP et personfradrag, + topskat 15 % au-delà de 611 800 DKK.
//
// AM-bidrag lu en base ; ATP et impôt calculés ici. Devise DKK.
// Sources : Arbejdsmarkedsbidragsloven ; Personskatteloven ; ATP-loven (2025).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn generer_bulletin_dk(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    // Paramètres par année : personfradrag, kommuneskat moyen, seuil topskat (après AM-bidrag).
    // bundskat 12,01 % stable ; AM-bidrag 8 % lu en base ; ATP forfait.
    // 2026 (réforme) : l'ancien topskat 15 % est scindé en mellemskat 7,5 % (> 641 200 DKK),
    // topskat 7,5 % (> 845 543) et toptopskat 5 % (> 2 592 700), sur le revenu après AM-bidrag.
    let (personfradrag_an, kommune, topskat_seuil_an) = match annee {
        2024 => (dec!(49700), dec!(0.25067), dec!(588900)),
        2025 => (dec!(51600), dec!(0.251),   dec!(611800)),
        2026 => (dec!(54100), dec!(0.25049), dec!(641200)), // seuil = entrée mellemskat
        _ => return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "DKK", "Danemark : données disponibles pour 2024-2026."),
    };
    let personfradrag_m = personfradrag_an / dec!(12);
    let bund_kommune    = dec!(0.1201) + kommune;
    let topskat_seuil_m = topskat_seuil_an / dec!(12);
    let atp_mensuel     = dec!(94.65);

    let mut cotisations = Vec::new();

    // AM-bidrag (taux en base)
    let am_ts = ctx.taux_sal("DK_AM");
    let am = (brut * am_ts).round_dp(2);
    cotisations.push(LigneCotisation {
        code: "DK_AM".into(), libelle: ctx.libelle("DK_AM", "AM-bidrag — Contribution marché du travail"), base: brut,
        taux_sal: am_ts, montant_sal: am, taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("DK_AM",
            "AM-bidrag — 8 % du salaire brut, prélevé avant l'impôt.\n\
            Montant : {am} DKK.\n\nBase légale : Arbejdsmarkedsbidragsloven.")
            .replace("{am}", &format!("{:.2}", am)),
        loi_ref: Some(ctx.loi_ref("Arbejdsmarkedsbidragsloven")),
    });

    // ATP (forfait)
    cotisations.push(LigneCotisation {
        code: "DK_ATP".into(), libelle: ctx.libelle("DK_ATP", "ATP — Pension complémentaire"), base: brut,
        taux_sal: Decimal::ZERO, montant_sal: atp_mensuel, taux_pat: (atp_mensuel * dec!(2)),
        montant_pat: (atp_mensuel * dec!(2)),
        categorie: "Retraite".into(),
        explication: ctx.expl("DK_ATP",
            "ATP — pension complémentaire du marché du travail (forfait).\n\
            Temps plein 2025 : {a} DKK/mois salarié (2/3 employeur).\n\n\
            Base légale : ATP-loven.")
            .replace("{a}", &format!("{:.2}", atp_mensuel)),
        loi_ref: Some(ctx.loi_ref("ATP-loven")),
    });

    // Impôt sur le revenu
    let apres_am = brut - am;                              // base des tranches d'État
    let taxable  = (apres_am - atp_mensuel - personfradrag_m).max(Decimal::ZERO);
    let impot_base = taxable * bund_kommune;
    // Tranches d'État : topskat unique 15 % (≤ 2025) ou mellem/top/toptop (2026+).
    let topskat = if annee >= 2026 {
        (apres_am - dec!(641200) / dec!(12)).max(Decimal::ZERO) * dec!(0.075)   // mellemskat
        + (apres_am - dec!(845543) / dec!(12)).max(Decimal::ZERO) * dec!(0.075) // topskat
        + (apres_am - dec!(2592700) / dec!(12)).max(Decimal::ZERO) * dec!(0.05) // toptopskat
    } else {
        (apres_am - topskat_seuil_m).max(Decimal::ZERO) * dec!(0.15)
    };
    let impot = (impot_base + topskat).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "DK_INDKOMSTSKAT".into(), libelle: ctx.libelle("DK_INDKOMSTSKAT", "Indkomstskat — Impôt sur le revenu"), base: brut,
        taux_sal: taux_imp, montant_sal: impot, taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("DK_INDKOMSTSKAT",
            "Impôt sur le revenu {annee} — bundskat 12,01 % + kommuneskat moyen {km} % (= {bk} %)\n\
            sur le revenu après AM-bidrag, ATP et personfradrag ({pf} DKK/mois).\n\
            {tranches}\n\
            Base imposable : {tx} DKK → {ib} DKK ; tranches d'État {tk} DKK.\n\
            = {im} DKK/mois.\n\n\
            Base légale : Personskatteloven. Kommuneskat = moyenne nationale.")
            .replace("{annee}", &annee.to_string())
            .replace("{km}", &format!("{:.3}", kommune * dec!(100)))
            .replace("{bk}", &format!("{:.2}", bund_kommune * dec!(100)))
            .replace("{pf}", &format!("{:.0}", personfradrag_m))
            .replace("{tranches}", if annee >= 2026 {
                "+ mellemskat 7,5 % (> 641 200), topskat 7,5 % (> 845 543), toptopskat 5 % (> 2 592 700 DKK/an, après AM)."
            } else {
                "+ topskat 15 % au-delà du seuil (revenu après AM)."
            })
            .replace("{ts}", &format!("{:.0}", topskat_seuil_m))
            .replace("{tx}", &format!("{:.2}", taxable))
            .replace("{ib}", &format!("{:.2}", impot_base))
            .replace("{tk}", &format!("{:.2}", topskat))
            .replace("{im}", &format!("{:.2}", impot)),
        loi_ref: Some(ctx.loi_ref("Personskatteloven")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "DKK".into(), absence: None, heures_sup: None, salarie,
    }
}
