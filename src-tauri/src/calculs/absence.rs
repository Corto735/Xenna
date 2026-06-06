// Absence maladie : retenue sur salaire, maintien de salaire conventionnel
// (indemnité complémentaire employeur) et IJSS (indemnités journalières SS).
//
// Calcul purement synchrone à partir du ContextPaie (SMIC + date de paie).
// Hypothèses v1 (validées) : maladie NON professionnelle, salarié > 1 an
// d'ancienneté, subrogation (l'employeur perçoit les IJSS et les réintègre
// au net). Voir le plan pour les simplifications assumées.
//
// Modèle de carence « complet » :
//   - IJSS : versées par jour calendaire dès le 4e jour (carence SS de 3 j).
//   - Maintien employeur (IDCC 0016) : dès le 8e jour (carence conv. de 7 j),
//     90 % pendant 30 jours puis 66,66 % les 30 jours suivants.

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{AbsenceInput, AbsenceResult};

// Coefficient net des IJSS : abattement CSG (6,2 %) + CRDS (0,5 %) = 6,7 %.
const IJSS_NET_COEFF: Decimal = dec!(0.933);

// ── Jours fériés français ─────────────────────────────────────────────────────

/// Dimanche de Pâques (algorithme de Meeus/Gauss).
fn paques(annee: i32) -> NaiveDate {
    let a = annee % 19;
    let b = annee / 100;
    let c = annee % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let mois = (h + l - 7 * m + 114) / 31;
    let jour = ((h + l - 7 * m + 114) % 31) + 1;
    NaiveDate::from_ymd_opt(annee, mois as u32, jour as u32).unwrap()
}

/// Jours fériés français (métropole) pour une année donnée.
fn jours_feries(annee: i32) -> Vec<NaiveDate> {
    let ymd = |m, d| NaiveDate::from_ymd_opt(annee, m, d).unwrap();
    let p = paques(annee);
    vec![
        ymd(1, 1),                    // Jour de l'an
        ymd(5, 1),                    // Fête du travail
        ymd(5, 8),                    // Victoire 1945
        ymd(7, 14),                   // Fête nationale
        ymd(8, 15),                   // Assomption
        ymd(11, 1),                   // Toussaint
        ymd(11, 11),                  // Armistice
        ymd(12, 25),                  // Noël
        p + Duration::days(1),        // Lundi de Pâques
        p + Duration::days(39),       // Ascension
        p + Duration::days(50),       // Lundi de Pentecôte
    ]
}

// ── Comptage des jours ────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum TypeJour { Calendaire, Ouvres, Ouvrables }

/// Détermine le type de jour à compter selon la méthode et le sous-choix.
fn type_jour(methode: &str, jours_type: &str) -> TypeJour {
    match methode {
        "calendaire"          => TypeJour::Calendaire,
        "ouvrables"           => TypeJour::Ouvrables,
        "ouvres"              => TypeJour::Ouvres,
        // "moyens" / "heures" : piloté par le toggle ouvré/ouvrable.
        _ if jours_type == "ouvrables" => TypeJour::Ouvrables,
        _                              => TypeJour::Ouvres,
    }
}

/// Un jour donné est-il compté pour ce type de jour ?
fn est_compte(d: NaiveDate, kind: TypeJour, feries: &[NaiveDate]) -> bool {
    match kind {
        TypeJour::Calendaire => true,
        TypeJour::Ouvres => {
            !matches!(d.weekday(), Weekday::Sat | Weekday::Sun) && !feries.contains(&d)
        }
        TypeJour::Ouvrables => {
            d.weekday() != Weekday::Sun && !feries.contains(&d)
        }
    }
}

/// Nombre de jours comptés (du type donné) entre deux dates incluses.
fn compter(debut: NaiveDate, fin: NaiveDate, kind: TypeJour) -> i64 {
    if fin < debut { return 0; }
    // Les fériés peuvent chevaucher deux années (rare) : on charge les deux.
    let mut feries = jours_feries(debut.year());
    if fin.year() != debut.year() { feries.extend(jours_feries(fin.year())); }
    let mut n = 0i64;
    let mut cur = debut;
    while cur <= fin {
        if est_compte(cur, kind, &feries) { n += 1; }
        cur += Duration::days(1);
    }
    n
}

/// Nombre de jours dans le mois d'une date.
fn jours_du_mois(d: NaiveDate) -> i64 {
    let (y, m) = (d.year(), d.month());
    let premier_suivant = if m == 12 {
        NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(y, m + 1, 1).unwrap()
    };
    (premier_suivant - NaiveDate::from_ymd_opt(y, m, 1).unwrap()).num_days()
}

