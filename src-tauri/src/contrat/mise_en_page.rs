//! Le moteur : transforme le document reçu en une suite de pages, chacune décrite
//! par des tracés absolus. Rien ici ne connaît printpdf — c'est de la géométrie.
//!
//! Le repère est celui de la lecture : `y` est une **ligne de base mesurée depuis
//! le haut de la page**. La conversion vers le repère PDF (origine en bas à gauche)
//! est faite au dernier moment, dans `pdf.rs`.

use crate::contrat::modele::{ContratPdf, Mention, Run};
use crate::contrat::police::{Face, Polices};

// ── Géométrie de la page ──────────────────────────────────────────────────────
const MM: f32 = 72.0 / 25.4;
pub const PAGE_L: f32 = 210.0 * MM;
pub const PAGE_H: f32 = 297.0 * MM;
const MARGE_G: f32 = 22.0 * MM;
const MARGE_D: f32 = 22.0 * MM;
const MARGE_H: f32 = 24.0 * MM;
const MARGE_B: f32 = 22.0 * MM;
const COL: f32 = PAGE_L - MARGE_G - MARGE_D;
/// Largeur de la colonne des libellés dans le préambule.
const COL_LBL: f32 = 44.0 * MM;

// ── Échelle typographique ─────────────────────────────────────────────────────
const T_CORPS: f32 = 10.5;
const INTER: f32 = 14.6;
const T_TITRE_DOC: f32 = 15.0;
const T_SOUS_TITRE: f32 = 9.0;
const T_CHAPEAU: f32 = 9.5;
const T_ART: f32 = 11.0;
const T_MENTION: f32 = 9.5;
const T_PIED: f32 = 8.0;

const NOIR: f32 = 0.0;
const GRIS: f32 = 0.42;

#[derive(Debug, Clone)]
pub enum Dessin {
    Texte {
        x: f32,
        /// Ligne de base, mesurée depuis le haut de la page.
        y: f32,
        texte: String,
        face: Face,
        taille: f32,
        gris: f32,
    },
    Filet {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        ep: f32,
        gris: f32,
    },
}

/// Un mot, avec le style qu'il porte et sa largeur déjà mesurée.
struct Mot {
    texte: String,
    face: Face,
    largeur: f32,
    espace_avant: bool,
}

fn face_de(style: &str) -> Face {
    match style {
        "b" => Face::Gras,
        "i" | "r" => Face::Italique,
        _ => Face::Regulier,
    }
}

struct Composeur<'a> {
    p: &'a Polices,
    pages: Vec<Vec<Dessin>>,
    cur: Vec<Dessin>,
    /// Ligne de base courante, depuis le haut de la page.
    y: f32,
}

impl<'a> Composeur<'a> {
    fn nouveau(p: &'a Polices) -> Self {
        Self { p, pages: Vec::new(), cur: Vec::new(), y: MARGE_H }
    }

    fn bas(&self) -> f32 {
        PAGE_H - MARGE_B
    }

    fn saut(&mut self) {
        self.pages.push(std::mem::take(&mut self.cur));
        self.y = MARGE_H;
    }

    /// Réserve `h` sans rien tracer : sert aux blocs qu'on refuse de couper
    /// (un titre d'article seul en bas de page, le bloc de signatures).
    fn besoin(&mut self, h: f32) {
        if self.y + h > self.bas() {
            self.saut();
        }
    }

    /// Descend d'une ligne et rend la ligne de base où écrire.
    fn avance(&mut self, h: f32) -> f32 {
        self.besoin(h);
        self.y += h;
        self.y
    }

    fn ecrire(&mut self, x: f32, y: f32, texte: &str, face: Face, taille: f32, gris: f32) {
        if texte.is_empty() {
            return;
        }
        self.cur.push(Dessin::Texte {
            x,
            y,
            texte: texte.to_string(),
            face,
            taille,
            gris,
        });
    }

