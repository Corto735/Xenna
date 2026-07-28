// Absences : retenue sur salaire, maintien de salaire (indemnité complémentaire
// employeur) et IJSS (indemnités journalières SS) selon le type d'arrêt :
//   - "maladie" (non professionnelle) : IJSS 50 % dès le 4e jour (carence SS
//     3 j), maintien légal/conventionnel IDCC 0016 avec carence ;
//   - "pro" (accident du travail / maladie professionnelle) : IJSS SANS carence
//     (60 % du SJR j1-28 puis 80 %, SJR = brut ÷ 30,42 plafonné à 0,834 % du
//     PASS), maintien sans carence (D1226-3), imposables à 50 % ;
//   - "sans_solde" : retenue sèche, ni maintien ni IJSS ;
//   - "conge" : traité par crate::calculs::conges_payes (retourne None ici).
//
// Calcul purement synchrone à partir du ContextPaie (SMIC + PMSS + date de
// paie). Hypothèse commune : subrogation (l'employeur perçoit les IJSS et les
// réintègre au net, garantie du net résolue dans bulletin.rs).

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
pub(crate) fn jours_feries(annee: i32) -> Vec<NaiveDate> {
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
pub(crate) enum TypeJour { Calendaire, Ouvres, Ouvrables }

/// Détermine le type de jour à compter selon la méthode et le sous-choix.
pub(crate) fn type_jour(methode: &str, jours_type: &str) -> TypeJour {
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
pub(crate) fn est_compte(d: NaiveDate, kind: TypeJour, feries: &[NaiveDate]) -> bool {
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
pub(crate) fn compter(debut: NaiveDate, fin: NaiveDate, kind: TypeJour) -> i64 {
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
pub(crate) fn jours_du_mois(d: NaiveDate) -> i64 {
    let (y, m) = (d.year(), d.month());
    let premier_suivant = if m == 12 {
        NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(y, m + 1, 1).unwrap()
    };
    (premier_suivant - NaiveDate::from_ymd_opt(y, m, 1).unwrap()).num_days()
}

/// Diviseur mensuel selon la méthode (cf. _calcRetenue côté front).
pub(crate) fn diviseur(methode: &str, kind: TypeJour, debut: NaiveDate, heures_mois: f64) -> Decimal {
    match methode {
        "calendaire" => Decimal::from(jours_du_mois(debut)),
        "ouvrables"  => dec!(26),
        "ouvres"     => dec!(21.67),
        "moyens"     => if kind == TypeJour::Ouvrables { dec!(26) } else { dec!(21.67) },
        // "heures" : diviseur = horaire mensuel contractuel en heures
        // (151,67 h temps plein), PAS les jours réels du mois. round_dp(4)
        // gomme le bruit binaire du f64 (151,6699999…).
        "heures" => {
            let hm = Decimal::from_f64_retain(heures_mois)
                .map(|d| d.round_dp(4))
                .unwrap_or(dec!(151.67));
            if hm > Decimal::ZERO { hm } else { dec!(151.67) }
        }
        _ => Decimal::from(jours_du_mois(debut)),
    }
}

/// Heures par jour compté pour la méthode "heures réelles" : horaire
/// hebdomadaire (horaire mensuel × 12 ÷ 52) réparti sur 5 jours ouvrés ou
/// 6 ouvrables — 151,67 h/mois → 7 h/jour ouvré.
pub(crate) fn heures_par_jour(kind: TypeJour, heures_mois: Decimal) -> Decimal {
    let semaine = heures_mois * dec!(12) / dec!(52);
    match kind {
        TypeJour::Ouvrables => semaine / dec!(6),
        _                   => semaine / dec!(5),
    }
}

/// Unités d'absence rapportées au diviseur : des heures (jours × heures/jour)
/// pour la méthode "heures", des jours comptés pour toutes les autres.
pub(crate) fn unites_absence(methode: &str, kind: TypeJour, nb_jours: i64, div: Decimal) -> Decimal {
    if methode == "heures" {
        Decimal::from(nb_jours) * heures_par_jour(kind, div)
    } else {
        Decimal::from(nb_jours)
    }
}

// ── Calcul principal ──────────────────────────────────────────────────────────

/// Calcule retenue + maintien + IJSS pour une absence : maladie, AT/MP ("pro")
/// ou congé sans solde (les congés payés retournent None, voir conges_payes).
/// `base_brut` = brut mensuel plein (référence SJB/SJR et per-day). `anciennete`
/// en années entières — pilote le régime de maintien (voir bareme ci-dessous).
/// Retourne None si les dates sont absentes/invalides ou si la période est vide.
pub fn compute_absence(base_brut: Decimal, abs: &AbsenceInput, anciennete: i64, alsace_moselle: bool, ctx: &ContextPaie) -> Option<AbsenceResult> {
    // Les congés payés ont leur propre valorisation (crate::calculs::conges_payes).
    if abs.type_arret == "conge" { return None; }
    let debut = NaiveDate::parse_from_str(&abs.date_debut, "%Y-%m-%d").ok()?;
    let fin   = NaiveDate::parse_from_str(&abs.date_fin,   "%Y-%m-%d").ok()?;
    if fin < debut { return None; }

    // Bornes du mois de paie : ce bulletin ne retient (retenue, maintien, IJSS)
    // que les jours de l'arrêt tombant dans le mois de `date_paie` — un arrêt
    // pluri-mensuel est scindé sur plusieurs bulletins. Le barème de maintien et
    // les carences restent indexés depuis le VRAI 1er jour de l'arrêt (`debut`),
    // pas depuis le début du mois : la carence déjà consommée un mois antérieur
    // ne se rejoue pas. `eff_debut`/`eff_fin` = fenêtre effective de ce mois.
    let mois_debut = NaiveDate::from_ymd_opt(ctx.date_paie.year(), ctx.date_paie.month(), 1).unwrap();
    let mois_fin = {
        let (y, m) = if ctx.date_paie.month() == 12 {
            (ctx.date_paie.year() + 1, 1)
        } else {
            (ctx.date_paie.year(), ctx.date_paie.month() + 1)
        };
        NaiveDate::from_ymd_opt(y, m, 1).unwrap() - Duration::days(1)
    };
    let eff_debut = debut.max(mois_debut);
    let eff_fin   = fin.min(mois_fin);
    if eff_fin < eff_debut { return None; } // arrêt entièrement hors du mois de paie

    let methode    = if abs.methode.is_empty() { "moyens" } else { abs.methode.as_str() };
    let jours_type = if abs.jours_type.is_empty() { "ouvres" } else { abs.jours_type.as_str() };
    let heures_mois = abs.heures_mois.unwrap_or(151.67);
    // Choix de simulation du maintien : "0016" = convention transport routier ;
    // toute autre valeur (ou absente) = droit du travail général (mensualisation légale).
    let idcc = abs.convention_idcc.clone().unwrap_or_else(|| "general".into());
    let kind = type_jour(methode, jours_type);

    let mut feries = jours_feries(debut.year());
    if fin.year() != debut.year() { feries.extend(jours_feries(fin.year())); }

    // ── Retenue ── (jours de l'arrêt DANS le mois de paie uniquement)
    let nb_jours = compter(eff_debut, eff_fin, kind);
    if nb_jours == 0 { return None; }
    let div = diviseur(methode, kind, eff_debut, heures_mois);
    if div <= Decimal::ZERO { return None; }
    let retenue = (base_brut * unites_absence(methode, kind, nb_jours, div) / div).round_dp(2);

    // ── Congé sans solde : retenue sèche, sans maintien ni IJSS ──
    // Stricte proportionnalité (Cass. soc. 11 févr. 1982 ; 24 juin 1992 — la
    // méthode des heures réelles est la seule exacte, les autres sont tolérées).
    if abs.type_arret == "sans_solde" {
        return Some(AbsenceResult {
            retenue,
            maintien: Decimal::ZERO,
            ijss_brut: Decimal::ZERO,
            ijss_net: Decimal::ZERO,
            brut_mensuel: base_brut.round_dp(2),
            ijss_imposable: Decimal::ZERO,
            ajustement_net: Decimal::ZERO,
            diviseur_retenue: div,
            per_day_maintien: Decimal::ZERO,
            carence_maintien: 0,
            jours_maintien_t1: 0,
            jours_maintien_t2: 0,
            taux_maintien_t1: Decimal::ZERO,
            taux_maintien_t2: Decimal::ZERO,
            am_local: false,
            salaire_ref_ijss: Decimal::ZERO,
            coeff_plafond_ijss: Decimal::ZERO,
            sjb: Decimal::ZERO,
            ijss_jour: Decimal::ZERO,
            type_arret: abs.type_arret.clone(),
            ijss_jour_t2: Decimal::ZERO,
            jours_ijss_t1: 0,
            jours_ijss_t2: 0,
            taux_ijss_t1: Decimal::ZERO,
            taux_ijss_t2: Decimal::ZERO,
            plafond_sjr_ijss: Decimal::ZERO,
            assiette_ref: Decimal::ZERO,
            net_cible: Decimal::ZERO,
            jours_absence: nb_jours,
            jours_ijss: 0,
            jours_maintien: 0,
            frise_maintien: Vec::new(),
            frise_ijss: Vec::new(),
            net_reference: Decimal::ZERO,
            cout_reference: Decimal::ZERO,
            libelle: format!("congé sans solde · {}", libelle_methode(methode, kind)),
            convention: String::new(),
        });
    }
    let est_at = abs.type_arret == "pro";

    // ── Maintien employeur ──
    // Barème = (carence, fin1, taux1, fin2, taux2), bornes en index calendaire
    // 1-based depuis le début de l'arrêt. DEUX MODES selon le choix de simulation :
    //
    //  • Droit du travail GÉNÉRAL (mensualisation, CT art. L1226-1 / D1226-1) :
    //    ancienneté ≥ 1 an, carence 7 j (maladie) ou 0 (AT/MP), 90 % puis 66,66 %,
    //    30 j par tranche + 10 j par période de 5 ans dès la 6e année (plafond 90 j).
    //
    //  • Convention IDCC 0016 (transport routier) :
    //    maladie — 1 à < 3 ans : repli légal ; ≥ 3 ans conventionnel dès le 6e jour
    //      (100 % j6-40/70/100 puis 75 %, périodes allongées à 5 et 10 ans) ;
    //    AT/MP — garantie de ressources (100 %/75 % dès j1, sans carence) au-delà de
    //      3 ans, repli légal 90 %/66,66 % entre 1 et 3 ans.
    type Bareme = Option<(i64, i64, Decimal, i64, Decimal)>;
    let convention_16 = idcc == "0016";
    let (bareme_dc, regime_dc, conventionnel): (Bareme, &str, bool) = if est_at {
        if      convention_16 && anciennete >= 10 { (Some((0, 90, dec!(1.00), 210, dec!(0.75))), "garantie de ressources AT 100 %/75 %", true) }
        else if convention_16 && anciennete >= 5  { (Some((0, 60, dec!(1.00), 150, dec!(0.75))), "garantie de ressources AT 100 %/75 %", true) }
        else if convention_16 && anciennete >= 3  { (Some((0, 30, dec!(1.00), 90,  dec!(0.75))), "garantie de ressources AT 100 %/75 %", true) }
        else if anciennete >= 1 { let d = duree_legale(anciennete); (Some((0, d, dec!(0.90), 2 * d, dec!(0.6666))), "légal AT 90 %/66,66 % sans carence", false) }
        else { (None, "sans maintien — ancienneté < 1 an", false) }
    } else if convention_16 {
        if      anciennete >= 10 { (Some((5, 100, dec!(1.00), 190, dec!(0.75))), "conventionnel 100 % / 75 %", true) }
        else if anciennete >= 5  { (Some((5, 70,  dec!(1.00), 130, dec!(0.75))), "conventionnel 100 % / 75 %", true) }
        else if anciennete >= 3  { (Some((5, 40,  dec!(1.00), 70,  dec!(0.75))), "conventionnel 100 % / 75 %", true) }
        else if anciennete >= 1  { let d = duree_legale(anciennete); (Some((7, 7 + d, dec!(0.90), 7 + 2 * d, dec!(0.6666))), "légal 90 % / 66,66 %", false) }
        else { (None, "sans maintien — ancienneté < 1 an", false) }
    } else {
        // Droit du travail général : mensualisation légale (carence 7 j en maladie).
        if anciennete >= 1 { let d = duree_legale(anciennete); (Some((7, 7 + d, dec!(0.90), 7 + 2 * d, dec!(0.6666))), "légal — mensualisation (Code du travail)", false) }
        else { (None, "sans maintien — ancienneté < 1 an", false) }
    };

    // Alsace-Moselle (droit local, art. L1226-23, ex-art. 616 code civil local) :
    // 100 % du salaire dès le 1er jour, SANS carence ni condition d'ancienneté,
    // pendant 42 jours calendaires (6 semaines). Couvre toute absence sans faute
    // du salarié : maladie ET accident du travail.
    let am = alsace_moselle;

    let taux_bareme = |b: Bareme, idx: i64| -> Decimal {
        match b {
            Some((carence, fin1, taux1, fin2, taux2)) if idx > carence => {
                if idx <= fin1 { taux1 } else if idx <= fin2 { taux2 } else { Decimal::ZERO }
            }
            _ => Decimal::ZERO,
        }
    };
    // Barème de la mensualisation LÉGALE seule (droit du travail général), isolé du
    // barème conventionnel : il sert de terme de comparaison, pas d'appoint jour
    // par jour. Identique à `bareme_dc` quand aucune convention ne s'applique.
    let bareme_legal: Bareme = if anciennete >= 1 {
        let d = duree_legale(anciennete);
        if est_at { Some((0, d, dec!(0.90), 2 * d, dec!(0.6666))) }
        else      { Some((7, 7 + d, dec!(0.90), 7 + 2 * d, dec!(0.6666))) }
    } else {
        None
    };
    let bareme_conv: Bareme = if conventionnel { bareme_dc } else { None };

    let am_rate    = |idx: i64| -> Decimal { if am && idx <= 42 { dec!(1.00) } else { Decimal::ZERO } };
    let conv_rate  = |idx: i64| -> Decimal { taux_bareme(bareme_conv, idx) };
    let legal_rate = |idx: i64| -> Decimal { taux_bareme(bareme_legal, idx) };
    // Droits propres du salarié : droit local puis conventionnel, en relais l'un de
    // l'autre (l'Alsace-Moselle efface la carence conventionnelle sur ses 42 jours).
    let acquis_rate = |idx: i64| -> Decimal { am_rate(idx).max(conv_rate(idx)) };

    // ── Articulation des régimes (principe de faveur, comparaison GLOBALE) ──
    // On n'additionne pas les régimes et on ne prend pas le meilleur taux jour par
    // jour : le salarié épuise d'abord ses droits locaux et conventionnels, et le
    // droit du travail général n'intervient QUE s'il verse davantage sur l'ensemble
    // de l'arrêt (Cass. soc. : comparaison globale des avantages de même nature,
    // jamais de panachage). Il n'ajoute alors que le COMPLÉMENT : le salarié touche
    // le maximum des deux régimes, jamais leur somme. Conséquence pratique : un
    // salarié couvert par l'Alsace-Moselle ou par une convention ne voit presque
    // jamais la mensualisation légale s'appliquer.
    // Drapeaux de libellé : quels régimes paient effectivement des jours de l'arrêt.
    let (total_acquis, total_legal, paye_am, paye_conv, legal_couvre) = {
        let (mut a, mut l) = (Decimal::ZERO, Decimal::ZERO);
        let (mut f_am, mut f_conv, mut f_legal) = (false, false, false);
        let mut cur = debut;
        let mut idx = 1i64;
        while cur <= fin {
            if est_compte(cur, kind, &feries) {
                let (r_am, r_conv) = (am_rate(idx), conv_rate(idx));
                a += r_am.max(r_conv);
                l += legal_rate(idx);
                if r_am > Decimal::ZERO { f_am = true; }
                if r_conv > r_am { f_conv = true; }
                // Jours que le droit général couvrirait là où les droits acquis
                // sont épuisés (le relais reste conditionné à la comparaison).
                if r_am.max(r_conv).is_zero() && legal_rate(idx) > Decimal::ZERO { f_legal = true; }
            }
            cur += Duration::days(1);
            idx += 1;
        }
        (a, l, f_am, f_conv, f_legal)
    };
    let relais_legal = total_legal > total_acquis;

    // Taux effectivement dû, jour calendaire par jour calendaire (index 1-based
    // depuis le VRAI début de l'arrêt) : droits acquis tant qu'ils courent, puis
    // complément légal une fois épuisés — celui-ci s'interrompt dès que le total
    // versé atteint ce qu'aurait donné le droit général seul (`reste`, exprimé en
    // jours-équivalents comme les taux). Le dernier jour du complément est payé
    // ENTIER : le total dépasse alors le droit légal d'une fraction de journée
    // (arrondi en faveur du salarié), au bénéfice de la lisibilité — un jour au
    // taux bâtard casserait la décomposition « n jours × taux » du bulletin et des
    // frises. Une même grille sert au montant et aux frises : aucune divergence.
    let taux_par_jour: Vec<Decimal> = {
        let mut v = Vec::new();
        let mut reste = if relais_legal { total_legal - total_acquis } else { Decimal::ZERO };
        let mut cur = debut;
        let mut idx = 1i64;
        while cur <= fin {
            let mut rate = Decimal::ZERO;
            if est_compte(cur, kind, &feries) {
                let acquis = acquis_rate(idx);
                if acquis > Decimal::ZERO {
                    rate = acquis;
                } else if reste > Decimal::ZERO {
                    rate = legal_rate(idx);
                    reste -= rate;
                }
            }
            v.push(rate);
            cur += Duration::days(1);
            idx += 1;
        }
        v
    };
    let taux_jour = |idx: i64| -> Decimal {
        taux_par_jour.get((idx - 1) as usize).copied().unwrap_or(Decimal::ZERO)
    };

    // per_day = gross moyen perdu par jour compté → indépendant de la méthode.
    let per_day = retenue / Decimal::from(nb_jours);
    let mut maintien = Decimal::ZERO;
    let mut jours_maintien = 0i64;
    // `rates_arret` : taux distincts du régime sur TOUT l'arrêt (identité des
    // tranches, indépendante du mois — au plus 2 : 100 % puis 75 %/66,66 %).
    // `paliers_mois` : jours indemnisés DANS LE MOIS par taux (montants du mois).
    let mut rates_arret: Vec<Decimal> = Vec::new();
    let mut paliers_mois: Vec<(Decimal, i64)> = Vec::new();
    {
        let mut cur = debut;
        let mut idx = 1i64; // index calendaire 1-based depuis le VRAI début de l'arrêt
        while cur <= fin {
            if est_compte(cur, kind, &feries) {
                let rate = taux_jour(idx);
                if rate > Decimal::ZERO {
                    if !rates_arret.contains(&rate) { rates_arret.push(rate); }
                    if cur >= eff_debut && cur <= eff_fin {
                        maintien += rate * per_day;
                        jours_maintien += 1;
                        match paliers_mois.iter_mut().find(|(t, _)| *t == rate) {
                            Some(p) => p.1 += 1,
                            None => paliers_mois.push((rate, 1)),
                        }
                    }
                }
            }
            cur += Duration::days(1);
            idx += 1;
        }
    }
    let maintien = maintien.round_dp(2);
    // Les deux taux du régime (t1 = le plus élevé), fixés par l'ancienneté/régime
    // et donc constants d'un mois à l'autre.
    rates_arret.sort_by(|a, b| b.cmp(a));
    let taux_maintien_t1 = rates_arret.first().copied().unwrap_or(Decimal::ZERO);
    let taux_maintien_t2 = rates_arret.get(1).copied().unwrap_or(Decimal::ZERO);
    // Jours indemnisés DANS LE MOIS à chacun de ces deux taux (0 si le mois n'en
    // touche aucun jour — ex. mois entièrement en tranche réduite).
    let jours_maintien_t1 = paliers_mois.iter().find(|(t, _)| *t == taux_maintien_t1).map(|(_, n)| *n).unwrap_or(0);
    let jours_maintien_t2 = paliers_mois.iter().find(|(t, _)| *t == taux_maintien_t2).map(|(_, n)| *n).unwrap_or(0);
    // Carence opposable = celle du régime qui ouvre les droits (le droit local n'en
    // a aucune ; sinon la convention, à défaut la mensualisation légale).
    let carence_maintien = if am {
        0
    } else if conventionnel {
        bareme_conv.map(|(c, ..)| c).unwrap_or(0)
    } else {
        bareme_legal.map(|(c, ..)| c).unwrap_or(0)
    };

    // Libellé du régime appliqué (affiché sur la ligne « Maintien de salaire ») :
    // les régimes réellement payeurs, dans l'ordre où ils prennent le relais.
    let regime: String = {
        let mut segs: Vec<String> = Vec::new();
        if paye_am   { segs.push("Alsace-Moselle — droit local 100 % (6 sem.), art. L1226-23".into()); }
        if paye_conv { segs.push(if paye_am { format!("relais {regime_dc}") } else { regime_dc.to_string() }); }
        if relais_legal && legal_couvre {
            segs.push(if paye_am || paye_conv { "relais légal 90 % / 66,66 %".into() } else { regime_dc.to_string() });
        }
        if segs.is_empty() { regime_dc.to_string() } else { segs.join(" + ") }
    };
    // Préfixe IDCC seulement quand un barème conventionnel (ou le droit local)
    // s'applique — « légal » et « sans maintien » ne relèvent pas de la convention.
    // Préfixe IDCC seulement en mode convention IDCC 0016 (barème conventionnel ou
    // relais Alsace-Moselle). En droit général, jamais de préfixe IDCC.
    let convention = if convention_16 && (conventionnel || am) {
        format!("IDCC {idcc} · {regime}")
    } else {
        regime.clone()
    };

    // ── IJSS (par jour calendaire) ──
    // Longueur totale de l'arrêt (pour les frises) et index GLOBAL, 1-based depuis
    // le vrai début, des 1er/dernier jours du mois de paie : carence SS, tranches
    // et règle fiscale des 60 jours se comptent depuis le début RÉEL de l'arrêt,
    // mais seuls les jours du mois sont indemnisés/valorisés sur ce bulletin.
    let jours_cal = (fin - debut).num_days() + 1;
    let idx_lo = (eff_debut - debut).num_days() + 1;
    let idx_hi = (eff_fin - debut).num_days() + 1;
    // ── Fenêtre de subrogation ──
    // Les IJSS figurent sur le bulletin de la fin de la carence SS JUSQU'À LA FIN DU
    // MAINTIEN de salaire. La carence de maintien (début) ne les suspend PAS : la
    // carence SS n'étant que de 3 jours, les IJSS sont déjà versées pendant la
    // carence de maintien. C'est seulement à la FIN du maintien que la subrogation
    // cesse ; au-delà (et s'il n'y a aucun maintien), la CPAM verse directement au
    // salarié — les IJSS ne passent plus par le bulletin. Bornes en index calendaire :
    //   début = fin de carence SS (jour 4 en maladie, jour 1 en AT/MP) ;
    //   fin   = dernier jour de maintien (fin de tranche 2 du barème ; Alsace-Moselle :
    //           42 jours, ou relais du droit commun au-delà). Pas de maintien → fin 0 → 0.
    // Fin du maintien = dernier jour du dernier régime qui paie réellement. Sans
    // droit local ni conventionnel, c'est la fin du barème légal (le droit peut
    // courir au-delà de l'arrêt saisi) ; sinon, le dernier jour effectivement payé
    // de la grille — le complément légal, plafonné, s'arrête avant la fin du barème.
    let fin_am   = if am { 42 } else { 0 };
    let fin_conv = bareme_conv.map(|(_, _, _, f2, _)| f2).unwrap_or(0);
    let maintien_end_idx = if fin_am == 0 && fin_conv == 0 {
        bareme_legal.map(|(_, _, _, f2, _)| f2).unwrap_or(0)
    } else {
        let dernier_paye = taux_par_jour.iter().rposition(|t| *t > Decimal::ZERO)
            .map(|p| p as i64 + 1).unwrap_or(0);
        fin_am.max(fin_conv).max(dernier_paye)
    };
    // Carence SS (3 j en maladie, aucune en AT/MP). Fenêtre des IJSS sur le bulletin
    // = mois ∩ post-carence SS ∩ [1 ; fin du maintien] (vide → aucune IJSS).
    let ss_carence = if est_at { 0 } else { 3 };
    let ijss_lo = idx_lo.max(ss_carence + 1);
    let ijss_hi = idx_hi.min(maintien_end_idx);
    let (jours_ijss, sjb, salaire_ref, coeff_plafond, plafond_sjr,
         ijss_jour, ijss_jour_t2, jours_t1, jours_t2, taux_t1, taux_t2,
         ijss_brut, ijss_imposable);
    if est_at {
        // AT/MP : SANS carence — IJ dès le 1er jour (CSS, versement dès le
        // lendemain de l'accident ; le jour même est payé par l'employeur, la
        // période saisie démarre au 1er jour indemnisé).
        // SJR = brut mensuel ÷ 30,42, plafonné à 0,834 % du PASS annuel
        // (12 × PMSS) → IJ max 2026 : 240,49 € (60 %) / 320,66 € (80 %).
        plafond_sjr = ctx.pmss * dec!(12) * dec!(0.00834);
        let sjr = (base_brut / dec!(30.42)).min(plafond_sjr);
        sjb = sjr;
        salaire_ref = base_brut;
        coeff_plafond = Decimal::ZERO; // plafond SMIC : sans objet en AT/MP
        taux_t1 = dec!(0.60); // j1 à j28
        taux_t2 = dec!(0.80); // dès le 29e jour
        ijss_jour    = (taux_t1 * sjr).round_dp(2);
        ijss_jour_t2 = (taux_t2 * sjr).round_dp(2);
        // Tranches par index global (60 % j1-28, 80 % dès j29), bornées au mois et à la
        // FIN du maintien : au-delà, la CPAM verse directement et ces jours ne figurent
        // pas sur le bulletin. Sans maintien → fenêtre vide → 0.
        jours_t1 = (ijss_hi.min(28) - ijss_lo.max(1) + 1).max(0);
        jours_t2 = (ijss_hi - ijss_lo.max(29) + 1).max(0);
        jours_ijss = jours_t1 + jours_t2;
        ijss_brut = (ijss_jour * Decimal::from(jours_t1)
                   + ijss_jour_t2 * Decimal::from(jours_t2)).round_dp(2);
        // IJ AT/MP imposables à hauteur de 50 % de leur montant (pas de règle
        // des 60 jours, contrairement à la maladie).
        ijss_imposable = (ijss_brut * dec!(0.5)).round_dp(2);
    } else {
        // Maladie : carence SS de 3 jours calendaires (index global 1-3).
        // Plafond : 1,4 SMIC depuis le 01/04/2025, 1,8 avant.
        coeff_plafond = if ctx.date_paie >= NaiveDate::from_ymd_opt(2025, 4, 1).unwrap() {
            dec!(1.4)
        } else {
            dec!(1.8)
        };
        salaire_ref = base_brut.min(coeff_plafond * ctx.smic_mensuel);
        sjb = salaire_ref * dec!(3) / dec!(91.25);
        taux_t1 = dec!(0.5);
        taux_t2 = Decimal::ZERO;
        plafond_sjr = Decimal::ZERO;
        ijss_jour = (taux_t1 * sjb).round_dp(2);
        ijss_jour_t2 = Decimal::ZERO;
        // Jours indemnisés sur le bulletin = post-carence SS jusqu'à la fin du maintien
        // (∩ mois). Au-delà de la fin du maintien (ou sans maintien), la CPAM verse
        // directement → ces jours ne figurent pas ici. Pas de maintien → fenêtre vide.
        jours_ijss = (ijss_hi - ijss_lo + 1).max(0);
        jours_t1 = jours_ijss;
        jours_t2 = 0;
        ijss_brut = (ijss_jour * Decimal::from(jours_ijss)).round_dp(2);
        // IJSS imposables (base PAS) : maladie imposable sur les 60 premiers jours
        // d'arrêt uniquement → index global 4..=60, dans la fenêtre de subrogation.
        let jours_imposables = (ijss_hi.min(60) - ijss_lo + 1).max(0);
        ijss_imposable = (ijss_jour * Decimal::from(jours_imposables)).round_dp(2).min(ijss_brut);
    }
    let ijss_net = (ijss_brut * IJSS_NET_COEFF).round_dp(2);

    let prefixe = if est_at { "AT/MP" } else { "maladie" };
    let libelle = format!("{prefixe} · {}", libelle_methode(methode, kind));

    // ── Frises jour par jour (une case = un jour calendaire) ──
    // Maintien : reclasse chaque jour selon est_compte + taux effectif (même
    // logique que la boucle d'accumulation). La carence (calendaire) reste
    // "carence" même les week-ends ; les jours non comptés ou hors barème → "hors".
    let mut frise_maintien: Vec<String> = Vec::with_capacity(jours_cal as usize);
    {
        let mut cur = debut;
        let mut idx = 1i64;
        while cur <= fin {
            let rate = taux_jour(idx);
            let code = if est_compte(cur, kind, &feries) && rate > Decimal::ZERO {
                if rate == taux_maintien_t1 { "t1" } else { "t2" }
            } else if !am && carence_maintien > 0 && idx <= carence_maintien {
                "carence"
            } else {
                "hors"
            };
            frise_maintien.push(code.to_string());
            cur += Duration::days(1);
            idx += 1;
        }
    }
    // IJSS : jours calendaires purs, classés par index GLOBAL (indépendant du mois
    // de paie ; le front encadre les jours effectivement retenus ce mois). Les IJSS
    // apparaissent de la carence SS JUSQU'À LA FIN DU MAINTIEN (subrogation) ; au-delà
    // (ou sans maintien), la CPAM verse directement → "hors" (grisé). Maladie :
    // carence 3 j puis 50 % ; AT/MP : 60 % j1-28 puis 80 % dès j29.
    let mut frise_ijss: Vec<String> = Vec::with_capacity(jours_cal as usize);
    for idx in 1..=jours_cal {
        let subroge = idx <= maintien_end_idx; // la subrogation cesse à la fin du maintien
        let code = if est_at {
            if subroge {
                if idx <= 28 { "ijss1" } else { "ijss2" }
            } else { "hors" }
        } else if idx <= 3 {
            "carence"
        } else if subroge {
            "ijss1"
        } else {
            "hors"
        };
        frise_ijss.push(code.to_string());
    }

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
        type_arret: abs.type_arret.clone(),
        ijss_jour_t2,
        jours_ijss_t1: jours_t1,
        jours_ijss_t2: jours_t2,
        taux_ijss_t1: taux_t1,
        taux_ijss_t2: taux_t2,
        plafond_sjr_ijss: plafond_sjr.round_dp(2),
        assiette_ref: Decimal::ZERO, // rempli par le bulletin France
        net_cible: Decimal::ZERO,    // rempli par le bulletin France
        jours_absence: nb_jours,
        jours_ijss,
        jours_maintien,
        frise_maintien,
        frise_ijss,
        net_reference: Decimal::ZERO,  // rempli par le bulletin France
        cout_reference: Decimal::ZERO, // rempli par le bulletin France
        libelle,
        convention,
    })
}

/// Durée légale d'indemnisation de CHAQUE tranche (mensualisation, CT art. D1226-1) :
/// 30 jours de base, + 10 jours par période de 5 ans d'ancienneté à partir de la
/// 6e année, plafonnée à 90 jours (atteinte à 31 ans et plus).
///   1–5 ans → 30 · 6–10 → 40 · 11–15 → 50 · … · 31 ans et + → 90.
fn duree_legale(anciennete: i64) -> i64 {
    30 + ((anciennete - 1) / 5).clamp(0, 6) * 10
}

pub(crate) fn libelle_methode(methode: &str, kind: TypeJour) -> String {
    match methode {
        "calendaire" => "jours cal.".into(),
        "heures"     => "heures réelles".into(),
        _ => match kind {
            TypeJour::Ouvrables => "÷26 ouvrables".into(),
            _                   => "÷21,67 ouvrés".into(),
        },
    }
}
