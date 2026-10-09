//! Alertes de cohérence (calculs/alertes.rs, migration 0142) : salaire de
//! base sous le SMIC au prorata du temps de travail, temps partiel sous 24 h.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use sqlx::SqlitePool;

use xenna_paie_lib::calculs::generer_bulletin;
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{Pays, Salarie, Statut};

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_alertes_{}_{}.db", std::process::id(), n));
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

fn salarie_base(pays: Pays, brut: &str) -> Salarie {
    Salarie {
        nom: "Test".into(),
        prenom: "Inverse".into(),
        salaire_brut: brut.parse().unwrap(),
        statut: Statut::NonCadre,
        alsace_moselle: false,
        pays,
        canton: Some("GE".into()),
        tarif_is: None,
        assujetti_is: false,
        regione: Some("LO".into()),
        contratto_termine: false,
        province: Some("ON".into()),
        steuerklasse: Some(1),
        kinderlos: Some(false),
        land: Some("BY".into()),
        kirchenmitglied: Some(false),
        region_be: Some("bruxelles".into()),
        etp: 100.0,
        entreprise_adaptee: false,
        esat: false,
        convention_idcc: None,
        indemnites_repas: None,
        avantages_nature: Vec::new(),
        tranche_age_ea: None,
        heures_supp_25: 0.0,
        heures_supp_50: 0.0,
        heures_comp_10: 0.0,
        heures_comp_25: 0.0,
        heures_struct_25: 0.0,
        heures_struct_50: 0.0,
        salaire_base: None,
        effectif: Some("moins20".into()),
        anciennete: None,
        taux_pas: None,
        pas_zone: None,
        contrat_court: false,
        ccn_branche: None, ccn_categorie: None, ccn_coefficient: None,
        us_state: None,
        inde_regime: None,
        emirati_national: None,
    }
}




async fn alertes(s: Salarie, date_paie: &str) -> Vec<String> {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date(date_paie)).await.unwrap();
    let a = generer_bulletin(s, &ctx, None).alertes.into_iter().map(|a| a.titre).collect();
    nettoyer(&path);
    a
}

#[tokio::test]
async fn sous_le_smic() {
    let t = alertes(salarie_base(Pays::France, "1500"), "2026-06-30").await;
    assert!(t.iter().any(|x| x.contains("SMIC")), "{t:?}");
}

#[tokio::test]
async fn salaire_normal_sans_alerte() {
    assert!(alertes(salarie_base(Pays::France, "3200"), "2026-06-30").await.is_empty());
}

/// Mi-temps à 1 000 € : au-dessus du SMIC au prorata, mais 17,5 h < 24 h.
#[tokio::test]
async fn temps_partiel_court() {
    let mut s = salarie_base(Pays::France, "1000");
    s.etp = 50.0;
    let t = alertes(s, "2026-06-30").await;
    assert_eq!(t, vec!["Temps partiel sous la durée minimale".to_string()]);
}

/// ESAT : pas de contrat de travail, pas de contrôle SMIC ; étranger : rien.
#[tokio::test]
async fn hors_champ() {
    let mut s = salarie_base(Pays::France, "900");
    s.esat = true;
    assert!(!alertes(s, "2026-06-30").await.iter().any(|x| x.contains("SMIC")));
    assert!(alertes(salarie_base(Pays::Suisse, "900"), "2026-06-30").await.is_empty());
}

/// Exactement le SMIC mensuel officiel de juin 2026 (1 867,02 €, et non
/// 12,31 × 151,67 = 1 867,06) : aucune alerte.
#[tokio::test]
async fn smic_officiel_exact() {
    assert!(alertes(salarie_base(Pays::France, "1867.02"), "2026-06-30").await.is_empty());
}

/// Le SMIC de 11,88 € s'applique dès le 1er novembre 2024 (migration 0143).
#[tokio::test]
async fn smic_novembre_2024() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2024-11-30")).await.unwrap();
    assert_eq!(ctx.plafond("SMIC_HORAIRE"), Some("11.88".parse().unwrap()));
    assert_eq!(ctx.smic_mensuel, "1801.80".parse().unwrap());
    nettoyer(&path);
}

