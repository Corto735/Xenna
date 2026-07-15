// Congés payés : retenue sur salaire + indemnité de congés payés.
//
// Art. L3141-24 C. trav. :
//   I   — méthode du dixième : 1/10 de la rémunération brute totale perçue sur
//         la période de référence, proratisé aux jours de congé pris ;
//   II  — maintien de salaire : rémunération qu'aurait perçue le salarié s'il
//         avait travaillé (= la retenue, elles se neutralisent) ;
//   III — on applique la règle la plus favorable au salarié.
//
// Hypothèse du simulateur (mono-mensuel) : salaire constant sur l'année et
// 13e mois versé en décembre → assiette du dixième = 13 × brut mensuel.
// La jurisprudence exclut normalement de l'assiette un 13e mois rémunérant
// indistinctement périodes de travail et de congés (Cass. soc. 8 juin 2011,
// n° 09-71056) — inclusion assumée ici par choix pédagogique, signalée dans
// le panneau f(x) du front.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::models::{AbsenceInput, CongesPayesResult};
use super::absence::{compter, diviseur, libelle_methode, type_jour, unites_absence, TypeJour};

/// Calcule retenue + indemnité (max maintien/dixième) pour des congés payés.
/// `base_brut` = brut mensuel plein (saisi ou reconstitué en paie inversée).
/// Retourne None si les dates sont absentes/invalides ou si la période est vide.
pub fn compute_conges(base_brut: Decimal, abs: &AbsenceInput) -> Option<CongesPayesResult> {
    let debut = NaiveDate::parse_from_str(&abs.date_debut, "%Y-%m-%d").ok()?;
    let fin   = NaiveDate::parse_from_str(&abs.date_fin,   "%Y-%m-%d").ok()?;
    if fin < debut { return None; }

    let methode    = if abs.methode.is_empty() { "moyens" } else { abs.methode.as_str() };
    let jours_type = if abs.jours_type.is_empty() { "ouvres" } else { abs.jours_type.as_str() };
    let heures_mois = abs.heures_mois.unwrap_or(151.67);
    let kind = type_jour(methode, jours_type);

    // ── Retenue : mêmes méthodes de valorisation que l'absence maladie ──
    let nb_jours = compter(debut, fin, kind);
    if nb_jours == 0 { return None; }
    let div = diviseur(methode, kind, debut, heures_mois);
    if div <= Decimal::ZERO { return None; }
    let retenue = (base_brut * unites_absence(methode, kind, nb_jours, div) / div).round_dp(2);

    // ── Maintien (L3141-24 II) = la retenue ──
    let montant_maintien = retenue;

    // ── Dixième (L3141-24 I) : le prorata se compte toujours en jours
    // ouvrés (25 acquis/an) ou ouvrables (30), jamais en calendaire. ──
    let kind10 = match kind {
        TypeJour::Calendaire if jours_type == "ouvrables" => TypeJour::Ouvrables,
        TypeJour::Calendaire => TypeJour::Ouvres,
        k => k,
    };
    let (jours_acquis, unite_dixieme) = match kind10 {
        TypeJour::Ouvrables => (30i64, "ouvrables"),
        _                   => (25i64, "ouvres"),
    };
    let jours10 = compter(debut, fin, kind10);
    let assiette_dixieme = (base_brut * dec!(13)).round_dp(2);
    let dixieme_total = (assiette_dixieme / dec!(10)).round_dp(2);
    // Arrondi unique en bout de chaîne pour ne pas cumuler les erreurs.
    let montant_dixieme = (assiette_dixieme / dec!(10)
        * Decimal::from(jours10) / Decimal::from(jours_acquis)).round_dp(2);

    // ── Règle la plus favorable (L3141-24 III) ──
    let indemnite = montant_maintien.max(montant_dixieme);
    let methode_indemnite = if montant_dixieme > montant_maintien { "dixieme" } else { "maintien" };

    Some(CongesPayesResult {
        retenue,
        indemnite,
        methode_indemnite: methode_indemnite.into(),
        montant_maintien,
        montant_dixieme,
        brut_mensuel: base_brut.round_dp(2),
        diviseur_retenue: div,
        jours_pris: nb_jours,
        assiette_dixieme,
        dixieme_total,
        jours_pris_dixieme: jours10,
        jours_acquis,
        unite_dixieme: unite_dixieme.into(),
        libelle: format!("congés payés · {}", libelle_methode(methode, kind)),
    })
}
