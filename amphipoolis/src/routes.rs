use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Extension, Json,
};
use chrono::{DateTime, Duration, SecondsFormat, Utc};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

pub type Db = Arc<SqlitePool>;

/// Vérificateur de la preuve de travail ALTCHA, fourni par l'hôte.
#[derive(Clone)]
pub struct Captcha(pub Arc<dyn Fn(&str) -> bool + Send + Sync>);

/// Adresse IP du client, posée dans les extensions de la requête par l'hôte
/// (qui sait lire `x-forwarded-for` derrière son proxy). Elle ne sert qu'au
/// plafond de créations de pseudonymes, tenu en mémoire et jamais écrit en base.
#[derive(Clone)]
pub struct IpClient(pub String);

// ── Bornes ────────────────────────────────────────────────────────────────────

const PSEUDO_MIN: usize = 3;
const PSEUDO_MAX: usize = 30;
const PHRASE_MIN: usize = 8;
const PHRASE_MAX: usize = 200;
const TITRE_MIN: usize = 3;
const TITRE_MAX: usize = 120;
const TEXTE_MAX: usize = 2000;
/// Au-delà, un pseudonyme ne peut plus rien déposer tant que la modération
/// n'a pas avancé : un seul auteur ne doit pas noyer la file d'attente.
const ATTENTE_MAX_PAR_AUTEUR: i64 = 20;
const SESSION_JOURS: i64 = 30;
/// Durée de vie d'un texte, et ce que lui ajoute chaque +1.
const VIE_JOURS: i64 = 7;
/// Plafond de vie d'un texte publié, compté depuis sa publication.
const VIE_MAX_JOURS: i64 = 30;
/// Un pseudonyme de chaque rôle par adresse et par période.
const CREATION_PERIODE_SECS: i64 = 24 * 3600;
/// Les plus récents seulement : le front relit le fil toutes les quelques secondes.
const FIL_MAX: i64 = 500;

/// Motifs de refus admis (codes ; libellés côté front).
const MOTIFS: &[&str] = &["hors_sujet", "discourtois", "donnees_personnelles", "illicite", "autre"];

// ── Erreurs ───────────────────────────────────────────────────────────────────

pub enum AmphError {
    Requete(String),
    NonConnecte,
    Interdit(String),
    Introuvable,
    Conflit(String),
    Interne,
}

impl From<sqlx::Error> for AmphError {
    fn from(e: sqlx::Error) -> Self {
        eprintln!("[amphipoolis] erreur base : {e}");
        AmphError::Interne
    }
}

impl IntoResponse for AmphError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            AmphError::Requete(m)  => (StatusCode::BAD_REQUEST, m),
            AmphError::NonConnecte => (StatusCode::UNAUTHORIZED, "Session absente ou expirée.".into()),
            AmphError::Interdit(m) => (StatusCode::FORBIDDEN, m),
            AmphError::Introuvable => (StatusCode::NOT_FOUND, "Introuvable.".into()),
            AmphError::Conflit(m)  => (StatusCode::CONFLICT, m),
            AmphError::Interne     => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne.".into()),
        };
        (code, Json(json!({ "erreur": msg }))).into_response()
    }
}

type Res<T> = Result<T, AmphError>;

// ── Utilitaires ───────────────────────────────────────────────────────────────

