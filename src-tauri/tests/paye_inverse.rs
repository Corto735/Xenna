//! Paye inversée — le brut reconstitué doit produire exactement le net demandé
//! (net AVANT impôt à la source : brut − cotisations sociales salariales),
//! l'impôt étant ensuite calculé normalement sur ce brut.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::SqlitePool;

use xenna_paie_lib::calculs::generer_bulletin;
use xenna_paie_lib::calculs::paye_inverse::{net_avant_impot, resoudre_brut_pour_net};
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{Pays, Salarie, Statut};

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_inv_{}_{}.db", std::process::id(), n));
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
        tranche_age_ea: None,
        heures_supp: 0.0,
        heures_comp: 0.0,
        salaire_base: None,
        effectif: Some("moins20".into()),
        anciennete: None,
        us_state: None,
    }
}

const TOL: &str = "0.02";

fn proche(a: Decimal, b: Decimal) -> bool {
    (a - b).abs() <= TOL.parse().unwrap()
}

/// France : pas d'impôt modélisé → la cible coïncide avec net_a_payer.
#[tokio::test]
async fn france_net_atteint() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let cible = dec!(2500);
    let b = resoudre_brut_pour_net(cible, &salarie_base(Pays::France, "1.00"), &ctx, None);

    assert!(proche(b.net_a_payer, cible), "net_a_payer = {} ≠ {cible}", b.net_a_payer);
    assert!(proche(net_avant_impot(&b), cible), "net avant impôt = {}", net_avant_impot(&b));
    assert!(b.brut > cible, "le brut reconstitué ({}) doit dépasser le net", b.brut);
    assert!(b.cotisations.len() >= 8, "bulletin France complet attendu");

    nettoyer(&path);
}

/// Aller-retour : net(bulletin(B)) → résoudre → brut ≈ B.
#[tokio::test]
async fn france_aller_retour() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let direct = generer_bulletin(salarie_base(Pays::France, "3000.00"), &ctx, None);
    let inverse = resoudre_brut_pour_net(direct.net_a_payer, &salarie_base(Pays::France, "1.00"), &ctx, None);

    assert!(
        proche(inverse.brut, dec!(3000)),
        "aller-retour : brut reconstitué {} ≠ 3000", inverse.brut
    );

    nettoyer(&path);
}

/// Italie : la cible est le net AVANT IRPEF ; l'IRPEF, calculée sur le brut
/// reconstitué, rend le net à payer inférieur à la cible.
#[tokio::test]
async fn italie_net_avant_impot() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let cible = dec!(2200);
    let b = resoudre_brut_pour_net(cible, &salarie_base(Pays::Italia, "1.00"), &ctx, None);

    assert!(proche(net_avant_impot(&b), cible), "net avant impôt = {}", net_avant_impot(&b));
    assert!(
        b.cotisations.iter().any(|c| c.code.starts_with("IT_IRPEF")),
        "la ligne IRPEF doit exister sur le brut reconstitué"
    );
    assert!(
        b.net_a_payer < cible,
        "net à payer ({}) doit être < cible ({cible}) une fois l'IRPEF retenue", b.net_a_payer
    );

    nettoyer(&path);
}

/// Allemagne : idem avec la Lohnsteuer.
#[tokio::test]
async fn allemagne_net_avant_impot() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let cible = dec!(2800);
    let b = resoudre_brut_pour_net(cible, &salarie_base(Pays::Allemagne, "1.00"), &ctx, None);

    assert!(proche(net_avant_impot(&b), cible), "net avant impôt = {}", net_avant_impot(&b));
    assert!(b.net_a_payer < cible, "Lohnsteuer attendue en plus : net = {}", b.net_a_payer);

    nettoyer(&path);
}

/// France + heures supplémentaires : le net cible inclut l'effet des HS
/// (majoration + réduction salariale), dérivées du brut sondé.
#[tokio::test]
async fn france_heures_sup() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-03-15")).await.unwrap();

    let mut s = salarie_base(Pays::France, "1.00");
    s.heures_supp = 10.0;
    let cible = dec!(2600);
    let b = resoudre_brut_pour_net(cible, &s, &ctx, None);

    assert!(proche(b.net_a_payer, cible), "net avec HS = {} ≠ {cible}", b.net_a_payer);
    assert!(b.heures_sup.is_some(), "les HS doivent être calculées sur le brut reconstitué");

    nettoyer(&path);
}

/// Pays non couvert à la date demandée : net = brut → solution triviale exacte.
#[tokio::test]
async fn pays_non_couvert_trivial() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2020-03-15")).await.unwrap();

    let cible = dec!(2500);
    let b = resoudre_brut_pour_net(cible, &salarie_base(Pays::Autriche, "1.00"), &ctx, None);

    assert_eq!(b.brut, cible, "net = brut → brut reconstitué = cible exactement");
    assert_eq!(b.net_a_payer, cible);

    nettoyer(&path);
}
