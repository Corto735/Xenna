//! Absence maladie + subrogation IJSS — garantie du net.
//!
//! Propriété centrale (neutralité) : le net à payer du bulletin avec IJSS
//! déduites en haut, ajustement du net retenu et IJSS brutes réintégrées en
//! bas doit être ÉGAL au net du bulletin de référence (assiette = base −
//! retenue + maintien, sans passage des IJSS). Sans l'ajustement, le salarié
//! gagnerait un net indu (les IJSS échappent aux cotisations).

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::SqlitePool;

use xenna_paie_lib::calculs::generer_bulletin;
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{AbsenceInput, Pays, Salarie, Statut};

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_ijss_{}_{}.db", std::process::id(), n));
    for suffixe in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{}", path.display(), suffixe));
    }
    let pool = init_db(&path).await.expect("les migrations doivent passer");
    (pool, path)
}

fn nettoyer(path: &PathBuf) {
    for suffixe in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{}", path.display(), suffixe));
    }
}

fn date(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

fn salarie_france(brut: &str) -> Salarie {
    Salarie {
        nom: "Test".into(),
        prenom: "Ijss".into(),
        salaire_brut: brut.parse().unwrap(),
        statut: Statut::NonCadre,
        alsace_moselle: false,
        pays: Pays::France,
        canton: None,
        tarif_is: None,
        assujetti_is: false,
        regione: None,
        contratto_termine: false,
        province: None,
        steuerklasse: None,
        kinderlos: None,
        land: None,
        kirchenmitglied: None,
        region_be: None,
        etp: 100.0,
        entreprise_adaptee: false,
        tranche_age_ea: None,
        heures_supp: 0.0,
        heures_comp: 0.0,
        salaire_base: None,
        effectif: Some("moins20".into()),
        anciennete: None,
        us_state: None,
    }
}

fn absence(debut: &str, fin: &str) -> AbsenceInput {
    AbsenceInput {
        type_arret: "maladie".into(),
        date_debut: debut.into(),
        date_fin: fin.into(),
        methode: "calendaire".into(),
        jours_type: String::new(),
        heures_mois: None,
        convention_idcc: Some("0016".into()),
    }
}

fn d(s: &str) -> Decimal {
    s.parse().unwrap()
}

/// Neutralité de la subrogation : net à payer == net du bulletin de référence.
#[tokio::test]
async fn neutralite_garantie_du_net() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    let abs = absence("2026-03-02", "2026-03-11"); // 10 j calendaires → 7 j d'IJSS
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&abs));
    let a = b.absence.as_ref().expect("absence calculée");

    assert!(a.ijss_brut > Decimal::ZERO, "7 jours d'IJSS attendus");
    assert_eq!(a.brut_mensuel, d("3000.00"), "mode brut : brut_mensuel = brut saisi");
    assert!(
        a.ajustement_net > Decimal::ZERO,
        "l'ajustement du net doit exister dès qu'il y a des IJSS (ajustement = {})",
        a.ajustement_net
    );

    // Bulletin de référence : assiette = base − retenue + maintien, sans IJSS.
    let assiette_ref = d("3000.00") - a.retenue + a.maintien;
    let reference = generer_bulletin(salarie_france(&assiette_ref.to_string()), &ctx, None);

    let ecart = (b.net_a_payer - reference.net_a_payer).abs();
    assert!(
        ecart <= d("0.03"),
        "garantie du net violée : net subrogé {} ≠ net de référence {} (écart {})",
        b.net_a_payer, reference.net_a_payer, ecart
    );

    // L'assiette publiée est bien référence − IJSS − ajustement.
    let attendu = assiette_ref - a.ijss_brut - a.ajustement_net;
    assert!(
        (b.brut - attendu).abs() <= d("0.01"),
        "assiette publiée {} ≠ attendue {}", b.brut, attendu
    );

    nettoyer(&path);
}

/// Réintégration en bas de bulletin : ce sont les IJSS NETTES (brutes × 0,933,
/// CSG/CRDS précomptées par la CPAM) qui alimentent le net à payer, pas les brutes.
#[tokio::test]
async fn reintegration_ijss_nettes() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    let abs = absence("2026-03-02", "2026-03-11");
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&abs));
    let a = b.absence.as_ref().unwrap();

    // ijss_net = ijss_brut × 0,933 (coefficient CSG 6,2 % + CRDS 0,5 %).
    let attendu_net = (a.ijss_brut * d("0.933")).round_dp(2);
    assert_eq!(a.ijss_net, attendu_net, "IJSS nettes = brutes × 0,933");
    assert!(a.ijss_net < a.ijss_brut, "les nettes doivent être < brutes");

    // net_a_payer = (assiette − cotisations salariales) + IJSS NETTES.
    let total_sal: Decimal = b.cotisations.iter().map(|c| c.montant_sal).sum();
    let attendu = (b.brut - total_sal + a.ijss_net).round_dp(2);
    assert_eq!(b.net_a_payer, attendu, "le net à payer réintègre les IJSS nettes");

    nettoyer(&path);
}

