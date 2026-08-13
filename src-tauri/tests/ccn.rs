//! Intégrité du référentiel des conventions collectives.
//!
//! Le contenu de la page « Conventions collectives » vit en base, posé par
//! une migration de seed. C'est du texte : rien ne le compile, rien ne le
//! type. Une coquille dans un code d'activité ou de thème ne casse aucun
//! calcul — elle produit juste une règle invisible derrière un filtre qui
//! ne la sélectionne jamais. Personne ne s'en apercevrait.
//!
//! D'où ces tests, qui rejouent les vraies migrations et vérifient que le
//! référentiel se tient : clés résolues, énumérations respectées, seed
//! IDCC 0016 complet sur ses sept activités.
//!
//! Aucun de ces enregistrements n'entre dans un bulletin : le moteur garde
//! ses barèmes dans `ContextPaie` et `calculs::absence`. On teste ici de la
//! documentation, pas du droit appliqué.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use sqlx::SqlitePool;

use xenna_paie_lib::db::init_db;

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_ccn_{}_{}.db", std::process::id(), n));
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

async fn compter(pool: &SqlitePool, sql: &str) -> i64 {
    sqlx::query_as::<_, (i64,)>(sql)
        .fetch_one(pool)
        .await
        .expect("requête de comptage")
        .0
}

/// Toute règle doit pointer vers une activité et un thème existants.
/// SQLite n'applique pas les clés étrangères par défaut : la contrainte
/// déclarée dans la migration ne protège de rien toute seule.
#[tokio::test]
async fn aucune_reference_orpheline() {
    let (pool, path) = base_test().await;

    let act_orphelines = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations r
          WHERE NOT EXISTS (SELECT 1 FROM ccn_activites a
                             WHERE a.code = r.activite AND a.idcc = r.idcc)",
    )
    .await;
    assert_eq!(act_orphelines, 0, "règles rattachées à une activité inexistante");

    let themes_orphelins = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations r
          WHERE NOT EXISTS (SELECT 1 FROM ccn_themes t WHERE t.code = r.theme)",
    )
    .await;
    assert_eq!(themes_orphelins, 0, "règles rattachées à un thème inexistant");

    let ccn_orphelines = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations r
          WHERE NOT EXISTS (SELECT 1 FROM ccn_conventions c WHERE c.idcc = r.idcc)",
    )
    .await;
    assert_eq!(ccn_orphelines, 0, "règles rattachées à une convention inexistante");

    nettoyer(&path);
}

/// Les valeurs d'énumération portées par le front doivent correspondre à
/// celles présentes en base. Un libellé d'impact inventé s'afficherait tel
/// quel dans une pastille, sans jamais lever d'erreur.
#[tokio::test]
async fn enumerations_respectees() {
    let (pool, path) = base_test().await;

    let impacts_hs = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations
          WHERE impact NOT IN ('Brut','Cotisations','Net imposable',
                               'Cout employeur','Temps de travail','Hors bulletin')",
    )
    .await;
    assert_eq!(impacts_hs, 0, "impact hors des valeurs attendues");

    let statuts_hs = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations
          WHERE statut_verif NOT IN ('brouillon','a_verifier','verifie')",
    )
    .await;
    assert_eq!(statuts_hs, 0, "statut de vérification hors des valeurs attendues");

    // Une date d'effet non nulle doit être en ISO : le front la reformate
    // en français par expression régulière et la laisse passer telle quelle
    // sinon — le lecteur verrait alors une date brute au milieu du texte.
    let dates_hs = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations
          WHERE date_effet IS NOT NULL
            AND date_effet NOT GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'",
    )
    .await;
    assert_eq!(dates_hs, 0, "date d'effet hors format AAAA-MM-JJ");

    // Un lien qui ne serait pas http(s) est écarté au rendu : autant ne pas
    // en stocker qui ne s'afficheront jamais.
    let liens_hs = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations
          WHERE source_url IS NOT NULL
            AND source_url NOT LIKE 'http://%' AND source_url NOT LIKE 'https://%'",
    )
    .await;
    assert_eq!(liens_hs, 0, "lien de source hors http(s)");

    nettoyer(&path);
}

/// Le seed IDCC 0016 couvre les sept champs de la convention. Si une
/// activité se vide, ses filtres deviennent des culs-de-sac.
#[tokio::test]
async fn seed_idcc_0016_couvre_toutes_les_activites() {
    let (pool, path) = base_test().await;

    let activites: Vec<(String,)> =
        sqlx::query_as("SELECT code FROM ccn_activites WHERE idcc = '0016' ORDER BY ordre")
            .fetch_all(&pool)
            .await
            .expect("activités IDCC 0016");

    assert_eq!(activites.len(), 7, "les sept champs de la CCN 16 doivent être déclarés");

    for (code,) in &activites {
        let n = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM ccn_reglementations
              WHERE idcc = '0016' AND activite = ? AND publie = 1",
        )
        .bind(code)
        .fetch_one(&pool)
        .await
        .expect("comptage par activité")
        .0;
        assert!(n > 0, "l'activité '{code}' n'a aucune règle publiée");
    }

    nettoyer(&path);
}

