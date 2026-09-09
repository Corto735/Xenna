//! Socle PDF commun aux documents produits par l'application.
//!
//! Il ne contient aucune règle métier : des fontes mesurables (`police`) et un
//! traducteur de tracés en opérateurs PDF (`rendu`). Chaque document a son
//! propre moteur de composition — `crate::contrat` pour le contrat de travail,
//! `crate::paie_pdf` pour le bulletin de paie — et tous deux finissent ici.

pub mod police;
pub mod rendu;

pub use police::{Face, Polices};
pub use rendu::{rendre, rendre_base64, Dessin, MM, PAGE_H, PAGE_L};