/// Horodatages à format FIXE (secondes, suffixe Z) : les dates sont comparées
/// comme des chaînes en SQL, une fraction de seconde de longueur variable
/// fausserait l'ordre.
fn horodatage(d: DateTime<Utc>) -> String {
    d.to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn maintenant() -> String {
    horodatage(Utc::now())
}

fn dans_jours(j: i64) -> String {
    horodatage(Utc::now() + Duration::days(j))
}

/// Plafond de créations : clé = empreinte de (IP, rôle), valeur = date de la
/// dernière création. En mémoire seulement — un redémarrage l'efface, et
/// l'IP elle-même n'est jamais conservée, pas même ici.
fn creations() -> &'static std::sync::Mutex<std::collections::HashMap<String, i64>> {
    use std::sync::{Mutex, OnceLock};
    static T: OnceLock<Mutex<std::collections::HashMap<String, i64>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

fn cle_creation(ip: &str, moderateur: bool) -> String {
    sha256_hex(&format!("amphipoolis:{ip}:{}", if moderateur { "modo" } else { "part" }))
}

fn creation_autorisee(cle: &str) -> bool {
    let now = Utc::now().timestamp();
    let mut t = creations().lock().unwrap_or_else(|e| e.into_inner());
    t.retain(|_, le| now - *le < CREATION_PERIODE_SECS);
    !t.contains_key(cle)
}

fn noter_creation(cle: String) {
    let mut t = creations().lock().unwrap_or_else(|e| e.into_inner());
    t.insert(cle, Utc::now().timestamp());
}

/// Texte brut : retire les caractères de contrôle (sauf saut de ligne pour
/// les messages), normalise les fins de ligne, coupe les blancs aux bords.
/// Le HTML n'est pas filtré ici — « salaire < SMIC » est un message légitime :
/// le front n'insère jamais ces textes autrement qu'échappés.
fn texte_propre(s: &str, multiligne: bool) -> String {
    s.replace("\r\n", "\n")
        .chars()
        .filter(|c| !c.is_control() || (multiligne && *c == '\n'))
        .collect::<String>()
        .trim()
        .to_string()
}

/// Pseudonyme : lettres, chiffres, espace et `- _ . '`, blancs internes
/// réduits à un seul. `None` si invalide.
fn pseudo_valide(brut: &str) -> Option<String> {
    let nom = brut.split_whitespace().collect::<Vec<_>>().join(" ");
    let n = nom.chars().count();
    let chars_ok = nom
        .chars()
        .all(|c| c.is_alphanumeric() || matches!(c, ' ' | '-' | '_' | '.' | '\''));
    (chars_ok && (PSEUDO_MIN..=PSEUDO_MAX).contains(&n)).then_some(nom)
}

fn hacher_phrase(phrase: &str) -> Res<String> {
    let sel = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(phrase.as_bytes(), &sel)
        .map(|h| h.to_string())
        .map_err(|_| AmphError::Interne)
}

fn phrase_correcte(phrase: &str, hash: &str) -> bool {
    PasswordHash::new(hash)
        .map(|p| Argon2::default().verify_password(phrase.as_bytes(), &p).is_ok())
        .unwrap_or(false)
}

fn sha256_hex(s: &str) -> String {
    hex::encode(Sha256::digest(s.as_bytes()))
}

async fn ouvrir_session(pool: &SqlitePool, pseudo_id: i64) -> Res<String> {
    let mut octets = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut octets);
    let jeton = hex::encode(octets);
    // Purge opportuniste des sessions expirées : la table reste bornée.
    sqlx::query("DELETE FROM amph_sessions WHERE expire_le < ?")
        .bind(maintenant())
        .execute(pool)
        .await?;
    sqlx::query("INSERT INTO amph_sessions (jeton_hash, pseudo_id, expire_le) VALUES (?, ?, ?)")
        .bind(sha256_hex(&jeton))
        .bind(pseudo_id)
        .bind(dans_jours(SESSION_JOURS))
        .execute(pool)
        .await?;
    Ok(jeton)
}

fn jeton_de(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[derive(sqlx::FromRow, Clone)]
pub struct Moi {
    id:         i64,
    nom:        String,
    moderateur: bool,
    etat:       String,
}

impl Moi {
    fn a_renommer(&self) -> bool {
        self.etat == "a_renommer"
    }
    /// Écrire ou modérer exige un pseudonyme en règle.
    fn exiger_actif(&self) -> Res<()> {
        if self.a_renommer() {
            return Err(AmphError::Interdit(
                "Votre pseudonyme a été refusé par la modération : choisissez-en un autre pour continuer.".into(),
            ));
        }
        Ok(())
    }
    fn exiger_moderateur(&self) -> Res<()> {
        self.exiger_actif()?;
        if !self.moderateur {
            return Err(AmphError::Interdit("La file de modération est réservée aux pseudonymes modérateurs.".into()));
        }
        Ok(())
    }
}

async fn session(pool: &SqlitePool, headers: &HeaderMap) -> Res<Moi> {
    let jeton = jeton_de(headers).ok_or(AmphError::NonConnecte)?;
    sqlx::query_as::<_, Moi>(
        "SELECT p.id, p.nom, p.moderateur, p.etat
           FROM amph_sessions s JOIN amph_pseudos p ON p.id = s.pseudo_id
          WHERE s.jeton_hash = ? AND s.expire_le > ?",
    )
    .bind(sha256_hex(&jeton))
    .bind(maintenant())
    .fetch_optional(pool)
    .await?
    .ok_or(AmphError::NonConnecte)
}

/// Nom affiché d'un auteur : un pseudonyme refusé n'apparaît plus nulle part
/// tant que son titulaire n'en a pas choisi un autre.
const NOM_AFFICHE: &str =
    "CASE WHEN p.etat = 'a_renommer' THEN NULL ELSE p.nom END";

#[derive(Serialize)]
struct MoiJson {
    nom:        Option<String>,
    moderateur: bool,
    a_renommer: bool,
}

fn moi_json(m: &Moi) -> MoiJson {
    MoiJson {
        // Le titulaire d'un pseudonyme refusé ne le voit plus non plus : il
        // sait seulement qu'il doit en choisir un autre.
        nom:        (!m.a_renommer()).then(|| m.nom.clone()),
        moderateur: m.moderateur,
        a_renommer: m.a_renommer(),
    }
}

async fn nom_disponible(pool: &SqlitePool, nom: &str, sauf_id: Option<i64>) -> Res<()> {
    let reserve: Option<(String,)> =
        sqlx::query_as("SELECT nom FROM amph_noms_reserves WHERE nom = ?")
            .bind(nom)
            .fetch_optional(pool)
            .await?;
    if reserve.is_some() {
        return Err(AmphError::Conflit("Ce pseudonyme n'est pas disponible.".into()));
    }
    let pris: Option<(i64,)> = sqlx::query_as("SELECT id FROM amph_pseudos WHERE nom = ?")
        .bind(nom)
        .fetch_optional(pool)
        .await?;
    match pris {
        Some((id,)) if Some(id) != sauf_id => {
            Err(AmphError::Conflit("Ce pseudonyme est déjà pris.".into()))
        }
        _ => Ok(()),
    }
}

async fn trop_en_attente(pool: &SqlitePool, auteur: i64) -> Res<()> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT (SELECT COUNT(*) FROM amph_messages WHERE auteur_id = ?1 AND statut = 'attente')
              + (SELECT COUNT(*) FROM amph_sujets   WHERE auteur_id = ?1 AND statut = 'attente')",
    )
    .bind(auteur)
    .fetch_one(pool)
    .await?;
    if n >= ATTENTE_MAX_PAR_AUTEUR {
        return Err(AmphError::Interdit(format!(
            "Vous avez déjà {n} textes en attente de modération : patientez avant d'en déposer d'autres."
        )));
    }
    Ok(())
}

