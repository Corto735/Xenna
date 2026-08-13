//! Consultation publique des conventions collectives.
//!
//! Lecture seule. L'écriture passe par l'espace admin, qui a déjà
//! son authentification — inutile d'en réinventer une ici.

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use super::models::{
    Activite, Branche, Convention, ConventionResume, DossierCcn, DossierGrilles, Grille, Maintien,
    Reglementation, ReglementationAdmin, ReglementationInput, Theme,
};
use crate::admin::auth::AdminAuth;

type Db = Arc<SqlitePool>;

// ── Error ─────────────────────────────────────────────────────────────────────

pub enum CcnError {
    Introuvable(String),
    Validation(String),
    Db(sqlx::Error),
}

impl IntoResponse for CcnError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CcnError::Introuvable(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            CcnError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response(),
            CcnError::Db(e) => {
                tracing::error!("CCN DB error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne du serveur").into_response()
            }
        }
    }
}

impl From<sqlx::Error> for CcnError {
    fn from(e: sqlx::Error) -> Self {
        CcnError::Db(e)
    }
}

/// Tout ce qui n'est pas un IDCC est refusé avant d'atteindre la base.
fn valider_idcc(idcc: &str) -> Result<(), CcnError> {
    if !idcc_valide(idcc) {
        return Err(CcnError::Validation(format!("Code IDCC invalide : '{idcc}'")));
    }
    Ok(())
}

// ── Lecture ───────────────────────────────────────────────────────────────────
//
// Ces deux fonctions sont le cœur métier de la consultation ; elles ne
// dépendent d'aucun extracteur Axum pour rester appelables depuis les
// commandes Tauri du bureau. Le serveur web les habille de handlers,
// `commands::ccn` les appelle directement.

/// Liste des conventions publiées, avec le nombre de règles de chacune.
pub async fn charger_conventions(pool: &SqlitePool) -> Result<Vec<ConventionResume>, sqlx::Error> {
    sqlx::query_as::<_, ConventionResume>(
        "SELECT c.idcc,
                c.libelle,
                c.libelle_court,
                (SELECT COUNT(*)
                   FROM ccn_reglementations r
                  WHERE r.idcc = c.idcc AND r.publie = 1) AS nb_regles
           FROM ccn_conventions c
          WHERE c.publie = 1
          ORDER BY c.idcc",
    )
    .fetch_all(pool)
    .await
}

/// Dossier complet d'une convention. `Ok(None)` = convention inconnue,
/// à distinguer d'une panne de base.
pub async fn charger_dossier(
    pool: &SqlitePool,
    idcc: &str,
) -> Result<Option<DossierCcn>, sqlx::Error> {
    let convention = sqlx::query_as::<_, Convention>(
        "SELECT idcc, libelle, libelle_court, champ,
                brochure_jo, legifrance_id, date_signature
           FROM ccn_conventions
          WHERE idcc = ? AND publie = 1",
    )
    .bind(idcc)
    .fetch_optional(pool)
    .await?;

    let Some(convention) = convention else {
        return Ok(None);
    };

    let activites = sqlx::query_as::<_, Activite>(
        "SELECT code, libelle, detail, ordre
           FROM ccn_activites
          WHERE idcc = ?
          ORDER BY ordre, libelle",
    )
    .bind(idcc)
    .fetch_all(pool)
    .await?;

    // Les thèmes sont communs à toutes les conventions ; on ne remonte
    // que ceux effectivement utilisés par celle-ci, pour éviter des
    // filtres qui ne ramèneraient rien.
    let themes = sqlx::query_as::<_, Theme>(
        "SELECT t.code, t.libelle, t.icone, t.ordre
           FROM ccn_themes t
          WHERE EXISTS (SELECT 1
                          FROM ccn_reglementations r
                         WHERE r.theme = t.code AND r.idcc = ? AND r.publie = 1)
          ORDER BY t.ordre, t.libelle",
    )
    .bind(idcc)
    .fetch_all(pool)
    .await?;

    let reglementations = sqlx::query_as::<_, Reglementation>(
        "SELECT id, activite, theme, titre, resume, corps, valeur,
                source, source_url, date_effet, impact, regime_social,
                statut_verif, tableaux, ordre
           FROM ccn_reglementations
          WHERE idcc = ? AND publie = 1
          ORDER BY ordre, id",
    )
    .bind(idcc)
    .fetch_all(pool)
    .await?;

    Ok(Some(DossierCcn {
        convention,
        activites,
        themes,
        reglementations,
    }))
}

