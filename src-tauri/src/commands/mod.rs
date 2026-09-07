pub mod ccn;
pub mod contrat;
pub mod paie;
pub use ccn::{conventions_ccn, dossier_ccn};
pub use contrat::generer_contrat_pdf;
pub use paie::{calculer_bulletin, simuler_annee};
