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
    // bundskat 12,01 % et topskat 15 % stables ; AM-bidrag 8 % lu en base ; ATP forfait.
    let (personfradrag_an, kommune, topskat_seuil_an) = match annee {
        2024 => (dec!(49700), dec!(0.25067), dec!(588900)),
        2025 => (dec!(51600), dec!(0.251),   dec!(611800)),
        _ => return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "DKK", "Danemark : données disponibles pour 2024-2025."),
    };
    let personfradrag_m = personfradrag_an / dec!(12);
    let bund_kommune    = dec!(0.1201) + kommune;
    let topskat_taux    = dec!(0.15);
    let topskat_seuil_m = topskat_seuil_an / dec!(12);
    let atp_mensuel     = dec!(94.65);

    let mut cotisations = Vec::new();

    // AM-bidrag (taux en base)
    let am_ts = ctx.taux_sal("DK_AM");
    let am = (brut * am_ts).round_dp(2);
    cotisations.push(LigneCotisation {
        code: "DK_AM".into(), libelle: "AM-bidrag — Contribution marché du travail".into(), base: brut,
        taux_sal: am_ts, montant_sal: am, taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "AM-bidrag — 8 % du salaire brut, prélevé avant l'impôt.\n\
            Montant : {am:.2} DKK.\n\nBase légale : Arbejdsmarkedsbidragsloven.",
            am = am,
        ),
        loi_ref: Some("Arbejdsmarkedsbidragsloven".into()),
    });

    // ATP (forfait)
    cotisations.push(LigneCotisation {
        code: "DK_ATP".into(), libelle: "ATP — Pension complémentaire".into(), base: brut,
        taux_sal: Decimal::ZERO, montant_sal: atp_mensuel, taux_pat: (atp_mensuel * dec!(2)),
        montant_pat: (atp_mensuel * dec!(2)),
        categorie: "Retraite".into(),
        explication: format!(
            "ATP — pension complémentaire du marché du travail (forfait).\n\
            Temps plein 2025 : {a:.2} DKK/mois salarié (2/3 employeur).\n\n\
            Base légale : ATP-loven.",
            a = atp_mensuel,
        ),
        loi_ref: Some("ATP-loven".into()),
    });

    // Impôt sur le revenu
    let apres_am = brut - am;                              // base topskat
    let taxable  = (apres_am - atp_mensuel - personfradrag_m).max(Decimal::ZERO);
    let impot_base = taxable * bund_kommune;
    let topskat = (apres_am - topskat_seuil_m).max(Decimal::ZERO) * topskat_taux;
    let impot = (impot_base + topskat).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "DK_INDKOMSTSKAT".into(), libelle: "Indkomstskat — Impôt sur le revenu".into(), base: brut,
        taux_sal: taux_imp, montant_sal: impot, taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu — bundskat 12,01 % + kommuneskat moyen 25,1 % (= 37,11 %)\n\
            sur le revenu après AM-bidrag, ATP et personfradrag (4 300 DKK/mois).\n\
            + topskat 15 % au-delà de {ts:.0} DKK/mois (revenu après AM).\n\
            Base imposable : {tx:.2} DKK → {ib:.2} DKK ; topskat {tk:.2} DKK.\n\
            = {im:.2} DKK/mois.\n\n\
            Base légale : Personskatteloven (2025). Kommuneskat = moyenne nationale.",
            ts = topskat_seuil_m, tx = taxable, ib = impot_base, tk = topskat, im = impot,
        ),
        loi_ref: Some("Personskatteloven".into()),
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