/// Grilles de minima et régimes de maintien de salaire d'une convention.
///
/// `Ok(None)` = convention inconnue. Une convention publiée sans aucune
/// grille reste un succès : le front sait afficher « rien ici », il ne
/// sait pas quoi faire d'une 404 qui n'en est pas une.
pub async fn charger_grilles(
    pool: &SqlitePool,
    idcc: &str,
) -> Result<Option<DossierGrilles>, sqlx::Error> {
    let convention = sqlx::query_as::<_, Convention>(
        "SELECT idcc, libelle, libelle_court, champ,
                brochure_jo, legifrance_id, date_signature
           FROM ccn_conventions
          WHERE idcc = ? AND publie = 1",
    )
    .bind(idcc)
    .fetch_optional(pool)
    .await?;

    let Some(convention) = convention else {
        return Ok(None);
    };

    // Seules les branches qui portent au moins une grille sont
    // remontées : proposer un choix qui n'affiche rien est une
    // promesse non tenue.
    let branches = sqlx::query_as::<_, Branche>(
        "SELECT b.code, b.libelle, b.detail, b.ordre
           FROM ccn_branches b
          WHERE b.idcc = ?
            AND EXISTS (SELECT 1 FROM ccn_grilles g
                         WHERE g.idcc = b.idcc AND g.branche = b.code)
          ORDER BY b.ordre, b.libelle",
    )
    .bind(idcc)
    .fetch_all(pool)
    .await?;

    let grilles = sqlx::query_as::<_, Grille>(
        "SELECT id, branche, categorie, intitule, corps, tableaux,
                source, source_url, extension, date_effet, consulte_le, ordre
           FROM ccn_grilles
          WHERE idcc = ?
          ORDER BY ordre, id",
    )
    .bind(idcc)
    .fetch_all(pool)
    .await?;

    let maintien = sqlx::query_as::<_, Maintien>(
        "SELECT id, categorie, intitule, article, corps, tableaux,
                source, source_url, consulte_le, ordre
           FROM ccn_maintien
          WHERE idcc = ?
          ORDER BY ordre, id",
    )
    .bind(idcc)
    .fetch_all(pool)
    .await?;

    Ok(Some(DossierGrilles {
        convention,
        branches,
        grilles,
        maintien,
    }))
}

/// Un IDCC est un code court numérique. Partagé par le web et le bureau.
pub fn idcc_valide(idcc: &str) -> bool {
    !idcc.is_empty() && idcc.len() <= 6 && idcc.chars().all(|c| c.is_ascii_digit())
}

// ── Handlers HTTP ─────────────────────────────────────────────────────────────

/// Liste des conventions publiées, avec le nombre de règles de chacune.
async fn lister_conventions(
    State(pool): State<Db>,
) -> Result<Json<Vec<ConventionResume>>, CcnError> {
    Ok(Json(charger_conventions(&pool).await?))
}

