fn main() {
    // tauri_build::build() est compilé conditionnellement avec la feature "desktop".
    // Sans #[cfg], le compilateur exigerait tauri-build même pour le binaire web,
    // tirant toute la chaîne Tauri (serde_with, time…) qui requiert rustc ≥ 1.88.
    #[cfg(feature = "desktop")]
    tauri_build::build();
}
