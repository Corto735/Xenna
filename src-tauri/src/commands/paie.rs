// Commandes Tauri exposées au frontend via window.__TAURI__.invoke().
//
// Convention de nommage Tauri : les paramètres Rust (snake_case) sont
// automatiquement attendus en camelCase côté JS.
//   date_paie   → JS envoie { datePaie: "2025-01-01" }
//   salaire_brut → JS envoie { salaireBrut: "3500.00" }
//
// Toutes les erreurs remontent en String pour que le frontend puisse les afficher.
// La chaîne d'erreur provient soit du parsing, soit d'anyhow via .to_string().

use chrono::NaiveDate;
use rust_decimal::Decimal;
use crate::{
    AppState,
    calculs::{generer_bulletin, generer_annee},
    db::ContextPaie,
    models::{Bulletin, Salarie, SimulationAnnuelle, Statut},
};

#[tauri::command]
pub async fn calculer_bulletin(
    state: tauri::State<'_, AppState>,
    salarie: Salarie,
    date_paie: String,
) -> Result<Bulletin, String> {
    // La date est validée ici pour renvoyer un message lisible plutôt que
    // laisser SQLx échouer avec une erreur opaque sur la requête.
    let date = NaiveDate::parse_from_str(&date_paie, "%Y-%m-%d")
        .map_err(|_| format!("Date invalide : '{date_paie}' (format attendu : YYYY-MM-DD)"))?;

    // Charge PMSS, SMIC et tous les taux valides à cette date depuis SQLite.
    // Échoue si la date est hors couverture de la base (avant 2015 ou table vide).
    let ctx = ContextPaie::charger(&state.db, date)
        .await
        .map_err(|e| e.to_string())?;

    Ok(generer_bulletin(salarie, &ctx))
}

#[tauri::command]
pub async fn simuler_annee(
    state: tauri::State<'_, AppState>,
    annee: i32,
    salaire_brut: String,
    statut: Statut,
) -> Result<SimulationAnnuelle, String> {
    // salaire_brut est transmis en String (pas f64) pour préserver la précision
    // exacte via rust_decimal — pas de perte d'arrondi flottant.
    let brut: Decimal = salaire_brut
        .parse()
        .map_err(|_| format!("Salaire invalide : '{salaire_brut}'"))?;

    // Génère 12 bulletins mensuels avec Fillon régularisé cumulé.
    // Peut échouer si l'année n'est pas couverte en base pour certains mois.
    generer_annee(&state.db, brut, statut, annee)
        .await
        .map_err(|e| e.to_string())
}
