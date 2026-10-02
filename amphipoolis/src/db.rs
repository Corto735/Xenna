//! Schéma d'Amphipoolis, créé au démarrage dans la base partagée de Xenna.
//!
//! Ce que la base sait d'un participant : un pseudonyme, l'empreinte Argon2id
//! de sa phrase secrète, et ce qu'il a écrit. Rien d'autre — ni e-mail, ni
//! adresse IP, ni empreinte de navigateur. Les jetons de session ne sont
//! stockés que hachés (SHA-256) : une fuite de la table ne permet pas de
//! parler sous le nom de quelqu'un.
//!
//! Rien n'est éternel : chaque texte porte une date d'expiration (`expire_le`).
//! Publié, il vit 7 jours, rallongés de 7 jours par +1, sans dépasser 30 jours
//! après sa publication ; refusé ou jamais modéré, il disparaît 7 jours après
//! sa dernière étape. `purger` efface ce qui a expiré, journal compris.

use sqlx::SqlitePool;

const SCHEMA: &[&str] = &[
    // Pseudonymes. `nom` est unique sans tenir compte de la casse : « Xena »
    // et « xena » ne peuvent pas coexister (usurpation par homoglyphe de casse).
    // etat : 'actif' | 'a_renommer' (pseudonyme refusé par la modération).
    r#"CREATE TABLE IF NOT EXISTS amph_pseudos (
        id           INTEGER PRIMARY KEY AUTOINCREMENT,
        nom          TEXT NOT NULL UNIQUE COLLATE NOCASE,
        phrase_hash  TEXT NOT NULL,
        moderateur   INTEGER NOT NULL DEFAULT 0,   -- rôle fixé à la création
        etat         TEXT NOT NULL DEFAULT 'actif',
        cree_le      TEXT NOT NULL
    )"#,
    // Noms refusés par la modération : plus personne ne peut les prendre.
    r#"CREATE TABLE IF NOT EXISTS amph_noms_reserves (
        nom      TEXT PRIMARY KEY COLLATE NOCASE,
        le       TEXT NOT NULL
    )"#,
    r#"CREATE TABLE IF NOT EXISTS amph_sessions (
        jeton_hash  TEXT PRIMARY KEY,
        pseudo_id   INTEGER NOT NULL REFERENCES amph_pseudos(id),
        expire_le   TEXT NOT NULL
    )"#,
    // Sujet = titre + message d'ouverture, modérés ensemble.
    // statut : 'attente' | 'publie' | 'refuse'.
    r#"CREATE TABLE IF NOT EXISTS amph_sujets (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        auteur_id   INTEGER NOT NULL REFERENCES amph_pseudos(id),
        titre       TEXT NOT NULL,
        texte       TEXT NOT NULL,
        statut      TEXT NOT NULL DEFAULT 'attente',
        motif       TEXT,
        cree_le     TEXT NOT NULL,
        decide_le   TEXT,
        decide_par  INTEGER REFERENCES amph_pseudos(id)
    )"#,
    r#"CREATE TABLE IF NOT EXISTS amph_messages (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        sujet_id    INTEGER NOT NULL REFERENCES amph_sujets(id),
        auteur_id   INTEGER NOT NULL REFERENCES amph_pseudos(id),
        texte       TEXT NOT NULL,
        statut      TEXT NOT NULL DEFAULT 'attente',
        motif       TEXT,
        cree_le     TEXT NOT NULL,
        decide_le   TEXT,
        decide_par  INTEGER REFERENCES amph_pseudos(id)
    )"#,
    // +1 : un par pseudonyme et par texte, jamais sur le sien (contrôlé à l'écriture).
    r#"CREATE TABLE IF NOT EXISTS amph_plus1 (
        objet      TEXT NOT NULL,
        objet_id   INTEGER NOT NULL,
        pseudo_id  INTEGER NOT NULL REFERENCES amph_pseudos(id),
        le         TEXT NOT NULL,
        PRIMARY KEY (objet, objet_id, pseudo_id)
    )"#,
    "CREATE INDEX IF NOT EXISTS amph_messages_sujet ON amph_messages(sujet_id, id)",
    "CREATE INDEX IF NOT EXISTS amph_messages_statut ON amph_messages(statut)",
    "CREATE INDEX IF NOT EXISTS amph_sujets_statut ON amph_sujets(statut)",
    // Journal de modération : qui (pseudonyme modérateur) a décidé quoi.
    // objet : 'sujet' | 'message' | 'pseudo' ; decision : 'publier' |
    // 'refuser' | 'renommer'. ancien_nom : pour 'renommer', le nom refusé.
    r#"CREATE TABLE IF NOT EXISTS amph_journal (
        id             INTEGER PRIMARY KEY AUTOINCREMENT,
        objet          TEXT NOT NULL,
        objet_id       INTEGER NOT NULL,
        decision       TEXT NOT NULL,
        motif          TEXT,
        ancien_nom     TEXT,
        moderateur_id  INTEGER NOT NULL REFERENCES amph_pseudos(id),
        le             TEXT NOT NULL
    )"#,
];

