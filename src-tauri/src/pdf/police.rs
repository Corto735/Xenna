//! Les six fontes embarquées dans le binaire — trois romaines, trois linéales.
//!
//! `include_bytes!` plutôt qu'une lecture de fichier : le module doit produire le
//! même PDF sur le poste de l'utilisateur (Tauri) et dans le conteneur de
//! production, sans dépendre de ce qui est installé sur la machine. Liberation
//! (SIL Open Font License 1.1) couvre le français complet — œ, €, guillemets
//! et apostrophe typographique compris —, ce que les quatorze fontes de base du
//! format PDF ne garantissent pas.
//!
//! Deux familles parce que deux documents : le contrat de travail est un texte
//! courant, il se lit en romaine ; le bulletin de paie est une grille de
//! chiffres, il se lit en linéale. Aucun PDF ne les embarque toutes les six —
//! `crate::pdf::rendu::rendre` n'inclut que les faces réellement tracées.

use printpdf::font::ParsedFont;

const SERIF_REGULIER: &[u8] = include_bytes!("../../assets/fonts/LiberationSerif-Regular.ttf");
const SERIF_GRAS: &[u8] = include_bytes!("../../assets/fonts/LiberationSerif-Bold.ttf");
const SERIF_ITALIQUE: &[u8] = include_bytes!("../../assets/fonts/LiberationSerif-Italic.ttf");
const SANS_REGULIER: &[u8] = include_bytes!("../../assets/fonts/LiberationSans-Regular.ttf");
const SANS_GRAS: &[u8] = include_bytes!("../../assets/fonts/LiberationSans-Bold.ttf");
const SANS_ITALIQUE: &[u8] = include_bytes!("../../assets/fonts/LiberationSans-Italic.ttf");

/// Les trois premières valeurs sont les romaines (Liberation Serif), les trois
/// dernières les linéales (Liberation Sans). Les noms courts vont à la romaine
/// parce que le contrat de travail, plus ancien, les portait déjà.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    Regulier,
    Gras,
    Italique,
    Sans,
    SansGras,
    SansItalique,
}

impl Face {
    pub const TOUTES: [Face; 6] = [
        Face::Regulier,
        Face::Gras,
        Face::Italique,
        Face::Sans,
        Face::SansGras,
        Face::SansItalique,
    ];
}

pub struct Polices {
    fontes: Vec<Fonte>,
}

/// Une fonte analysée, accompagnée de son em.
///
/// `ParsedFont` change de surface selon les features de printpdf : `units_per_em`
/// est un champ public tant que `text_layout` est éteint, et disparaît dès qu'on
/// l'allume. Comme cette valeur commande tout le calcul des largeurs, on la lit
/// nous-mêmes dans la table `head` du fichier plutôt que de dépendre d'une
/// configuration de la bibliothèque — vingt lignes contre une compilation qui
/// casse le jour où quelqu'un touche à la ligne de Cargo.toml.
pub struct Fonte {
    police: ParsedFont,
    em: f32,
}

/// Unités par em, lues dans la table `head` d'un sfnt (TrueType ou OpenType).
///
/// Format : `numTables` en u16 à l'offset 4, puis des enregistrements de 16 octets
/// à partir de l'offset 12 — étiquette (4), somme de contrôle (4), position (4),
/// longueur (4). Dans `head`, `unitsPerEm` est un u16 à l'offset 18.
fn unites_par_em(octets: &[u8]) -> Option<u16> {
    let u16_a = |i: usize| -> Option<u16> {
        octets.get(i..i + 2).map(|o| u16::from_be_bytes([o[0], o[1]]))
    };
    let u32_a = |i: usize| -> Option<u32> {
        octets.get(i..i + 4).map(|o| u32::from_be_bytes([o[0], o[1], o[2], o[3]]))
    };
    let tables = u16_a(4)? as usize;
    (0..tables)
        .filter_map(|n| {
            let rec = 12 + n * 16;
            (octets.get(rec..rec + 4)? == b"head").then(|| u32_a(rec + 8))?
        })
        .find_map(|debut| u16_a(debut as usize + 18))
        .filter(|em| *em > 0)
}

impl Polices {
    pub fn charger() -> Result<Self, String> {
        let lire = |octets: &[u8], nom: &str| -> Result<Fonte, String> {
            let mut avertissements = Vec::new();
            let police = ParsedFont::from_bytes(octets, 0, &mut avertissements)
                .ok_or_else(|| format!("police {nom} illisible"))?;
            let em = unites_par_em(octets)
                .ok_or_else(|| format!("police {nom} : table head illisible"))?;
            Ok(Fonte { police, em: f32::from(em) })
        };
        // L'ordre suit celui des variantes de `Face` : c'est lui qui sert d'index.
        Ok(Self {
            fontes: vec![
                lire(SERIF_REGULIER, "Serif Regular")?,
                lire(SERIF_GRAS, "Serif Bold")?,
                lire(SERIF_ITALIQUE, "Serif Italic")?,
                lire(SANS_REGULIER, "Sans Regular")?,
                lire(SANS_GRAS, "Sans Bold")?,
                lire(SANS_ITALIQUE, "Sans Italic")?,
            ],
        })
    }

    pub fn face(&self, f: Face) -> &ParsedFont {
        &self.fonte(f).police
    }

    fn fonte(&self, f: Face) -> &Fonte {
        &self.fontes[f as usize]
    }

    /// Largeur d'une chaîne, en points, à la taille donnée.
    ///
    /// C'est la seule mesure dont dépend toute la mise en page : sans elle, pas de
    /// découpe de ligne ni de justification. Elle somme les avances des glyphes,
    /// ramenées à l'em de la fonte. Un caractère absent de la fonte retombe sur
    /// `.notdef` (glyphe 0), qui a une avance : la ligne reste correctement
    /// dimensionnée même si le caractère s'imprime en blanc.
    pub fn largeur(&self, f: Face, texte: &str, taille: f32) -> f32 {
        let fonte = self.fonte(f);
        let unites: f32 = texte
            .chars()
            .map(|c| {
                let gid = fonte.police.lookup_glyph_index(c as u32).unwrap_or(0);
                f32::from(fonte.police.get_glyph_width(gid).unwrap_or(0))
            })
            .sum();
        unites / fonte.em * taille
    }
}
