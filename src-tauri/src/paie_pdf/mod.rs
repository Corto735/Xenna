//! Bulletin de paie — fabrication du PDF.
//!
//! Même partage des rôles que pour le contrat de travail, et pour la même
//! raison : **le front est la source de vérité du document, le back n'en fait
//! que la composition typographique.**
//!
//! Ce n'est pas un caprice d'architecture. Le regroupement réglementaire des
//! cotisations (SANTÉ, RETRAITE, FAMILLE…) dépend du modèle applicable à la
//! date de paie — « adapté » jusqu'au 31/12/2026, « rénové » à partir du
//! 01/01/2027 (arrêté du 25 février 2016 modifié, report par l'arrêté du
//! 11 août 2025). Cette traduction vit dans `src/bulletin_pdf.js`, aux côtés de
//! celle de la DSN, qui est de même nature. Ce module-ci ne sait ni ce qu'est
//! une cotisation, ni ce qu'est un net social : il sait placer une grille de
//! huit colonnes (six dans l'annexe) sur une page A4 sans qu'elle déborde ni se
//! coupe au mauvais endroit.
//!
//! Rien n'est écrit sur disque ni en base : les octets sont fabriqués en
//! mémoire et rendus à l'appelant.

pub mod mise_en_page;
pub mod modele;
pub mod pdf;

pub use modele::{
    Annexe, BulletinPdf, Champ, Groupe, Ligne, LigneAnnexe, ReponsePdf, Rubrique, Total,
};
pub use pdf::generer;
