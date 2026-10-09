//! Prélèvement à la source — grille de taux par défaut (art. 204 H, III CGI,
//! migration 0140) et taux personnalisé.
//!
//! Propriété centrale : le taux de la grille est UNIQUE et s'applique à la
//! totalité de la base. Le simulateur a longtemps calculé un PAS progressif
//! tranche par tranche ; ces tests verrouillent la règle.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::SqlitePool;

use xenna_paie_lib::calculs::{generer_bulletin, pas};
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{Pays, Salarie, Statut};

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_pas_{}_{}.db", std::process::id(), n));
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


/// Taux unique sur toute la base : 2 500 € en juin 2026 tombent dans la plage
/// [2 315 ; 2 738[ à 5,3 % → 132,50 €. Le calcul progressif en donnait ~53 €.
#[tokio::test]
async fn taux_unique_sur_toute_la_base() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-06-30")).await.unwrap();
    let p = pas::calculer(&salarie_base(Pays::France, "3000"), dec!(2500), &ctx).unwrap();
    assert_eq!(p.taux, dec!(0.053));
    assert_eq!(p.montant, dec!(132.50));
    assert_eq!(p.origine, "defaut");
    nettoyer(&path);
}

/// Bornes : « supérieure ou égale à » la borne basse, « inférieure à » la haute.
#[tokio::test]
async fn bornes_incluses_a_gauche() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-06-30")).await.unwrap();
    let s = salarie_base(Pays::France, "3000");
    assert_eq!(pas::calculer(&s, dec!(1634.99), &ctx).unwrap().taux, dec!(0));
    assert_eq!(pas::calculer(&s, dec!(1635), &ctx).unwrap().taux, dec!(0.005));
    assert_eq!(pas::calculer(&s, dec!(55558), &ctx).unwrap().taux, dec!(0.43));
    nettoyer(&path);
}

/// La grille 2026 ne s'applique qu'au 1er mai 2026 : en mars, c'est encore
/// celle de 2025 (1 625 € → 0,5 %) ; en juin, 1 625 € < 1 635 € → 0 %.
#[tokio::test]
async fn bascule_au_premier_mai() {
    let (pool, path) = base_test().await;
    let s = salarie_base(Pays::France, "3000");
    let mars = ContextPaie::charger(&pool, date("2026-03-31")).await.unwrap();
    let juin = ContextPaie::charger(&pool, date("2026-06-30")).await.unwrap();
    assert_eq!(pas::calculer(&s, dec!(1625), &mars).unwrap().taux, dec!(0.005));
    assert_eq!(pas::calculer(&s, dec!(1625), &juin).unwrap().taux, dec!(0));
    nettoyer(&path);
}

/// Avant 2019, le PAS n'existait pas.
#[tokio::test]
async fn pas_de_pas_avant_2019() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2018-12-31")).await.unwrap();
    assert!(pas::calculer(&salarie_base(Pays::France, "3000"), dec!(2500), &ctx).is_none());
    nettoyer(&path);
}

/// Taux personnalisé : il remplace la grille, toujours sur toute la base.
#[tokio::test]
async fn taux_personnalise() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-06-30")).await.unwrap();
    let mut s = salarie_base(Pays::France, "3000");
    s.taux_pas = Some(dec!(0.074));
    let p = pas::calculer(&s, dec!(2500), &ctx).unwrap();
    assert_eq!(p.montant, dec!(185.00));
    assert_eq!(p.origine, "personnalise");
    assert!(!p.grille.is_empty(), "la grille reste fournie pour comparaison");
    nettoyer(&path);
}

/// Bulletin complet : PAS présent en France et en FPT, absent ailleurs.
#[tokio::test]
async fn pas_sur_le_bulletin() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-06-30")).await.unwrap();
    let fr = generer_bulletin(salarie_base(Pays::France, "3000"), &ctx, None);
    let p = fr.pas.expect("PAS attendu en France");
    assert_eq!(p.base, fr.net_imposable);
    assert_eq!(p.montant, (fr.net_imposable * p.taux).round_dp(2));
    assert!(generer_bulletin(salarie_base(Pays::FonctionPublique, "3000"), &ctx, None).pas.is_some());
    assert!(generer_bulletin(salarie_base(Pays::Suisse, "3000"), &ctx, None).pas.is_none());
    nettoyer(&path);
}

