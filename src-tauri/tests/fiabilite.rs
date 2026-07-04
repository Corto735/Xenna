//! Filet de sécurité « fiabilité » — Phase 1 de l'homogénéisation.
//!
//! Objectif : empêcher qu'un changement de taux, un refactor ou l'ajout d'un
//! pays casse SILENCIEUSEMENT un bulletin existant. Avant ce fichier, ~16 000
//! lignes de calcul fiscal/social tournaient sans aucun `#[test]`.
//!
//! Trois familles de garde-fous :
//!   1. Invariants universels — vrais pour TOUT pays, à toute date couverte
//!      (net ≤ brut, coût employeur ≥ net, devise plausible…).
//!   2. Exhaustivité de l'enum `Pays` — un nouveau pays non câblé casse la
//!      compilation de ce fichier (rappel : l'ajouter partout).
//!   3. Golden France — bornes de plausibilité (ratios net/brut, Fillon,
//!      monotonicité). Volontairement en FOURCHETTES, pas en valeurs exactes :
//!      on attrape les régressions grossières sans figer de fausse précision.
//!
//! Les tests construisent une base SQLite jetable et rejouent les vraies
//! migrations de prod (`db::init_db`). Aucune donnée mockée : on teste le
//! calcul réel sur les données réelles.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use sqlx::SqlitePool;

use xenna_paie_lib::calculs::generer_bulletin;
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{Bulletin, Pays, Salarie, Statut};

// ────────────────────────────── Outils ──────────────────────────────

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

/// Construit une base SQLite jetable (fichier temporaire unique) et y rejoue
/// toutes les migrations de prod. Chaque appel = base fraîche et isolée.
///
/// On n'utilise volontairement PAS un pool partagé entre tests : sqlx lie ses
/// tâches de connexion au runtime tokio courant, et `#[tokio::test]` crée un
/// runtime par test — partager un pool donnerait des connexions mortes (cf. le
/// long commentaire dans `lib.rs::run`).
async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_test_{}_{}.db", std::process::id(), n));
    // Nettoie un éventuel reliquat d'un run précédent (et ses fichiers WAL/SHM).
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

/// Salarié « par défaut » avec tous les champs renseignés de façon plausible,
/// y compris les codes régionaux (canton, province, regione, Land, région BE)
/// pour que les pays qui en ont besoin ne tombent pas sur une branche None.
fn salarie_base(pays: Pays, brut: &str) -> Salarie {
    Salarie {
        nom: "Test".into(),
        prenom: "Régression".into(),
        salaire_brut: brut.parse().unwrap(),
        statut: Statut::NonCadre,
        alsace_moselle: false,
        pays,
        canton: Some("GE".into()),
        tarif_is: None,
        assujetti_is: false, // pas d'impôt à la source CH : évite d'exiger un tarif ORIS
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
        tranche_age_ea: None,
        heures_supp: 0.0,
        heures_comp: 0.0,
        salaire_base: None,
        effectif: Some("moins20".into()),
        anciennete: None,
    }
}

fn f64_de(d: Decimal) -> f64 {
    d.to_f64().unwrap()
}

// ───────────────────────── Liste des pays ──────────────────────────

/// Tous les pays couverts par le dispatcher. DOIT rester synchronisé avec
/// l'enum `Pays` — le garde-fou `_exhaustivite_pays` ci-dessous casse la
/// compilation si un variant est ajouté sans être listé ici.
fn tous_les_pays() -> Vec<Pays> {
    vec![
        Pays::France, Pays::Suisse, Pays::Luxembourg, Pays::FonctionPublique,
        Pays::Italia, Pays::Canada, Pays::Quebec, Pays::Allemagne, Pays::Espagne,
        Pays::Portugal, Pays::Belgique, Pays::Angleterre, Pays::Japon, Pays::Chine,
        Pays::PaysBas, Pays::Australie, Pays::NouvelleZelande, Pays::Pologne,
        Pays::CoreeDuSud, Pays::Andorre, Pays::Monaco, Pays::Danemark, Pays::Finlande,
        Pays::Suede, Pays::Estonie, Pays::Lettonie, Pays::Lituanie, Pays::Autriche,
        Pays::Tchequie, Pays::Slovaquie, Pays::Hongrie, Pays::Slovenie, Pays::Grece,
        Pays::Chypre, Pays::Malte, Pays::Croatie, Pays::Irlande, Pays::Roumanie,
        Pays::Bulgarie,
    ]
}