fn exiger_texte(texte: &str) -> Res<String> {
    let t = texte_propre(texte, true);
    let n = t.chars().count();
    if n == 0 {
        return Err(AmphError::Requete("Le message est vide.".into()));
    }
    if n > TEXTE_MAX {
        return Err(AmphError::Requete(format!("{TEXTE_MAX} caractères au plus.")));
    }
    Ok(t)
}

// ── Entrer / sortir ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreerReq {
    pseudo:     String,
    phrase:     String,
    altcha:     Option<String>,
    /// Rôle, fixé une fois pour toutes à la création.
    #[serde(default)]
    moderateur: bool,
}

/// POST /api/amphipoolis/creer — nouveau pseudonyme (preuve de travail exigée).
/// Une adresse ne crée qu'un pseudonyme participant et un pseudonyme
/// modérateur par 24 h.
pub async fn creer(
    State(pool): State<Db>,
    Extension(captcha): Extension<Captcha>,
    ip: Option<Extension<IpClient>>,
    Json(req): Json<CreerReq>,
) -> Res<impl IntoResponse> {
    let cle = ip.map(|Extension(IpClient(ip))| cle_creation(&ip, req.moderateur));
    if cle.as_deref().is_some_and(|c| !creation_autorisee(c)) {
        return Err(AmphError::Interdit(format!(
            "Un seul pseudonyme {} par jour : revenez demain, ou entrez avec celui que vous avez déjà.",
            if req.moderateur { "modérateur" } else { "participant" }
        )));
    }
    if !req.altcha.as_deref().is_some_and(|p| (captcha.0)(p)) {
        return Err(AmphError::Requete("Vérification anti-robot échouée : réessayez.".into()));
    }
    let nom = pseudo_valide(&req.pseudo).ok_or_else(|| {
        AmphError::Requete(format!(
            "Pseudonyme : {PSEUDO_MIN} à {PSEUDO_MAX} caractères, lettres, chiffres, espace, - _ . '"
        ))
    })?;
    let n = req.phrase.chars().count();
    if !(PHRASE_MIN..=PHRASE_MAX).contains(&n) {
        return Err(AmphError::Requete(format!(
            "Phrase secrète : {PHRASE_MIN} à {PHRASE_MAX} caractères."
        )));
    }
    nom_disponible(&pool, &nom, None).await?;

    let phrase = req.phrase.clone();
    let hash = tokio::task::spawn_blocking(move || hacher_phrase(&phrase))
        .await
        .map_err(|_| AmphError::Interne)??;

    let id = sqlx::query("INSERT INTO amph_pseudos (nom, phrase_hash, moderateur, cree_le) VALUES (?, ?, ?, ?)")
        .bind(&nom)
        .bind(&hash)
        .bind(req.moderateur)
        .bind(maintenant())
        .execute(pool.as_ref())
        .await
        // Course entre deux créations du même nom : l'index UNIQUE tranche.
        .map_err(|e| match e {
            sqlx::Error::Database(d) if d.is_unique_violation() => {
                AmphError::Conflit("Ce pseudonyme est déjà pris.".into())
            }
            e => e.into(),
        })?
        .last_insert_rowid();

    if let Some(c) = cle {
        noter_creation(c);
    }
    let jeton = ouvrir_session(&pool, id).await?;
    let moi = Moi { id, nom, moderateur: req.moderateur, etat: "actif".into() };
    Ok((StatusCode::CREATED, Json(json!({ "jeton": jeton, "moi": moi_json(&moi) }))))
}

