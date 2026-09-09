//! Le moteur du bulletin : un document reçu devient une suite de pages, chacune
//! décrite par des tracés absolus. Rien ici ne connaît printpdf, et rien ici ne
//! connaît la paie — c'est de la géométrie appliquée à une grille.
//!
//! Le repère est celui de la lecture : `y` est une ligne de base mesurée depuis
//! le haut de la page. La conversion vers le repère PDF est faite au dernier
//! moment, dans `crate::pdf::rendu`.
//!
//! ── Ce qui distingue cette grille d'un tableau ordinaire ─────────────────────
//! Un bulletin de paie ne se coupe pas n'importe où. Trois règles gouvernent la
//! pagination : un bandeau de rubrique ne reste jamais seul en bas de page, le
//! bloc des totaux ne se scinde pas, et l'en-tête des colonnes se répète en tête
//! de chaque page — sans quoi les chiffres de la deuxième page ne veulent plus
//! rien dire.

use crate::paie_pdf::modele::{Annexe, BulletinPdf, Champ, Ligne, Total};
use crate::pdf::police::{Face, Polices};
use crate::pdf::rendu::{Dessin, MM, PAGE_H, PAGE_L};

// ── Géométrie ─────────────────────────────────────────────────────────────────
const MARGE_G: f32 = 13.0 * MM;
const MARGE_D: f32 = 13.0 * MM;
const MARGE_H: f32 = 13.0 * MM;
const MARGE_B: f32 = 15.0 * MM;
const COL: f32 = PAGE_L - MARGE_G - MARGE_D;

/// Largeurs des six colonnes, en points, dans l'ordre du modèle réglementaire :
/// libellé · base · taux salarial · part salarié · taux patronal · part employeur.
///
/// Les cinq colonnes de chiffres sont dimensionnées pour leur contenu — un
/// montant à six chiffres, un taux à trois décimales — et le libellé prend tout
/// le reste. C'est lui qui absorbe un changement de marge ou de format de page,
/// et c'est bien : il se découpe en lignes, une colonne de chiffres non.
const CHIFFRES: f32 = 68.0 + 50.0 + 74.0 + 50.0 + 74.0;
const LARGEURS: [f32; 6] = [COL - CHIFFRES, 68.0, 50.0, 74.0, 50.0, 74.0];

// ── Échelle typographique ─────────────────────────────────────────────────────
const T_TITRE: f32 = 14.0;
const T_SOUS_TITRE: f32 = 8.2;
const T_CHAMP: f32 = 7.0;
const T_CHAMP_LBL: f32 = 6.2;
const T_ENTETE: f32 = 6.4;
const T_BANDE: f32 = 7.4;
const T_LIGNE: f32 = 7.6;
const T_TOTAL: f32 = 8.4;
const T_MENTION: f32 = 6.4;
const T_PIED: f32 = 6.4;
const T_FILIGRANE: f32 = 74.0;
/// Diagonale de la page : l'angle qui fait traverser le mot d'un coin à l'autre.
const ANGLE_FILIGRANE: f32 = 52.0;
const ALPHA_FILIGRANE: f32 = 0.11;

const H_LIGNE: f32 = 11.4;
const H_BANDE: f32 = 12.6;
const H_TOTAL: f32 = 14.0;

const NOIR: f32 = 0.0;
const ENCRE: f32 = 0.13;
const GRIS: f32 = 0.42;
const GRIS_CLAIR: f32 = 0.62;
const TRAME: f32 = 0.90;
const TRAME_FORTE: f32 = 0.82;
const BANDE: f32 = 0.28;
const FILIGRANE: f32 = 0.0;

struct Composeur<'a> {
    p: &'a Polices,
    pages: Vec<Vec<Dessin>>,
    cur: Vec<Dessin>,
    /// Ligne de base courante, depuis le haut de la page.
    y: f32,
    /// En-têtes de colonnes à répéter en tête de page. Vide tant qu'on n'est pas
    /// entré dans la grille (l'en-tête du document n'en a pas besoin).
    entetes: Vec<String>,
}

impl<'a> Composeur<'a> {
    fn nouveau(p: &'a Polices) -> Self {
        Self { p, pages: Vec::new(), cur: Vec::new(), y: MARGE_H, entetes: Vec::new() }
    }