/// Dossier complet d'une convention : en-tête, activités, thèmes, règles.
///
/// L'IDCC arrive soit du chemin (`GET /api/ccn/0016`), soit du corps JSON
/// (`POST /api/dossier_ccn`) : le front unifié passe par la seconde forme,
/// celle que `api()` sait router aussi bien vers Tauri que vers HTTP.
async fn dossier(
    State(pool): State<Db>,
    Path(idcc): Path<String>,
) -> Result<Json<DossierCcn>, CcnError> {
    dossier_par_idcc(&pool, &idcc).await.map(Json)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DossierReq {
    idcc: Option<String>,
}

async fn dossier_post(
    State(pool): State<Db>,
    Json(req): Json<DossierReq>,
) -> Result<Json<DossierCcn>, CcnError> {
    let idcc = req.idcc.unwrap_or_else(|| "0016".to_string());
    dossier_par_idcc(&pool, &idcc).await.map(Json)
}

async fn conventions_post(
    State(pool): State<Db>,
) -> Result<Json<Vec<ConventionResume>>, CcnError> {
    Ok(Json(charger_conventions(&pool).await?))
}

/// Grilles de salaires et maintien de salaire d'une convention.
async fn grilles(
    State(pool): State<Db>,
    Path(idcc): Path<String>,
) -> Result<Json<DossierGrilles>, CcnError> {
    grilles_par_idcc(&pool, &idcc).await.map(Json)
}

async fn grilles_post(
    State(pool): State<Db>,
    Json(req): Json<DossierReq>,
) -> Result<Json<DossierGrilles>, CcnError> {
    let idcc = req.idcc.unwrap_or_else(|| "0016".to_string());
    grilles_par_idcc(&pool, &idcc).await.map(Json)
}

async fn grilles_par_idcc(pool: &SqlitePool, idcc: &str) -> Result<DossierGrilles, CcnError> {
    valider_idcc(idcc)?;
    charger_grilles(pool, idcc)
        .await?
        .ok_or_else(|| CcnError::Introuvable(format!("Convention IDCC {idcc} introuvable")))
}

async fn dossier_par_idcc(pool: &SqlitePool, idcc: &str) -> Result<DossierCcn, CcnError> {
    valider_idcc(idcc)?;
    charger_dossier(pool, idcc)
        .await?
        .ok_or_else(|| CcnError::Introuvable(format!("Convention IDCC {idcc} introuvable")))
}

// ── Administration ────────────────────────────────────────────────────────────
// Écriture réservée à l'admin authentifié. Le contenu est éditorial :
// il vieillit, il se corrige, et il doit pouvoir être corrigé sans
// redéploiement — c'est toute la raison d'être de ces routes.

const IMPACTS: &[&str] = &[
    "Brut", "Cotisations", "Net imposable",
    "Cout employeur", "Temps de travail", "Hors bulletin",
];
const STATUTS: &[&str] = &["brouillon", "a_verifier", "verifie"];

/// Texte brut seulement : on retire les chevrons et les caractères de
/// contrôle. Les sauts de ligne sont conservés, le corps des règles en
/// vit. Même politique que le module quizz.
fn sanitiser(s: &str) -> String {
    s.chars()
        .filter(|c| match c {
            '<' | '>' => false,
            c => !c.is_control() || matches!(c, '\n' | '\r' | '\t' | ' '),
        })
        .collect::<String>()
        .trim()
        .to_string()
}

fn sanitiser_opt(s: Option<&str>) -> Option<String> {
    s.map(sanitiser).filter(|s| !s.is_empty())
}

/// Champs nettoyés et contrôlés, prêts à être liés à une requête.
struct ReglementationValide {
    idcc:          String,
    activite:      String,
    theme:         String,
    titre:         String,
    resume:        String,
    corps:         String,
    valeur:        Option<String>,
    source:        String,
    source_url:    Option<String>,
    date_effet:    Option<String>,
    impact:        String,
    regime_social: Option<String>,
    statut_verif:  String,
    tableaux:      Option<String>,
    publie:        i64,
    ordre:         i64,
}

/// Contrôle la forme des barèmes avant écriture.
///
/// Le front rend ces tableaux sans filet : une ligne qui n'a pas autant
/// de cellules que d'en-têtes produirait un tableau décalé, et un JSON
/// malformé ferait disparaître la règle entière de l'affichage. Mieux
/// vaut refuser à l'entrée que déboguer à l'écran.
fn valider_tableaux(brut: &str) -> Result<String, CcnError> {
    let v: serde_json::Value = serde_json::from_str(brut)
        .map_err(|e| CcnError::Validation(format!("Barèmes : JSON invalide ({e})")))?;

    let tableaux = v
        .as_array()
        .ok_or_else(|| CcnError::Validation("Barèmes : une liste de tableaux est attendue".into()))?;

    for (i, t) in tableaux.iter().enumerate() {
        let rang = i + 1;
        let colonnes = t
            .get("colonnes")
            .and_then(|c| c.as_array())
            .ok_or_else(|| CcnError::Validation(format!("Tableau {rang} : 'colonnes' manquant")))?;
        if colonnes.is_empty() {
            return Err(CcnError::Validation(format!("Tableau {rang} : aucune colonne")));
        }

        let lignes = t
            .get("lignes")
            .and_then(|l| l.as_array())
            .ok_or_else(|| CcnError::Validation(format!("Tableau {rang} : 'lignes' manquant")))?;

        for (j, ligne) in lignes.iter().enumerate() {
            let cells = ligne.as_array().ok_or_else(|| {
                CcnError::Validation(format!("Tableau {rang}, ligne {} : liste attendue", j + 1))
            })?;
            if cells.len() != colonnes.len() {
                return Err(CcnError::Validation(format!(
                    "Tableau {rang}, ligne {} : {} cellules pour {} colonnes",
                    j + 1,
                    cells.len(),
                    colonnes.len()
                )));
            }
        }
    }

    Ok(brut.to_string())
}

async fn valider_input(
    pool: &SqlitePool,
    r: ReglementationInput,
) -> Result<ReglementationValide, CcnError> {
    valider_idcc(&r.idcc)?;

    let titre  = sanitiser(&r.titre);
    let resume = sanitiser(&r.resume);
    let corps  = sanitiser(&r.corps);
    let source = sanitiser(&r.source);

    if titre.is_empty() || titre.len() > 200 {
        return Err(CcnError::Validation("Titre : 1 à 200 caractères".into()));
    }
    if resume.is_empty() || resume.len() > 500 {
        return Err(CcnError::Validation("Résumé : 1 à 500 caractères".into()));
    }
    if corps.is_empty() || corps.len() > 20_000 {
        return Err(CcnError::Validation("Corps : 1 à 20 000 caractères".into()));
    }
    if source.is_empty() || source.len() > 500 {
        return Err(CcnError::Validation("Source : 1 à 500 caractères".into()));
    }
    if !IMPACTS.contains(&r.impact.as_str()) {
        return Err(CcnError::Validation(format!("Impact invalide : '{}'", r.impact)));
    }
    if !STATUTS.contains(&r.statut_verif.as_str()) {
        return Err(CcnError::Validation(format!(
            "Statut de vérification invalide : '{}'",
            r.statut_verif
        )));
    }

    // Une date d'effet mal formée passerait sans bruit en SQLite (pas de
    // type date) et ressortirait telle quelle côté front. On la refuse ici.
    let date_effet = match sanitiser_opt(r.date_effet.as_deref()) {
        Some(d) => {
            if chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").is_err() {
                return Err(CcnError::Validation(format!(
                    "Date d'effet attendue au format AAAA-MM-JJ : '{d}'"
                )));
            }
            Some(d)
        }
        None => None,
    };

    let source_url = match sanitiser_opt(r.source_url.as_deref()) {
        Some(u) if !u.starts_with("http://") && !u.starts_with("https://") => {
            return Err(CcnError::Validation("Lien : http:// ou https:// attendu".into()))
        }
        autre => autre,
    };

    // Les barèmes ne passent pas par `sanitiser` : retirer les chevrons
    // d'un JSON n'aurait aucun sens, et le front échappe chaque cellule
    // au rendu. On contrôle en revanche sa structure.
    let tableaux = match r.tableaux.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(t) => Some(valider_tableaux(t)?),
        None => None,
    };

    // Activité et thème sont des clés : on vérifie qu'elles existent
    // plutôt que de laisser la contrainte remonter en erreur 500.
    let activite = sanitiser(&r.activite);
    let theme = sanitiser(&r.theme);

    let act_ok: Option<(i64,)> =
        sqlx::query_as("SELECT 1 FROM ccn_activites WHERE idcc = ? AND code = ?")
            .bind(&r.idcc)
            .bind(&activite)
            .fetch_optional(pool)
            .await?;
    if act_ok.is_none() {
        return Err(CcnError::Validation(format!("Activité inconnue : '{activite}'")));
    }

    let theme_ok: Option<(i64,)> = sqlx::query_as("SELECT 1 FROM ccn_themes WHERE code = ?")
        .bind(&theme)
        .fetch_optional(pool)
        .await?;
    if theme_ok.is_none() {
        return Err(CcnError::Validation(format!("Thème inconnu : '{theme}'")));
    }

    Ok(ReglementationValide {
        idcc: r.idcc,
        activite,
        theme,
        titre,
        resume,
        corps,
        valeur: sanitiser_opt(r.valeur.as_deref()),
        source,
        source_url,
        date_effet,
        impact: r.impact,
        regime_social: sanitiser_opt(r.regime_social.as_deref()),
        statut_verif: r.statut_verif,
        tableaux,
        publie: if r.publie { 1 } else { 0 },
        ordre: r.ordre,
    })
}

