//! Couverture i18n — chaque libellé et chaque explication de calcul doit être
//! traduit dans les 5 langues non natives du menu 🌐 (en, de, nl, it, es).
//!
//! Méthode : on génère le bulletin de CHAQUE pays en français puis dans chaque
//! langue cible, et on exige que libellé et explication diffèrent du texte
//! français — sauf liste blanche explicite (acronymes et noms propres
//! volontairement identiques dans toutes les langues). Le test passe par le
//! vrai pipeline (`ctx.libelle` / `ctx.expl` / dispatcher i18n), donc un code
//! ajouté sans traduction (repli français silencieux) fait échouer le test.
//!
//! C'est le garde-fou qui aurait attrapé FPT_CNRACL, PAYS_NON_COUVERT et
//! l'explication manquante de NL_LOONHEFFING.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use sqlx::SqlitePool;

use xenna_paie_lib::calculs::generer_bulletin;
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{Pays, Salarie, Statut};

// ────────────────────────────── Outils ──────────────────────────────

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_i18n_{}_{}.db", std::process::id(), n));
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
        prenom: "I18n".into(),
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
        Pays::Bulgarie, Pays::EtatsUnis,
    ]
}

const LANGUES: [&str; 5] = ["en", "de", "nl", "it", "es"];

/// Identités de LIBELLÉ légitimes : soit un acronyme invariant (toutes
/// langues), soit un libellé natif déjà rédigé dans la langue cible (le
/// libellé français d'une cotisation allemande est en allemand, etc.).
/// Ajouter ici UNIQUEMENT après vérification — sinon c'est un oubli.
fn libelle_identique_ok(code: &str, lang: &str) -> bool {
    matches!(
        (code, lang),
        // Acronymes invariants dans toutes les langues.
        ("CRDS", _) | ("IE_USC", _)
        // Libellés natifs déjà dans la langue cible.
        | ("DE_KRANKENVERSICHERUNG", "de")
        | ("DE_RENTENVERSICHERUNG", "de")
        | ("DE_ARBEITSLOSENVERSICHERUNG", "de")
        | ("AGIRC_ARRCO_T1", "de") | ("AGIRC_ARRCO_T2", "de") // « Tranche » se dit Tranche en allemand
        | ("IT_IVS", "it") | ("IT_FONDO_GARANZIA", "it") | ("IT_INAIL", "it")
        | ("ES_FOGASA", "es") | ("ES_MEI", "es")
        | ("NL_OPSLAG_KO", "nl")
        | ("RO_CAS", "en") // « Pension » identique en anglais
        // Noms propres américains (Social Security, Medicare, SDI) — invariants.
        | ("US_SS", _) | ("US_MEDICARE", _) | ("US_ADD_MEDICARE", _) | ("US_CA_SDI", _)
    ) || (code.starts_with("IT_ADD_REG") && lang == "it")
}

/// Identités d'EXPLICATION légitimes (aucun cas connu à ce jour).
fn explication_identique_ok(_code: &str, _lang: &str) -> bool {
    false
}

/// Détecte un placeholder `{mot}` non substitué.
fn placeholder_restant(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            let fin = s[i + 1..].find('}').map(|j| i + 1 + j);
            if let Some(fin) = fin {
                let interieur = &s[i + 1..fin];
                if !interieur.is_empty()
                    && interieur.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                {
                    return true;
                }
            }
        }
        i += 1;
    }
    false
}

// ────────────────────────────── Tests ───────────────────────────────

/// Chaque ligne de chaque bulletin, dans chaque langue, doit différer du
/// français (libellé ET explication), sans placeholder oublié.
#[tokio::test]
async fn couverture_toutes_langues_tous_pays() {
    let (pool, path) = base_test().await;
    let d = date("2026-03-15");

    let ctx_fr = ContextPaie::charger(&pool, d).await.expect("contexte fr");
    let mut manques: Vec<String> = Vec::new();

    for pays in tous_les_pays() {
        let bulletin_fr = generer_bulletin(salarie_base(pays.clone(), "3500.00"), &ctx_fr, None);

        for lang in LANGUES {
            let mut ctx = ContextPaie::charger(&pool, d).await.expect("contexte");
            ctx.lang = lang.to_string();
            let bulletin = generer_bulletin(salarie_base(pays.clone(), "3500.00"), &ctx, None);

            assert_eq!(
                bulletin.cotisations.len(),
                bulletin_fr.cotisations.len(),
                "{pays:?}/{lang} : nombre de lignes différent du bulletin fr"
            );

            for (ligne, ligne_fr) in bulletin.cotisations.iter().zip(&bulletin_fr.cotisations) {
                assert_eq!(ligne.code, ligne_fr.code, "{pays:?}/{lang} : ordre des lignes divergent");
                let code = ligne.code.as_str();

                if ligne.libelle == ligne_fr.libelle && !libelle_identique_ok(code, lang) {
                    manques.push(format!("{pays:?}/{lang} libellé non traduit : {code} = '{}'", ligne.libelle));
                }
                if ligne.explication == ligne_fr.explication
                    && !explication_identique_ok(code, lang)
                {
                    manques.push(format!("{pays:?}/{lang} explication non traduite : {code}"));
                }
                if placeholder_restant(&ligne.libelle) {
                    manques.push(format!("{pays:?}/{lang} placeholder dans libellé {code} : '{}'", ligne.libelle));
                }
                if placeholder_restant(&ligne.explication) {
                    manques.push(format!("{pays:?}/{lang} placeholder dans explication {code}"));
                }
            }
        }
    }

    assert!(
        manques.is_empty(),
        "{} trou(s) de traduction :\n{}",
        manques.len(),
        manques.join("\n")
    );

    nettoyer(&path);
}

