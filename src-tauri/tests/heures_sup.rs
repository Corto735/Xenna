//! Heures supplémentaires et complémentaires — saisies par taux de majoration.
//!
//! Chaque tranche saisie est payée au taux horaire de BASE (salaire de base /
//! heures contractuelles) multiplié par sa majoration, sans aucun découpage
//! automatique : 12 h saisies à +25 % restent 12 h à +25 %.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::SqlitePool;

use xenna_paie_lib::calculs::generer_bulletin;
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{Pays, Salarie, Statut};

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_hs_{}_{}.db", std::process::id(), n));
    nettoyer(&path);
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

fn salarie(base: &str, etp: f64) -> Salarie {
    Salarie {
        nom: "Test".into(),
        prenom: "Heures".into(),
        salaire_brut: base.parse().unwrap(),
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
        etp,
        entreprise_adaptee: false,
        tranche_age_ea: None,
        heures_supp_25: 0.0,
        heures_supp_50: 0.0,
        heures_comp_10: 0.0,
        heures_comp_25: 0.0,
        salaire_base: Some(base.into()),
        effectif: Some("moins20".into()),
        anciennete: None,
        us_state: None,
        inde_regime: None,
        emirati_national: None,
    }
}

/// Taux horaire de base, arrondi comme le fait le moteur (4 décimales).
fn taux_horaire(base: Decimal, etp: Decimal) -> Decimal {
    (base / (dec!(151.67) * etp / dec!(100))).round_dp(4)
}

/// Temps plein : les tranches +25 % et +50 % sont prises telles quelles, chacune
/// arrondie au centime comme une ligne de bulletin.
#[tokio::test]
async fn heures_supp_par_taux() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let mut s = salarie("3000.00", 100.0);
    s.heures_supp_25 = 12.0; // au-delà des 8 h : aucun basculement vers +50 %
    s.heures_supp_50 = 3.0;
    let b = generer_bulletin(s, &ctx, None);
    let hs = b.heures_sup.expect("des heures supp sont saisies");

    let th = taux_horaire(dec!(3000), dec!(100));
    let attendu = (dec!(12) * th * dec!(1.25)).round_dp(2) + (dec!(3) * th * dec!(1.50)).round_dp(2);
    assert_eq!(hs.taux_horaire, th);
    assert_eq!(hs.h_supp_25, 12.0);
    assert_eq!(hs.h_supp_50, 3.0);
    assert_eq!(hs.gain_hs, attendu);
    assert_eq!(hs.gain_hc, Decimal::ZERO);
    assert_eq!(b.brut, dec!(3000) + attendu, "la majoration s'ajoute au brut");
    // Déduction forfaitaire patronale : sur TOUTES les heures supp, quel que soit le taux.
    assert_eq!(hs.deduction_patronale, (dec!(15) * dec!(1.50)).round_dp(2));

    nettoyer(&path);
}

/// Temps partiel : les tranches +10 % et +25 % sont prises telles quelles, au
/// taux horaire de base proratisé par l'ETP.
#[tokio::test]
async fn heures_comp_par_taux() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let mut s = salarie("2000.00", 80.0);
    s.heures_comp_10 = 20.0; // au-delà du dixième (12,13 h) : aucun basculement
    s.heures_comp_25 = 4.0;
    let b = generer_bulletin(s, &ctx, None);
    let hs = b.heures_sup.expect("des heures complémentaires sont saisies");

    let th = taux_horaire(dec!(2000), dec!(80));
    let attendu = (dec!(20) * th * dec!(1.10)).round_dp(2) + (dec!(4) * th * dec!(1.25)).round_dp(2);
    assert_eq!(hs.h_comp_10, 20.0);
    assert_eq!(hs.h_comp_25, 4.0);
    assert_eq!(hs.gain_hc, attendu);
    assert_eq!(hs.gain_hs, Decimal::ZERO);
    assert_eq!(b.brut, dec!(2000) + attendu);
    // Pas de déduction forfaitaire patronale sur les heures complémentaires.
    assert_eq!(hs.deduction_patronale, Decimal::ZERO);
    assert!(hs.reduction_salariale > Decimal::ZERO, "la réduction salariale vaut aussi pour les HC");

    nettoyer(&path);
}

/// Aucune heure saisie : pas de bloc heures sup dans le bulletin.
#[tokio::test]
async fn sans_heures() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let b = generer_bulletin(salarie("3000.00", 100.0), &ctx, None);
    assert!(b.heures_sup.is_none());
    assert_eq!(b.brut, dec!(3000));

    nettoyer(&path);
}