#[derive(Deserialize)]
pub struct EntrerReq {
    pseudo: String,
    phrase: String,
}

/// Empreinte de référence : vérifiée quand le pseudonyme n'existe pas, pour
/// que la durée de la réponse ne dise pas si un nom est pris.
fn empreinte_leurre() -> &'static str {
    use std::sync::OnceLock;
    static H: OnceLock<String> = OnceLock::new();
    H.get_or_init(|| hacher_phrase("amphipoolis-leurre").unwrap_or_default())
}

/// POST /api/amphipoolis/entrer — reprendre son pseudonyme.
pub async fn entrer(State(pool): State<Db>, Json(req): Json<EntrerReq>) -> Res<impl IntoResponse> {
    let nom = pseudo_valide(&req.pseudo).unwrap_or_default();
    let ligne: Option<(i64, String)> =
        sqlx::query_as("SELECT id, phrase_hash FROM amph_pseudos WHERE nom = ?")
            .bind(&nom)
            .fetch_optional(pool.as_ref())
            .await?;
    let (id, hash) = ligne.unwrap_or((0, empreinte_leurre().to_string()));
    let phrase = req.phrase.clone();
    let ok = tokio::task::spawn_blocking(move || phrase_correcte(&phrase, &hash))
        .await
        .map_err(|_| AmphError::Interne)?;
    if !ok || id == 0 {
        return Err(AmphError::Interdit("Pseudonyme ou phrase secrète incorrects.".into()));
    }
    let moi = sqlx::query_as::<_, Moi>("SELECT id, nom, moderateur, etat FROM amph_pseudos WHERE id = ?")
        .bind(id)
        .fetch_one(pool.as_ref())
        .await?;
    let jeton = ouvrir_session(&pool, id).await?;
    Ok(Json(json!({ "jeton": jeton, "moi": moi_json(&moi) })))
}

/// POST /api/amphipoolis/sortir — ferme la session courante.
pub async fn sortir(State(pool): State<Db>, headers: HeaderMap) -> Res<impl IntoResponse> {
    if let Some(j) = jeton_de(&headers) {
        sqlx::query("DELETE FROM amph_sessions WHERE jeton_hash = ?")
            .bind(sha256_hex(&j))
            .execute(pool.as_ref())
            .await?;
    }
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/amphipoolis/moi
pub async fn moi(State(pool): State<Db>, headers: HeaderMap) -> Res<impl IntoResponse> {
    let m = session(&pool, &headers).await?;
    let a_moderer = if m.moderateur && !m.a_renommer() {
        let (n,): (i64,) = sqlx::query_as(
            "SELECT (SELECT COUNT(*) FROM amph_messages WHERE statut = 'attente' AND auteur_id != ?1)
                  + (SELECT COUNT(*) FROM amph_sujets   WHERE statut = 'attente' AND auteur_id != ?1)",
        )
        .bind(m.id)
        .fetch_one(pool.as_ref())
        .await?;
        n
    } else {
        0
    };
    Ok(Json(json!({ "moi": moi_json(&m), "a_moderer": a_moderer })))
}

#[derive(Deserialize)]
pub struct RenommerReq {
    nouveau: String,
}

/// POST /api/amphipoolis/renommer — nouveau nom après un refus de pseudonyme.
/// Les textes déjà écrits suivent : ils sont rattachés au pseudonyme, pas au nom.
pub async fn renommer(
    State(pool): State<Db>,
    headers: HeaderMap,
    Json(req): Json<RenommerReq>,
) -> Res<impl IntoResponse> {
    let mut m = session(&pool, &headers).await?;
    if !m.a_renommer() {
        return Err(AmphError::Interdit("Votre pseudonyme n'a pas à être changé.".into()));
    }
    let nom = pseudo_valide(&req.nouveau).ok_or_else(|| {
        AmphError::Requete(format!(
            "Pseudonyme : {PSEUDO_MIN} à {PSEUDO_MAX} caractères, lettres, chiffres, espace, - _ . '"
        ))
    })?;
    nom_disponible(&pool, &nom, None).await?;
    sqlx::query("UPDATE amph_pseudos SET nom = ?, etat = 'actif' WHERE id = ?")
        .bind(&nom)
        .bind(m.id)
        .execute(pool.as_ref())
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(d) if d.is_unique_violation() => {
                AmphError::Conflit("Ce pseudonyme est déjà pris.".into())
            }
            e => e.into(),
        })?;
    m.nom = nom;
    m.etat = "actif".into();
    Ok(Json(json!({ "moi": moi_json(&m) })))
}