    fn bas(&self) -> f32 {
        PAGE_H - MARGE_B
    }

    fn saut(&mut self) {
        self.pages.push(std::mem::take(&mut self.cur));
        self.y = MARGE_H;
        if !self.entetes.is_empty() {
            let titres = self.entetes.clone();
            self.entete_colonnes(&titres);
        }
    }

    /// Réserve `h` sans rien tracer : sert aux blocs qu'on refuse de couper.
    fn besoin(&mut self, h: f32) {
        if self.y + h > self.bas() {
            self.saut();
        }
    }

    fn avance(&mut self, h: f32) -> f32 {
        self.besoin(h);
        self.y += h;
        self.y
    }

    fn ecrire(&mut self, x: f32, y: f32, texte: &str, face: Face, taille: f32, gris: f32) {
        if texte.is_empty() {
            return;
        }
        self.cur.push(Dessin::Texte { x, y, texte: texte.to_string(), face, taille, gris });
    }

    /// Écrit au fer à droite d'une abscisse. Toutes les colonnes de chiffres du
    /// bulletin s'alignent ainsi : c'est ce qui permet de comparer d'un coup
    /// d'œil deux montants sans les lire.
    fn ecrire_droite(&mut self, droite: f32, y: f32, texte: &str, face: Face, taille: f32, gris: f32) {
        if texte.is_empty() {
            return;
        }
        let l = self.p.largeur(face, texte, taille);
        self.ecrire(droite - l, y, texte, face, taille, gris);
    }

    fn filet(&mut self, y: f32, x1: f32, x2: f32, ep: f32, gris: f32) {
        self.cur.push(Dessin::Filet { x1, y1: y, x2, y2: y, ep, gris });
    }

    fn pave(&mut self, y_haut: f32, h: f32, x: f32, l: f32, gris: f32) {
        self.cur.push(Dessin::Pave { x, y: y_haut, l, h, gris });
    }