/// Diviseur mensuel selon la méthode (cf. _calcRetenue côté front).
fn diviseur(methode: &str, kind: TypeJour, debut: NaiveDate, heures_mois: f64) -> Decimal {
    match methode {
        "calendaire" => Decimal::from(jours_du_mois(debut)),
        "ouvrables"  => dec!(26),
        "ouvres"     => dec!(21.67),
        "moyens"     => if kind == TypeJour::Ouvrables { dec!(26) } else { dec!(21.67) },
        // "heures" : diviseur = jours de référence réels du mois (cf. front).
        "heures" => {
            let prem = NaiveDate::from_ymd_opt(debut.year(), debut.month(), 1).unwrap();
            let dern = prem + Duration::days(jours_du_mois(debut) - 1);
            let n = compter(prem, dern, kind);
            if n > 0 { Decimal::from(n) } else { Decimal::from_f64_retain(heures_mois).unwrap_or(dec!(151.67)) }
        }
        _ => Decimal::from(jours_du_mois(debut)),
    }
}

// ── Calcul principal ──────────────────────────────────────────────────────────

/// Calcule retenue + maintien + IJSS pour une absence maladie.
/// `base_brut` = brut mensuel plein (référence SJB et per-day). Retourne None
/// si les dates sont absentes/invalides ou si la période est vide.
pub fn compute_absence(base_brut: Decimal, abs: &AbsenceInput, ctx: &ContextPaie) -> Option<AbsenceResult> {
    let debut = NaiveDate::parse_from_str(&abs.date_debut, "%Y-%m-%d").ok()?;
    let fin   = NaiveDate::parse_from_str(&abs.date_fin,   "%Y-%m-%d").ok()?;
    if fin < debut { return None; }

    let methode    = if abs.methode.is_empty() { "moyens" } else { abs.methode.as_str() };
    let jours_type = if abs.jours_type.is_empty() { "ouvres" } else { abs.jours_type.as_str() };
    let heures_mois = abs.heures_mois.unwrap_or(151.67);
    let idcc = abs.convention_idcc.clone().unwrap_or_else(|| "0016".into());
    let kind = type_jour(methode, jours_type);

    let mut feries = jours_feries(debut.year());
    if fin.year() != debut.year() { feries.extend(jours_feries(fin.year())); }

    // ── Retenue ──
    let nb_jours = compter(debut, fin, kind);
    if nb_jours == 0 { return None; }
    let div = diviseur(methode, kind, debut, heures_mois);
    if div <= Decimal::ZERO { return None; }
    let retenue = (base_brut * Decimal::from(nb_jours) / div).round_dp(2);

    // ── Maintien IDCC 0016 (90 % puis 66,66 %), carence conventionnelle 7 j ──
    // per_day = gross moyen perdu par jour compté → indépendant de la méthode.
    let per_day = retenue / Decimal::from(nb_jours);
    let mut maintien = Decimal::ZERO;
    let mut jours_maintien = 0i64;
    if idcc == "0016" && abs.type_arret != "pro" {
        let mut cur = debut;
        let mut idx = 1i64; // index calendaire 1-based depuis le début de l'arrêt
        while cur <= fin {
            if est_compte(cur, kind, &feries) && idx > 7 {
                let j = idx - 7; // position dans la période indemnisée
                let rate = if j <= 30 { dec!(0.90) } else if j <= 60 { dec!(0.6666) } else { Decimal::ZERO };
                if rate > Decimal::ZERO {
                    maintien += rate * per_day;
                    jours_maintien += 1;
                }
            }
            cur += Duration::days(1);
            idx += 1;
        }
    }
    let maintien = maintien.round_dp(2);

    // ── IJSS (carence SS 3 j, par jour calendaire) ──
    let jours_cal = (fin - debut).num_days() + 1;
    let jours_ijss = (jours_cal - 3).max(0);
    // Plafond : 1,4 SMIC depuis le 01/04/2025, 1,8 avant.
    let coeff_plafond = if ctx.date_paie >= NaiveDate::from_ymd_opt(2025, 4, 1).unwrap() {
        dec!(1.4)
    } else {
        dec!(1.8)
    };
    let salaire_ref = base_brut.min(coeff_plafond * ctx.smic_mensuel);
    let sjb = salaire_ref * dec!(3) / dec!(91.25);
    let ijss_jour = (dec!(0.5) * sjb).round_dp(2);
    let ijss_brut = (ijss_jour * Decimal::from(jours_ijss)).round_dp(2);
    let ijss_net  = (ijss_brut * IJSS_NET_COEFF).round_dp(2);

    let libelle = format!("maladie · {}", libelle_methode(methode, kind));

    Some(AbsenceResult {
        retenue,
        maintien,
        ijss_brut,
        ijss_net,
        jours_absence: nb_jours,
        jours_ijss,
        jours_maintien,
        libelle,
        convention: format!("IDCC {idcc}"),
    })
}

fn libelle_methode(methode: &str, kind: TypeJour) -> String {
    match methode {
        "calendaire" => "jours cal.".into(),
        "heures"     => "heures réelles".into(),
        _ => match kind {
            TypeJour::Ouvrables => "÷26 ouvrables".into(),
            _                   => "÷21,67 ouvrés".into(),
        },
    }
}
