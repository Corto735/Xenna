pub mod bulletin_pdf;
pub mod ccn;
pub mod contrat;
pub mod paie;
pub use ccn::{conventions_ccn, dossier_ccn};
pub use bulletin_pdf::generer_bulletin_pdf;
pub use contrat::generer_contrat_pdf;
pub use paie::{calculer_bulletin, simuler_annee};