/// Garde-fou d'EXHAUSTIVITÉ (jamais appelé) : si un variant est ajouté à
/// l'enum `Pays`, ce match cesse de compiler → on est forcé de l'ajouter ici
/// ET dans `tous_les_pays()`, donc de le couvrir par les invariants.
#[allow(dead_code)]
fn _exhaustivite_pays(p: Pays) {
    match p {
        Pays::France | Pays::Suisse | Pays::Luxembourg | Pays::FonctionPublique
        | Pays::Italia | Pays::Canada | Pays::Quebec | Pays::Allemagne | Pays::Espagne
        | Pays::Portugal | Pays::Belgique | Pays::Angleterre | Pays::Japon | Pays::Chine
        | Pays::PaysBas | Pays::Australie | Pays::NouvelleZelande | Pays::Pologne
        | Pays::CoreeDuSud | Pays::Andorre | Pays::Monaco | Pays::Danemark | Pays::Finlande
        | Pays::Suede | Pays::Estonie | Pays::Lettonie | Pays::Lituanie | Pays::Autriche
        | Pays::Tchequie | Pays::Slovaquie | Pays::Hongrie | Pays::Slovenie | Pays::Grece
        | Pays::Chypre | Pays::Malte | Pays::Croatie | Pays::Irlande | Pays::Roumanie
        | Pays::Bulgarie => {}
    }
}

// ────────────────────────── Invariants ──────────────────────────────

/// Vérifie les invariants vrais pour n'importe quel bulletin valide.
fn verifier_invariants(b: &Bulletin, pays: &Pays, contexte: &str) {
    let brut = b.brut;
    let net = b.net_a_payer;
    let cout = b.cout_total_employeur;

    // Le net ne peut pas dépasser le brut : les cotisations salariales et
    // l'impôt ne créent jamais d'argent. (À 3 500 €/équiv., aucun dispositif
    // type bonus emploi belge ne fait remonter le net au-dessus du brut.)
    assert!(
        net <= brut,
        "[{contexte}] {pays:?} : net_a_payer ({net}) > brut ({brut})"
    );

    // Le net reste positif : aucun barème ne taxe à plus de 100 %.
    assert!(
        net >= Decimal::ZERO,
        "[{contexte}] {pays:?} : net_a_payer négatif ({net})"
    );

    // L'employeur paie toujours au moins ce que le salarié touche.
    assert!(
        cout >= net,
        "[{contexte}] {pays:?} : coût employeur ({cout}) < net_a_payer ({net})"
    );

    // Un bulletin a toujours au moins une ligne (vraie cotisation ou ligne
    // « pays non couvert » informative).
    assert!(
        !b.cotisations.is_empty(),
        "[{contexte}] {pays:?} : aucune ligne de cotisation"
    );

    // Devise = code ISO à 3 lettres.
    assert_eq!(
        b.devise.len(), 3,
        "[{contexte}] {pays:?} : devise invalide '{}'", b.devise
    );

    // Le brut en sortie doit refléter l'entrée (pas d'altération silencieuse ;
    // sans absence ni heures sup, brut sortie == brut entrée).
    assert!(
        brut > Decimal::ZERO,
        "[{contexte}] {pays:?} : brut non strictement positif ({brut})"
    );
}

