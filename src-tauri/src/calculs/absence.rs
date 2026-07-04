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
/// `base_brut` = brut mensuel plein (référence SJB et per-day). `anciennete` en
/// années entières — pilote le régime de maintien (voir bareme ci-dessous).
/// Retourne None si les dates sont absentes/invalides ou si la période est vide.
pub fn compute_absence(base_brut: Decimal, abs: &AbsenceInput, anciennete: i64, alsace_moselle: bool, ctx: &ContextPaie) -> Option<AbsenceResult> {
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

    // ── Maintien employeur (maladie non professionnelle) ──
    // Barème DROIT COMMUN par ancienneté / IDCC 0016 (= (carence, fin1, taux1,
    // fin2, taux2), bornes en index calendaire 1-based depuis le début) :
    //   < 1 an : aucun maintien.
    //   1 à < 3 ans : régime légal de mensualisation (CT art. L1226-1 / D1226-1)
    //     — carence 7 j, 90 % pendant 30 j puis 66,66 % pendant 30 j.
    //   ≥ 3 ans : conventionnel IDCC 0016, dès le 6e jour :
    //     ≥ 3 ans  : 100 % j6-40,  puis 75 % j41-70 ;
    //     ≥ 5 ans  : 100 % j6-70,  puis 75 % j71-130 ;
    //     ≥ 10 ans : 100 % j6-100, puis 75 % j101-190.
    let bareme_dc: Option<(i64, i64, Decimal, i64, Decimal)> =
        if idcc == "0016" && abs.type_arret != "pro" {
            if      anciennete >= 10 { Some((5, 100, dec!(1.00), 190, dec!(0.75))) }
            else if anciennete >= 5  { Some((5, 70,  dec!(1.00), 130, dec!(0.75))) }
            else if anciennete >= 3  { Some((5, 40,  dec!(1.00), 70,  dec!(0.75))) }
            else if anciennete >= 1  { Some((7, 37,  dec!(0.90), 67,  dec!(0.6666))) }
            else { None }
        } else {
            None
        };

    // Alsace-Moselle (droit local, art. L1226-23, ex-art. 616 code civil local) :
    // 100 % du salaire dès le 1er jour, SANS carence ni condition d'ancienneté,
    // pendant 42 jours calendaires (6 semaines), puis relais du droit commun.
    let am = alsace_moselle && abs.type_arret != "pro";

    // Taux du jour = max(Alsace-Moselle, droit commun). Le max réalise le relais :
    // jours 1-42 à 100 % (AM l'emporte et efface la carence du droit commun),
    // au-delà le droit commun reprend (75 % / 66,66 %).
    let dc_rate = |idx: i64| -> Decimal {
        match bareme_dc {
            Some((carence, fin1, taux1, fin2, taux2)) if idx > carence => {
                if idx <= fin1 { taux1 } else if idx <= fin2 { taux2 } else { Decimal::ZERO }
            }
            _ => Decimal::ZERO,
        }
    };
    let am_rate = |idx: i64| -> Decimal {
        if am && idx <= 42 { dec!(1.00) } else { Decimal::ZERO }
    };

    // per_day = gross moyen perdu par jour compté → indépendant de la méthode.
    let per_day = retenue / Decimal::from(nb_jours);
    let mut maintien = Decimal::ZERO;
    let mut jours_maintien = 0i64;
    // Paliers (taux, nb jours) — au plus 2 taux distincts (100 % puis 75 %/66,66 %).
    let mut paliers: Vec<(Decimal, i64)> = Vec::new();
    {
        let mut cur = debut;
        let mut idx = 1i64; // index calendaire 1-based depuis le début de l'arrêt
        while cur <= fin {
            if est_compte(cur, kind, &feries) {
                let rate = am_rate(idx).max(dc_rate(idx));
                if rate > Decimal::ZERO {
                    maintien += rate * per_day;
                    jours_maintien += 1;
                    match paliers.iter_mut().find(|(t, _)| *t == rate) {
                        Some(p) => p.1 += 1,
                        None => paliers.push((rate, 1)),
                    }
                }
            }
            cur += Duration::days(1);
            idx += 1;
        }
    }
    let maintien = maintien.round_dp(2);
    // Deux tranches pour le panneau f(x) : taux décroissant (t1 = plus élevé).
    paliers.sort_by(|a, b| b.0.cmp(&a.0));
    let (taux_maintien_t1, jours_maintien_t1) = paliers.first().copied().unwrap_or((Decimal::ZERO, 0));
    let (taux_maintien_t2, jours_maintien_t2) = paliers.get(1).copied().unwrap_or((Decimal::ZERO, 0));
    let carence_maintien = if am { 0 } else { bareme_dc.map(|(c, ..)| c).unwrap_or(0) };

    // Libellé du régime appliqué (affiché sur la ligne « Maintien de salaire »).
    let regime: String = if am {
        let mut s = "Alsace-Moselle — droit local 100 % (6 sem.), art. L1226-23".to_string();
        if jours_maintien_t2 > 0 { s.push_str(" + relais droit commun"); }
        s
    } else {
        match bareme_dc {
            Some((5, ..)) => "conventionnel 100 % / 75 %",
            Some(_)       => "légal 90 % / 66,66 %",
            None if anciennete < 1 => "sans maintien — ancienneté < 1 an",
            None          => "sans maintien",
        }.to_string()
    };

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
    // IJSS imposables (base PAS) : maladie imposable sur les 60 premiers jours
    // d'arrêt uniquement → 60 − 3 j de carence = 57 jours indemnisés au plus.
    let jours_imposables = jours_ijss.min(57);
    let ijss_imposable = (ijss_jour * Decimal::from(jours_imposables)).round_dp(2).min(ijss_brut);

    let libelle = format!("maladie · {}", libelle_methode(methode, kind));

    Some(AbsenceResult {
        retenue,
        maintien,
        ijss_brut,
        ijss_net,
        brut_mensuel: base_brut.round_dp(2),
        ijss_imposable,
        // Rempli par le bulletin France après résolution de la garantie du net.
        ajustement_net: Decimal::ZERO,
        diviseur_retenue: div,
        per_day_maintien: per_day.round_dp(2),
        carence_maintien,
        jours_maintien_t1,
        jours_maintien_t2,
        taux_maintien_t1,
        taux_maintien_t2,
        am_local: am,
        salaire_ref_ijss: salaire_ref.round_dp(2),
        coeff_plafond_ijss: coeff_plafond,
        sjb: sjb.round_dp(2),
        ijss_jour,
        assiette_ref: Decimal::ZERO, // rempli par le bulletin France
        net_cible: Decimal::ZERO,    // rempli par le bulletin France
        jours_absence: nb_jours,
        jours_ijss,
        jours_maintien,
        libelle,
        convention: format!("IDCC {idcc} · {regime}"),
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
