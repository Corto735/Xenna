pub mod calculs;
pub mod commands;
pub mod db;
pub mod models;

use commands::{calculer_bulletin, simuler_annee};
use sqlx::SqlitePool;
use tauri::Manager;

pub struct AppState {
    pub db: SqlitePool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_data = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data)?;

            let db_path = app_data.join("xenna.db");

            let pool = tauri::async_runtime::block_on(db::init_db(&db_path))
                .map_err(|e| e.to_string())?;

            app.manage(AppState { db: pool });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![calculer_bulletin, simuler_annee])
        .run(tauri::generate_context!())
        .expect("Tauri error");
}
