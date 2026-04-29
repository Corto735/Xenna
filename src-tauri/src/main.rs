#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(feature = "desktop")]
    xenna_paie_lib::run();
    #[cfg(not(feature = "desktop"))]
    eprintln!("Utilise le binaire 'web' pour le déploiement serveur.");
}