// ── Sujets et messages ────────────────────────────────────────────────────────

#[derive(sqlx::FromRow, Serialize)]
struct SujetResume {
    id:            i64,
    titre:         String,
    auteur:        Option<String>,
    statut:        String,
    motif:         Option<String>,
    cree_le:       String,
    nb_messages:   i64,
    derniere:      String,
    a_moi:         bool,
    plus1:         i64,
    expire_le:     Option<String>,
}

/// GET /api/amphipoolis/sujets — sujets publiés, plus les siens quel que soit
/// leur statut. Un motif de refus n'est renvoyé qu'à l'auteur.
pub async fn liste_sujets(State(pool): State<Db>, headers: HeaderMap) -> Res<impl IntoResponse> {
    let m = session(&pool, &headers).await?;
    let sql = format!(
        "SELECT s.id, s.titre, {NOM_AFFICHE} AS auteur, s.statut,
                CASE WHEN s.auteur_id = ?1 THEN s.motif END AS motif,
                s.cree_le,
                (SELECT COUNT(*) FROM amph_messages x WHERE x.sujet_id = s.id AND x.statut = 'publie') AS nb_messages,
                COALESCE((SELECT MAX(x.cree_le) FROM amph_messages x WHERE x.sujet_id = s.id AND x.statut = 'publie'),
                         s.cree_le) AS derniere,
                (s.auteur_id = ?1) AS a_moi,
                (SELECT COUNT(*) FROM amph_plus1 v WHERE v.objet = 'sujet' AND v.objet_id = s.id) AS plus1,
                s.expire_le
           FROM amph_sujets s JOIN amph_pseudos p ON p.id = s.auteur_id
          WHERE (s.statut = 'publie' OR s.auteur_id = ?1)
          ORDER BY derniere DESC
          LIMIT 300"
    );
    let sujets = sqlx::query_as::<_, SujetResume>(&sql)
        .bind(m.id)
        .fetch_all(pool.as_ref())
        .await?;
    Ok(Json(json!({ "sujets": sujets })))
}

#[derive(Deserialize)]
pub struct SujetReq {
    titre: String,
    texte: String,
}

/// POST /api/amphipoolis/sujets — ouvre un sujet (titre + message d'ouverture),
/// en attente de modération.
pub async fn creer_sujet(
    State(pool): State<Db>,
    headers: HeaderMap,
    Json(req): Json<SujetReq>,
) -> Res<impl IntoResponse> {
    let m = session(&pool, &headers).await?;
    m.exiger_actif()?;
    let titre = texte_propre(&req.titre, false);
    let n = titre.chars().count();
    if !(TITRE_MIN..=TITRE_MAX).contains(&n) {
        return Err(AmphError::Requete(format!("Titre : {TITRE_MIN} à {TITRE_MAX} caractères.")));
    }
    let texte = exiger_texte(&req.texte)?;
    trop_en_attente(&pool, m.id).await?;
    // En attente : 7 jours pour être modéré, sinon il disparaît.
    let id = sqlx::query("INSERT INTO amph_sujets (auteur_id, titre, texte, cree_le, expire_le) VALUES (?, ?, ?, ?, ?)")
        .bind(m.id)
        .bind(&titre)
        .bind(&texte)
        .bind(maintenant())
        .bind(dans_jours(VIE_JOURS))
        .execute(pool.as_ref())
        .await?
        .last_insert_rowid();
    Ok((StatusCode::CREATED, Json(json!({ "id": id }))))
}

#[derive(sqlx::FromRow, Serialize)]
struct SujetDetail {
    id:      i64,
    titre:   String,
    texte:   String,
    auteur:  Option<String>,
    statut:    String,
    motif:     Option<String>,
    cree_le:   String,
    a_moi:     bool,
    plus1:     i64,
    a_plus1:   bool,
    expire_le: Option<String>,
}

#[derive(sqlx::FromRow, Serialize)]
struct MessageJson {
    id:        i64,
    auteur:    Option<String>,
    texte:     String,
    statut:    String,
    motif:     Option<String>,
    cree_le:   String,
    a_moi:     bool,
    plus1:     i64,
    a_plus1:   bool,
    expire_le: Option<String>,
}

/// Colonnes +1 d'un texte (alias de table `t`, objet `o`, lecteur `?2`).
fn cols_plus1(t: &str, o: &str) -> String {
    format!(
        "(SELECT COUNT(*) FROM amph_plus1 v WHERE v.objet = '{o}' AND v.objet_id = {t}.id) AS plus1,
         EXISTS (SELECT 1 FROM amph_plus1 v WHERE v.objet = '{o}' AND v.objet_id = {t}.id AND v.pseudo_id = ?2) AS a_plus1,
         {t}.expire_le"
    )
}