/// Arrêt dans la carence SS (≤ 3 j calendaires) : pas d'IJSS, pas d'ajustement,
/// flux historique inchangé.
#[tokio::test]
async fn carence_seule_sans_ajustement() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    let abs = absence("2026-03-02", "2026-03-04"); // 3 j calendaires
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&abs));
    let a = b.absence.as_ref().expect("absence calculée");

    assert_eq!(a.ijss_brut, Decimal::ZERO, "carence SS : aucune IJSS");
    assert_eq!(a.ajustement_net, Decimal::ZERO, "pas d'IJSS → pas d'ajustement");

    // Net = assiette − cotisations, sans réintégration.
    let total_sal: Decimal = b.cotisations.iter().map(|c| c.montant_sal).sum();
    assert_eq!(b.net_a_payer, (b.brut - total_sal).round_dp(2));

    nettoyer(&path);
}

/// Règle fiscale des 60 premiers jours : arrêt court → tout imposable ;
/// arrêt long → plafonné à 57 jours indemnisés (60 − 3 de carence).
#[tokio::test]
async fn ijss_imposables_regle_60_jours() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    // Court (10 j cal → 7 j d'IJSS ≤ 57) : tout est imposable.
    let court = generer_bulletin(
        salarie_france("3000.00"), &ctx, Some(&absence("2026-03-02", "2026-03-11")));
    let ac = court.absence.as_ref().unwrap();
    assert_eq!(ac.ijss_imposable, ac.ijss_brut, "arrêt court : IJSS entièrement imposables");

    // Long (75 j cal → 72 j d'IJSS > 57) : imposable plafonné à 57 jours.
    let long = generer_bulletin(
        salarie_france("3000.00"), &ctx, Some(&absence("2026-01-05", "2026-03-20")));
    let al = long.absence.as_ref().unwrap();
    assert_eq!(al.jours_ijss, 72);
    assert!(
        al.ijss_imposable < al.ijss_brut,
        "arrêt long : imposable ({}) doit être < brut ({})", al.ijss_imposable, al.ijss_brut
    );
    // ijss_imposable = ijss_jour × 57 et ijss_brut = ijss_jour × 72.
    let ratio_ok = (al.ijss_imposable * Decimal::from(72) - al.ijss_brut * Decimal::from(57)).abs()
        <= d("0.72"); // tolérance d'arrondi (ijss_jour arrondi au centime × 72)
    assert!(ratio_ok, "plafond 57 j non respecté : {} vs {}", al.ijss_imposable, al.ijss_brut);

    nettoyer(&path);
}

/// Maintien selon l'ancienneté : < 1 an → aucun maintien ; 1-3 ans → régime
/// légal (carence 7 j, 90 %) ; ≥ 3 ans → conventionnel IDCC 16 (dès le 6e jour,
/// 100 %) donc plus favorable.
#[tokio::test]
async fn maintien_selon_anciennete() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();
    let abs = absence("2026-03-02", "2026-03-11"); // 10 j calendaires

    let bulletin = |anc: Option<i64>| {
        let mut s = salarie_france("3000.00");
        s.anciennete = anc;
        generer_bulletin(s, &ctx, Some(&abs))
    };

    // < 1 an : aucun maintien.
    let b0 = bulletin(Some(0));
    let a0 = b0.absence.as_ref().unwrap();
    assert_eq!(a0.maintien, Decimal::ZERO, "ancienneté < 1 an : pas de maintien");
    assert!(a0.convention.contains("sans maintien"), "libellé : {}", a0.convention);

    // 1 à < 3 ans (et défaut None → 1 an) : régime légal, jours 8-10 à 90 %.
    let b1 = bulletin(Some(1));
    let a1 = b1.absence.as_ref().unwrap();
    assert!(a1.maintien > Decimal::ZERO);
    assert!(a1.convention.contains("légal"), "libellé : {}", a1.convention);
    let bdef = bulletin(None);
    assert_eq!(bdef.absence.as_ref().unwrap().maintien, a1.maintien, "défaut = 1 an");

    // ≥ 3 ans : conventionnel (100 % dès le 6e jour) → maintien strictement supérieur.
    let b4 = bulletin(Some(4));
    let a4 = b4.absence.as_ref().unwrap();
    assert!(a4.convention.contains("conventionnel"), "libellé : {}", a4.convention);
    assert!(
        a4.maintien > a1.maintien,
        "conventionnel ({}) doit dépasser le légal ({})", a4.maintien, a1.maintien
    );
    // Jours indemnisés : légal = jours 8-10 (3 j), conventionnel = jours 6-10 (5 j).
    assert_eq!(a1.jours_maintien, 3);
    assert_eq!(a4.jours_maintien, 5);

    nettoyer(&path);
}

