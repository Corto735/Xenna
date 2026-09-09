//! Commande Tauri : fabrication du PDF du bulletin de paie.
//!
//! Elle ne touche ni à la base, ni au disque — le moteur travaille en mémoire et
//! rend les octets encodés en base64, exactement comme la route HTTP homonyme.

use crate::paie_pdf::{modele::ReponsePdf, pdf, BulletinPdf};

#[tauri::command]
pub fn generer_bulletin_pdf(bulletin: BulletinPdf) -> Result<ReponsePdf, String> {
    let (pdf_base64, pages) = pdf::generer_base64(&bulletin)?;
    Ok(ReponsePdf { pdf_base64, pages })
}
