//! Congés payés — retenue + indemnité (art. L3141-24 C. trav.).
//!
//! Propriété centrale : indemnité = MAX(maintien de salaire ; méthode du
//! dixième), le maintien étant égal à la retenue (neutralité quand il gagne).
//! Assiette du dixième (hypothèse du simulateur) : 13 × brut mensuel
//! (salaire constant + 13e mois versé en décembre).

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::SqlitePool;

use xenna_paie_lib::calculs::generer_bulletin;
use xenna_paie_lib::calculs::paye_inverse::resoudre_brut_pour_net;
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{AbsenceInput, Pays, Salarie, Statut};

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_cp_{}_{}.db", std::process::id(), n));
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
        prenom: "Conges".into(),
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

fn conge(debut: &str, fin: &str, methode: &str, jours_type: &str) -> AbsenceInput {
    AbsenceInput {
        type_arret: "conge".into(),
        date_debut: debut.into(),
        date_fin: fin.into(),
        methode: methode.into(),
        jours_type: jours_type.into(),
        heures_mois: None,
        convention_idcc: None,
    }
}

fn d(s: &str) -> Decimal {
    s.parse().unwrap()
}

/// Semaine pleine en jours ouvrés : le dixième (assiette 13 × brut) l'emporte
/// toujours sur le maintien — 1,3 B/25 > B/21,67 par jour.
#[tokio::test]
async fn dixieme_gagne_jours_ouvres() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2025-06-30")).await.unwrap();

    // Lun 16 → ven 20 juin 2025 : 5 jours ouvrés, aucun férié.
    let cp = conge("2025-06-16", "2025-06-20", "moyens", "ouvres");
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&cp));
    let c = b.conges.as_ref().expect("congés payés calculés");

    assert_eq!(c.jours_pris, 5);
    assert_eq!(c.diviseur_retenue, d("21.67"));
    assert_eq!(c.retenue, d("692.20"), "3000 × 5 ÷ 21,67");
    assert_eq!(c.montant_maintien, c.retenue, "maintien = retenue (L3141-24 II)");

    assert_eq!(c.assiette_dixieme, d("39000.00"), "13 × brut");
    assert_eq!(c.dixieme_total, d("3900.00"));
    assert_eq!(c.jours_acquis, 25, "25 jours ouvrés acquis par an");
    assert_eq!(c.jours_pris_dixieme, 5);
    assert_eq!(c.montant_dixieme, d("780.00"), "39000 ÷ 10 × 5 ÷ 25");

    assert_eq!(c.methode_indemnite, "dixieme");
    assert_eq!(c.indemnite, d("780.00"));
    assert_eq!(b.brut, d("3087.80"), "3000 − 692,20 + 780");

    // Un CP ne produit ni IJSS ni garantie du net.
    assert!(b.absence.is_none(), "pas d'AbsenceResult pour un CP");

    nettoyer(&path);
}

/// Même semaine en jours ouvrables (samedi inclus) : prorata sur 30 jours.
#[tokio::test]
async fn dixieme_gagne_jours_ouvrables() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2025-06-30")).await.unwrap();

    // Lun 16 → sam 21 juin 2025 : 6 jours ouvrables.
    let cp = conge("2025-06-16", "2025-06-21", "moyens", "ouvrables");
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&cp));
    let c = b.conges.as_ref().unwrap();

    assert_eq!(c.jours_pris, 6);
    assert_eq!(c.diviseur_retenue, d("26"));
    assert_eq!(c.retenue, d("692.31"), "3000 × 6 ÷ 26");
    assert_eq!(c.jours_acquis, 30, "30 jours ouvrables acquis par an");
    assert_eq!(c.unite_dixieme, "ouvrables");
    assert_eq!(c.montant_dixieme, d("780.00"), "3900 × 6 ÷ 30");
    assert_eq!(c.methode_indemnite, "dixieme");

    nettoyer(&path);
}

/// Méthode calendaire sur un week-end : la retenue compte 4 jours mais le
/// prorata du dixième n'en compte que 2 ouvrés → le maintien l'emporte, et le
/// bulletin est strictement neutre (brut et net inchangés).
#[tokio::test]
async fn maintien_gagne_calendaire_et_neutralite() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2025-06-30")).await.unwrap();

    // Ven 13 → lun 16 juin 2025 : 4 jours calendaires, 2 jours ouvrés.
    let cp = conge("2025-06-13", "2025-06-16", "calendaire", "");
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&cp));
    let c = b.conges.as_ref().unwrap();

    assert_eq!(c.jours_pris, 4);
    assert_eq!(c.diviseur_retenue, d("30"), "juin = 30 jours");
    assert_eq!(c.retenue, d("400.00"), "3000 × 4 ÷ 30");
    assert_eq!(c.jours_pris_dixieme, 2, "seuls ven. et lun. comptent au dixième");
    assert_eq!(c.montant_dixieme, d("312.00"), "3900 × 2 ÷ 25");

    assert_eq!(c.methode_indemnite, "maintien");
    assert_eq!(c.indemnite, c.retenue);

    // Neutralité : retenue et indemnité s'annulent.
    assert_eq!(b.brut, d("3000.00"));
    let reference = generer_bulletin(salarie_france("3000.00"), &ctx, None);
    assert_eq!(b.net_a_payer, reference.net_a_payer, "net identique au mois plein");
    assert_eq!(b.net_imposable, reference.net_imposable);

    nettoyer(&path);
}