/// Alsace-Moselle (droit local) : 100 % dès le 1er jour, sans carence ni
/// ancienneté ; le net à payer égale le net du salaire plein (garantie du net).
#[tokio::test]
async fn maintien_alsace_moselle() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    // Arrêt court, ancienneté 0 : l'AM ignore l'ancienneté → 100 % sur tous les
    // jours comptés, aucune carence.
    let abs = absence("2026-03-02", "2026-03-11"); // 10 j cal → 10 j comptés (calendaire)
    let mut s = salarie_france("3000.00");
    s.alsace_moselle = true;
    s.anciennete = Some(0);
    let b = generer_bulletin(s, &ctx, Some(&abs));
    let a = b.absence.as_ref().unwrap();

    assert!(a.am_local, "régime local Alsace-Moselle attendu");
    assert_eq!(a.carence_maintien, 0, "AM : aucune carence");
    assert_eq!(a.taux_maintien_t1, d("1.00"), "AM : 100 %");
    assert_eq!(a.jours_maintien_t1, a.jours_absence, "AM : tous les jours comptés à 100 %");
    assert_eq!(a.jours_maintien_t2, 0, "arrêt court : pas de relais");
    assert!(a.convention.contains("Alsace-Moselle"), "libellé : {}", a.convention);

    // Maintien = 100 % de la retenue → assiette de référence = brut plein →
    // net à payer = net du salaire plein (neutralité de la garantie du net).
    let reference = generer_bulletin(salarie_france("3000.00"), &ctx, None);
    // La cotisation maladie Alsace-Moselle s'ajoute côté salarié → on compare
    // au bulletin de référence AVEC la même option locale.
    let mut sr = salarie_france("3000.00");
    sr.alsace_moselle = true;
    let reference_am = generer_bulletin(sr, &ctx, None);
    let _ = reference; // référence hors AM non utilisée pour l'égalité stricte
    assert!(
        (b.net_a_payer - reference_am.net_a_payer).abs() <= d("0.03"),
        "AM 100 % : net subrogé {} ≠ net plein {}", b.net_a_payer, reference_am.net_a_payer
    );

    nettoyer(&path);
}

/// Alsace-Moselle, arrêt long (> 6 semaines) : 42 jours à 100 % puis relais du
/// droit commun (conventionnel 75 % pour ≥ 3 ans d'ancienneté).
#[tokio::test]
async fn maintien_alsace_moselle_relais() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    // 50 jours calendaires (02/03 → 20/04), méthode calendaire → 50 jours comptés.
    let abs = absence("2026-03-02", "2026-04-20");
    let mut s = salarie_france("3000.00");
    s.alsace_moselle = true;
    s.anciennete = Some(4); // ≥ 3 ans → relais conventionnel 75 %
    let b = generer_bulletin(s, &ctx, Some(&abs));
    let a = b.absence.as_ref().unwrap();

    assert!(a.am_local);
    assert_eq!(a.jours_maintien_t1, 42, "42 jours calendaires à 100 %");
    assert_eq!(a.taux_maintien_t1, d("1.00"));
    assert_eq!(a.jours_maintien_t2, a.jours_absence - 42, "relais sur les jours 43+");
    assert_eq!(a.taux_maintien_t2, d("0.75"), "relais conventionnel 75 %");
    assert!(a.convention.contains("relais"), "libellé : {}", a.convention);

    nettoyer(&path);
}

/// Le net imposable intègre les IJSS imposables (base PAS fidèle).
#[tokio::test]
async fn net_imposable_inclut_ijss() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    let b = generer_bulletin(
        salarie_france("3000.00"), &ctx, Some(&absence("2026-03-02", "2026-03-11")));
    let a = b.absence.as_ref().unwrap();

    let total_sal: Decimal = b.cotisations.iter().map(|c| c.montant_sal).sum();
    let csg_nd_crds: Decimal = b.cotisations.iter()
        .filter(|c| c.code == "CSG_NON_DEDUCTIBLE" || c.code == "CRDS")
        .map(|c| c.montant_sal)
        .sum();
    let attendu = (b.brut - total_sal + csg_nd_crds + a.ijss_imposable).round_dp(2);
    assert_eq!(b.net_imposable, attendu, "net imposable doit inclure les IJSS imposables");

    nettoyer(&path);
}
