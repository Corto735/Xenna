//! Le bulletin tel que le front l'envoie.
//!
//! Toutes les valeurs arrivent **déjà formatées** : montants, taux et quantités
//! avec leurs décimales, dates en toutes lettres. Le back ne calcule rien et
//! n'arrondit rien — il place. C'est la même règle que pour le contrat de
//! travail, et pour la même raison : le bulletin est une *traduction* d'un
//! calcul déjà fait, et cette traduction (regroupement des cotisations compris)
//! vit du côté qui connaît le calcul.
//!
//! La disposition reproduit celle d'un bulletin de logiciel de paie du marché :
//! en-tête employeur et salarié, cadre d'adresse, grille *Éléments de paie ·
//! Base · Taux · À déduire · À payer · Charges patronales*, puis un pied fixe
//! (cumuls mensuels et annuels, compteurs de congés, net payé).

use serde::{Deserialize, Serialize};

/// Un couple libellé / valeur (« Matricule : 00068 »).
#[derive(Debug, Clone, Deserialize)]
pub struct Champ {
    #[serde(default)]
    pub l: String,
    #[serde(default)]
    pub v: String,
}

/// Une ligne de la grille. Colonnes : libellé · base · taux · à déduire ·
/// à payer · puis les trois colonnes des charges patronales (base, taux,
/// montant). Une colonne vide ne s'imprime pas.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Ligne {
    #[serde(default)]
    pub libelle: String,
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub taux: String,
    #[serde(default)]
    pub a_deduire: String,
    #[serde(default)]
    pub a_payer: String,
    #[serde(default)]
    pub base_pat: String,
    #[serde(default)]
    pub taux_pat: String,
    #[serde(default)]
    pub montant_pat: String,
    /// Libellé et montants en gras (titre de groupe, salaire brut, totaux).
    #[serde(default)]
    pub fort: bool,
    /// Grand corps gras : « Net à payer avant impôt sur le revenu ».
    #[serde(default)]
    pub grand: bool,
    /// Commentaire en italique grisé, sans montant.
    #[serde(default)]
    pub note: bool,
    /// Libellé centré dans sa colonne (« Taux personnalisé »).
    #[serde(default)]
    pub centre: bool,
    /// Ligne blanche : l'aération entre deux blocs.
    #[serde(default)]
    pub vide: bool,
}

/// Une ligne du pied : son intitulé (« Mensuel ») puis une valeur par colonne.
#[derive(Debug, Clone, Deserialize)]
pub struct LignePied {
    #[serde(default)]
    pub l: String,
    #[serde(default)]
    pub v: Vec<String>,
}

/// Un titre qui chapeaute plusieurs colonnes voisines de l'en-tête de l'annexe —
/// « PART SALARIÉ » au-dessus de taux et montant. Indices de colonnes inclusifs,
/// la désignation étant la colonne 0.
#[derive(Debug, Clone, Deserialize)]
pub struct Groupe {
    #[serde(default)]
    pub titre: String,
    #[serde(default)]
    pub de: usize,
    #[serde(default)]
    pub a: usize,
}

/// L'annexe : le détail ligne à ligne que la grille regroupe, et ce que le
/// simulateur ne sait pas. Elle n'est pas le bulletin — elle explique comment il
/// a été obtenu.
#[derive(Debug, Clone, Deserialize)]
pub struct Annexe {
    #[serde(default)]
    pub titre: String,
    /// Paragraphes d'introduction, imprimés avant le tableau.
    #[serde(default)]
    pub chapeau: Vec<String>,
    /// Les six en-têtes du tableau de l'annexe.
    #[serde(default)]
    pub colonnes: Vec<String>,
    #[serde(default)]
    pub groupes: Vec<Groupe>,
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

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BulletinPdf {
    /// « BULLETIN DE SALAIRE ».
    #[serde(default)]
    pub titre: String,
    /// « Septembre 2026 », imprimé après « Période : ».
    #[serde(default)]
    pub periode: String,
    /// Référence en haut à droite de la page.
    #[serde(default)]
    pub reference: String,
    /// Mot imprimé en très gros et très clair en travers de chaque page. Vide =
    /// aucun. Il n'est pas décoratif : c'est ce qui empêche de confondre la
    /// sortie d'un simulateur avec une pièce justificative.
    #[serde(default)]
    pub filigrane: String,
    /// Raison sociale, en gras en tête de page.
    #[serde(default)]
    pub employeur: String,
    #[serde(default)]
    pub employeur_adresse: Vec<String>,
    /// Identifiants de l'employeur, une ligne par entrée, plusieurs couples par
    /// ligne (« Siret : … Code Naf : … »).
    #[serde(default)]
    pub identifiants: Vec<Vec<Champ>>,
    /// Blocs du salarié (matricule ; emploi et classification ; entrée et
    /// ancienneté), séparés d'une ligne blanche.
    #[serde(default)]
    pub blocs: Vec<Vec<Champ>>,
    /// Ligne pleine largeur sous les blocs (« Convention collective : … »).
    #[serde(default)]
    pub convention: Option<Champ>,
    /// Cadre d'adresse du salarié, une ligne par entrée.
    #[serde(default)]
    pub destinataire: Vec<String>,
    /// Les six en-têtes de la grille.
    #[serde(default)]
    pub colonnes: Vec<String>,
    #[serde(default)]
    pub lignes: Vec<Ligne>,
    /// En-têtes du tableau des cumuls (la première colonne, celle des
    /// intitulés de ligne, comprise).
    #[serde(default)]
    pub pied_entetes: Vec<String>,
    #[serde(default)]
    pub pied_lignes: Vec<LignePied>,
    /// En-têtes des compteurs de congés (intitulé de ligne compris).
    #[serde(default)]
    pub conges_entetes: Vec<String>,
    #[serde(default)]
    pub conges_lignes: Vec<LignePied>,
    /// « Net payé : 1 809.65 euros ».
    #[serde(default)]
    pub net_paye: String,
    /// « Paiement le 30/09/2026 par Virement ».
    #[serde(default)]
    pub paiement: String,
    /// Mention légale de bas de page.
    #[serde(default)]
    pub mention: String,
    #[serde(default)]
    pub annexe: Option<Annexe>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReponsePdf {
    pub pdf_base64: String,
    pub pages: usize,
}
