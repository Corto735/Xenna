// ── Suède — arbetsgivaravgifter (100 % patronales) + impôt communal + impôt d'État ──
//
// Particularité suédoise : pas de cotisations sociales salariales nettes. La seule
// retenue salariale légale est l'« allmän pensionsavgift » (7 %), mais elle est
// INTÉGRALEMENT compensée par une réduction d'impôt → effet net nul. Elle n'est donc
// pas affichée (sinon le net serait sous-estimé). L'employeur paie les
// arbetsgivaravgifter (31,42 % en 2025), lues en base.
//
// Impôt sur le revenu : impôt communal moyen (kommunalskatt, ≈ 32,41 % en 2025)
// + impôt d'État (statlig inkomstskatt) 20 % au-delà de 625 800 SEK/an (skiktgräns 2025).
//
// Simplifications documentées (net prudent, impôt légèrement surestimé) :
//   • grundavdrag (abattement de base) NON modélisé ;
//   • jobbskatteavdrag (crédit d'impôt sur revenus du travail) NON modélisé.
// Sources : Skatteverket (kommunalskatt 2025, skiktgräns 2025) ; arbetsgivaravgifter 2025.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn generer_bulletin_se(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "SEK", "Suède : données disponibles pour 2025.");
    }

    let g = brut * dec!(12); // revenu annuel

    // Cotisation patronale unique : arbetsgivaravgifter 31,42 % (taux lu en base).
    let tp = ctx.taux_pat("SE_ARBETSGIVARAVGIFT");
    let mut cotisations = vec![LigneCotisation {
        code: "SE_ARBETSGIVARAVGIFT".into(),
        libelle: "Arbetsgivaravgifter — cotisations patronales".into(),
        base: brut,
        taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "Arbetsgivaravgifter — {tp:.2} % à la charge de l'employeur (retraite, maladie, \
            parentalité, accident, marché du travail, taxe générale sur salaires).\n\n\
            Côté salarié : l'allmän pensionsavgift (7 %) est intégralement compensée par \
            une réduction d'impôt (effet net nul) → non affichée.",
            tp = tp * dec!(100),
        ),
        loi_ref: Some("Socialavgiftslagen (2000:980)".into()),
    }];

    // Impôt sur le revenu : communal moyen + État 20 % au-delà de 625 800 SEK/an.
    let communal = g * dec!(0.3241);
    let etat = if g > dec!(625800) { (g - dec!(625800)) * dec!(0.20) } else { Decimal::ZERO };
    let impot_mens = ((communal + etat) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "SE_SKATT".into(),
        libelle: "Inkomstskatt — Impôt (communal + État)".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025 (annualisé).\n\n\
            Revenu annuel : {g:.0} SEK\n\
            Impôt communal moyen 32,41 % → {co:.0} SEK\n\
            Impôt d'État 20 % au-delà de 625 800 SEK/an → {et:.0} SEK\n\
            = {im:.2} SEK/mois.\n\n\
            Note : grundavdrag et jobbskatteavdrag non modélisés (net prudent).\n\
            Source : Skatteverket.",
            g = g, co = communal, et = etat, im = impot_mens,
        ),
        loi_ref: Some("Inkomstskattelagen (1999:1229)".into()),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "SEK".into(), absence: None, heures_sup: None, salarie,
    }
}
