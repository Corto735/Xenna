//! Le document tel que le front l'envoie.
//!
//! Un `Run` est un fragment de texte porteur d'un style : c'est ce qui permet au
//! PDF de distinguer, comme l'aperçu à l'écran, une valeur saisie (en gras) d'un
//! libellé encore en attente (en italique).

use serde::{Deserialize, Serialize};

/// `s` : "n" courant · "b" valeur saisie · "i" libellé d'attente · "r" renvoi mort.
#[derive(Debug, Clone, Deserialize)]
pub struct Run {
    #[serde(default)]
    pub t: String,
    #[serde(default)]
    pub s: String,
}

/// Une ligne du préambule : un libellé et sa valeur.
#[derive(Debug, Clone, Deserialize)]
pub struct Mention {
    #[serde(default)]
    pub l: String,
    #[serde(default)]
    pub runs: Vec<Run>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Article {
    #[serde(default)]
    pub numero: u32,
    #[serde(default)]
    pub titre: String,
    /// Un élément par paragraphe.
    #[serde(default)]
    pub corps: Vec<Vec<Run>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContratPdf {
    #[serde(default)]
    pub titre: String,
    #[serde(default)]
    pub sous_titre: Vec<Run>,
    #[serde(default)]
    pub employeur: Vec<Mention>,
    #[serde(default)]
    pub salarie: Vec<Mention>,
    #[serde(default)]
    pub articles: Vec<Article>,
    #[serde(default)]
    pub lieu: Vec<Run>,
    #[serde(default)]
    pub date: Vec<Run>,
    /// Raison sociale reprise en en-tête des pages suivantes.
    #[serde(default)]
    pub pied: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReponsePdf {
    pub pdf_base64: String,
    pub pages: usize,
}
