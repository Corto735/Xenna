//! DTOs de consultation des conventions collectives.
//!
//! Contenu éditorial pur : aucune de ces structures n'alimente un
//! calcul de bulletin. Le moteur de paie garde ses barèmes dans
//! `ContextPaie` et `calculs::absence`.

use serde::{Deserialize, Serialize};

/// Entrée de la liste des conventions disponibles.
#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ConventionResume {
    pub idcc:          String,
    pub libelle:       String,
    pub libelle_court: String,
    /// Nombre de règles publiées — évite au front un second appel
    /// juste pour afficher un compteur.
    pub nb_regles:     i64,
}

/// Convention avec son champ d'application et ses références.
#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Convention {
    pub idcc:           String,
    pub libelle:        String,
    pub libelle_court:  String,
    pub champ:          String,
    pub brochure_jo:    Option<String>,
    pub legifrance_id:  Option<String>,
    pub date_signature: Option<String>,
}

/// Sous-champ conventionnel (marchandises, voyageurs, sanitaire…).
#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Activite {
    pub code:    String,
    pub libelle: String,
    pub detail:  Option<String>,
    pub ordre:   i64,
}

/// Axe thématique de lecture, transversal aux activités.
#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub code:    String,
    pub libelle: String,
    pub icone:   Option<String>,
    pub ordre:   i64,
}

/// Une règle conventionnelle et son effet sur le bulletin.
#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Reglementation {
    pub id:            i64,
    pub activite:      String,
    pub theme:         String,
    pub titre:         String,
    pub resume:        String,
    pub corps:         String,
    pub valeur:        Option<String>,
    pub source:        String,
    pub source_url:    Option<String>,
    pub date_effet:    Option<String>,
    pub impact:        String,
    pub regime_social: Option<String>,
    /// `brouillon` | `a_verifier` | `verifie` — affiché tel quel au
    /// lecteur. Un contenu non validé par un praticien doit se
    /// signaler comme tel.
    pub statut_verif:  String,
    /// Barèmes chiffrés, en JSON : liste de
    /// `{ titre, colonnes[], lignes[][], note }`. Transporté en
    /// chaîne brute — le front le désérialise, le serveur n'a
    /// aucune raison de le manipuler.
    pub tableaux:      Option<String>,
    pub ordre:         i64,
}

/// Vue admin d'une règle : inclut ce que le public ne voit pas
/// (règles dépubliées, convention de rattachement).
#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ReglementationAdmin {
    pub id:            i64,
    pub idcc:          String,
    pub activite:      String,
    pub theme:         String,
    pub titre:         String,
    pub resume:        String,
    pub corps:         String,
    pub valeur:        Option<String>,
    pub source:        String,
    pub source_url:    Option<String>,
    pub date_effet:    Option<String>,
    pub impact:        String,
    pub regime_social: Option<String>,
    pub statut_verif:  String,
    pub tableaux:      Option<String>,
    pub publie:        i64,
    pub ordre:         i64,
    pub maj_le:        String,
}

/// Charge utile de création et de modification d'une règle.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReglementationInput {
    pub idcc:          String,
    pub activite:      String,
    pub theme:         String,
    pub titre:         String,
    pub resume:        String,
    pub corps:         String,
    pub valeur:        Option<String>,
    pub source:        String,
    pub source_url:    Option<String>,
    pub date_effet:    Option<String>,
    pub impact:        String,
    pub regime_social: Option<String>,
    pub statut_verif:  String,
    /// JSON des barèmes. Validé à l'entrée : un JSON malformé
    /// casserait silencieusement l'affichage de la règle.
    pub tableaux:      Option<String>,
    pub publie:        bool,
    pub ordre:         i64,
}

/// Réponse complète pour une convention : tout ce qu'il faut au
/// front en un seul aller-retour. Le volume reste modeste (quelques
/// dizaines de règles), la pagination serait du zèle.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DossierCcn {
    pub convention:      Convention,
    pub activites:       Vec<Activite>,
    pub themes:          Vec<Theme>,
    pub reglementations: Vec<Reglementation>,
}
