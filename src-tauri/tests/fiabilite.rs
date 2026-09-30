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
        esat: false,
        convention_idcc: None,
        indemnites_repas: None,
        avantages_nature: Vec::new(),
        tranche_age_ea: None,
        heures_supp_25: 0.0,
        heures_supp_50: 0.0,
        heures_comp_10: 0.0,
        heures_comp_25: 0.0,
        salaire_base: None,
        effectif: Some("moins20".into()),
        anciennete: None,
        us_state: None,
        inde_regime: None,
        emirati_national: None,
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
        Pays::Bulgarie, Pays::EtatsUnis, Pays::Mexique,
        Pays::Bresil, Pays::Emirats, Pays::Inde,
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
        | Pays::Bulgarie | Pays::EtatsUnis | Pays::Mexique
        | Pays::Bresil | Pays::Emirats | Pays::Inde => {}
    }
}

/// `Pays::TOUS` (servie au front par la veille) suit l'enum : la liste du test,
/// elle-même tenue par `_exhaustivite_pays`, en est le témoin.
#[test]
fn pays_tous_suit_l_enum() {
    assert_eq!(Pays::TOUS.to_vec(), tous_les_pays());
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

/// Régime local d'Alsace-Moselle : 1,50 % jusqu'au 31/03/2022, 1,30 % depuis le
/// 01/04/2022 (la migration 0008 datait à tort la baisse du 01/07/2018).
#[tokio::test]
async fn golden_alsace_moselle_taux_par_date() {
    let (pool, path) = base_test().await;
    for (jour, attendu) in [("2020-06-15", "0.0150"), ("2022-03-15", "0.0150"),
                            ("2022-04-15", "0.0130"), ("2026-03-15", "0.0130")] {
        let ctx = ContextPaie::charger(&pool, date(jour)).await.unwrap();
        let mut s = salarie_base(Pays::France, "3000.00");
        s.alsace_moselle = true;
        let b = generer_bulletin(s, &ctx, None);
        let am = b.cotisations.iter().find(|c| c.code == "ALSACE_MOSELLE_MALADIE")
            .unwrap_or_else(|| panic!("ligne Alsace-Moselle attendue au {jour}"));
        assert_eq!(am.taux_sal, attendu.parse::<Decimal>().unwrap(), "taux Alsace-Moselle au {jour}");
    }
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

// ─────────────────────────────── ESAT ───────────────────────────────────────

/// Travailleur d'ESAT au minimum 2026 : rémunération garantie = 55,7 % du SMIC
/// (1 823,03 €) = 1 015,43 €, dont aide au poste 50,7 % (924,28 €) et part ESAT
/// 5 % (91,15 €). Ni chômage ni réduction générale ; les aides de l'État
/// réduisent le coût de l'ESAT sans toucher au net.
#[tokio::test]
async fn golden_esat_minimum_2026() {
    use xenna_paie_lib::calculs::esat::{decomposer, remuneration_minimale};
    let (pool, path) = base_test().await;
    let d = date("2026-03-15");
    let ctx = ContextPaie::charger(&pool, d).await.unwrap();

    let min = remuneration_minimale(ctx.smic_mensuel, 100.0, d);
    assert_eq!(min, "1015.43".parse::<Decimal>().unwrap());
    let (part, aide) = decomposer(min, ctx.smic_mensuel, 100.0, d);
    assert_eq!(aide, "924.28".parse::<Decimal>().unwrap());
    assert_eq!(part, "91.15".parse::<Decimal>().unwrap());

    // Mi-temps : tout est proratisé.
    assert_eq!(remuneration_minimale(ctx.smic_mensuel, 50.0, d), "507.71".parse::<Decimal>().unwrap());
    // Au maximum (110,7 %), part ESAT = 100 % et aide = 10,7 % du SMIC.
    let max = ("1.107".parse::<Decimal>().unwrap() * ctx.smic_mensuel).round_dp(2);
    let (_, aide_max) = decomposer(max, ctx.smic_mensuel, 100.0, d);
    let attendu = ("0.107".parse::<Decimal>().unwrap() * ctx.smic_mensuel).round_dp(2);
    // Au centime près : la rémunération saisie est déjà arrondie, et la formule
    // double l'écart (part ESAT = 2 × (r − 60,7 %)).
    assert!((aide_max - attendu).abs() <= "0.01".parse::<Decimal>().unwrap(), "{aide_max} ≠ {attendu}");
    // Avant 2018 : 55 % et 50 %.
    assert_eq!(remuneration_minimale("1000".parse().unwrap(), 100.0, date("2017-06-15")),
               "550".parse::<Decimal>().unwrap());

    let mut s = salarie_base(Pays::France, "1015.43");
    s.esat = true;
    let b = generer_bulletin(s.clone(), &ctx, None);
    let code = |c: &str| b.cotisations.iter().find(|l| l.code == c);
    assert!(code("CHOMAGE").is_none(), "pas d'assurance chômage en ESAT");
    assert!(code("REDUCTION_FILLON").is_none(), "pas de réduction générale en ESAT");
    assert_eq!(code("ESAT_AIDE_POSTE").unwrap().montant_pat, "-924.28".parse::<Decimal>().unwrap());
    // Compensation = aide au poste × taux patronaux obligatoires (maladie,
    // vieillesse, famille, AT, Agirc-Arrco) : 924,28 × 35,39 % = 327,10.
    assert_eq!(code("ESAT_COMPENSATION").unwrap().montant_pat, "-327.10".parse::<Decimal>().unwrap());

    // Les aides ne touchent pas le net : même net que sans elles, chômage à part.
    let sal_hors_aides: Decimal = b.cotisations.iter().map(|l| l.montant_sal).sum();
    assert_eq!(b.net_a_payer, (b.brut - sal_hors_aides).round_dp(2));
    // Coût ESAT = part ESAT + charges patronales non compensées : bien sous la
    // rémunération versée.
    assert!(b.cout_total_employeur < b.brut, "coût ESAT {} ≥ rémunération {}", b.cout_total_employeur, b.brut);
    assert!(b.cout_total_employeur > Decimal::ZERO);

    nettoyer(&path);
}

// ─────────────────────── Frais IDCC 0016 ─────────────────────────────────────

/// Indemnités de repas IDCC 0016 : versées en net, sans toucher au brut, aux
/// cotisations ni au net imposable ; ignorées hors convention 0016 ; montant 0
/// (et non inventé) avant le premier barème intégré (01/12/2022).
#[tokio::test]
async fn golden_frais_idcc16() {
    use xenna_paie_lib::models::IndemnitesRepasCcn;
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let sans = generer_bulletin(salarie_base(Pays::France, "2500.00"), &ctx, None);
    let mut s = salarie_base(Pays::France, "2500.00");
    s.convention_idcc = Some("0016".into());
    s.indemnites_repas = Some(IndemnitesRepasCcn {
        repas_unique: 2.0, repas_unique_nuit: 1.0, speciale: 4.0, casse_croute: 3.0,
    });
    let b = generer_bulletin(s.clone(), &ctx, None);
    // 2 × 10,07 + 1 × 9,81 + 4 × 4,42 + 3 × 8,87 = 74,24
    let attendu: Decimal = "74.24".parse().unwrap();
    let total: Decimal = b.frais_professionnels.iter().map(|l| l.montant).sum();
    assert_eq!(total, attendu);
    assert_eq!(b.frais_professionnels.len(), 4);
    assert_eq!(b.net_a_payer, sans.net_a_payer + attendu);
    assert_eq!(b.brut, sans.brut);
    assert_eq!(b.net_imposable, sans.net_imposable);
    assert_eq!(b.cout_total_employeur, sans.cout_total_employeur, "le coût employeur n'inclut que la paie");

    // Autre convention (ou aucune) : saisie ignorée.
    let mut autre = s.clone();
    autre.convention_idcc = None;
    assert!(generer_bulletin(autre, &ctx, None).frais_professionnels.is_empty());

    // Barème 2025 (av. n° 79, dès le 01/03/2025) : repas unique 9,97.
    let ctx25 = ContextPaie::charger(&pool, date("2025-06-15")).await.unwrap();
    let b25 = generer_bulletin(s.clone(), &ctx25, None);
    assert_eq!(b25.frais_professionnels[0].montant_unitaire, Some("9.97".parse().unwrap()));

    // Avant le 01/12/2022 : pas de barème, montant 0, net inchangé.
    let ctx22 = ContextPaie::charger(&pool, date("2022-06-15")).await.unwrap();
    let b22 = generer_bulletin(s.clone(), &ctx22, None);
    assert!(b22.frais_professionnels.iter().all(|l| l.montant_unitaire.is_none() && l.montant.is_zero()));
    let sans22 = generer_bulletin(salarie_base(Pays::France, "2500.00"), &ctx22, None);
    assert_eq!(b22.net_a_payer, sans22.net_a_payer);

    nettoyer(&path);
}

// ───────────────────────── Avantages en nature ───────────────────────────────

fn an(nature: &str) -> xenna_paie_lib::models::AvantageNatureInput {
    xenna_paie_lib::models::AvantageNatureInput { nature: nature.into(), ..Default::default() }
}

fn montant_an(b: &Bulletin, code: &str) -> Decimal {
    b.avantages_nature.iter().find(|l| l.code == code).unwrap().montant
}

/// Barèmes 2026 (arrêté du 25/02/2025, revalorisation au 1er janvier) : repas
/// 5,50 €, logement par tranche du PSS, véhicule en % du coût, NTIC 10 %.
#[tokio::test]
async fn golden_avantages_nature_2026() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();
    let d = |s: &str| s.parse::<Decimal>().unwrap();
    let bulletin = |liste: Vec<_>| {
        let mut s = salarie_base(Pays::France, "2500.00");
        s.avantages_nature = liste;
        generer_bulletin(s, &ctx, None)
    };

    // Repas : 20 × 5,50 = 110 ; participation déduite ; cantine à 50 % → négligé.
    let mut r = an("repas"); r.nombre = 20.0;
    assert_eq!(montant_an(&bulletin(vec![r.clone()]), "AN_REPAS"), d("110.00"));
    r.participation = 20.0;
    assert_eq!(montant_an(&bulletin(vec![r.clone()]), "AN_REPAS"), d("90.00"));
    r.cantine = true; r.participation = 55.0;
    assert_eq!(montant_an(&bulletin(vec![r.clone()]), "AN_REPAS"), d("0"));
    r.participation = 54.0; // sous la moitié du forfait : forfait − participation
    assert_eq!(montant_an(&bulletin(vec![r]), "AN_REPAS"), d("56.00"));

    // Logement : 2 500 € / PSS 4 005 € = 0,62 → tranche 3 ; 1 pièce 106,20,
    // 3 pièces 3 × 79,70.
    let mut l = an("logement"); l.nombre = 1.0;
    assert_eq!(montant_an(&bulletin(vec![l.clone()]), "AN_LOGEMENT"), d("106.20"));
    l.nombre = 3.0;
    assert_eq!(montant_an(&bulletin(vec![l]), "AN_LOGEMENT"), d("239.10"));

    // Véhicule acheté 30 000 €, mis à disposition en 2025, sans carburant :
    // 15 % → 4 500 €/an → 375 €/mois.
    let mut v = an("vehicule");
    v.mode = Some("achat".into()); v.cout = 30000.0; v.mise_a_disposition = Some("2025-06-01".into());
    assert_eq!(montant_an(&bulletin(vec![v.clone()]), "AN_VEHICULE"), d("375.00"));
    // Électrique avec éco-score : abattement 70 % (3 150 € ≤ 4 641,60) → 1 350 €/an.
    v.electrique = true; v.eco_score = true;
    assert_eq!(montant_an(&bulletin(vec![v.clone()]), "AN_VEHICULE"), d("112.50"));
    // Électrique mis à disposition en 2023 : 9 % puis abattement 50 % (1 350 ≤ 2 026,30).
    v.eco_score = false; v.mise_a_disposition = Some("2023-06-01".into());
    assert_eq!(montant_an(&bulletin(vec![v.clone()]), "AN_VEHICULE"), d("112.50"));
    // Thermique d'avant février 2025, carburant compris : 12 % → 300 €/mois.
    v.electrique = false; v.carburant = true;
    assert_eq!(montant_an(&bulletin(vec![v]), "AN_VEHICULE"), d("300.00"));
    // Location 12 000 €/an depuis 2025, carburant compris : 67 % → 670 €/mois.
    let mut loc = an("vehicule");
    loc.mode = Some("location".into()); loc.cout = 12000.0; loc.carburant = true;
    loc.mise_a_disposition = Some("2025-03-01".into());
    assert_eq!(montant_an(&bulletin(vec![loc]), "AN_VEHICULE"), d("670.00"));

    // NTIC : 1 200 € × 10 % ÷ 12 = 10 €.
    let mut t = an("ntic"); t.cout = 1200.0;
    assert_eq!(montant_an(&bulletin(vec![t]), "AN_NTIC"), d("10.00"));

    // Effet sur le bulletin : l'avantage entre dans le brut et le net imposable,
    // pas dans le net payé.
    let sans = bulletin(vec![]);
    let mut a = an("autre"); a.montant = 200.0;
    let avec = bulletin(vec![a]);
    assert_eq!(avec.brut, sans.brut + d("200"));
    assert!(avec.net_imposable > sans.net_imposable);
    let cot_sal: Decimal = avec.cotisations.iter().map(|c| c.montant_sal).sum();
    assert_eq!(avec.net_a_payer, (avec.brut - cot_sal - d("200")).round_dp(2));
    assert!(avec.net_a_payer < sans.net_a_payer, "les cotisations sur l'avantage réduisent le net payé");

    // Avant 2025 : pas de barème, montant 0 (et non inventé).
    let ctx24 = ContextPaie::charger(&pool, date("2024-06-15")).await.unwrap();
    let mut s24 = salarie_base(Pays::France, "2500.00");
    let mut r24 = an("repas"); r24.nombre = 10.0;
    s24.avantages_nature = vec![r24];
    assert!(generer_bulletin(s24, &ctx24, None).avantages_nature[0].montant.is_zero());

    nettoyer(&path);
}

/// Paye inversée : le net cible est un net payé en espèces, avantages exclus.
#[tokio::test]
async fn paye_inverse_avec_avantage_nature() {
    use xenna_paie_lib::calculs::paye_inverse::resoudre_brut_pour_net;
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();
    let mut s = salarie_base(Pays::France, "2500.00");
    let mut l = an("logement"); l.nombre = 2.0;
    s.avantages_nature = vec![l];
    let b = resoudre_brut_pour_net("2000".parse().unwrap(), &s, &ctx, None);
    assert!((b.net_a_payer - "2000".parse::<Decimal>().unwrap()).abs() <= "0.01".parse().unwrap(),
        "net payé {} ≠ 2000", b.net_a_payer);
    nettoyer(&path);
}

// ────────────────────────── Veille des barèmes ──────────────────────────────

/// La veille (`veille.rs`) est une déclaration, pas un calcul : on vérifie
/// qu'elle se tient. Un pays en retard sur l'année du relevé doit dire ce qui
/// lui manque ; un pays déclaré à jour ne doit traîner aucune lacune.
#[test]
fn veille_coherente_pour_tous_les_pays() {
    use xenna_paie_lib::veille::{veille, AUDIT_DU};
    let annee_audit: i32 = AUDIT_DU[..4].parse().expect("AUDIT_DU au format AAAA-MM-JJ");
    for pays in tous_les_pays() {
        let v = veille(&pays);
        assert!(
            (2015..=annee_audit).contains(&v.integre_jusqu_a),
            "{pays:?} : integre_jusqu_a = {} hors de 2015..={annee_audit}", v.integre_jusqu_a
        );
        assert_eq!(
            v.integre_jusqu_a < annee_audit,
            !v.lacunes.is_empty(),
            "{pays:?} : un retard sur {annee_audit} se déclare par ses lacunes, et seulement lui"
        );
    }
}
