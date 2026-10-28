//! Lectures de la table bareme_tranche (migration 0150) : paliers, linéaire par
//! tranche, progressif. Valeurs de contrôle tirées des barèmes japonais en base,
//! recalculées à la main.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use xenna_paie_lib::db::{init_db, ContextPaie};

use std::sync::atomic::{AtomicU64, Ordering};

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

fn d(s: &str) -> Decimal { s.parse().unwrap() }

async fn ctx(date: &str) -> ContextPaie {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_baremes_{}_{n}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let pool = init_db(&path).await.unwrap();
    let c = ContextPaie::charger(&pool, NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()).await.unwrap();
    let _ = std::fs::remove_file(&path);
    c
}

/// Palier : borne exclue. 92 999,99 ¥ → 88 000 ; 93 000 → 98 000 ; au-delà du
/// dernier palier, sa valeur (650 000 pension, 1 390 000 maladie).
#[tokio::test]
async fn palier_borne_exclue() {
    let c = ctx("2026-06-15").await;
    assert_eq!(c.bareme_palier("JP_HYOJUN_KOSEI", d("92999.99")), Some(d("88000")));
    assert_eq!(c.bareme_palier("JP_HYOJUN_KOSEI", d("93000")), Some(d("98000")));
    assert_eq!(c.bareme_palier("JP_HYOJUN_KOSEI", d("5000000")), Some(d("650000")));
    assert_eq!(c.bareme_palier("JP_HYOJUN_KENPO", d("1355000")), Some(d("1390000")));
}

/// Linéaire : borne incluse. 1 800 000 × 40 % − 100 000 = 620 000 ;
/// 1 800 001 passe à la tranche suivante : × 30 % + 80 000 = 620 000,3.
#[tokio::test]
async fn lineaire_borne_incluse() {
    let c = ctx("2026-06-15").await;
    assert_eq!(c.bareme_lineaire("JP_KYUYO_KOJO", d("1800000")), Some(d("620000")));
    assert_eq!(c.bareme_lineaire("JP_KYUYO_KOJO", d("1800001")), Some(d("620000.30")));
    assert_eq!(c.bareme_lineaire("JP_KYUYO_KOJO", d("9000000")), Some(d("1950000")));
}

/// Progressif : 3 300 000 = 1 950 000 × 5 % + 1 350 000 × 10 % = 232 500 ;
/// 40 000 000 = 13 204 000 (cumul de l'art. 89) ; 0 → 0.
#[tokio::test]
async fn progressif_cumul() {
    let c = ctx("2026-06-15").await;
    assert_eq!(c.bareme_progressif("JP_SHOTOKU", d("3300000")), Some(d("232500")));
    assert_eq!(c.bareme_progressif("JP_SHOTOKU", d("40000000")), Some(d("13204000")));
    assert_eq!(c.bareme_progressif("JP_SHOTOKU", d("40000100")), Some(d("13204045")));
    assert_eq!(c.bareme_progressif("JP_SHOTOKU", Decimal::ZERO), Some(Decimal::ZERO));
}

/// Datation : la déduction de base change au 1er janvier 2025 puis 2026 ; avant
/// 2020, le barème n'est pas en base (régime non couvert).
#[tokio::test]
async fn baremes_dates() {
    assert_eq!(ctx("2024-06-15").await.bareme_lineaire("JP_KISO_KOJO", d("3000000")), Some(d("480000")));
    assert_eq!(ctx("2025-06-15").await.bareme_lineaire("JP_KISO_KOJO", d("3000000")), Some(d("880000")));
    assert_eq!(ctx("2026-06-15").await.bareme_lineaire("JP_KISO_KOJO", d("3000000")), Some(d("1040000")));
    assert!(ctx("2019-06-15").await.bareme("JP_KISO_KOJO").is_none());
}