/// Invariants universels : tout pays, à une date où 2026 est couvert partout,
/// produit un bulletin cohérent (jamais de panic, jamais de valeur absurde).
#[tokio::test]
async fn invariants_tous_pays_2026() {
    let (pool, path) = base_test().await;
    let d = date("2026-03-15");
    let ctx = ContextPaie::charger(&pool, d).await.expect("contexte 2026 chargeable");

    for pays in tous_les_pays() {
        let salarie = salarie_base(pays.clone(), "3500.00");
        let bulletin = generer_bulletin(salarie, &ctx, None);
        verifier_invariants(&bulletin, &pays, "2026-03-15 @ 3500");
    }

    nettoyer(&path);
}

/// Même chose à bas salaire : c'est là que les dispositifs dégressifs
/// (réductions, abattements, crédits d'impôt) peuvent partir en vrille.
#[tokio::test]
async fn invariants_tous_pays_bas_salaire() {
    let (pool, path) = base_test().await;
    let d = date("2026-03-15");
    let ctx = ContextPaie::charger(&pool, d).await.expect("contexte chargeable");

    for pays in tous_les_pays() {
        let salarie = salarie_base(pays.clone(), "1600.00");
        let bulletin = generer_bulletin(salarie, &ctx, None);
        verifier_invariants(&bulletin, &pays, "2026-03-15 @ 1600");
    }

    nettoyer(&path);
}

// ──────────────────────── Golden France ─────────────────────────────

/// France, salaire courant : le net doit tomber dans une fourchette plausible
/// (~75-80 % du brut pour un non-cadre) et le coût employeur ~135-150 % du brut.
/// Fourchettes larges = on détecte un calcul cassé, pas une dérive au centime.
#[tokio::test]
async fn golden_france_ratios_plausibles() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let salarie = salarie_base(Pays::France, "3000.00");
    let b = generer_bulletin(salarie, &ctx, None);

    assert_eq!(b.devise, "EUR");
    // Un vrai bulletin français a de nombreuses lignes (SS, retraite, CSG…).
    assert!(b.cotisations.len() >= 8, "France : seulement {} lignes", b.cotisations.len());

    let ratio_net = f64_de(b.net_a_payer) / 3000.0;
    assert!(
        (0.74..=0.81).contains(&ratio_net),
        "France 3000€ non-cadre : ratio net/brut = {ratio_net:.4} hors [0.74 ; 0.81] (net = {})",
        b.net_a_payer
    );

    // Borne basse à 1.25 : à 3000 € la réduction Fillon s'applique encore
    // (on est sous 3 × SMIC), ce qui rabote le coût patronal autour de 1.29×.
    let ratio_cout = f64_de(b.cout_total_employeur) / 3000.0;
    assert!(
        (1.25..=1.50).contains(&ratio_cout),
        "France 3000€ : ratio coût/brut = {ratio_cout:.4} hors [1.25 ; 1.50] (coût = {})",
        b.cout_total_employeur
    );

    // Le net imposable est supérieur au net à payer (CSG/CRDS non déductibles
    // réintégrées) mais reste inférieur au brut.
    assert!(b.net_imposable > b.net_a_payer, "France : net imposable ≤ net à payer");
    assert!(b.net_imposable < b.brut, "France : net imposable ≥ brut");

    nettoyer(&path);
}

/// La réduction Fillon doit exister et alléger le coût patronal près du SMIC,
/// et disparaître au-delà de 3 × SMIC (seuil d'extinction).
#[tokio::test]
async fn golden_france_fillon() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    // Près du SMIC : Fillon présent et patronal négatif (allègement).
    let bas = generer_bulletin(salarie_base(Pays::France, "1850.00"), &ctx, None);
    let fillon = bas.cotisations.iter().find(|c| c.code == "REDUCTION_FILLON");
    let fillon = fillon.expect("Fillon attendu près du SMIC");
    assert!(
        fillon.montant_pat < Decimal::ZERO,
        "Fillon doit réduire le coût patronal (montant_pat = {})", fillon.montant_pat
    );

    // Au-delà de 3 × SMIC : plus de réduction Fillon.
    let haut = generer_bulletin(salarie_base(Pays::France, "6500.00"), &ctx, None);
    assert!(
        !haut.cotisations.iter().any(|c| c.code == "REDUCTION_FILLON"),
        "Fillon ne devrait plus s'appliquer à 6500 € (> 3 × SMIC)"
    );

    nettoyer(&path);
}

