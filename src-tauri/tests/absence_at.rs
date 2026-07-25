//! Accident du travail / maladie professionnelle + congé sans solde.
//!
//! AT/MP : IJSS SANS carence (60 % du SJR j1-j28 puis 80 %, SJR = brut ÷ 30,42
//! plafonné à 0,834 % du PASS), imposables à 50 %, maintien employeur sans
//! carence (D1226-3, garantie de ressources IDCC 0016 dès 3 ans, légal 1-3 ans).
//! Sans solde : retenue sèche — ni maintien, ni IJSS, ni ajustement.

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
    let path = std::env::temp_dir().join(format!("xenna_at_{}_{}.db", std::process::id(), n));
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
        prenom: "At".into(),
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
        inde_regime: None,
        emirati_national: None,
    }
}

fn arret(type_arret: &str, debut: &str, fin: &str, methode: &str) -> AbsenceInput {
    AbsenceInput {
        type_arret: type_arret.into(),
        date_debut: debut.into(),
        date_fin: fin.into(),
        methode: methode.into(),
        jours_type: String::new(),
        heures_mois: None,
        convention_idcc: Some("0016".into()),
    }
}

fn d(s: &str) -> Decimal {
    s.parse().unwrap()
}

/// AT court (< 28 j) : IJ dès le 1er jour (sans carence), tout à 60 % du SJR,
/// imposables à 50 %.
#[tokio::test]
async fn at_court_sans_carence_60_pourcent() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    let abs = arret("pro", "2026-03-02", "2026-03-11", "calendaire"); // 10 j cal
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&abs));
    let a = b.absence.as_ref().expect("absence AT calculée");

    assert_eq!(a.type_arret, "pro");
    assert_eq!(a.jours_ijss, 10, "AT : aucune carence, IJ dès le 1er jour");
    assert_eq!(a.jours_ijss_t1, 10);
    assert_eq!(a.jours_ijss_t2, 0, "arrêt court : pas de tranche 80 %");
    assert_eq!(a.taux_ijss_t1, d("0.60"));
    assert_eq!(a.taux_ijss_t2, d("0.80"));
    // SJR = 3000 ÷ 30,42 = 98,6193… → IJ 60 % = 59,17.
    assert_eq!(a.ijss_jour, d("59.17"));
    assert_eq!(a.ijss_brut, d("591.70"), "10 j × 59,17");
    assert_eq!(a.ijss_net, d("552.06"), "brutes × 0,933");
    assert_eq!(a.ijss_imposable, d("295.85"), "AT : imposables à 50 %");
    assert!(a.libelle.starts_with("AT/MP"), "libellé : {}", a.libelle);

    nettoyer(&path);
}

/// AT long chevauchant le 28e jour : 28 j à 60 % puis 80 % dès le 29e.
#[tokio::test]
async fn at_long_deux_tranches() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    let abs = arret("pro", "2026-03-01", "2026-03-31", "calendaire"); // 31 j cal
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&abs));
    let a = b.absence.as_ref().unwrap();

    assert_eq!(a.jours_ijss_t1, 28);
    assert_eq!(a.jours_ijss_t2, 3);
    assert_eq!(a.ijss_jour, d("59.17"));
    assert_eq!(a.ijss_jour_t2, d("78.90"), "80 % du SJR dès j29");
    assert_eq!(a.ijss_brut, d("1893.46"), "28 × 59,17 + 3 × 78,90");
    assert_eq!(a.ijss_imposable, d("946.73"), "50 % du montant");

    nettoyer(&path);
}

/// Plafonnement du SJR à 0,834 % du PASS (PMSS 2026 = 4005) :
/// IJ max = 240,49 € (60 %) / 320,66 € (80 %).
#[tokio::test]
async fn at_plafond_sjr() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    let abs = arret("pro", "2026-03-01", "2026-03-31", "calendaire");
    let b = generer_bulletin(salarie_france("15000.00"), &ctx, Some(&abs));
    let a = b.absence.as_ref().unwrap();

    assert_eq!(a.plafond_sjr_ijss, d("400.82"), "0,834 % × 12 × 4005");
    assert_eq!(a.sjb, d("400.82"), "SJR plafonné (15000 ÷ 30,42 = 493,10 > plafond)");
    assert_eq!(a.ijss_jour, d("240.49"), "IJ max 60 %");
    assert_eq!(a.ijss_jour_t2, d("320.66"), "IJ max 80 %");

    nettoyer(&path);
}

/// Maintien AT sans carence (D1226-3) : légal 90 % dès j1 pour 1-3 ans
/// d'ancienneté, garantie de ressources 100 % dès j1 au-delà de 3 ans.
#[tokio::test]
async fn at_maintien_sans_carence() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();
    let abs = arret("pro", "2026-03-02", "2026-03-11", "calendaire"); // 10 j

    let bulletin = |anc: Option<i64>| {
        let mut s = salarie_france("3000.00");
        s.anciennete = anc;
        generer_bulletin(s, &ctx, Some(&abs))
    };

    // Ancienneté par défaut (1 an) : légal AT 90 % dès le 1er jour.
    let b1 = bulletin(None);
    let a1 = b1.absence.as_ref().unwrap();
    assert_eq!(a1.retenue, d("967.74"), "3000 × 10 ÷ 31 (mars)");
    assert_eq!(a1.carence_maintien, 0, "AT : aucune carence de maintien");
    assert_eq!(a1.jours_maintien, 10, "tous les jours comptés indemnisés");
    assert_eq!(a1.taux_maintien_t1, d("0.90"));
    assert_eq!(a1.maintien, d("870.97"), "10 j × 90 % × per-day");
    assert!(a1.convention.contains("légal AT"), "libellé : {}", a1.convention);

    // ≥ 10 ans : garantie de ressources 100 % j1-90 → maintien = retenue.
    let b10 = bulletin(Some(10));
    let a10 = b10.absence.as_ref().unwrap();
    assert_eq!(a10.taux_maintien_t1, d("1.00"));
    assert_eq!(a10.maintien, d("967.74"), "100 % de la retenue");
    assert!(a10.convention.contains("garantie de ressources AT"), "libellé : {}", a10.convention);
    assert!(a10.convention.contains("IDCC 0016"), "barème conventionnel préfixé");

    nettoyer(&path);
}