/// Champs textuels obligatoires : ni vides, ni réduits à des espaces.
/// Une règle sans résumé s'afficherait comme un titre orphelin.
#[tokio::test]
async fn champs_obligatoires_non_vides() {
    let (pool, path) = base_test().await;

    let vides = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations
          WHERE TRIM(titre) = '' OR TRIM(resume) = ''
             OR TRIM(corps) = '' OR TRIM(source) = ''",
    )
    .await;
    assert_eq!(vides, 0, "titre, résumé, corps et source sont obligatoires");

    // Le corps est censé développer : s'il est plus court que le résumé,
    // c'est que la règle a été saisie de travers.
    let corps_indigents = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations WHERE LENGTH(corps) <= LENGTH(resume)",
    )
    .await;
    assert_eq!(corps_indigents, 0, "corps plus court que le résumé");

    nettoyer(&path);
}

/// Les barèmes sont du JSON dans une colonne texte : SQLite ne vérifie
/// ni sa validité, ni sa forme. Un tableau dont une ligne n'a pas autant
/// de cellules que d'en-têtes s'afficherait décalé — silencieusement.
#[tokio::test]
async fn tableaux_bien_formes() {
    let (pool, path) = base_test().await;

    let json_casse = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations
          WHERE tableaux IS NOT NULL AND json_valid(tableaux) = 0",
    )
    .await;
    assert_eq!(json_casse, 0, "barèmes en JSON invalide");

    // Chaque tableau doit porter des colonnes et des lignes non vides.
    let sans_structure = compter(
        &pool,
        "SELECT COUNT(*)
           FROM ccn_reglementations r, json_each(r.tableaux) t
          WHERE r.tableaux IS NOT NULL
            AND (json_extract(t.value, '$.colonnes') IS NULL
              OR json_extract(t.value, '$.lignes')   IS NULL
              OR json_array_length(json_extract(t.value, '$.colonnes')) = 0
              OR json_array_length(json_extract(t.value, '$.lignes'))   = 0)",
    )
    .await;
    assert_eq!(sans_structure, 0, "tableau sans colonnes ou sans lignes");

    // Le vrai piège : le décalage lignes / colonnes.
    let decalees = compter(
        &pool,
        "SELECT COUNT(*)
           FROM ccn_reglementations r,
                json_each(r.tableaux) t,
                json_each(json_extract(t.value, '$.lignes')) l
          WHERE r.tableaux IS NOT NULL
            AND json_array_length(l.value)
             <> json_array_length(json_extract(t.value, '$.colonnes'))",
    )
    .await;
    assert_eq!(decalees, 0, "lignes dont le nombre de cellules ne suit pas les en-têtes");

    // Garde-fou de contenu : si les grilles de minima disparaissaient
    // d'une migration future, la page redeviendrait un sommaire creux.
    let branches_avec_grille = compter(
        &pool,
        "SELECT COUNT(DISTINCT activite) FROM ccn_reglementations
          WHERE idcc = '0016' AND theme = 'minima' AND tableaux IS NOT NULL",
    )
    .await;
    assert!(
        branches_avec_grille >= 5,
        "seulement {branches_avec_grille} branches ont une grille de minima chiffrée"
    );

    nettoyer(&path);
}

/// Le contenu seedé n'est pas validé par un praticien : il doit se
/// signaler comme tel. Si ce test tombe un jour parce que tout est passé
/// en 'verifie', tant mieux — c'est qu'une relecture a eu lieu.
#[tokio::test]
async fn le_seed_ne_se_pretend_pas_verifie() {
    let (pool, path) = base_test().await;

    let total = compter(&pool, "SELECT COUNT(*) FROM ccn_reglementations WHERE idcc = '0016'").await;
    assert!(total >= 40, "seed IDCC 0016 anormalement maigre : {total} règles");

    let verifiees = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_reglementations
          WHERE idcc = '0016' AND statut_verif = 'verifie'",
    )
    .await;
    assert_eq!(
        verifiees, 0,
        "aucune règle du seed ne doit s'annoncer vérifiée avant relecture humaine"
    );

    nettoyer(&path);
}

// ── Grilles de salaires et maintien de salaire ───────────────────────────────
//
// Même logique que ci-dessus, sur les tables posées par la migration
// 0113 : ce sont des chiffres recopiés à la main dans un fichier SQL,
// c'est-à-dire l'endroit exact où une colonne se décale sans que rien
// ne proteste.