/// Un jour férié dans la période (8 mai) n'est ni retenu ni compté au dixième.
#[tokio::test]
async fn ferie_exclu_du_decompte() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2025-05-31")).await.unwrap();

    // Lun 5 → ven 9 mai 2025, jeudi 8 mai férié → 4 jours ouvrés.
    let cp = conge("2025-05-05", "2025-05-09", "moyens", "ouvres");
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&cp));
    let c = b.conges.as_ref().unwrap();

    assert_eq!(c.jours_pris, 4, "le 8 mai est férié");
    assert_eq!(c.jours_pris_dixieme, 4);

    nettoyer(&path);
}

/// Paye inversée : le brut de base reconstitué ne bouge pas quand on ajoute un
/// CP — c'est le net du mois qui varie ; le détail CP est bien renvoyé.
#[tokio::test]
async fn paye_inverse_avec_conges() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2025-06-30")).await.unwrap();

    let cible = d("2400.00");
    let sans = resoudre_brut_pour_net(cible, &salarie_france("1.00"), &ctx, None);

    let cp = conge("2025-06-16", "2025-06-20", "moyens", "ouvres");
    let avec = resoudre_brut_pour_net(cible, &salarie_france("1.00"), &ctx, Some(&cp));
    let c = avec.conges.as_ref().expect("détail CP en paye inversée");

    assert_eq!(
        c.brut_mensuel, sans.brut,
        "le brut de base reconstitué est identique avec et sans CP"
    );
    // Le dixième gagne (semaine ouvrée pleine) → le net du mois augmente.
    assert_eq!(c.methode_indemnite, "dixieme");
    assert!(avec.net_a_payer > sans.net_a_payer);

    nettoyer(&path);
}

/// Méthode « heures réelles » : le diviseur est l'horaire mensuel contractuel
/// (151,67 h temps plein), PAS les jours de référence réels du mois. Retenue =
/// brut × (jours × 7 h) ÷ 151,67 — indépendante du calendrier du mois.
#[tokio::test]
async fn heures_reelles_divise_par_151_67() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2025-06-30")).await.unwrap();

    // Lun 16 → ven 20 juin 2025 : 5 jours ouvrés (et 5 ouvrables), aucun férié.
    let cp = conge("2025-06-16", "2025-06-20", "heures", "ouvres");
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&cp));
    let c = b.conges.as_ref().expect("congés payés calculés");

    assert_eq!(c.diviseur_retenue, d("151.67"), "diviseur = horaire mensuel");
    // 3000 × (5 j × 7,0002 h) ÷ 151,67 = 3000 × 5 × 12 ÷ 260 = 692,31.
    assert_eq!(c.retenue, d("692.31"), "3000 × 35 h ÷ 151,67");

    // Ouvrables : 35 h hebdo réparties sur 6 jours (5,83 h/j), même diviseur.
    let cp = conge("2025-06-16", "2025-06-20", "heures", "ouvrables");
    let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&cp));
    let c = b.conges.as_ref().expect("congés payés calculés");
    assert_eq!(c.diviseur_retenue, d("151.67"));
    assert_eq!(c.retenue, d("576.92"), "3000 × 29,17 h ÷ 151,67 (≡ 5/26)");

    nettoyer(&path);
}

/// Invariants structurels du résultat CP.
#[tokio::test]
async fn invariants_conges_payes() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2025-06-30")).await.unwrap();

    for (methode, jours_type) in [
        ("moyens", "ouvres"), ("moyens", "ouvrables"),
        ("calendaire", ""), ("heures", "ouvres"), ("heures", "ouvrables"),
    ] {
        let cp = conge("2025-06-09", "2025-06-20", methode, jours_type);
        let b = generer_bulletin(salarie_france("3000.00"), &ctx, Some(&cp));
        let c = b.conges.as_ref().unwrap_or_else(|| panic!("CP absent ({methode}/{jours_type})"));

        assert_eq!(c.indemnite, c.montant_maintien.max(c.montant_dixieme),
            "indemnité = max ({methode}/{jours_type})");
        assert_eq!(c.montant_maintien, c.retenue, "maintien = retenue ({methode}/{jours_type})");
        assert!(b.absence.is_none(), "pas d'IJSS pour un CP ({methode}/{jours_type})");
        assert!(b.net_a_payer <= b.brut, "net ≤ brut ({methode}/{jours_type})");
        assert!(c.retenue > Decimal::ZERO && c.indemnite > Decimal::ZERO);
    }

    nettoyer(&path);
}
