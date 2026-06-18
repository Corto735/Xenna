// ── Slovénie — prispevki (cotisations) + dohodnina (impôt progressif) ────────────
//
// 2025 :
//   • Prispevki salarié 22,1 % (retraite/invalidité 15,5 % + maladie 6,36 %
//     + chômage 0,14 % + parental 0,10 %) / employeur 16,1 %.
//   • Dohodnina : barème progressif 2025 (16 / 26 / 33 / 39 / 50 %), assiette
//     = revenu après cotisations salariales et abattement général (5 000 €/an).
//
// Simplification : abattement général fixé à 5 000 €/an (l'abattement majoré
// dégressif pour bas revenus n'est pas modélisé → net prudent).
// Source : ZPIZ/ZZZS (prispevki) ; FURS (dohodnina 2025).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Dohodnina annuelle 2025 (barème progressif, cumul par tranche).
fn dohodnina(t: Decimal) -> Decimal {
    if t <= dec!(9210.26) {
        t * dec!(0.16)
    } else if t <= dec!(27089.34) {
        dec!(1473.64) + (t - dec!(9210.26)) * dec!(0.26)
    } else if t <= dec!(54178.69) {
        dec!(6122.20) + (t - dec!(27089.34)) * dec!(0.33)
    } else if t <= dec!(78016.32) {
        dec!(15061.69) + (t - dec!(54178.69)) * dec!(0.39)
    } else {
        dec!(24358.37) + (t - dec!(78016.32)) * dec!(0.50)
    }
}

pub fn generer_bulletin_si(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Slovénie : données disponibles pour 2025.");
    }

    let ts = ctx.taux_sal("SI_PRISPEVKI");
    let tp = ctx.taux_pat("SI_PRISPEVKI");
    let prisp_sal = (brut * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "SI_PRISPEVKI".into(),
        libelle: "Prispevki — Cotisations sociales".into(),
        base: brut, taux_sal: ts, montant_sal: prisp_sal,
        taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "Prispevki — salarié {ts:.2} % (retraite/invalidité 15,5 %, maladie 6,36 %, \
            chômage 0,14 %, parental 0,10 %) / employeur {tp:.2} %. Salarié : {ms:.2} €.",
            ts = ts * dec!(100), tp = tp * dec!(100), ms = prisp_sal,
        ),
        loi_ref: Some("ZPIZ-2 / ZZVZZ".into()),
    }];

    // Dohodnina : base annuelle = (brut − cotisations salariales) × 12 − abattement 5 000 €.
    let base_an = (((brut - prisp_sal).max(Decimal::ZERO)) * dec!(12) - dec!(5000)).max(Decimal::ZERO);
    let impot_mens = (dohodnina(base_an) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "SI_DOHODNINA".into(),
        libelle: "Dohodnina — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025 (annualisé).\n\n\
            Base = (brut − cotisations) × 12 − abattement 5 000 € = {b:.0} €\n\
            Barème 16 / 26 / 33 / 39 / 50 % (seuils 9 210 / 27 089 / 54 179 / 78 016 €)\n\
            → {im:.2} €/mois.\n\n\
            Note : abattement majoré pour bas revenus non modélisé (net prudent).\n\
            Source : FURS.",
            b = base_an, im = impot_mens,
        ),
        loi_ref: Some("Zakon o dohodnini (ZDoh-2)".into()),
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