/// Structure des barèmes : JSON valide, colonnes et lignes cohérentes.
#[tokio::test]
async fn grilles_tableaux_bien_formes() {
    let (pool, path) = base_test().await;

    for table in ["ccn_grilles", "ccn_maintien"] {
        let json_casse = compter(
            &pool,
            &format!("SELECT COUNT(*) FROM {table} WHERE json_valid(tableaux) = 0"),
        )
        .await;
        assert_eq!(json_casse, 0, "{table} : barèmes en JSON invalide");

        let sans_structure = compter(
            &pool,
            &format!(
                "SELECT COUNT(*)
                   FROM {table} g, json_each(g.tableaux) t
                  WHERE json_extract(t.value, '$.colonnes') IS NULL
                     OR json_extract(t.value, '$.lignes')   IS NULL
                     OR json_array_length(json_extract(t.value, '$.colonnes')) = 0
                     OR json_array_length(json_extract(t.value, '$.lignes'))   = 0"
            ),
        )
        .await;
        assert_eq!(sans_structure, 0, "{table} : tableau sans colonnes ou sans lignes");

        // Le décalage d'une cellule sur une grille de salaires, c'est un
        // taux horaire lu dans la mauvaise colonne d'ancienneté.
        let decalees = compter(
            &pool,
            &format!(
                "SELECT COUNT(*)
                   FROM {table} g,
                        json_each(g.tableaux) t,
                        json_each(json_extract(t.value, '$.lignes')) l
                  WHERE json_array_length(l.value)
                     <> json_array_length(json_extract(t.value, '$.colonnes'))"
            ),
        )
        .await;
        assert_eq!(decalees, 0, "{table} : lignes qui ne suivent pas leurs en-têtes");
    }

    nettoyer(&path);
}

/// Une grille sans source ni date est une grille invérifiable : c'est
/// précisément ce que cette page refuse d'afficher.
#[tokio::test]
async fn grilles_toujours_datees_et_sourcees() {
    let (pool, path) = base_test().await;

    let muettes = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_grilles
          WHERE trim(source) = ''
             OR date_effet  NOT GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'
             OR consulte_le NOT GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'",
    )
    .await;
    assert_eq!(muettes, 0, "grille sans source ou sans date exploitable");

    let maintien_muet = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_maintien
          WHERE trim(source) = '' OR trim(article) = ''
             OR consulte_le NOT GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'",
    )
    .await;
    assert_eq!(maintien_muet, 0, "régime de maintien sans source, sans article ou sans date");

    // Un lien qui n'est pas http(s) ne sera pas rendu par le front :
    // autant le refuser ici plutôt que de le voir disparaître à l'écran.
    let liens_douteux = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_grilles
          WHERE source_url IS NOT NULL AND source_url NOT LIKE 'https://%'
         UNION ALL
         SELECT COUNT(*) FROM ccn_maintien
          WHERE source_url IS NOT NULL AND source_url NOT LIKE 'https://%'",
    )
    .await;
    assert_eq!(liens_douteux, 0, "lien de source qui n'est pas en https");

    nettoyer(&path);
}

/// Le menu déroulant ne doit proposer que des branches qui affichent
/// quelque chose, et les quatre catégories de la convention doivent
/// toutes avoir leur régime de maintien de salaire.
#[tokio::test]
async fn grilles_couvrent_le_choix_offert() {
    let (pool, path) = base_test().await;

    let branches_vides = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_branches b
          WHERE NOT EXISTS (SELECT 1 FROM ccn_grilles g
                             WHERE g.idcc = b.idcc AND g.branche = b.code)",
    )
    .await;
    assert_eq!(branches_vides, 0, "branche déclarée sans aucune grille");

    let grilles_orphelines = compter(
        &pool,
        "SELECT COUNT(*) FROM ccn_grilles g
          WHERE NOT EXISTS (SELECT 1 FROM ccn_branches b
                             WHERE b.idcc = g.idcc AND b.code = g.branche)",
    )
    .await;
    assert_eq!(grilles_orphelines, 0, "grille rattachée à une branche inexistante");

    let categories_maintien = compter(
        &pool,
        "SELECT COUNT(DISTINCT categorie) FROM ccn_maintien WHERE idcc = '0016'",
    )
    .await;
    assert_eq!(
        categories_maintien, 4,
        "les quatre catégories doivent avoir leur régime de maintien de salaire"
    );

    // Garde-fou de contenu : marchandises, voyageurs, déménagement et
    // logistique portent leurs quatre annexes.
    for branche in ["marchandises", "voyageurs", "demenagement", "logistique"] {
        let n = compter(
            &pool,
            &format!(
                "SELECT COUNT(*) FROM ccn_grilles
                  WHERE idcc = '0016' AND branche = '{branche}'"
            ),
        )
        .await;
        assert_eq!(n, 4, "branche {branche} : {n} grilles au lieu des 4 annexes");
    }

    nettoyer(&path);
}