    // ── Découpe en mots ───────────────────────────────────────────────────────
    // Les blancs qui séparent deux `Run` comptent : « …de {{valeur}} et… » arrive
    // en trois fragments dont les espaces vivent aux extrémités. On les mémorise
    // sur le mot suivant plutôt que d'en faire des mots vides.
    fn mots(&self, runs: &[Run], taille: f32) -> Vec<Mot> {
        let mut out: Vec<Mot> = Vec::new();
        let mut espace = false;
        for r in runs {
            let face = face_de(&r.s);
            if r.t.starts_with(char::is_whitespace) {
                espace = true;
            }
            let mut premier = true;
            for w in r.t.split_whitespace() {
                out.push(Mot {
                    largeur: self.p.largeur(face, w, taille),
                    texte: w.to_string(),
                    face,
                    espace_avant: espace || !premier,
                });
                espace = false;
                premier = false;
            }
            if r.t.ends_with(char::is_whitespace) {
                espace = true;
            }
        }
        if let Some(m) = out.first_mut() {
            m.espace_avant = false;
        }
        out
    }

    /// Paragraphe justifié (dernière ligne au fer à gauche), à partir de `x`, sur
    /// `largeur` points.
    fn paragraphe(
        &mut self,
        runs: &[Run],
        x: f32,
        largeur: f32,
        taille: f32,
        gris: f32,
        justifie: bool,
    ) {
        let mots = self.mots(runs, taille);
        if mots.is_empty() {
            return;
        }
        let espace = self.p.largeur(Face::Regulier, " ", taille);

        // Découpe gloutonne : on remplit tant que ça tient.
        let mut lignes: Vec<(usize, usize, f32, usize)> = Vec::new(); // début, fin, largeur, nb blancs
        let (mut debut, mut large, mut blancs) = (0usize, 0.0f32, 0usize);
        for (i, m) in mots.iter().enumerate() {
            let ajout = if i > debut && m.espace_avant { espace } else { 0.0 } + m.largeur;
            if i > debut && large + ajout > largeur {
                lignes.push((debut, i, large, blancs));
                debut = i;
                large = m.largeur;
                blancs = 0;
            } else {
                large += ajout;
                if i > debut && m.espace_avant {
                    blancs += 1;
                }
            }
        }
        lignes.push((debut, mots.len(), large, blancs));

        let derniere = lignes.len() - 1;
        let interligne = taille * 1.39;
        for (n, (d, f, large, blancs)) in lignes.into_iter().enumerate() {
            let y = self.avance(interligne);
            // Le blanc supplémentaire est réparti sur les intervalles ; au-delà de
            // 1,7 espace on renonce à justifier, sinon la ligne se creuse de rivières.
            let sup = if justifie && n != derniere && blancs > 0 {
                let s = (largeur - large) / blancs as f32;
                if s > espace * 1.7 { 0.0 } else { s }
            } else {
                0.0
            };
            let mut cx = x;
            for (i, m) in mots[d..f].iter().enumerate() {
                if i > 0 && m.espace_avant {
                    cx += espace + sup;
                }
                self.ecrire(cx, y, &m.texte, m.face, taille, gris);
                cx += m.largeur;
            }
        }
    }

    /// Ligne unique, centrée ou au fer à droite, sans découpe.
    ///
    /// `face_forcee` impose une fonte à toute la ligne. Les largeurs portées par les
    /// mots ayant été mesurées dans LEUR fonte d'origine, il faut alors les remesurer :
    /// une italique n'a pas la chasse d'une romaine, et caler la ligne sur la mauvaise
    /// mesure la fait déborder de la colonne.
    fn ligne_calee(&mut self, runs: &[Run], taille: f32, gris: f32, centree: bool, face_forcee: Option<Face>) {
        let mots = self.mots(runs, taille);
        if mots.is_empty() {
            return;
        }
        let largeurs: Vec<f32> = match face_forcee {
            None => mots.iter().map(|m| m.largeur).collect(),
            Some(f) => mots.iter().map(|m| self.p.largeur(f, &m.texte, taille)).collect(),
        };
        let espace = self.p.largeur(face_forcee.unwrap_or(Face::Regulier), " ", taille);
        let large: f32 = largeurs
            .iter()
            .enumerate()
            .map(|(i, l)| l + if i > 0 && mots[i].espace_avant { espace } else { 0.0 })
            .sum();
        let y = self.avance(taille * 1.4);
        let mut cx = if centree {
            MARGE_G + (COL - large) / 2.0
        } else {
            MARGE_G + COL - large
        };
        for (i, m) in mots.iter().enumerate() {
            if i > 0 && m.espace_avant {
                cx += espace;
            }
            self.ecrire(cx, y, &m.texte, face_forcee.unwrap_or(m.face), taille, gris);
            cx += largeurs[i];
        }
    }

