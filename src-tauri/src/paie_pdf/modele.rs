//! Le bulletin tel que le front l'envoie.
//!
//! Toutes les valeurs arrivent **déjà formatées** : montants avec leur virgule
//! décimale et leur symbole, taux avec leur signe de pourcentage, dates en
//! toutes lettres. Le back ne calcule rien et n'arrondit rien — il place. C'est
//! la même règle que pour le contrat de travail, et pour la même raison : le
//! bulletin est une *traduction* d'un calcul déjà fait, et cette traduction
//! (regroupement réglementaire compris) vit du côté qui connaît le modèle
//! applicable à la date de paie.

use serde::{Deserialize, Serialize};

/// Un couple libellé / valeur des cadres d'en-tête et des cumuls.
#[derive(Debug, Clone, Deserialize)]
pub struct Champ {
    #[serde(default)]
    pub l: String,
    #[serde(default)]
    pub v: String,
}

/// Une ligne de la grille des cotisations, sur les six colonnes du modèle
/// réglementaire. Une colonne vide ne s'imprime pas : la ligne « Famille » n'a
/// pas de part salariale, et cette absence est une information.
#[derive(Debug, Clone, Deserialize)]
pub struct Ligne {
    #[serde(default)]
    pub libelle: String,
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub taux_sal: String,
    #[serde(default)]
    pub montant_sal: String,
    #[serde(default)]
    pub taux_pat: String,
    #[serde(default)]
    pub montant_pat: String,
    /// Sous-total de rubrique ou ligne de brut : tramée, en gras.
    #[serde(default)]
    pub fort: bool,
    /// Ligne sans montant — un commentaire dans la grille, en italique grisé.
    #[serde(default)]
    pub note: bool,
}

/// Un regroupement du modèle réglementaire : SANTÉ, RETRAITE, FAMILLE…
/// Un `titre` vide compose les lignes sans bandeau (haut de bulletin).
#[derive(Debug, Clone, Deserialize)]
pub struct Rubrique {
    #[serde(default)]
    pub titre: String,
    #[serde(default)]
    pub lignes: Vec<Ligne>,
}

/// Un poste du bas de bulletin.
#[derive(Debug, Clone, Deserialize)]
pub struct Total {
    #[serde(default)]
    pub libelle: String,
    #[serde(default)]
    pub valeur: String,
    /// 0 courant · 1 tramé (net social, net avant impôt) · 2 encadré (net payé).
    #[serde(default)]
    pub poids: u8,
    /// Précision imprimée sous le libellé, en petit corps grisé.
    #[serde(default)]
    pub note: String,
}

/// L'annexe : le détail ligne à ligne que le modèle réglementaire regroupe.
/// Elle n'est pas le bulletin — elle explique comment il a été obtenu.
#[derive(Debug, Clone, Deserialize)]
pub struct Annexe {
    #[serde(default)]
    pub titre: String,
    #[serde(default)]
    pub chapeau: String,
    #[serde(default)]
    pub colonnes: Vec<String>,
    #[serde(default)]
    pub lignes: Vec<LigneAnnexe>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LigneAnnexe {
    #[serde(default)]
    pub libelle: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub taux_sal: String,
    #[serde(default)]
    pub montant_sal: String,
    #[serde(default)]
    pub taux_pat: String,
    #[serde(default)]
    pub montant_pat: String,
    /// Référence légale, imprimée sous le libellé.
    #[serde(default)]
    pub reference: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BulletinPdf {
    #[serde(default)]
    pub titre: String,
    #[serde(default)]
    pub sous_titre: String,
    /// Mot imprimé en très gros et très clair derrière chaque page. Vide = aucun.
    /// Il n'est pas décoratif : c'est ce qui empêche de confondre la sortie d'un
    /// simulateur avec une pièce justificative.
    #[serde(default)]
    pub filigrane: String,
    /// Bandeau d'avertissement en tête de la première page.
    #[serde(default)]
    pub avertissement: String,
    #[serde(default)]
    pub employeur: Vec<Champ>,
    #[serde(default)]
    pub salarie: Vec<Champ>,
    #[serde(default)]
    pub periode: Vec<Champ>,
    /// Les six en-têtes de la grille, dans l'ordre des colonnes.
    #[serde(default)]
    pub colonnes: Vec<String>,
    #[serde(default)]
    pub rubriques: Vec<Rubrique>,
    #[serde(default)]
    pub totaux: Vec<Total>,
    #[serde(default)]
    pub cumuls: Vec<Champ>,
    /// Mentions légales de pied de bulletin, une par paragraphe.
    #[serde(default)]
    pub mentions: Vec<String>,
    #[serde(default)]
    pub annexe: Option<Annexe>,
    /// Raison sociale reprise en en-tête des pages suivantes.
    #[serde(default)]
    pub pied: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReponsePdf {
    pub pdf_base64: String,
    pub pages: usize,
}
