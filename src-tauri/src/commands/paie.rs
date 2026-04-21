use crate::{calculs::generer_bulletin, models::{Bulletin, Salarie}};

#[tauri::command]
pub fn calculer_bulletin(salarie: Salarie) -> Result<Bulletin, String> {
    Ok(generer_bulletin(salarie))
}