/// GET /api/amphipoolis/sujets/{id} — le fil : messages publiés, plus les
/// siens en attente ou refusés.
pub async fn fil(
    State(pool): State<Db>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Res<impl IntoResponse> {
    let m = session(&pool, &headers).await?;
    let sql = format!(
        "SELECT s.id, s.titre, s.texte, {NOM_AFFICHE} AS auteur, s.statut,
                CASE WHEN s.auteur_id = ?2 THEN s.motif END AS motif,
                s.cree_le, (s.auteur_id = ?2) AS a_moi, {}
           FROM amph_sujets s JOIN amph_pseudos p ON p.id = s.auteur_id
          WHERE s.id = ?1 AND (s.statut = 'publie' OR s.auteur_id = ?2)",
        cols_plus1("s", "sujet")
    );
    let sujet = sqlx::query_as::<_, SujetDetail>(&sql)
        .bind(id)
        .bind(m.id)
        .fetch_optional(pool.as_ref())
        .await?
        .ok_or(AmphError::Introuvable)?;

    let sql = format!(
        "SELECT * FROM (
            SELECT x.id, {NOM_AFFICHE} AS auteur, x.texte, x.statut,
                   CASE WHEN x.auteur_id = ?2 THEN x.motif END AS motif,
                   x.cree_le, (x.auteur_id = ?2) AS a_moi, {}
              FROM amph_messages x JOIN amph_pseudos p ON p.id = x.auteur_id
             WHERE x.sujet_id = ?1 AND (x.statut = 'publie' OR x.auteur_id = ?2)
               AND (x.expire_le IS NULL OR x.expire_le > ?3)
             ORDER BY x.id DESC LIMIT {FIL_MAX}
         ) ORDER BY id ASC",
        cols_plus1("x", "message")
    );
    let messages = sqlx::query_as::<_, MessageJson>(&sql)
        .bind(id)
        .bind(m.id)
        .bind(maintenant())
        .fetch_all(pool.as_ref())
        .await?;
    Ok(Json(json!({ "sujet": sujet, "messages": messages })))
}

#[derive(Deserialize)]
pub struct MessageReq {
    texte: String,
}

/// POST /api/amphipoolis/sujets/{id}/messages — répond dans un sujet publié.
pub async fn ecrire(
    State(pool): State<Db>,
    headers: HeaderMap,
    Path(id): Path<i64>,
    Json(req): Json<MessageReq>,
) -> Res<impl IntoResponse> {
    let m = session(&pool, &headers).await?;
    m.exiger_actif()?;
    let texte = exiger_texte(&req.texte)?;
    let publie: Option<(i64,)> =
        sqlx::query_as("SELECT id FROM amph_sujets WHERE id = ? AND statut = 'publie'")
            .bind(id)
            .fetch_optional(pool.as_ref())
            .await?;
    if publie.is_none() {
        return Err(AmphError::Interdit("On ne répond qu'à un sujet publié.".into()));
    }
    trop_en_attente(&pool, m.id).await?;
    let mid = sqlx::query("INSERT INTO amph_messages (sujet_id, auteur_id, texte, cree_le, expire_le) VALUES (?, ?, ?, ?, ?)")
        .bind(id)
        .bind(m.id)
        .bind(&texte)
        .bind(maintenant())
        .bind(dans_jours(VIE_JOURS))
        .execute(pool.as_ref())
        .await?
        .last_insert_rowid();
    Ok((StatusCode::CREATED, Json(json!({ "id": mid }))))
}

// ── Modération ────────────────────────────────────────────────────────────────

#[derive(sqlx::FromRow, Serialize)]
struct AModerer {
    objet:       String,
    id:          i64,
    sujet_id:    i64,
    sujet_titre: String,
    titre:       Option<String>,
    texte:       String,
    auteur:      Option<String>,
    cree_le:     String,
}

/// GET /api/amphipoolis/moderation — tout ce qui attend, du plus ancien au plus
/// récent, sauf ce que le modérateur a lui-même écrit.
pub async fn file_moderation(State(pool): State<Db>, headers: HeaderMap) -> Res<impl IntoResponse> {
    let m = session(&pool, &headers).await?;
    m.exiger_moderateur()?;
    let sql = format!(
        "SELECT * FROM (
         SELECT 'sujet' AS objet, s.id AS id, s.id AS sujet_id, s.titre AS sujet_titre, s.titre AS titre,
                s.texte AS texte, {NOM_AFFICHE} AS auteur, s.cree_le AS cree_le
           FROM amph_sujets s JOIN amph_pseudos p ON p.id = s.auteur_id
          WHERE s.statut = 'attente' AND s.auteur_id != ?1
         UNION ALL
         SELECT 'message', x.id, x.sujet_id, s.titre, NULL,
                x.texte, {NOM_AFFICHE}, x.cree_le
           FROM amph_messages x
           JOIN amph_sujets s  ON s.id = x.sujet_id
           JOIN amph_pseudos p ON p.id = x.auteur_id
          WHERE x.statut = 'attente' AND x.auteur_id != ?1
         ) ORDER BY cree_le ASC
         LIMIT 200"
    );
    let file = sqlx::query_as::<_, AModerer>(&sql)
        .bind(m.id)
        .fetch_all(pool.as_ref())
        .await?;
    Ok(Json(json!({ "file": file })))
}