/// Intégrité de chaque grille en base : plages contiguës depuis 0, taux
/// croissants, une seule plage ouverte ; périodes contiguës depuis 2019.
#[tokio::test]
async fn grilles_coherentes() {
    let (pool, path) = base_test().await;
    let periodes: Vec<(String, Option<String>)> = sqlx::query_as(
        "SELECT DISTINCT date_debut, date_fin FROM pas_grille_defaut ORDER BY date_debut")
        .fetch_all(&pool).await.unwrap();
    assert_eq!(periodes.first().unwrap().0, "2019-01-01");
    assert!(periodes.last().unwrap().1.is_none(), "la dernière grille doit rester ouverte");
    for w in periodes.windows(2) {
        assert_eq!(w[0].1.as_deref(), Some(w[1].0.as_str()), "trou ou chevauchement entre grilles");
    }
    for (debut, _) in &periodes {
        let ctx = ContextPaie::charger(&pool, date(debut)).await.unwrap();
        let g = &ctx.pas_grille;
        assert_eq!(g.len(), 20, "{debut} : 20 plages attendues");
        assert_eq!(g[0].borne_min, Decimal::ZERO);
        assert_eq!(g[0].taux, Decimal::ZERO);
        for w in g.windows(2) {
            assert_eq!(w[0].borne_max, Some(w[1].borne_min), "{debut} : plages non contiguës");
            assert!(w[1].taux > w[0].taux, "{debut} : taux non croissants");
        }
        assert!(g.last().unwrap().borne_max.is_none());
    }
    nettoyer(&path);
}

/// Domicile : à 2 000 € en juin 2026, métropole 2,9 % (1 928 ≤ 2 000 < 2 060),
/// Guadeloupe/Réunion/Martinique 1,3 % (1 989 ≤ 2 000 < 2 191),
/// Guyane/Mayotte 0 % (< 2 008).
#[tokio::test]
async fn grille_selon_le_domicile() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-06-30")).await.unwrap();
    let mut s = salarie_base(Pays::France, "3000");
    let taux = |s: &Salarie| pas::calculer(s, dec!(2000), &ctx).unwrap();
    assert_eq!(taux(&s).taux, dec!(0.029));
    s.pas_zone = Some("grm".into());
    let p = taux(&s);
    assert_eq!((p.taux, p.zone.as_str()), (dec!(0.013), "GRM"));
    s.pas_zone = Some("gm".into());
    assert_eq!(taux(&s).taux, dec!(0));
    nettoyer(&path);
}

/// Contrat court, sur le modèle de l'exemple 2 du BOFiP (BOI-IR-PAS-20-20-30-10
/// § 270) : 2 300 € en février 2023, abattement de 701 € → assiette 1 599 €,
/// taux de la grille 2023 sur 1 599 € = 1,3 %, appliqué à 1 599 € = 20,79 €.
#[tokio::test]
async fn contrat_court_abattement() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2023-02-28")).await.unwrap();
    let mut s = salarie_base(Pays::France, "3000");
    s.contrat_court = true;
    let p = pas::calculer(&s, dec!(2300), &ctx).unwrap();
    assert_eq!(p.abattement, dec!(701));
    assert_eq!(p.assiette, dec!(1599));
    assert_eq!(p.taux, dec!(0.013));
    assert_eq!(p.montant, dec!(20.79));
    // Un taux personnalisé l'emporte : ni grille, ni abattement.
    s.taux_pas = Some(dec!(0.05));
    let p = pas::calculer(&s, dec!(2300), &ctx).unwrap();
    assert_eq!((p.abattement, p.montant), (dec!(0), dec!(115.00)));
    nettoyer(&path);
}

/// Grilles d'outre-mer : même intégrité que la métropole, pour chaque période.
#[tokio::test]
async fn grilles_outremer_coherentes() {
    let (pool, path) = base_test().await;
    for d in ["2019-06-30", "2022-06-30", "2025-03-31", "2026-06-30"] {
        let ctx = ContextPaie::charger(&pool, date(d)).await.unwrap();
        for zone in ["METROPOLE", "GRM", "GM"] {
            let (g, source) = ctx.pas_grilles.get(zone).unwrap_or_else(|| panic!("{d} : grille {zone} absente"));
            assert!(source.starts_with("BOI-BAREME-000037"));
            assert_eq!(g.len(), 20, "{d} {zone}");
            for w in g.windows(2) {
                assert_eq!(w[0].borne_max, Some(w[1].borne_min), "{d} {zone} : plages non contiguës");
                assert!(w[1].taux > w[0].taux, "{d} {zone} : taux non croissants");
            }
        }
        // Outre-mer, les seuils sont plus hauts qu'en métropole.
        assert!(ctx.pas_grilles["GRM"].0[1].borne_min > ctx.pas_grilles["METROPOLE"].0[1].borne_min);
        assert!(ctx.pas_grilles["GM"].0[1].borne_min > ctx.pas_grilles["GRM"].0[1].borne_min);
    }
    nettoyer(&path);
}
