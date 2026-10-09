// Alertes de cohérence (France, secteur privé).
//
// Un simulateur calcule ce qu'on lui donne, même un salaire illégal. Ces
// contrôles signalent ce qu'un gestionnaire vérifierait d'un coup d'œil, avec
// le texte qui fonde la règle. Ils n'empêchent rien et ne changent aucun
// montant. Seuils en base (SMIC_HORAIRE, TEMPS_PARTIEL_MIN_HEBDO, 0142).
//
// Minimum conventionnel : IDCC 0016 seulement, quand le salarié est classé
// (branche, catégorie, coefficient), sur la table ccn_minima (0146). Contrôle
// mensuel du taux ou du salaire garanti ; la garantie annuelle (GAR/RAG) se
// juge sur l'année et reste hors champ.
//
// Volontairement absent : le plafond annuel d'exonération des heures
// supplémentaires (il faudrait le cumul de l'année).

use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Alerte, Salarie};

const HEURES_TEMPS_PLEIN: Decimal = dec!(151.67);

/// Nombre à la française dans un texte : virgule décimale, zéros inutiles ôtés.
fn fr(d: Decimal) -> String {
    d.normalize().to_string().replace('.', ",")
}

/// Montant en euros : toujours deux décimales (« 1 867,10 », pas « 1867,1 »).
fn euros(d: Decimal) -> String {
    format!("{:.2}", d.round_dp(2)).replace('.', ",")
}
const HEBDO_TEMPS_PLEIN: Decimal = dec!(35);

pub fn controler(s: &Salarie, ctx: &ContextPaie) -> Vec<Alerte> {
    let mut alertes = Vec::new();
    let etp = Decimal::from_f64(s.etp / 100.0).unwrap_or(Decimal::ONE).round_dp(6);
    if etp <= Decimal::ZERO {
        return alertes;
    }

    // 1. Salaire de base au moins égal au SMIC, au prorata du temps de travail.
    // Les travailleurs d'ESAT n'ont pas de contrat de travail : leur
    // rémunération garantie obéit à d'autres règles.
    if !s.esat {
        if let Some(smic_h) = ctx.plafond("SMIC_HORAIRE") {
            let base = s.salaire_base.as_deref()
                .and_then(|v| v.parse::<Decimal>().ok())
                .filter(|v| *v > Decimal::ZERO)
                .unwrap_or(s.salaire_brut);
            let heures = HEURES_TEMPS_PLEIN * etp;
            let taux_h = (base / heures).round_dp(4);
            // Minimum mensuel = SMIC MENSUEL officiel au prorata (calculé sur
            // 35 h × 52/12, il diffère de quelques centimes de 151,67 × horaire).
            let minimum = (ctx.smic_mensuel * etp).round_dp(2);
            if base + dec!(0.005) < minimum {
                let valeurs = [
                    ("taux_h", euros(taux_h)), ("heures", fr(heures.round_dp(2))),
                    ("smic_h", euros(smic_h)), ("minimum", euros(minimum)),
                ].into_iter().map(|(k, v)| (k.to_string(), v)).collect();
                alertes.push(Alerte {
                    code: "smic".into(),
                    valeurs,
                    niveau: "alerte".into(),
                    titre: "Salaire de base inférieur au SMIC".into(),
                    texte: format!(
                        "Le salaire de base revient à {} € de l'heure pour {} heures par mois, \
                         sous le SMIC horaire de {} €. Le minimum pour ce temps de travail est de {} € bruts. \
                         Seuls quelques cas y échappent (apprentis, jeunes de moins de 18 ans avec abattement).",
                        euros(taux_h), fr(heures.round_dp(2)), euros(smic_h), euros(minimum)),
                    source: "Code du travail, art. L3231-2".into(),
                });
            }
        }
    }

    // 2. Minimum conventionnel du coefficient, au prorata du temps de travail.
    if !s.esat {
        if let Some(a) = minimum_conventionnel(s, ctx, etp) {
            alertes.push(a);
        }
    }

    // 3. Temps partiel sous la durée minimale légale.
    if let Some(min_hebdo) = ctx.plafond("TEMPS_PARTIEL_MIN_HEBDO") {
        let hebdo = (HEBDO_TEMPS_PLEIN * etp).round_dp(2);
        if etp < Decimal::ONE && hebdo < min_hebdo {
            let valeurs = [("hebdo", fr(hebdo)), ("min_hebdo", fr(min_hebdo))]
                .into_iter().map(|(k, v)| (k.to_string(), v)).collect();
            alertes.push(Alerte {
                code: "temps_partiel".into(),
                valeurs,
                niveau: "info".into(),
                titre: "Temps partiel sous la durée minimale".into(),
                texte: format!(
                    "{} heures par semaine, sous la durée minimale de {} heures. C'est possible \
                     sur demande écrite et motivée du salarié, pour un étudiant de moins de 26 ans, ou si un \
                     accord de branche fixe une durée inférieure.", fr(hebdo), fr(min_hebdo)),
                source: "Code du travail, art. L3123-27 et L3123-7".into(),
            });
        }
    }
    alertes
}