/// Colonnes ajoutées après la première version du schéma : `CREATE TABLE IF
/// NOT EXISTS` ne les crée pas sur une base existante.
const COLONNES: &[(&str, &str, &str)] = &[
    ("amph_sujets",   "expire_le", "TEXT"),
    ("amph_messages", "expire_le", "TEXT"),
];

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    for sql in SCHEMA {
        sqlx::query(sql).execute(pool).await?;
    }
    for (table, col, typ) in COLONNES {
        let existe: Option<(String,)> =
            sqlx::query_as(&format!("SELECT name FROM pragma_table_info('{table}') WHERE name = ?"))
                .bind(col)
                .fetch_optional(pool)
                .await?;
        if existe.is_none() {
            sqlx::query(&format!("ALTER TABLE {table} ADD COLUMN {col} {typ}"))
                .execute(pool)
                .await?;
        }
    }
    sqlx::query("CREATE INDEX IF NOT EXISTS amph_messages_expire ON amph_messages(expire_le)")
        .execute(pool)
        .await?;
    Ok(())
}

/// Efface ce qui a expiré : messages, puis sujets expirés qui n'ont plus aucune
/// réponse en vie, puis les +1 et le journal devenus orphelins. Le journal des
/// refus de pseudonyme vit 30 jours. Les noms réservés, eux, restent.
pub async fn purger(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    let mut tx = pool.begin().await?;
    let mut n = sqlx::query("DELETE FROM amph_messages WHERE expire_le IS NOT NULL AND expire_le < ?")
        .bind(&now)
        .execute(&mut *tx)
        .await?
        .rows_affected();
    n += sqlx::query(
        "DELETE FROM amph_sujets
          WHERE expire_le IS NOT NULL AND expire_le < ?
            AND NOT EXISTS (SELECT 1 FROM amph_messages m WHERE m.sujet_id = amph_sujets.id)",
    )
    .bind(&now)
    .execute(&mut *tx)
    .await?
    .rows_affected();
    sqlx::query(
        "DELETE FROM amph_plus1
          WHERE (objet = 'sujet'   AND objet_id NOT IN (SELECT id FROM amph_sujets))
             OR (objet = 'message' AND objet_id NOT IN (SELECT id FROM amph_messages))",
    )
    .execute(&mut *tx)
    .await?;
    let limite_journal = (chrono::Utc::now() - chrono::Duration::days(30)).to_rfc3339();
    sqlx::query(
        "DELETE FROM amph_journal
          WHERE (objet = 'sujet'   AND objet_id NOT IN (SELECT id FROM amph_sujets))
             OR (objet = 'message' AND objet_id NOT IN (SELECT id FROM amph_messages))
             OR (objet = 'pseudo'  AND le < ?)",
    )
    .bind(&limite_journal)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(n)
}