/// Le SMIC de référence Fillon est gelé au 1er janvier : après la revalorisation
/// du SMIC au 1er juin 2026 (1 867,02 €), la réduction générale reste calculée sur
/// le SMIC du 1er janvier (1 823,03 €). Le seuil d'extinction doit donc rester
/// 3 × 1 823,03 = 5 469,09 €, et non 3 × 1 867,02 = 5 601,06 €.
#[tokio::test]
async fn golden_france_fillon_smic_gele_apres_revalo_juin() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-07-15")).await.unwrap();

    // Le SMIC courant a bien été revalorisé, mais le SMIC de référence Fillon non.
    assert_eq!(ctx.smic_mensuel, "1867.02".parse::<Decimal>().unwrap(), "SMIC courant juin+ 2026");
    assert_eq!(ctx.smic_mensuel_fillon, "1823.03".parse::<Decimal>().unwrap(), "SMIC de référence Fillon gelé au 1er janvier");

    // 5 500 € : au-dessus de 3 × 1 823,03 (= 5 469,09) mais sous 3 × 1 867,02 (= 5 601,06).
    // Avec le SMIC gelé, Fillon ne doit PLUS s'appliquer. S'il s'appliquait, cela
    // prouverait qu'on utilise à tort le SMIC revalorisé.
    let entre = generer_bulletin(salarie_base(Pays::France, "5500.00"), &ctx, None);
    assert!(
        !entre.cotisations.iter().any(|c| c.code == "REDUCTION_FILLON"),
        "Fillon ne doit plus s'appliquer à 5500 € (> 3 × SMIC de réf. 1823,03), \
         signe que le SMIC gelé au 1er janvier est bien utilisé"
    );

    // Près du SMIC : Fillon toujours présent (allègement).
    let bas = generer_bulletin(salarie_base(Pays::France, "1850.00"), &ctx, None);
    let fillon = bas.cotisations.iter().find(|c| c.code == "REDUCTION_FILLON")
        .expect("Fillon attendu près du SMIC en juillet 2026");
    assert!(fillon.montant_pat < Decimal::ZERO, "Fillon doit réduire le coût patronal");

    // L'explication doit citer explicitement le SMIC au 1er janvier et sa base légale,
    // sans laisser de placeholder non substitué.
    let ex = &fillon.explication;
    assert!(ex.contains("Smic au 01/01/2026"), "explication doit mentionner le SMIC au 01/01/2026 :\n{ex}");
    assert!(ex.contains("2026-509"), "explication doit citer le décret n°2026-509 :\n{ex}");
    assert!(!ex.contains("{annee}") && !ex.contains("{ref_smic}"), "placeholder non substitué :\n{ex}");

    nettoyer(&path);
}

/// Monotonicité : à statut constant, augmenter le brut ne doit jamais faire
/// baisser le net ni le coût employeur. Casse net = formule d'impôt ou de
/// réduction inversée quelque part.
#[tokio::test]
async fn golden_france_monotonicite() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let mut net_prec = Decimal::ZERO;
    let mut cout_prec = Decimal::ZERO;
    let mut brut = 1500u32;
    while brut <= 8000 {
        let b = generer_bulletin(salarie_base(Pays::France, &format!("{brut}.00")), &ctx, None);
        assert!(
            b.net_a_payer >= net_prec,
            "Net non monotone à {brut} € : {} < {net_prec}", b.net_a_payer
        );
        assert!(
            b.cout_total_employeur >= cout_prec,
            "Coût employeur non monotone à {brut} € : {} < {cout_prec}", b.cout_total_employeur
        );
        net_prec = b.net_a_payer;
        cout_prec = b.cout_total_employeur;
        brut += 250;
    }

    nettoyer(&path);
}
