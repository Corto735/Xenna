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
    let date = NaiveDate::parse_from_str(&date_paie, "%Y-%m-%d")
        .map_err(|_| format!("Date invalide : '{date_paie}' (format attendu : YYYY-MM-DD)"))?;

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
    let brut: Decimal = salaire_brut
        .parse()
        .map_err(|_| format!("Salaire invalide : '{salaire_brut}'"))?;

    generer_annee(&state.db, brut, statut, annee)
        .await
        .map_err(|e| e.to_string())
}