#[derive(Deserialize)]
struct FiltreAdmin {
    idcc: Option<String>,
}

/// Liste éditoriale : inclut les règles dépubliées, invisibles du public.
async fn admin_lister(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Query(f): Query<FiltreAdmin>,
) -> Result<Json<Vec<ReglementationAdmin>>, CcnError> {
    let idcc = f.idcc.unwrap_or_else(|| "0016".to_string());
    valider_idcc(&idcc)?;

    let rows = sqlx::query_as::<_, ReglementationAdmin>(
        "SELECT id, idcc, activite, theme, titre, resume, corps, valeur,
                source, source_url, date_effet, impact, regime_social,
                statut_verif, tableaux, publie, ordre, maj_le
           FROM ccn_reglementations
          WHERE idcc = ?
          ORDER BY ordre, id",
    )
    .bind(&idcc)
    .fetch_all(&*pool)
    .await?;

    Ok(Json(rows))
}

/// Référentiel complet pour les listes déroulantes de l'admin.
/// Le dossier public ne remonte que les thèmes déjà utilisés — bien vu
/// pour un lecteur, inutilisable pour créer une règle dans un thème vide.
async fn admin_referentiel(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Query(f): Query<FiltreAdmin>,
) -> Result<Json<serde_json::Value>, CcnError> {
    let idcc = f.idcc.unwrap_or_else(|| "0016".to_string());
    valider_idcc(&idcc)?;

    let activites = sqlx::query_as::<_, Activite>(
        "SELECT code, libelle, detail, ordre FROM ccn_activites WHERE idcc = ? ORDER BY ordre",
    )
    .bind(&idcc)
    .fetch_all(&*pool)
    .await?;

    let themes = sqlx::query_as::<_, Theme>(
        "SELECT code, libelle, icone, ordre FROM ccn_themes ORDER BY ordre",
    )
    .fetch_all(&*pool)
    .await?;

    Ok(Json(serde_json::json!({ "activites": activites, "themes": themes })))
}

