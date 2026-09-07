//! Contrat de travail — fabrication du PDF.
//!
//! Le front est la source de vérité du texte : il assemble les clauses retenues,
//! les numérote et y substitue les informations saisies. Ce module ne fait donc
//! aucun calcul métier — il reçoit un document déjà écrit et le **compose
//! typographiquement** : découpe des lignes, justification, pagination, en-têtes,
//! pieds et bloc de signatures.
//!
//! Rien n'est écrit sur disque ni en base : les octets du PDF sont fabriqués en
//! mémoire et rendus à l'appelant.

pub mod mise_en_page;
pub mod modele;
pub mod pdf;
pub mod police;

pub use modele::{Article, ContratPdf, Mention, ReponsePdf, Run};
pub use pdf::generer;