    fn texte_brut(&mut self, texte: &str, taille: f32, face: Face, gris: f32, centre: bool) {
        let large = self.p.largeur(face, texte, taille);
        let y = self.avance(taille * 1.5);
        let x = if centre { MARGE_G + (COL - large) / 2.0 } else { MARGE_G };
        self.ecrire(x, y, texte, face, taille, gris);
    }

    fn espace_vertical(&mut self, h: f32) {
        self.y += h;
    }

    /// Bandeau de section — « ENTRE LES SOUSSIGNÉS », « IL A ÉTÉ CONVENU… ».
    fn chapeau(&mut self, texte: &str) {
        self.besoin(INTER * 2.2);
        self.espace_vertical(INTER * 0.7);
        let y = self.avance(T_CHAPEAU * 1.5);
        self.ecrire(MARGE_G, y, texte, Face::Gras, T_CHAPEAU, NOIR);
        self.cur.push(Dessin::Filet {
            x1: MARGE_G,
            y1: y + 4.0,
            x2: MARGE_G + COL,
            y2: y + 4.0,
            ep: 0.5,
            gris: GRIS,
        });
        self.espace_vertical(4.0);
    }

    /// Ligne du préambule : libellé à gauche, valeur dans la colonne de droite,
    /// la valeur pouvant courir sur plusieurs lignes (une adresse, par exemple).
    fn mention(&mut self, m: &Mention) {
        // Libellé et première ligne de valeur partagent la même ligne de base. On
        // réserve donc la place AVANT d'écrire quoi que ce soit : si un saut de page
        // doit avoir lieu, les deux y basculent ensemble.
        let interligne = T_MENTION * 1.39;
        self.besoin(interligne);
        let y_lbl = self.y + interligne;
        self.ecrire(MARGE_G, y_lbl, &m.l, Face::Italique, T_MENTION, GRIS);
        let depart = self.y;
        self.paragraphe(&m.runs, MARGE_G + COL_LBL, COL - COL_LBL, T_MENTION, NOIR, false);
        if self.y == depart {
            self.y += interligne; // valeur vide : la ligne existe quand même
        }
    }
}