async fn admin_creer(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Json(input): Json<ReglementationInput>,
) -> Result<(StatusCode, Json<i64>), CcnError> {
    let r = valider_input(&pool, input).await?;

    let id: (i64,) = sqlx::query_as(
        "INSERT INTO ccn_reglementations
            (idcc, activite, theme, titre, resume, corps, valeur, source,
             source_url, date_effet, impact, regime_social, statut_verif,
             tableaux, publie, ordre, maj_le)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))
         RETURNING id",
    )
    .bind(&r.idcc).bind(&r.activite).bind(&r.theme)
    .bind(&r.titre).bind(&r.resume).bind(&r.corps)
    .bind(r.valeur.as_deref()).bind(&r.source)
    .bind(r.source_url.as_deref()).bind(r.date_effet.as_deref())
    .bind(&r.impact).bind(r.regime_social.as_deref())
    .bind(&r.statut_verif).bind(r.tableaux.as_deref())
    .bind(r.publie).bind(r.ordre)
    .fetch_one(&*pool)
    .await?;

    Ok((StatusCode::CREATED, Json(id.0)))
}

async fn admin_modifier(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Path(id): Path<i64>,
    Json(input): Json<ReglementationInput>,
) -> Result<StatusCode, CcnError> {
    let r = valider_input(&pool, input).await?;

    let res = sqlx::query(
        "UPDATE ccn_reglementations
            SET idcc = ?, activite = ?, theme = ?, titre = ?, resume = ?,
                corps = ?, valeur = ?, source = ?, source_url = ?,
                date_effet = ?, impact = ?, regime_social = ?,
                statut_verif = ?, tableaux = ?, publie = ?, ordre = ?,
                maj_le = datetime('now')
          WHERE id = ?",
    )
    .bind(&r.idcc).bind(&r.activite).bind(&r.theme)
    .bind(&r.titre).bind(&r.resume).bind(&r.corps)
    .bind(r.valeur.as_deref()).bind(&r.source)
    .bind(r.source_url.as_deref()).bind(r.date_effet.as_deref())
    .bind(&r.impact).bind(r.regime_social.as_deref())
    .bind(&r.statut_verif).bind(r.tableaux.as_deref())
    .bind(r.publie).bind(r.ordre)
    .bind(id)
    .execute(&*pool)
    .await?;

    if res.rows_affected() == 0 {
        return Err(CcnError::Introuvable(format!("Règle {id} introuvable")));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn admin_supprimer(
    State(pool): State<Db>,
    _auth: AdminAuth,
    Path(id): Path<i64>,
) -> Result<StatusCode, CcnError> {
    let res = sqlx::query("DELETE FROM ccn_reglementations WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(CcnError::Introuvable(format!("Règle {id} introuvable")));
    }
    Ok(StatusCode::NO_CONTENT)
}

// ── Routers ───────────────────────────────────────────────────────────────────

pub fn ccn_router() -> Router<Db> {
    Router::new()
        // Forme REST, pratique à interroger directement et cacheable.
        .route("/api/ccn/conventions", get(lister_conventions))
        .route("/api/ccn/grilles/{idcc}", get(grilles))
        .route("/api/ccn/{idcc}", get(dossier))
        // Forme « commande », convention de la maison : c'est celle que
        // `api()` sait router indifféremment vers Tauri ou vers HTTP, donc
        // celle qui fait fonctionner la page dans l'application bureau.
        .route("/api/dossier_ccn", post(dossier_post))
        .route("/api/grilles_ccn", post(grilles_post))
        .route("/api/conventions_ccn", post(conventions_post))
}

/// Routes d'écriture, à monter sous le préfixe admin (qui reste le
/// secret de `admin::routes`) pour partager son authentification.
pub fn ccn_admin_router(prefixe: &str) -> Router<Db> {
    Router::new()
        .route(&format!("{prefixe}/ccn/referentiel"), get(admin_referentiel))
        .route(&format!("{prefixe}/ccn/reglementations"), get(admin_lister).post(admin_creer))
        .route(&format!("{prefixe}/ccn/reglementations/{{id}}"), put(admin_modifier).delete(admin_supprimer))
}
