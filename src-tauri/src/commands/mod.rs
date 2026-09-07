pub mod contrat;
pub mod paie;
pub use contrat::generer_contrat_pdf;
pub use paie::{calculer_bulletin, simuler_annee};
