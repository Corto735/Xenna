//! Commande Tauri : fabrication du PDF du contrat.
//!
//! Elle ne touche ni à la base, ni au disque — le moteur travaille en mémoire et
//! rend les octets encodés en base64, exactement comme la route HTTP homonyme.

use crate::contrat::{pdf, ContratPdf, ReponsePdf};

#[tauri::command]
pub fn generer_contrat_pdf(contrat: ContratPdf) -> Result<ReponsePdf, String> {
    let (pdf_base64, pages) = pdf::generer_base64(&contrat)?;
    Ok(ReponsePdf { pdf_base64, pages })
}
