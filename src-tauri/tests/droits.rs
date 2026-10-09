//! Droits à la retraite ouverts par le mois (calculs/droits.rs, migration 0141) :
//! points Agirc-Arrco au taux de CALCUL (pas au taux appelé), trimestres au
//! seuil de 150 h × SMIC horaire du 1er janvier, 4 au plus par an.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use rust_decimal_macros::dec;
use sqlx::SqlitePool;

use xenna_paie_lib::calculs::generer_bulletin;
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{Pays, Salarie, Statut};

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_droits_{}_{}.db", std::process::id(), n));
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



async fn droits(pays: Pays, brut: &str, date_paie: &str) -> Option<xenna_paie_lib::models::DroitsResult> {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date(date_paie)).await.unwrap();
    let d = generer_bulletin(salarie_base(pays, brut), &ctx, None).droits;
    nettoyer(&path);
    d
}

/// 3 200 € en juin 2026 : tout en tranche 1 (sous le PMSS de 4 005 €).
/// Points = 3 200 × 6,20 % ÷ 20,1877 = 9,83 ; rente = 9,83 × 1,4386 = 14,14 €.
/// Seuil = 150 × 12,02 € (SMIC horaire au 01/01/2026) = 1 803 € → 1,77 seuil, 4 trimestres sur l'année.
#[tokio::test]
async fn sous_le_plafond() {
    let d = droits(Pays::France, "3200", "2026-06-30").await.expect("droits attendus");
    assert_eq!(d.assiette_t1, dec!(3200));
    assert_eq!(d.points, dec!(9.83));
    assert_eq!(d.rente_annuelle, dec!(14.14));
    assert_eq!(d.seuil_trimestre, dec!(1803.00));
    assert_eq!(d.part_trimestre_mois, dec!(1.77));
    assert_eq!(d.trimestres_an, 4);
    assert_eq!(d.salaire_porte_au_compte, dec!(3200));
}

/// 6 000 € : tranche 1 jusqu'au PMSS, tranche 2 au-delà, à 17 %.
/// (4 005 × 6,20 % + 1 995 × 17 %) ÷ 20,1877 = (248,31 + 339,15) ÷ 20,1877 = 29,10.
#[tokio::test]
async fn au_dessus_du_plafond() {
    let d = droits(Pays::France, "6000", "2026-06-30").await.unwrap();
    assert_eq!(d.assiette_t1, dec!(4005));
    assert_eq!(d.assiette_t2, dec!(1995));
    assert_eq!(d.points, dec!(29.10));
    assert_eq!(d.salaire_porte_au_compte, dec!(4005), "le régime de base s'arrête au plafond");
}

/// Petit salaire : 500 €/mois × 12 = 6 000 € → 3 trimestres (6 000 ÷ 1 803).
#[tokio::test]
async fn petit_salaire_moins_de_quatre_trimestres() {
    let d = droits(Pays::France, "500", "2026-06-30").await.unwrap();
    assert_eq!(d.trimestres_an, 3);
}

/// Pas de régime unifié Agirc-Arrco avant 2019 ; ni FPT, ni étranger.
#[tokio::test]
async fn hors_champ() {
    assert!(droits(Pays::France, "3200", "2018-06-30").await.is_none());
    assert!(droits(Pays::FonctionPublique, "3200", "2026-06-30").await.is_none());
    assert!(droits(Pays::Suisse, "3200", "2026-06-30").await.is_none());
}

// ── Compte personnel de formation (migration 0147) ───────────────────────────

async fn cpf(maj: impl FnOnce(&mut Salarie)) -> xenna_paie_lib::models::CpfResult {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-06-30")).await.unwrap();
    let mut s = salarie_base(Pays::France, "2500");
    maj(&mut s);
    let c = generer_bulletin(s, &ctx, None).droits.and_then(|d| d.cpf).expect("CPF attendu");
    nettoyer(&path);
    c
}

/// Au moins mi-temps : 500 €/an, plafond 5 000 € ; le mois en vaut 41,67 €
/// (500 ÷ 12 = 41,666…, arrondi au centime supérieur, art. R6323-1).
#[tokio::test]
async fn cpf_general() {
    let c = cpf(|_| {}).await;
    assert_eq!((c.regime.as_str(), c.annuel, c.mois, c.plafond, c.prorata),
               ("general", dec!(500), dec!(41.67), dec!(5000), false));
    // Exactement mi-temps : montant plein.
    let c = cpf(|s| s.etp = 50.0).await;
    assert_eq!((c.annuel, c.prorata), (dec!(500), false));
}

/// Sous le mi-temps : prorata. 40 % → 200 €/an, 16,67 € le mois.
#[tokio::test]
async fn cpf_prorata() {
    let c = cpf(|s| s.etp = 40.0).await;
    assert_eq!((c.annuel, c.mois, c.prorata), (dec!(200), dec!(16.67), true));
}

/// ESAT : 800 € par année d'admission, même à temps partiel ; entreprise
/// adaptée (travailleur handicapé) : 800 € au moins à mi-temps. Plafond 8 000 €.
#[tokio::test]
async fn cpf_majore() {
    let c = cpf(|s| { s.esat = true; s.etp = 30.0; }).await;
    assert_eq!((c.regime.as_str(), c.annuel, c.mois, c.plafond), ("esat", dec!(800), dec!(66.67), dec!(8000)));
    let c = cpf(|s| s.entreprise_adaptee = true).await;
    assert_eq!((c.regime.as_str(), c.annuel, c.plafond), ("handicap", dec!(800), dec!(8000)));
}