    /// Découpe un texte en lignes tenant dans `largeur`. Un mot plus large que la
    /// colonne n'est pas coupé : il déborde plutôt que de devenir illisible —
    /// mais le tirage aléatoire des libellés ne produit pas de tels mots, et le
    /// test de non-débordement le vérifie.
    fn decouper(&self, texte: &str, face: Face, taille: f32, largeur: f32) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut ligne = String::new();
        for mot in texte.split_whitespace() {
            let essai = if ligne.is_empty() { mot.to_string() } else { format!("{ligne} {mot}") };
            if !ligne.is_empty() && self.p.largeur(face, &essai, taille) > largeur {
                out.push(std::mem::take(&mut ligne));
                ligne = mot.to_string();
            } else {
                ligne = essai;
            }
        }
        if !ligne.is_empty() {
            out.push(ligne);
        }
        if out.is_empty() {
            out.push(String::new());
        }
        out
    }

    // ── Blocs du document ─────────────────────────────────────────────────────

    fn titre(&mut self, b: &BulletinPdf) {
        if !b.avertissement.is_empty() {
            let lignes = self.decouper(&b.avertissement, Face::SansGras, T_MENTION + 0.6, COL - 16.0);
            let h = 8.0 + lignes.len() as f32 * (T_MENTION + 4.0);
            let haut = self.y;
            self.pave(haut, h, MARGE_G, COL, TRAME_FORTE);
            for (i, l) in lignes.iter().enumerate() {
                let large = self.p.largeur(Face::SansGras, l, T_MENTION + 0.6);
                let y = haut + 9.0 + i as f32 * (T_MENTION + 4.0);
                self.ecrire(MARGE_G + (COL - large) / 2.0, y, l, Face::SansGras, T_MENTION + 0.6, NOIR);
            }
            self.y = haut + h + 10.0;
        }

        let y = self.avance(T_TITRE * 1.2);
        self.ecrire(MARGE_G, y, &b.titre, Face::SansGras, T_TITRE, NOIR);
        if !b.sous_titre.is_empty() {
            self.ecrire_droite(MARGE_G + COL, y, &b.sous_titre, Face::Sans, T_SOUS_TITRE, GRIS);
        }
        self.filet(y + 4.5, MARGE_G, MARGE_G + COL, 1.0, ENCRE);
        self.y += 6.0;
    }

    /// Les deux cadres d'identité, côte à côte. L'employeur à gauche parce que
    /// c'est lui qui établit le bulletin ; le salarié à droite, plus étroit.
    fn identites(&mut self, b: &BulletinPdf) {
        let l_gauche = COL * 0.56;
        let l_droite = COL - l_gauche - 10.0;
        let x_droite = MARGE_G + l_gauche + 10.0;

        let h_g = self.hauteur_cadre(&b.employeur, l_gauche);
        let h_d = self.hauteur_cadre(&b.salarie, l_droite);
        let h = h_g.max(h_d);
        self.besoin(h + 8.0);
        let haut = self.y;

        self.pave(haut, h, MARGE_G, l_gauche, TRAME);
        self.pave(haut, h, x_droite, l_droite, TRAME);
        self.cadre(haut, MARGE_G, l_gauche, &b.employeur);
        self.cadre(haut, x_droite, l_droite, &b.salarie);

        self.y = haut + h + 8.0;

        if !b.periode.is_empty() {
            let haut = self.y;
            let h = H_LIGNE + 6.0;
            self.pave(haut, h, MARGE_G, COL, TRAME);
            let n = b.periode.len().max(1);
            let pas = COL / n as f32;
            for (i, c) in b.periode.iter().enumerate() {
                let x = MARGE_G + 6.0 + i as f32 * pas;
                self.ecrire(x, haut + 6.5, &c.l, Face::Sans, T_CHAMP_LBL, GRIS);
                self.ecrire(x, haut + 14.5, &c.v, Face::SansGras, T_CHAMP, NOIR);
            }
            self.y = haut + h + 9.0;
        }
    }

    fn hauteur_cadre(&self, champs: &[Champ], largeur: f32) -> f32 {
        let l_val = largeur - 12.0 - 74.0;
        let mut h = 7.0;
        for c in champs {
            let n = self.decouper(&c.v, Face::Sans, T_CHAMP, l_val).len().max(1);
            h += n as f32 * (T_CHAMP + 2.2);
        }
        h + 5.0
    }

    fn cadre(&mut self, haut: f32, x: f32, largeur: f32, champs: &[Champ]) {
        let x_val = x + 6.0 + 74.0;
        let l_val = largeur - 12.0 - 74.0;
        let mut y = haut + 7.0;
        for c in champs {
            let lignes = self.decouper(&c.v, Face::Sans, T_CHAMP, l_val);
            y += T_CHAMP + 2.2;
            self.ecrire(x + 6.0, y, &c.l, Face::Sans, T_CHAMP_LBL, GRIS);
            for (i, l) in lignes.iter().enumerate() {
                let yy = y + i as f32 * (T_CHAMP + 2.2);
                self.ecrire(x_val, yy, l, Face::SansGras, T_CHAMP, NOIR);
            }
            y += (lignes.len() as f32 - 1.0) * (T_CHAMP + 2.2);
        }
    }

    /// Abscisses de fin de chacune des six colonnes.
    fn bords(&self) -> [f32; 6] {
        let mut b = [0.0f32; 6];
        let mut x = MARGE_G;
        for i in 0..6 {
            x += LARGEURS[i];
            b[i] = x;
        }
        b
    }

    fn entete_colonnes(&mut self, titres: &[String]) {
        let haut = self.y;
        let h = T_ENTETE + 9.0;
        self.pave(haut, h, MARGE_G, COL, TRAME_FORTE);
        let bords = self.bords();
        let y = haut + h - 3.5;
        for (i, t) in titres.iter().enumerate().take(6) {
            if t.is_empty() {
                continue;
            }
            if i == 0 {
                self.ecrire(MARGE_G + 4.0, y, t, Face::SansGras, T_ENTETE, NOIR);
            } else {
                self.ecrire_droite(bords[i] - 4.0, y, t, Face::SansGras, T_ENTETE, NOIR);
            }
        }
        self.y = haut + h;
    }

    fn bandeau(&mut self, titre: &str) {
        // Un bandeau seul en bas de page annonce une rubrique qui commence
        // ailleurs : on exige de quoi loger le bandeau et sa première ligne.
        self.besoin(H_BANDE + H_LIGNE);
        let haut = self.y;
        self.pave(haut, H_BANDE, MARGE_G, COL, BANDE);
        self.ecrire(MARGE_G + 4.0, haut + H_BANDE - 3.8, titre, Face::SansGras, T_BANDE, 1.0);
        self.y = haut + H_BANDE;
    }

    fn ligne(&mut self, l: &Ligne) {
        let bords = self.bords();
        let face = if l.fort { Face::SansGras } else if l.note { Face::SansItalique } else { Face::Sans };
        let gris = if l.note { GRIS } else { ENCRE };

        // Une note n'a pas de montant : elle a droit à toute la largeur. La
        // cantonner à la colonne des libellés la ferait courir sur cinq lignes
        // en laissant les cinq sixièmes de la page blancs.
        let large_lbl = if l.note { COL - 10.0 } else { LARGEURS[0] - 10.0 };
        let lignes_lbl = self.decouper(&l.libelle, face, T_LIGNE, large_lbl);
        let h = (H_LIGNE * lignes_lbl.len() as f32).max(H_LIGNE);
        self.besoin(h);
        let haut = self.y;

        if l.fort {
            self.pave(haut, h, MARGE_G, COL, TRAME);
        }
        let base_y = haut + h - 3.6;
        for (i, t) in lignes_lbl.iter().enumerate() {
            self.ecrire(MARGE_G + 4.0, haut + H_LIGNE - 3.6 + i as f32 * H_LIGNE, t, face, T_LIGNE, gris);
        }
        let cols = [&l.base, &l.taux_sal, &l.montant_sal, &l.taux_pat, &l.montant_pat];
        for (i, v) in cols.iter().enumerate() {
            self.ecrire_droite(bords[i + 1] - 4.0, base_y, v, face, T_LIGNE, gris);
        }
        if !l.fort {
            self.filet(haut + h, MARGE_G, MARGE_G + COL, 0.25, TRAME_FORTE);
        }
        self.y = haut + h;
    }

    /// Le bas de bulletin. Il ne se coupe pas : un net à payer sur la page
    /// suivante que son brut, c'est un document qu'on relit deux fois.
    fn totaux(&mut self, totaux: &[Total]) {
        if totaux.is_empty() {
            return;
        }
        let h: f32 = totaux
            .iter()
            .map(|t| if t.note.is_empty() { H_TOTAL } else { H_TOTAL + T_MENTION + 1.5 })
            .sum();
        self.besoin(h + 10.0);
        self.y += 8.0;

        for t in totaux {
            let hl = if t.note.is_empty() { H_TOTAL } else { H_TOTAL + T_MENTION + 1.5 };
            let haut = self.y;
            match t.poids {
                1 => self.pave(haut, hl, MARGE_G, COL, TRAME),
                2 => self.pave(haut, hl, MARGE_G, COL, TRAME_FORTE),
                _ => {}
            }
            let face = if t.poids == 0 { Face::Sans } else { Face::SansGras };
            let taille = if t.poids == 2 { T_TOTAL + 1.6 } else { T_TOTAL };
            let y = haut + T_TOTAL + 3.5;
            self.ecrire(MARGE_G + 5.0, y, &t.libelle, face, taille, NOIR);
            self.ecrire_droite(MARGE_G + COL - 5.0, y, &t.valeur, face, taille, NOIR);
            if !t.note.is_empty() {
                let lignes = self.decouper(&t.note, Face::SansItalique, T_MENTION, COL - 12.0);
                self.ecrire(MARGE_G + 5.0, y + T_MENTION + 2.5, &lignes[0], Face::SansItalique, T_MENTION, GRIS);
            }
            if t.poids == 0 {
                self.filet(haut + hl, MARGE_G, MARGE_G + COL, 0.25, TRAME_FORTE);
            }
            self.y = haut + hl;
        }
    }

    fn cumuls(&mut self, cumuls: &[Champ]) {
        if cumuls.is_empty() {
            return;
        }
        self.besoin(H_LIGNE * 3.0);
        self.y += 12.0;
        let haut = self.y;
        // Deux rangs : les libellés au-dessus, les valeurs en dessous — la
        // disposition qu'ont tous les bandeaux de cumuls, et qui tient sur une
        // ligne ce qu'un tableau prendrait sur douze.
        let h = T_CHAMP_LBL + T_CHAMP + 12.0;
        self.pave(haut, h, MARGE_G, COL, TRAME);
        let n = cumuls.len().max(1);
        let pas = COL / n as f32;
        for (i, c) in cumuls.iter().enumerate() {
            let x = MARGE_G + 5.0 + i as f32 * pas;
            let lbl = self.decouper(&c.l, Face::Sans, T_CHAMP_LBL, pas - 8.0);
            self.ecrire(x, haut + 8.0, &lbl[0], Face::Sans, T_CHAMP_LBL, GRIS);
            self.ecrire(x, haut + 8.0 + T_CHAMP + 3.0, &c.v, Face::SansGras, T_CHAMP, NOIR);
        }
        self.y = haut + h;
    }

    fn mentions(&mut self, mentions: &[String]) {
        if mentions.is_empty() {
            return;
        }
        self.y += 10.0;
        for m in mentions {
            let lignes = self.decouper(m, Face::SansItalique, T_MENTION, COL);
            for l in lignes {
                let y = self.avance(T_MENTION + 2.6);
                self.ecrire(MARGE_G, y, &l, Face::SansItalique, T_MENTION, GRIS_CLAIR);
            }
            self.y += 2.0;
        }
    }

    // ── Annexe ────────────────────────────────────────────────────────────────

    fn annexe(&mut self, a: &Annexe) {
        self.entetes.clear();
        self.saut();

        let y = self.avance(T_TITRE * 1.1);
        self.ecrire(MARGE_G, y, &a.titre, Face::SansGras, T_TITRE - 2.0, NOIR);
        self.filet(y + 4.0, MARGE_G, MARGE_G + COL, 0.8, ENCRE);
        self.y += 6.0;

        if !a.chapeau.is_empty() {
            for l in self.decouper(&a.chapeau, Face::Sans, T_MENTION + 0.8, COL) {
                let y = self.avance(T_MENTION + 3.4);
                self.ecrire(MARGE_G, y, &l, Face::Sans, T_MENTION + 0.8, GRIS);
            }
            self.y += 6.0;
        }

        self.entetes = a.colonnes.clone();
        let titres = self.entetes.clone();
        self.entete_colonnes(&titres);

        let bords = self.bords();
        for l in &a.lignes {
            let lbl = self.decouper(&l.libelle, Face::Sans, T_LIGNE, LARGEURS[0] - 10.0);
            let refs = if l.reference.is_empty() {
                Vec::new()
            } else {
                self.decouper(&l.reference, Face::SansItalique, T_MENTION, LARGEURS[0] + LARGEURS[1] - 10.0)
            };
            let h = H_LIGNE * lbl.len() as f32 + refs.len() as f32 * (T_MENTION + 2.0) + 2.0;
            self.besoin(h);
            let haut = self.y;

            for (i, t) in lbl.iter().enumerate() {
                self.ecrire(MARGE_G + 4.0, haut + H_LIGNE - 3.6 + i as f32 * H_LIGNE, t, Face::Sans, T_LIGNE, ENCRE);
            }
            let base_y = haut + H_LIGNE * lbl.len() as f32 - 3.6;
            let cols = [&l.code, &l.base, &l.taux_sal, &l.montant_sal, &l.taux_pat, &l.montant_pat];
            // Sept colonnes pour six bords : le code se loge au fer à droite de
            // la colonne du libellé, en grisé, là où il ne gêne personne.
            self.ecrire_droite(bords[0] - 4.0, haut + H_LIGNE - 3.6, cols[0], Face::SansItalique, T_MENTION, GRIS_CLAIR);
            for (i, v) in cols[1..].iter().enumerate() {
                self.ecrire_droite(bords[i + 1] - 4.0, base_y, v, Face::Sans, T_LIGNE, ENCRE);
            }
            for (i, r) in refs.iter().enumerate() {
                let y = haut + H_LIGNE * lbl.len() as f32 + (i as f32 + 1.0) * (T_MENTION + 2.0) - 1.0;
                self.ecrire(MARGE_G + 8.0, y, r, Face::SansItalique, T_MENTION, GRIS_CLAIR);
            }
            self.filet(haut + h, MARGE_G, MARGE_G + COL, 0.25, TRAME_FORTE);
            self.y = haut + h;
        }
    }
}