fn minimum_conventionnel(s: &Salarie, ctx: &ContextPaie, etp: Decimal) -> Option<Alerte> {
    let idcc = s.convention_idcc.as_deref()?;
    let (Some(branche), Some(categorie), Some(coef)) =
        (s.ccn_branche.as_deref(), s.ccn_categorie.as_deref(), s.ccn_coefficient.as_deref())
    else { return None };
    let alerte = |code: &str, niveau: &str, titre: &str, texte: String, source: String, valeurs: Vec<(&str, String)>| Alerte {
        code: code.into(),
        valeurs: valeurs.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
        niveau: niveau.into(),
        titre: titre.into(),
        texte,
        source,
    };

    // Palier retenu : le plus élevé que l'ancienneté atteint (1 an par défaut,
    // comme le maintien de salaire).
    let anc_mois = s.anciennete.unwrap_or(1).max(0) * 12;
    let paliers: Vec<_> = ctx.ccn_minima.iter()
        .filter(|m| m.idcc == idcc && m.branche == branche && m.categorie == categorie && m.coefficient == coef)
        .collect();
    let Some(m) = paliers.iter().filter(|m| m.anciennete_mois <= anc_mois).max_by_key(|m| m.anciennete_mois) else {
        return Some(alerte("ccn_inconnu", "info", "Minimum conventionnel non contrôlé",
            format!("Aucune grille en base pour le coefficient {coef} à la date de paie (les grilles \
                     recopiées ne remontent qu'à leur dernier avenant) : pas de contrôle."),
            "Convention collective IDCC 0016".into(),
            vec![("coef", coef.to_string())]));
    };

    let (taux_h, plein) = if m.unite == "horaire" {
        (m.montant, m.montant * HEURES_TEMPS_PLEIN)
    } else {
        ((m.montant / m.heures_base).round_dp(4), m.montant)
    };
    let minimum = (plein * etp).round_dp(2);
    let smic = (ctx.smic_mensuel * etp).round_dp(2);
    let base = s.salaire_base.as_deref()
        .and_then(|v| v.parse::<Decimal>().ok())
        .filter(|v| *v > Decimal::ZERO)
        .unwrap_or(s.salaire_brut);
    let palier = fr(Decimal::from(m.anciennete_mois) / dec!(12));
    let mut source = m.source.clone();
    if categorie == "cadres" {
        source.push_str(" ; ancienneté appréciée dans le groupe (art. 5 CCNA 4), assimilée ici à l'ancienneté dans l'entreprise");
    }
    let valeurs = vec![
        ("coef", coef.to_string()), ("taux_h", euros(taux_h)), ("minimum", euros(minimum)),
        ("palier", palier.clone()), ("base", euros(base)), ("smic", euros(smic)),
        ("ecart", euros(minimum - base)),
    ];

    if minimum <= smic {
        return Some(alerte("ccn_sous_smic", "info", "Minimum conventionnel rattrapé par le SMIC",
            format!("Coefficient {coef} : le minimum conventionnel ({} € pour ce temps de travail) est \
                     inférieur au SMIC ({} €). C'est le SMIC qui s'applique : la grille de la branche n'a pas \
                     suivi la revalorisation du salaire minimum.", euros(minimum), euros(smic)),
            source, valeurs));
    }
    let texte_min = format!("Coefficient {coef} : minimum conventionnel de {} € de l'heure, soit {} € bruts \
                             par mois pour ce temps de travail (palier d'ancienneté : {palier} an(s)).",
                            euros(taux_h), euros(minimum));
    if base + dec!(0.005) < minimum {
        Some(alerte("ccn_sous_minimum", "alerte", "Salaire de base inférieur au minimum conventionnel",
            format!("{texte_min} Le salaire de base ({} €) est inférieur de {} €.", euros(base), euros(minimum - base)),
            source, valeurs))
    } else {
        Some(alerte("ccn_conforme", "info", "Minimum conventionnel respecté",
            format!("{texte_min} Le salaire de base ({} €) le respecte. La garantie annuelle de \
                     rémunération, elle, se vérifie sur l'année civile.", euros(base)),
            source, valeurs))
    }
}