/// SMIC 2021 : 10,25 € jusqu'au 30 septembre, 10,48 € dès le 1er octobre
/// (arrêté du 27/09/2021, migration 0145).
#[tokio::test]
async fn smic_octobre_2021() {
    let (pool, path) = base_test().await;
    let sept = ContextPaie::charger(&pool, date("2021-09-30")).await.unwrap();
    let oct = ContextPaie::charger(&pool, date("2021-10-01")).await.unwrap();
    assert_eq!(sept.plafond("SMIC_HORAIRE"), Some("10.25".parse().unwrap()));
    assert_eq!(sept.smic_mensuel, "1554.58".parse().unwrap());
    assert_eq!(oct.plafond("SMIC_HORAIRE"), Some("10.48".parse().unwrap()));
    assert_eq!(oct.smic_mensuel, "1589.47".parse().unwrap());
    nettoyer(&path);
}

// ── Minimum conventionnel IDCC 0016 (ccn_minima, migration 0146) ─────────────

fn classe(brut: &str, branche: &str, categorie: &str, coef: &str, anciennete: i64) -> Salarie {
    let mut s = salarie_base(Pays::France, brut);
    s.convention_idcc = Some("0016".into());
    s.ccn_branche = Some(branche.into());
    s.ccn_categorie = Some(categorie.into());
    s.ccn_coefficient = Some(coef.into());
    s.anciennete = Some(anciennete);
    s
}

async fn alertes_ccn(s: Salarie, date_paie: &str) -> Vec<(String, std::collections::BTreeMap<String, String>)> {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date(date_paie)).await.unwrap();
    let a = generer_bulletin(s, &ctx, None).alertes.into_iter()
        .filter(|a| a.code.starts_with("ccn_"))
        .map(|a| (a.code, a.valeurs)).collect();
    nettoyer(&path);
    a
}

/// Voyageurs, ouvrier 110 V, 5 ans : SMPG 1 997,89 € (avenant n° 120).
#[tokio::test]
async fn ccn_sous_minimum_voyageurs() {
    let a = alertes_ccn(classe("1950", "voyageurs", "ouvriers", "110 V", 5), "2026-06-30").await;
    assert_eq!(a.len(), 1, "{a:?}");
    assert_eq!(a[0].0, "ccn_sous_minimum");
    assert_eq!(a[0].1["minimum"], "1997,89");
    assert_eq!(a[0].1["ecart"], "47,89");
    assert_eq!(a[0].1["palier"], "5");
}

#[tokio::test]
async fn ccn_conforme_et_palier() {
    // 4 ans : palier « après 1 an » (1 922,50 €), pas celui de 5 ans.
    let a = alertes_ccn(classe("1950", "voyageurs", "ouvriers", "110 V", 4), "2026-06-30").await;
    assert_eq!(a[0].0, "ccn_conforme");
    assert_eq!(a[0].1["minimum"], "1922,50");
    // Mi-temps : minimum au prorata.
    let mut s = classe("1000", "voyageurs", "ouvriers", "110 V", 4);
    s.etp = 50.0;
    assert_eq!(alertes_ccn(s, "2026-06-30").await[0].1["minimum"], "961,25");
}

/// Marchandises 128M : 12,12 € × 151,67 = 1 838,24 € < SMIC 1 867,02 €.
#[tokio::test]
async fn ccn_rattrape_par_le_smic() {
    let a = alertes_ccn(classe("1900", "marchandises", "ouvriers", "128M", 0), "2026-06-30").await;
    assert_eq!(a[0].0, "ccn_sous_smic");
    assert_eq!(a[0].1["minimum"], "1838,24");
}

/// Cadre marchandises 100 : paiement mensuel minimum 2 621,80 €.
#[tokio::test]
async fn ccn_cadre_mensuel() {
    let a = alertes_ccn(classe("2600", "marchandises", "cadres", "100", 1), "2026-06-30").await;
    assert_eq!(a[0].0, "ccn_sous_minimum");
    assert_eq!(a[0].1["minimum"], "2621,80");
}

/// Grille voyageurs en vigueur au 1er janvier 2026 : rien de connu avant ;
/// sans classement complet, ni convention : aucun contrôle.
#[tokio::test]
async fn ccn_hors_champ() {
    let a = alertes_ccn(classe("1950", "voyageurs", "ouvriers", "110 V", 5), "2025-06-30").await;
    assert_eq!(a[0].0, "ccn_inconnu");
    let mut s = classe("1950", "voyageurs", "ouvriers", "110 V", 5);
    s.ccn_coefficient = None;
    assert!(alertes_ccn(s, "2026-06-30").await.is_empty());
    let mut s = classe("1950", "voyageurs", "ouvriers", "110 V", 5);
    s.convention_idcc = None;
    assert!(alertes_ccn(s, "2026-06-30").await.is_empty());
}