// ── Composition du document ───────────────────────────────────────────────────
pub fn composer(c: &ContratPdf, p: &Polices) -> Vec<Vec<Dessin>> {
    let mut co = Composeur::nouveau(p);

    // Titre
    co.espace_vertical(6.0);
    co.texte_brut(&c.titre, T_TITRE_DOC, Face::Gras, NOIR, true);
    if !c.sous_titre.is_empty() {
        co.ligne_calee(&c.sous_titre, T_SOUS_TITRE, GRIS, true, Some(Face::Italique));
    }
    co.espace_vertical(INTER);

    // Les parties
    co.chapeau("ENTRE LES SOUSSIGNÉS");
    for m in &c.employeur {
        co.mention(m);
    }
    co.espace_vertical(4.0);
    co.ligne_calee(
        &[Run { t: "ci-après « l’employeur », d’une part,".into(), s: "n".into() }],
        T_MENTION,
        GRIS,
        false,
        Some(Face::Italique),
    );
    co.espace_vertical(INTER * 0.6);
    for m in &c.salarie {
        co.mention(m);
    }
    co.espace_vertical(4.0);
    co.ligne_calee(
        &[Run { t: "ci-après « le salarié », d’autre part,".into(), s: "n".into() }],
        T_MENTION,
        GRIS,
        false,
        Some(Face::Italique),
    );

    // Les articles
    co.chapeau("IL A ÉTÉ CONVENU CE QUI SUIT");
    for a in &c.articles {
        // Un titre d'article ne reste jamais seul en bas de page : on exige la
        // place du titre et des deux premières lignes de son corps.
        co.besoin(T_ART * 1.6 + INTER * 2.0);
        co.espace_vertical(INTER * 0.55);
        let titre = format!("Article {} — {}", a.numero, a.titre);
        let y = co.avance(T_ART * 1.5);
        co.ecrire(MARGE_G, y, &titre, Face::Gras, T_ART, NOIR);
        co.espace_vertical(2.0);
        for para in &a.corps {
            co.paragraphe(para, MARGE_G, COL, T_CORPS, NOIR, true);
            co.espace_vertical(3.0);
        }
    }

    // Formule finale et signatures — insécables.
    let bloc_final = INTER * 2.0 + 26.0 * MM + T_MENTION * 3.0;
    co.besoin(bloc_final);
    co.espace_vertical(INTER);
    let mut fait: Vec<Run> = vec![Run { t: "Fait à ".into(), s: "n".into() }];
    fait.extend(c.lieu.iter().map(|r| Run { t: r.t.clone(), s: r.s.clone() }));
    fait.push(Run { t: ", le ".into(), s: "n".into() });
    fait.extend(c.date.iter().map(|r| Run { t: r.t.clone(), s: r.s.clone() }));
    fait.push(Run { t: ", en deux exemplaires originaux.".into(), s: "n".into() });
    co.paragraphe(&fait, MARGE_G, COL, T_CORPS, NOIR, false);

    co.espace_vertical(INTER * 1.2);
    let y = co.avance(T_MENTION * 1.5);
    let demi = COL / 2.0;
    co.ecrire(MARGE_G, y, "L’EMPLOYEUR", Face::Gras, T_MENTION, NOIR);
    co.ecrire(MARGE_G + demi, y, "LE SALARIÉ", Face::Gras, T_MENTION, NOIR);
    let y = co.avance(T_MENTION * 1.4);
    co.ecrire(MARGE_G, y, "signature et cachet", Face::Italique, T_PIED, GRIS);
    co.ecrire(
        MARGE_G + demi,
        y,
        "précédée de la mention « lu et approuvé »",
        Face::Italique,
        T_PIED,
        GRIS,
    );
    let y_cadre = y + 6.0;
    for dx in [0.0, demi] {
        co.cur.push(Dessin::Filet {
            x1: MARGE_G + dx,
            y1: y_cadre + 26.0 * MM,
            x2: MARGE_G + dx + demi - 12.0 * MM,
            y2: y_cadre + 26.0 * MM,
            ep: 0.5,
            gris: GRIS,
        });
    }
    co.y = y_cadre + 26.0 * MM;

    co.saut();
    habiller(&mut co.pages, c, p);
    co.pages
}

/// En-tête et pied de page, ajoutés une fois le nombre de pages connu.
fn habiller(pages: &mut [Vec<Dessin>], c: &ContratPdf, p: &Polices) {
    let total = pages.len();
    for (i, page) in pages.iter_mut().enumerate() {
        let n = i + 1;

        // La première page porte déjà le titre : elle se passe d'en-tête.
        if n > 1 {
            let y = MARGE_H - 10.0;
            if !c.pied.is_empty() {
                page.push(Dessin::Texte {
                    x: MARGE_G,
                    y,
                    texte: c.pied.clone(),
                    face: Face::Italique,
                    taille: T_PIED,
                    gris: GRIS,
                });
            }
            let droite = p.largeur(Face::Italique, &c.titre, T_PIED);
            page.push(Dessin::Texte {
                x: MARGE_G + COL - droite,
                y,
                texte: c.titre.clone(),
                face: Face::Italique,
                taille: T_PIED,
                gris: GRIS,
            });
            page.push(Dessin::Filet {
                x1: MARGE_G,
                y1: y + 4.0,
                x2: MARGE_G + COL,
                y2: y + 4.0,
                ep: 0.4,
                gris: GRIS,
            });
        }

        let y = PAGE_H - MARGE_B + 16.0;
        let folio = format!("Page {n} / {total}");
        let large = p.largeur(Face::Regulier, &folio, T_PIED);
        page.push(Dessin::Texte {
            x: MARGE_G + (COL - large) / 2.0,
            y,
            texte: folio,
            face: Face::Regulier,
            taille: T_PIED,
            gris: GRIS,
        });
        let paraphes = "Paraphes :  ______  /  ______";
        let large = p.largeur(Face::Italique, paraphes, T_PIED);
        page.push(Dessin::Texte {
            x: MARGE_G + COL - large,
            y,
            texte: paraphes.to_string(),
            face: Face::Italique,
            taille: T_PIED,
            gris: GRIS,
        });
    }
}
