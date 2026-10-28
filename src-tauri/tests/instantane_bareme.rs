//! Instantané d'équivalence pour la migration des barèmes en base (point 4a).
//! Écrit toutes les lignes d'un régime pour une grille de salaires et de dates ;
//! on compare le fichier avant et après la migration. Ignoré par défaut :
//! `INSTANTANE_PAYS=japon INSTANTANE_OUT=/chemin cargo test --test instantane_bareme -- --ignored`

use std::path::PathBuf;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use xenna_paie_lib::calculs::generer_bulletin;
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{Pays, Salarie, Statut};

fn d(s: &str) -> Decimal { s.parse().unwrap() }

/// Salarié neutre : aucun champ régional renseigné, à chaque test de poser
/// ce dont son pays a besoin.
fn salarie(pays: Pays, brut: &str) -> Salarie {
    Salarie {
        nom: "Golden".into(),
        prenom: "Référence".into(),
        salaire_brut: d(brut),
        statut: Statut::NonCadre,
        alsace_moselle: false,
        pays,
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

#[tokio::test]
#[ignore]
async fn instantane() {
    let pays: Pays = serde_json::from_str(&format!("\"{}\"", std::env::var("INSTANTANE_PAYS").unwrap())).unwrap();
    let out = std::env::var("INSTANTANE_OUT").unwrap();
    let path: PathBuf = std::env::temp_dir().join(format!("xenna_instantane_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let pool = init_db(&path).await.unwrap();
    let pas: u32 = std::env::var("INSTANTANE_PAS").ok().and_then(|v| v.parse().ok()).unwrap_or(5000);
    let max: u32 = std::env::var("INSTANTANE_MAX").ok().and_then(|v| v.parse().ok()).unwrap_or(3_000_000);
    let mut bruts: Vec<String> = (0..=max / pas).map(|i| (i * pas).to_string()).collect();
    for extra in std::env::var("INSTANTANE_EXTRA").unwrap_or_default().split(',').filter(|s| !s.is_empty()) {
        bruts.push(extra.to_string());
    }
    // Dates : INSTANTANE_DATES (séparées par des virgules), sinon le 15 de
    // février et de juin de 2015 à 2027 — de quoi traverser chaque changement
    // annuel, et l'exercice britannique (6 avril).
    let dates: Vec<String> = match std::env::var("INSTANTANE_DATES") {
        Ok(v) => v.split(',').map(str::to_string).collect(),
        Err(_) => (2015..=2027).flat_map(|a| [format!("{a}-02-15"), format!("{a}-06-15")]).collect(),
    };
    // Variantes du salarié : INSTANTANE_VARIANTES, objets JSON séparés par « ; »
    // fusionnés dans le salarié (ex. {"steuerklasse":3}), sinon le salarié neutre.
    let variantes: Vec<serde_json::Value> = std::env::var("INSTANTANE_VARIANTES").ok()
        .map(|v| v.split(';').map(|o| serde_json::from_str(o).unwrap()).collect())
        .unwrap_or_else(|| vec![serde_json::json!({})]);
    let mut lignes = Vec::new();
    for date in &dates {
      for variante in &variantes {
        let ctx = ContextPaie::charger(&pool, NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()).await.unwrap();
        for brut in &bruts {
            let mut v = serde_json::to_value(salarie(pays.clone(), brut)).unwrap();
            for (k, x) in variante.as_object().unwrap() { v[k] = x.clone(); }
            let s: Salarie = serde_json::from_value(v).unwrap();
            let b = generer_bulletin(s, &ctx, None);
            lignes.push(format!("{date} {variante} {brut} net={} cout={}", b.net_a_payer, b.cout_total_employeur));
            for c in &b.cotisations {
                lignes.push(format!("  {} base={} ts={} ms={} tp={} mp={}", c.code, c.base, c.taux_sal, c.montant_sal, c.taux_pat, c.montant_pat));
            }
        }
      }
    }
    std::fs::write(&out, lignes.join("\n")).unwrap();
    let _ = std::fs::remove_file(&path);
}