/// Alsace-Moselle actif pour l'AT (L1226-23 couvre toute absence sans faute) :
/// 100 % dès le 1er jour même sans ancienneté.
#[tokio::test]
async fn at_alsace_moselle_actif() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    let abs = arret("pro", "2026-03-02", "2026-03-11", "calendaire");
    let mut s = salarie_france("3000.00");
    s.alsace_moselle = true;
    s.anciennete = Some(0);
    let b = generer_bulletin(s, &ctx, Some(&abs));
    let a = b.absence.as_ref().unwrap();

    assert!(a.am_local, "droit local appliqué à l'AT");
    assert_eq!(a.taux_maintien_t1, d("1.00"), "AM : 100 % dès le 1er jour");
    assert_eq!(a.jours_maintien_t1, a.jours_absence);

    nettoyer(&path);
}

/// Ancienneté < 1 an : pas de maintien (condition L1226-1). La subrogation étant
/// adossée au maintien de salaire, AUCUNE IJSS ne figure alors sur le bulletin —
/// la CPAM les verse directement au salarié. Seule la retenue s'applique.
#[tokio::test]
async fn at_sans_maintien_pas_de_subrogation() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    let abs = arret("pro", "2026-03-02", "2026-03-11", "calendaire");
    let mut s = salarie_france("3000.00");
    s.anciennete = Some(0);
    let b = generer_bulletin(s, &ctx, Some(&abs));
    let a = b.absence.as_ref().unwrap();

    assert_eq!(a.maintien, Decimal::ZERO, "< 1 an : pas de maintien");
    assert!(a.convention.contains("sans maintien"), "libellé : {}", a.convention);
    // Sans maintien → subrogation inactive → pas d'IJSS au bulletin (versées en direct).
    assert_eq!(a.ijss_brut, Decimal::ZERO, "sans maintien → pas de subrogation → pas d'IJSS au bulletin");
    assert_eq!(a.jours_ijss, 0, "aucun jour d'IJSS subrogé");
    assert!(a.retenue > Decimal::ZERO, "la retenue pour absence s'applique quand même");

    nettoyer(&path);
}

/// Congé sans solde : retenue sèche — ni maintien, ni IJSS, ni ajustement ;
/// le bulletin est celui d'un brut réduit de la retenue.
#[tokio::test]
async fn sans_solde_retenue_seule() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    // Lun 2 → ven 6 mars 2026 : 5 jours ouvrés, aucun férié.
    let mut abs = arret("sans_solde", "2026-03-02", "2026-03-06", "moyens");
    abs.jours_type = "ouvres".into();
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&abs));
    let a = b.absence.as_ref().expect("absence sans solde calculée");

    assert_eq!(a.type_arret, "sans_solde");
    assert_eq!(a.retenue, d("692.20"), "3000 × 5 ÷ 21,67");
    assert_eq!(a.maintien, Decimal::ZERO);
    assert_eq!(a.ijss_brut, Decimal::ZERO);
    assert_eq!(a.ijss_net, Decimal::ZERO);
    assert_eq!(a.ijss_imposable, Decimal::ZERO);
    assert_eq!(a.ajustement_net, Decimal::ZERO);
    assert_eq!(a.convention, "", "pas de régime de maintien à afficher");
    assert!(a.libelle.starts_with("congé sans solde"), "libellé : {}", a.libelle);

    // Bulletin identique à un brut réduit de la retenue.
    assert_eq!(b.brut, d("2307.80"));
    let reference = generer_bulletin(salarie_france("2307.80"), &ctx, None);
    assert!(
        (b.net_a_payer - reference.net_a_payer).abs() <= d("0.01"),
        "net sans solde {} ≠ net(brut réduit) {}", b.net_a_payer, reference.net_a_payer
    );

    nettoyer(&path);
}

/// Neutralité de la garantie du net pour l'AT : net à payer == net du bulletin
/// de référence (assiette = base − retenue + maintien), comme en maladie.
#[tokio::test]
async fn at_neutralite_garantie_du_net() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();

    let abs = arret("pro", "2026-03-02", "2026-03-11", "calendaire");
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&abs));
    let a = b.absence.as_ref().unwrap();

    assert!(a.ijss_brut > Decimal::ZERO);
    assert!(a.ajustement_net > Decimal::ZERO, "IJSS présentes → ajustement du net");

    let assiette_ref = d("3000.00") - a.retenue + a.maintien;
    let reference = generer_bulletin(salarie_france(&assiette_ref.to_string()), &ctx, None);
    let ecart = (b.net_a_payer - reference.net_a_payer).abs();
    assert!(
        ecart <= d("0.03"),
        "garantie du net violée : {} ≠ {} (écart {})",
        b.net_a_payer, reference.net_a_payer, ecart
    );

    nettoyer(&path);
}
