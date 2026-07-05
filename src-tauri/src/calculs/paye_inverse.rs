// Paye inversée : reconstitue le BRUT permettant d'atteindre un NET souhaité.
//
// Cible (décision produit) : le net AVANT impôt à la source, c'est-à-dire
// brut − Σ cotisations sociales salariales. Les lignes d'impôt sur le revenu
// (PAS suisse, IRPEF, Lohnsteuer, Income Tax…) sont EXCLUES de la cible : elles
// sont ensuite calculées normalement sur le brut reconstitué et figurent dans
// le bulletin renvoyé. En France, aucun impôt n'étant modélisé, la cible
// coïncide avec `net_a_payer`.
//
// Une absence maladie éventuelle est EXCLUE de l'inversion : la cible est le net
// du salaire de base plein. La dichotomie tourne sur le mois complet (sans
// absence), puis l'absence est appliquée sur le brut trouvé dans le bulletin
// final. Ainsi le salaire de base (brut reconstitué) ne bouge pas quand on
// ajoute une absence — c'est le net du mois qui diminue.
//
// Méthode : `generer_bulletin` est une fonction pure et synchrone du
// (Salarie, &ContextPaie, absence) — le contexte est préchargé, aucune I/O.
// On inverse donc net(brut) par dichotomie (~60 itérations en mémoire).
// net(brut) est monotone croissant (testé pour la France dans fiabilite.rs ;
// invariant universel net ≤ brut pour la borne basse).

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::db::ContextPaie;
use crate::models::{AbsenceInput, Bulletin, Salarie};

/// Catégories des lignes d'impôt sur le revenu — exclues de la cible.
/// Valeurs françaises canoniques du champ `categorie` (cf. CAT_DICT côté front).
/// « Bonus IRPEF » (crédit d'impôt italien, salarial négatif) est côté impôt.
/// Cas limite assumé : NL_LOONHEFFING mélange impôt et premies
/// volksverzekeringen dans une seule ligne « Impôt sur le revenu » → exclue en bloc.
const CATEGORIES_IMPOT: &[&str] = &[
    "Impôt sur le revenu",
    "Impôt à la source",
    "Impôt fédéral",
    "Impôt provincial",
    "Imposta",
    "Imposta regionale",
    "Taxe locale",
    "Bonus IRPEF",
];

/// Net avant impôt d'un bulletin : brut − cotisations sociales salariales
/// (lignes d'impôt exclues).
pub fn net_avant_impot(b: &Bulletin) -> Decimal {
    let cotis_sociales_sal: Decimal = b
        .cotisations
        .iter()
        .filter(|c| !CATEGORIES_IMPOT.contains(&c.categorie.as_str()))
        .map(|c| c.montant_sal)
        .sum();
    (b.brut - cotis_sociales_sal).round_dp(2)
}

/// Bulletin pour un brut sondé : `salaire_base` est remis à None pour que tout
/// (taux horaire des heures supp compris) dérive du brut testé.
fn bulletin_pour(
    brut: Decimal,
    salarie: &Salarie,
    ctx: &ContextPaie,
    absence: Option<&AbsenceInput>,
) -> Bulletin {
    let mut s = salarie.clone();
    s.salaire_brut = brut;
    s.salaire_base = None;
    super::generer_bulletin(s, ctx, absence)
}

/// Résout le brut tel que `net_avant_impot(bulletin(brut)) ≈ net_cible`,
/// puis renvoie le bulletin complet (impôts compris) calculé sur ce brut
/// arrondi au centime.
pub fn resoudre_brut_pour_net(
    net_cible: Decimal,
    salarie: &Salarie,
    ctx: &ContextPaie,
    absence: Option<&AbsenceInput>,
) -> Bulletin {
    let cible = net_cible.round_dp(2);
    let tol = dec!(0.005);

    // La cible est le net du salaire de base PLEIN (mois complet). Une absence
    // éventuelle NE doit PAS gonfler le brut reconstitué : sinon le brut serait
    // remonté pour que le net *après* absence tombe sur la cible, faisant bondir
    // le salaire de base. On reconstitue donc le brut sur le mois plein (absence
    // = None dans tous les sondages), puis l'absence s'applique sur ce brut dans
    // le bulletin final — elle réduit alors le net du mois, salaire de base figé.
    let lo_initial = cible;
    if (net_avant_impot(&bulletin_pour(lo_initial, salarie, ctx, None)) - cible).abs() <= tol {
        return bulletin_pour(lo_initial, salarie, ctx, absence);
    }

    // Borne haute : doubler jusqu'à dépasser la cible (garde-fou ×16, soit
    // jusqu'à ~94 % de prélèvements sociaux — aucun barème réel n'y arrive).
    let mut lo = lo_initial;
    let mut hi = cible * dec!(2);
    for _ in 0..4 {
        if net_avant_impot(&bulletin_pour(hi, salarie, ctx, None)) >= cible {
            break;
        }
        hi *= dec!(2);
    }

    // Dichotomie : `hi` converge vers le plus petit brut atteignant la cible.
    for _ in 0..60 {
        if hi - lo <= tol {
            break;
        }
        let mid = ((lo + hi) / dec!(2)).round_dp(4);
        if net_avant_impot(&bulletin_pour(mid, salarie, ctx, None)) < cible {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    bulletin_pour(hi.round_dp(2), salarie, ctx, absence)
}
