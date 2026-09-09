//! Assemblage final du contrat : on compose, puis on délègue l'écriture du
//! fichier au socle commun (`crate::pdf::rendu`), partagé avec le bulletin de
//! paie.

use crate::contrat::mise_en_page;
use crate::contrat::modele::ContratPdf;
use crate::pdf::police::Polices;
use crate::pdf::rendu;

/// Rend les octets du PDF et le nombre de pages.
///
/// Tout se joue en mémoire : rien n'est écrit sur disque, rien n'est enregistré
/// en base, et le contenu du contrat n'est pas journalisé.
pub fn generer(c: &ContratPdf) -> Result<(Vec<u8>, usize), String> {
    let polices = Polices::charger()?;
    let pages = mise_en_page::composer(c, &polices);
    let titre = if c.titre.is_empty() { "Contrat de travail" } else { &c.titre };
    rendu::rendre(&pages, titre, &polices)
}

/// Encodage prêt à traverser le pont Tauri comme la route HTTP : les deux
/// renvoient exactement la même forme.
pub fn generer_base64(c: &ContratPdf) -> Result<(String, usize), String> {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let (octets, pages) = generer(c)?;
    Ok((STANDARD.encode(octets), pages))
}