#[derive(Deserialize)]
pub struct DecisionReq {
    objet:    String,
    id:       i64,
    decision: String,
    motif:    Option<String>,
}

fn table_de(objet: &str) -> Res<&'static str> {
    match objet {
        "sujet"   => Ok("amph_sujets"),
        "message" => Ok("amph_messages"),
        _ => Err(AmphError::Requete("Objet inconnu.".into())),
    }
}

/// POST /api/amphipoolis/moderation/decision — publier ou refuser.
/// Le premier modérateur qui tranche l'emporte ; on ne tranche jamais son propre texte.
pub async fn decider(
    State(pool): State<Db>,
    headers: HeaderMap,
    Json(req): Json<DecisionReq>,
) -> Res<impl IntoResponse> {
    let m = session(&pool, &headers).await?;
    m.exiger_moderateur()?;
    let table = table_de(&req.objet)?;
    let (statut, motif) = match req.decision.as_str() {
        "publier" => ("publie", None),
        "refuser" => {
            let motif = req.motif.as_deref().filter(|x| MOTIFS.contains(x)).ok_or_else(|| {
                AmphError::Requete("Un refus exige un motif.".into())
            })?;
            ("refuse", Some(motif.to_string()))
        }
        _ => return Err(AmphError::Requete("Décision inconnue.".into())),
    };

    let auteur: Option<(i64, String)> =
        sqlx::query_as(&format!("SELECT auteur_id, statut FROM {table} WHERE id = ?"))
            .bind(req.id)
            .fetch_optional(pool.as_ref())
            .await?;
    let (auteur_id, _) = auteur.ok_or(AmphError::Introuvable)?;
    if auteur_id == m.id {
        return Err(AmphError::Interdit("On ne modère pas ce qu'on a soi-même écrit.".into()));
    }

    let le = maintenant();
    let mut tx = pool.begin().await?;
    // `statut = 'attente'` dans la clause : si un autre modérateur vient de
    // trancher, rien n'est modifié et on le dit.
    // Publié : 7 jours de vie à compter de maintenant (les +1 les rallongent).
    // Refusé : l'auteur a 7 jours pour lire le motif, puis le texte disparaît.
    let fait = sqlx::query(&format!(
        "UPDATE {table} SET statut = ?, motif = ?, decide_le = ?, expire_le = ?, decide_par = ?
          WHERE id = ? AND statut = 'attente' AND auteur_id != ?"
    ))
    .bind(statut)
    .bind(&motif)
    .bind(&le)
    .bind(dans_jours(VIE_JOURS))
    .bind(m.id)
    .bind(req.id)
    .bind(m.id)
    .execute(&mut *tx)
    .await?
    .rows_affected();
    if fait == 0 {
        return Err(AmphError::Conflit("Déjà tranché par un autre modérateur.".into()));
    }
    sqlx::query(
        "INSERT INTO amph_journal (objet, objet_id, decision, motif, moderateur_id, le) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&req.objet)
    .bind(req.id)
    .bind(&req.decision)
    .bind(&motif)
    .bind(m.id)
    .bind(&le)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(json!({ "ok": true })))
}

#[derive(Deserialize)]
pub struct Plus1Req {
    objet: String,
    id:    i64,
}