/// Variantes France non couvertes par le balayage standard : cadre
/// (AGIRC-ARRCO T2, prévoyance cadre) et Alsace-Moselle.
#[tokio::test]
async fn couverture_variantes_france() {
    let (pool, path) = base_test().await;
    let d = date("2026-03-15");
    let ctx_fr = ContextPaie::charger(&pool, d).await.unwrap();

    let mut cadre = salarie_base(Pays::France, "6000.00");
    cadre.statut = Statut::Cadre;
    cadre.alsace_moselle = true;
    let bulletin_fr = generer_bulletin(cadre.clone(), &ctx_fr, None);

    let mut manques: Vec<String> = Vec::new();
    for lang in LANGUES {
        let mut ctx = ContextPaie::charger(&pool, d).await.unwrap();
        ctx.lang = lang.to_string();
        let bulletin = generer_bulletin(cadre.clone(), &ctx, None);

        for (ligne, ligne_fr) in bulletin.cotisations.iter().zip(&bulletin_fr.cotisations) {
            let code = ligne.code.as_str();
            if ligne.libelle == ligne_fr.libelle && !libelle_identique_ok(code, lang) {
                manques.push(format!("France cadre/{lang} libellé non traduit : {code}"));
            }
            if ligne.explication == ligne_fr.explication && !explication_identique_ok(code, lang) {
                manques.push(format!("France cadre/{lang} explication non traduite : {code}"));
            }
        }
    }
    assert!(manques.is_empty(), "{}", manques.join("\n"));

    nettoyer(&path);
}

/// Le bulletin « année non couverte » doit être traduit de bout en bout :
/// libellé, message par pays et phrase de conclusion.
#[tokio::test]
async fn couverture_pays_non_couvert() {
    let (pool, path) = base_test().await;
    // 2020 : l'Autriche n'est couverte que 2025-2026 → bulletin informatif.
    let d = date("2020-03-15");

    let mut ctx = ContextPaie::charger(&pool, d).await.expect("contexte 2020");
    ctx.lang = "en".to_string();
    let bulletin = generer_bulletin(salarie_base(Pays::Autriche, "3500.00"), &ctx, None);

    assert_eq!(bulletin.cotisations.len(), 1, "bulletin non couvert = 1 ligne");
    let ligne = &bulletin.cotisations[0];
    assert_eq!(ligne.code, "PAYS_NON_COUVERT");
    assert_eq!(ligne.libelle, "Data unavailable for this year");
    assert!(
        ligne.explication.starts_with("Austria: data available for 2025 and 2026."),
        "1re ligne du message attendue en anglais :\n{}",
        ligne.explication
    );
    assert!(
        ligne.explication.contains("No figures are applied"),
        "phrase de conclusion attendue en anglais :\n{}",
        ligne.explication
    );

    // En français, texte natif inchangé.
    let ctx_fr = ContextPaie::charger(&pool, d).await.unwrap();
    let bulletin_fr = generer_bulletin(salarie_base(Pays::Autriche, "3500.00"), &ctx_fr, None);
    let ligne_fr = &bulletin_fr.cotisations[0];
    assert_eq!(ligne_fr.libelle, "Données indisponibles pour cette année");
    assert!(ligne_fr.explication.starts_with("Autriche : données disponibles pour 2025 et 2026."));

    nettoyer(&path);
}

/// Les références légales doivent gloser les codes français (décision produit :
/// nom officiel + traduction entre parenthèses) et traduire les mots
/// descriptifs, dans les 5 langues.
#[tokio::test]
async fn glose_references_legales() {
    use xenna_paie_lib::i18n::t_loi_ref;

    let r = t_loi_ref("CSS art. L241-3", "en");
    assert_eq!(r, "CSS (Social Security Code) art. L241-3");

    let r = t_loi_ref("CT art. L5213-19 et R5213-76 — Arrêté du 16/01/2025", "en");
    assert_eq!(r, "CT (French Labour Code) art. L5213-19 and R5213-76 — Decree of 16/01/2025");

    let r = t_loi_ref(
        "Décret n°2011-291 du 15/03/2011 — CGFP art. L712-3 et s. — \
        Loi n°83-634 du 13/07/1983 (statut général FP)",
        "de",
    );
    assert!(r.contains("Dekret"), "Décret → Dekret : {r}");
    assert!(r.contains("CGFP (Allgemeines Beamtengesetzbuch) Art."), "glose CGFP : {r}");
    assert!(r.contains("(allgemeines Beamtenstatut)"), "statut général FP : {r}");

    // Les références déjà en langue nationale traversent inchangées.
    let grec = "Ν. 4387/2016 (EFKA)";
    assert_eq!(t_loi_ref(grec, "es"), grec);

    // En français : identité stricte.
    let fr = "Loi n°2003-47 du 17/01/2003 (Fillon) — CSS art. L241-13 et D241-7";
    assert_eq!(t_loi_ref(fr, "fr"), fr);
}
