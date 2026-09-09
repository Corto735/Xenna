//! Assemblage final du bulletin : on compose, puis on délègue l'écriture du
//! fichier au socle commun (`crate::pdf::rendu`), partagé avec le contrat.

use crate::paie_pdf::mise_en_page;
use crate::paie_pdf::modele::BulletinPdf;
use crate::pdf::police::Polices;
use crate::pdf::rendu;

/// Rend les octets du PDF et le nombre de pages.
///
/// Tout se joue en mémoire : rien n'est écrit sur disque, rien n'est enregistré
/// en base, et le contenu du bulletin — qui porte un nom, un salaire et une
/// adresse — n'est pas journalisé.
pub fn generer(b: &BulletinPdf) -> Result<(Vec<u8>, usize), String> {
    let polices = Polices::charger()?;
    let pages = mise_en_page::composer(b, &polices);
    let titre = if b.titre.is_empty() { "Bulletin de paie" } else { &b.titre };
    rendu::rendre(&pages, titre, &polices)
}

/// Encodage prêt à traverser le pont Tauri comme la route HTTP : les deux
/// renvoient exactement la même forme.
pub fn generer_base64(b: &BulletinPdf) -> Result<(String, usize), String> {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let (octets, pages) = generer(b)?;
    Ok((STANDARD.encode(octets), pages))
}