/// Compose le bulletin. Rend une page par élément.
pub fn composer(b: &BulletinPdf, p: &Polices) -> Vec<Vec<Dessin>> {
    let mut co = Composeur::nouveau(p);

    co.titre(b);
    co.identites(b);

    if !b.rubriques.is_empty() {
        let entetes = if b.colonnes.is_empty() {
            vec![String::new(); 6]
        } else {
            b.colonnes.clone()
        };
        co.entetes = entetes.clone();
        co.entete_colonnes(&entetes);
        for r in &b.rubriques {
            if !r.titre.is_empty() {
                co.bandeau(&r.titre);
            }
            for l in &r.lignes {
                co.ligne(l);
            }
        }
        co.entetes.clear();
    }

    co.totaux(&b.totaux);
    co.cumuls(&b.cumuls);
    co.mentions(&b.mentions);

    if let Some(a) = &b.annexe {
        if !a.lignes.is_empty() {
            co.annexe(a);
        }
    }

    co.pages.push(std::mem::take(&mut co.cur));
    habiller(&mut co.pages, b, p);
    co.pages
}

/// En-tête, filigrane et folio, ajoutés une fois le nombre de pages connu.
fn habiller(pages: &mut [Vec<Dessin>], b: &BulletinPdf, p: &Polices) {
    let total = pages.len();
    for (i, page) in pages.iter_mut().enumerate() {
        let n = i + 1;

        // Le filigrane passe PAR-DESSUS, en diagonale et translucide. L'avoir
        // mis derrière serait revenu à le faire dévorer par morceaux par les
        // aplats des bandeaux et des totaux — un mot à moitié mangé ne se lit
        // pas comme une mise en garde mais comme un défaut d'impression.
        if !b.filigrane.is_empty() {
            // La chaîne est tracée depuis son ancrage dans la direction de
            // l'angle : on recule d'une demi-longueur le long de cette direction
            // pour la centrer sur la page. Le `y` est ici une ordonnée de
            // LECTURE (comptée depuis le haut) que `rendu` retournera — d'où le
            // « + » là où la géométrie du PDF attendrait un « − ».
            let large = p.largeur(Face::SansGras, &b.filigrane, T_FILIGRANE);
            let rad = ANGLE_FILIGRANE.to_radians();
            page.push(Dessin::Filigrane {
                x: PAGE_L / 2.0 - large / 2.0 * rad.cos(),
                y: PAGE_H / 2.0 + large / 2.0 * rad.sin(),
                texte: b.filigrane.clone(),
                face: Face::SansGras,
                taille: T_FILIGRANE,
                angle: ANGLE_FILIGRANE,
                gris: FILIGRANE,
                alpha: ALPHA_FILIGRANE,
            });
        }

        if n > 1 {
            let y = MARGE_H - 5.0;
            if !b.pied.is_empty() {
                page.push(Dessin::Texte {
                    x: MARGE_G,
                    y,
                    texte: b.pied.clone(),
                    face: Face::SansItalique,
                    taille: T_PIED,
                    gris: GRIS,
                });
            }
            let droite = p.largeur(Face::SansItalique, &b.titre, T_PIED);
            page.push(Dessin::Texte {
                x: MARGE_G + COL - droite,
                y,
                texte: b.titre.clone(),
                face: Face::SansItalique,
                taille: T_PIED,
                gris: GRIS,
            });
            page.push(Dessin::Filet {
                x1: MARGE_G,
                y1: y + 3.5,
                x2: MARGE_G + COL,
                y2: y + 3.5,
                ep: 0.4,
                gris: GRIS,
            });
        }

        let y = PAGE_H - MARGE_B + 14.0;
        let folio = format!("Page {n} / {total}");
        let large = p.largeur(Face::Sans, &folio, T_PIED);
        page.push(Dessin::Texte {
            x: MARGE_G + (COL - large) / 2.0,
            y,
            texte: folio,
            face: Face::Sans,
            taille: T_PIED,
            gris: GRIS,
        });
    }
}

/// Exposées pour le test : la grille doit tenir dans la colonne, et c'est une
/// propriété qu'on vérifie plutôt qu'on ne la commente.
pub fn metriques() -> (f32, [f32; 6], f32, f32) {
    (COL, LARGEURS, MARGE_G, MARGE_B)
}
