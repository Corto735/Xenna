pub mod calculs;
pub mod db;
pub mod models;

#[cfg(feature = "desktop")]
pub mod commands;

#[cfg(feature = "desktop")]
use commands::{calculer_bulletin, simuler_annee};
#[cfg(feature = "desktop")]
use sqlx::SqlitePool;
#[cfg(feature = "desktop")]
use tauri::Manager;

#[cfg(feature = "desktop")]
pub struct AppState {
    pub db: SqlitePool,
}

#[cfg(feature = "desktop")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_data = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data)?;

            let db_path = app_data.join("xenna.db");

            // IMPORTANT : ne pas utiliser block_on ici.
            // block_on crée un runtime tokio TEMPORAIRE pour exécuter init_db.
            // sqlx::SqlitePool spawne ses tâches internes de gestion de connexions
            // SUR ce runtime temporaire. Quand block_on retourne, le runtime est
            // détruit → les tâches du pool meurent. Toute requête sqlx ultérieure
            // (dans les commandes Tauri) tente de communiquer avec ces tâches
            // mortes → panic → Tauri renvoie "" au frontend (erreur muette).
            //
            // Solution : spawner init_db sur le runtime PRINCIPAL de Tauri via
            // tauri::async_runtime::spawn, puis attendre le résultat par canal.
            // Le pool est ainsi lié au bon runtime et reste valide pour toute la
            // durée de vie de l'app.
            let (tx, rx) = std::sync::mpsc::channel();
            tauri::async_runtime::spawn(async move {
                tx.send(db::init_db(&db_path).await).ok();
            });
            let pool = rx.recv()
                .map_err(|_| "Le thread d'initialisation DB a planté")?
                .map_err(|e| e.to_string())?;

            app.manage(AppState { db: pool });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![calculer_bulletin, simuler_annee])
        .run(tauri::generate_context!())
        .expect("Tauri error");
}
