pub mod calculs;
pub mod commands;
pub mod models;

use commands::calculer_bulletin;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![calculer_bulletin])
        .run(tauri::generate_context!())
        .expect("Tauri error");
}