/// POST /api/amphipoolis/plus1 — un +1 sur le texte publié de quelqu'un
/// d'autre, une fois par texte. Chaque +1 rallonge sa vie de 7 jours, sans
/// dépasser 30 jours après sa publication.
pub async fn plus1(
    State(pool): State<Db>,
    headers: HeaderMap,
    Json(req): Json<Plus1Req>,
) -> Res<impl IntoResponse> {
    let m = session(&pool, &headers).await?;
    m.exiger_actif()?;
    let table = table_de(&req.objet)?;
    let cible: Option<(i64, String, Option<String>, Option<String>)> = sqlx::query_as(&format!(
        "SELECT auteur_id, statut, decide_le, expire_le FROM {table} WHERE id = ?"
    ))
    .bind(req.id)
    .fetch_optional(pool.as_ref())
    .await?;
    let (auteur, statut, publie_le, expire_le) = cible.ok_or(AmphError::Introuvable)?;
    if statut != "publie" {
        return Err(AmphError::Interdit("On ne donne un +1 qu'à un texte publié.".into()));
    }
    if auteur == m.id {
        return Err(AmphError::Interdit("Pas de +1 à soi-même.".into()));
    }
    let publie_le = publie_le
        .as_deref()
        .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
        .map(|d| d.with_timezone(&Utc))
        .ok_or(AmphError::Interne)?;
    let plafond = publie_le + Duration::days(VIE_MAX_JOURS);
    let actuelle = expire_le
        .as_deref()
        .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(Utc::now);
    let nouvelle = horodatage((actuelle + Duration::days(VIE_JOURS)).min(plafond));

    let mut tx = pool.begin().await?;
    let ajoute = sqlx::query("INSERT OR IGNORE INTO amph_plus1 (objet, objet_id, pseudo_id, le) VALUES (?, ?, ?, ?)")
        .bind(&req.objet)
        .bind(req.id)
        .bind(m.id)
        .bind(maintenant())
        .execute(&mut *tx)
        .await?
        .rows_affected();
    if ajoute == 0 {
        return Err(AmphError::Conflit("Vous avez déjà donné un +1 à ce texte.".into()));
    }
    sqlx::query(&format!("UPDATE {table} SET expire_le = ? WHERE id = ?"))
        .bind(&nouvelle)
        .bind(req.id)
        .execute(&mut *tx)
        .await?;
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM amph_plus1 WHERE objet = ? AND objet_id = ?")
        .bind(&req.objet)
        .bind(req.id)
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(Json(json!({ "plus1": n, "expire_le": nouvelle })))
}

#[derive(Deserialize)]
pub struct PseudoReq {
    objet: String,
    id:    i64,
}

/// POST /api/amphipoolis/moderation/pseudo — refuse le pseudonyme de l'auteur
/// d'un texte en attente. Le modérateur ne manipule jamais d'identifiant de
/// pseudonyme : il désigne un texte, la base retrouve son auteur.
pub async fn refuser_pseudo(
    State(pool): State<Db>,
    headers: HeaderMap,
    Json(req): Json<PseudoReq>,
) -> Res<impl IntoResponse> {
    let m = session(&pool, &headers).await?;
    m.exiger_moderateur()?;
    let table = table_de(&req.objet)?;
    let cible: Option<(i64, String, String)> = sqlx::query_as(&format!(
        "SELECT p.id, p.nom, p.etat FROM {table} t JOIN amph_pseudos p ON p.id = t.auteur_id WHERE t.id = ?"
    ))
    .bind(req.id)
    .fetch_optional(pool.as_ref())
    .await?;
    let (pid, nom, etat) = cible.ok_or(AmphError::Introuvable)?;
    if pid == m.id {
        return Err(AmphError::Interdit("On ne modère pas son propre pseudonyme.".into()));
    }
    if etat == "a_renommer" {
        return Err(AmphError::Conflit("Ce pseudonyme a déjà été refusé.".into()));
    }
    let le = maintenant();
    let mut tx = pool.begin().await?;
    let fait = sqlx::query("UPDATE amph_pseudos SET etat = 'a_renommer' WHERE id = ? AND etat = 'actif'")
        .bind(pid)
        .execute(&mut *tx)
        .await?
        .rows_affected();
    if fait == 0 {
        return Err(AmphError::Conflit("Ce pseudonyme a déjà été refusé.".into()));
    }
    sqlx::query("INSERT OR IGNORE INTO amph_noms_reserves (nom, le) VALUES (?, ?)")
        .bind(&nom)
        .bind(&le)
        .execute(&mut *tx)
        .await?;
    sqlx::query(
        "INSERT INTO amph_journal (objet, objet_id, decision, ancien_nom, moderateur_id, le)
         VALUES ('pseudo', ?, 'renommer', ?, ?, ?)",
    )
    .bind(pid)
    .bind(&nom)
    .bind(m.id)
    .bind(&le)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(json!({ "ok": true })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pseudos() {
        assert_eq!(pseudo_valide("  Xena   la  Guerrière ").as_deref(), Some("Xena la Guerrière"));
        assert_eq!(pseudo_valide("ab"), None);
        assert_eq!(pseudo_valide("<script>"), None);
        assert_eq!(pseudo_valide("O'Brien-2.0_x").as_deref(), Some("O'Brien-2.0_x"));
        assert_eq!(pseudo_valide(&"a".repeat(31)), None);
    }

    #[test]
    fn textes() {
        assert_eq!(texte_propre(" a\r\nb\u{0007} ", true), "a\nb");
        assert_eq!(texte_propre("a\nb", false), "ab");
    }
}
