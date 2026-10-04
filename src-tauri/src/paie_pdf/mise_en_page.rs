//! Le moteur du bulletin : un document reçu devient une suite de pages, chacune
//! décrite par des tracés absolus. Rien ici ne connaît printpdf, et rien ici ne
//! connaît la paie — c'est de la géométrie appliquée à une grille.
//!
//! Le repère est celui de la lecture : `y` est une ligne de base mesurée depuis
//! le haut de la page. La conversion vers le repère PDF est faite au dernier
//! moment, dans `crate::pdf::rendu`.
//!
//! ── La disposition ───────────────────────────────────────────────────────────
//! Celle d'un bulletin de logiciel de paie du marché, relevée sur un bulletin
//! réel : en-tête employeur à gauche et titre au centre, blocs du salarié, cadre
//! d'adresse bleuté pour l'enveloppe à fenêtre, puis une GRILLE À HAUTEUR FIXE —
//! ses filets verticaux descendent jusqu'au pied, quel que soit le nombre de
//! lignes — et un PIED FIXE en bas de page : cumuls mensuels et annuels,
//! compteurs de congés, net payé encadré, mention légale.
//!
//! Si les lignes ne tiennent pas, la grille continue sur la page suivante avec
//! son en-tête ; le pied n'est imprimé qu'une fois, sur la dernière page.

use crate::paie_pdf::modele::{Annexe, BulletinPdf, Groupe, Ligne, LignePied};
use crate::pdf::police::{Face, Polices};
use crate::pdf::rendu::{Dessin, PAGE_H, PAGE_L};

// ── Géométrie ─────────────────────────────────────────────────────────────────
const MARGE: f32 = 19.0;
const COL: f32 = PAGE_L - 2.0 * MARGE;

/// Largeurs des colonnes de chiffres de la grille, en points : base · taux ·
/// à déduire · à payer · puis base, taux et montant des charges patronales. Le
/// libellé prend le reste.
const CHIFFRES_GRILLE: [f32; 7] = [50.5, 42.3, 49.2, 50.5, 47.3, 41.0, 48.6];
/// Les charges patronales occupent les trois dernières colonnes sous un seul
/// titre, sans filet entre elles.
const N_PAT: usize = 3;
/// Retrait des libellés dans leur colonne.
const RETRAIT: f32 = 30.0;

/// Pied : intitulé de ligne puis neuf cumuls. Proportions relevées sur le
/// modèle, ramenées à la largeur utile par `pied()`.
const PIED_PROPORTIONS: [f32; 10] = [55.0, 87.0, 90.0, 90.0, 90.0, 90.0, 88.0, 88.0, 88.0, 116.0];
/// L'encadré du net payé commence à la colonne du coût global.
const PIED_NET_DE: usize = 7;

/// L'annexe détaille les cotisations seules : base · [taux · montant] × 2.
const CHIFFRES_ANNEXE: [f32; 5] = [68.0, 50.0, 74.0, 50.0, 74.0];

/// Haut de la grille sur la première page (sous l'en-tête) et sur les suivantes.
const GRILLE_HAUT: f32 = 231.0;
const GRILLE_HAUT_SUITE: f32 = 44.0;
const H_ENTETE: f32 = 26.0;
/// Bas de la grille = haut du pied, sur toutes les pages : la grille garde la
/// même hauteur, que le pied soit imprimé ou non.
const GRILLE_BAS: f32 = 736.0;
const H_PIED: f32 = 13.0;

// ── Échelle typographique ─────────────────────────────────────────────────────
const T_EMPLOYEUR: f32 = 8.6;
const T_TITRE: f32 = 15.0;
const T_TEXTE: f32 = 7.6;
const T_ENTETE: f32 = 8.0;
const T_LIGNE: f32 = 7.2;
const T_GRAND: f32 = 11.6;
const T_NOTE: f32 = 6.6;
const T_PIED: f32 = 7.0;
const T_NET: f32 = 12.0;
const T_MENTION: f32 = 6.4;
const T_FILIGRANE: f32 = 74.0;
const ANGLE_FILIGRANE: f32 = 52.0;
const ALPHA_FILIGRANE: f32 = 0.10;

const H_LIGNE: f32 = 9.9;
const H_GRAND: f32 = 16.0;

const NOIR: f32 = 0.0;
const GRIS: f32 = 0.40;
const BLANC: f32 = 1.0;
const BLEU: [f32; 3] = [0.05, 0.07, 0.55];
const BLEU_PALE: [f32; 3] = [0.84, 0.89, 0.98];
const EP_CADRE: f32 = 0.6;
const EP_FILET: f32 = 0.45;

/// Largeurs complètes d'une grille : le libellé d'abord, qui prend le reste.
fn grille(chiffres: &[f32]) -> Vec<f32> {
    let mut l = vec![COL - chiffres.iter().sum::<f32>()];
    l.extend_from_slice(chiffres);
    l
}

fn pied() -> Vec<f32> {
    let total: f32 = PIED_PROPORTIONS.iter().sum();
    PIED_PROPORTIONS.iter().map(|p| p * COL / total).collect()
}

/// Abscisses des bords droits de chaque colonne.
fn bords(largeurs: &[f32]) -> Vec<f32> {
    let mut x = MARGE;
    largeurs.iter().map(|l| { x += l; x }).collect()
}

struct Composeur<'a> {
    p: &'a Polices,
    pages: Vec<Vec<Dessin>>,
    cur: Vec<Dessin>,
    y: f32,
}

impl<'a> Composeur<'a> {
    fn ecrire(&mut self, x: f32, y: f32, texte: &str, face: Face, taille: f32, gris: f32) {
        if texte.is_empty() {
            return;
        }
        self.cur.push(Dessin::Texte { x, y, texte: texte.to_string(), face, taille, gris });
    }

    fn ecrire_droite(&mut self, droite: f32, y: f32, texte: &str, face: Face, taille: f32, gris: f32) {
        let l = self.p.largeur(face, texte, taille);
        self.ecrire(droite - l, y, texte, face, taille, gris);
    }

    fn ecrire_centre(&mut self, gauche: f32, droite: f32, y: f32, texte: &str, face: Face, taille: f32, gris: f32) {
        let l = self.p.largeur(face, texte, taille);
        self.ecrire(gauche + (droite - gauche - l) / 2.0, y, texte, face, taille, gris);
    }

    fn filet_h(&mut self, y: f32, x1: f32, x2: f32, ep: f32) {
        self.cur.push(Dessin::Filet { x1, y1: y, x2, y2: y, ep, gris: NOIR });
    }

    fn filet_v(&mut self, x: f32, y1: f32, y2: f32, ep: f32) {
        self.cur.push(Dessin::Filet { x1: x, y1, x2: x, y2, ep, gris: NOIR });
    }

    fn aplat(&mut self, x: f32, y: f32, l: f32, h: f32, rvb: [f32; 3]) {
        self.cur.push(Dessin::Aplat { x, y, l, h, rvb });
    }

    fn cadre(&mut self, x: f32, y: f32, l: f32, h: f32, ep: f32) {
        self.filet_h(y, x, x + l, ep);
        self.filet_h(y + h, x, x + l, ep);
        self.filet_v(x, y, y + h, ep);
        self.filet_v(x + l, y, y + h, ep);
    }

    /// Découpe un texte en lignes tenant dans `largeur`. Un mot plus large que la
    /// colonne n'est pas coupé : il déborde plutôt que de devenir illisible.
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

    fn page_suivante(&mut self) {
        self.pages.push(std::mem::take(&mut self.cur));
    }

    // ── En-tête de la première page ───────────────────────────────────────────

    fn entete(&mut self, b: &BulletinPdf) {
        // Employeur, en haut à gauche.
        self.ecrire(MARGE + 4.0, 27.0, &b.employeur, Face::Gras, T_EMPLOYEUR, NOIR);
        for (i, l) in b.employeur_adresse.iter().enumerate() {
            self.ecrire(MARGE + 4.0, 35.5 + i as f32 * 8.0, l, Face::Regulier, T_TEXTE, NOIR);
        }

        // Titre et période, sur la moitié droite.
        self.ecrire(333.0, 36.0, &b.titre, Face::Gras, T_TITRE, NOIR);
        if !b.periode.is_empty() {
            let lbl = "Période : ";
            self.ecrire(336.0, 50.0, lbl, Face::Regulier, T_TEXTE, NOIR);
            let x = 336.0 + self.p.largeur(Face::Regulier, lbl, T_TEXTE);
            self.ecrire(x, 50.0, &b.periode, Face::Gras, T_TEXTE, NOIR);
        }
        if !b.reference.is_empty() {
            self.ecrire_droite(MARGE + COL, 28.0, &b.reference, Face::Regulier, T_NOTE, NOIR);
        }

        // Identifiants de l'employeur : couples à la suite sur une même ligne.
        for (i, rang) in b.identifiants.iter().enumerate() {
            let y = 82.0 + i as f32 * 8.6;
            let mut x = MARGE + 4.0;
            for c in rang {
                let lbl = format!("{} : ", c.l);
                self.ecrire(x, y, &lbl, Face::Regulier, T_TEXTE, NOIR);
                x += self.p.largeur(Face::Regulier, &lbl, T_TEXTE);
                self.ecrire(x, y, &c.v, Face::Gras, T_TEXTE, NOIR);
                x += self.p.largeur(Face::Gras, &c.v, T_TEXTE) + 9.0;
            }
        }

        // Blocs du salarié : libellés au fer à droite d'une même abscisse,
        // valeurs en gras juste après.
        let axe = MARGE + 88.0;
        let mut y = 101.0;
        for bloc in &b.blocs {
            for c in bloc {
                self.ecrire_droite(axe, y, &format!("{} :", c.l), Face::Regulier, T_TEXTE, NOIR);
                self.ecrire(axe + 3.0, y, &c.v, Face::Gras, T_TEXTE, NOIR);
                y += 8.0;
            }
            y += 10.0;
        }
        if let Some(c) = &b.convention {
            let y = y.max(206.0) + 2.0;
            let lbl = format!("{} : ", c.l);
            let x = MARGE + 12.0;
            self.ecrire(x, y, &lbl, Face::Regulier, T_TEXTE, NOIR);
            let xv = x + self.p.largeur(Face::Regulier, &lbl, T_TEXTE);
            self.ecrire(xv, y, &c.v, Face::Gras, T_TEXTE, NOIR);
        }

        // Cadre d'adresse, placé pour la fenêtre d'une enveloppe.
        let (cx, cy, cl, ch) = (298.0, 115.0, 240.0, 84.0);
        self.aplat(cx, cy, cl, ch, BLEU_PALE);
        let n = b.destinataire.len() as f32;
        let y0 = cy + ch / 2.0 - (n - 1.0) * 10.6 / 2.0 + 2.5;
        for (i, l) in b.destinataire.iter().enumerate() {
            self.ecrire(cx + 17.0, y0 + i as f32 * 10.6, l, Face::Gras, T_TEXTE + 0.4, NOIR);
        }

        // Repères de pliage, dans les marges.
        for x in [2.0, PAGE_L - 12.0] {
            self.filet_h(291.0, x, x + 10.0, 0.5);
        }
    }

    // ── La grille ─────────────────────────────────────────────────────────────

    /// Cadre, en-tête bleu et filets verticaux d'une page de grille, de `haut`
    /// jusqu'au pied.
    fn cadre_grille(&mut self, haut: f32, colonnes: &[String]) {
        let l = grille(&CHIFFRES_GRILLE);
        let b = bords(&l);
        self.aplat(MARGE, haut, COL, H_ENTETE, BLEU);
        let y = haut + H_ENTETE / 2.0 + T_ENTETE * 0.35;
        // Libellé, base, taux, à déduire, à payer : une colonne chacun ; le
        // sixième titre couvre les trois colonnes des charges patronales.
        let n = l.len() - N_PAT + 1;
        for (i, t) in colonnes.iter().enumerate().take(n) {
            let gauche = if i == 0 { MARGE } else { b[i - 1] };
            let droite = if i + 1 == n { MARGE + COL } else { b[i] };
            self.ecrire_centre(gauche, droite, y, t, Face::Gras, T_ENTETE, BLANC);
        }
        self.cadre(MARGE, haut, COL, GRILLE_BAS - haut, EP_CADRE);
        for x in &b[..n - 1] {
            self.filet_v(*x, haut, GRILLE_BAS, EP_FILET);
        }
    }

    fn hauteur_ligne(&self, l: &Ligne) -> f32 {
        if l.vide {
            return H_LIGNE * 0.6;
        }
        if l.grand {
            return H_GRAND;
        }
        let larg = grille(&CHIFFRES_GRILLE)[0] - RETRAIT - 4.0;
        let (face, taille) = Self::style(l);
        self.decouper(&l.libelle, face, taille, larg).len().max(1) as f32 * H_LIGNE
    }

    fn style(l: &Ligne) -> (Face, f32) {
        if l.grand {
            (Face::Gras, T_GRAND)
        } else if l.note {
            (Face::Italique, T_NOTE)
        } else if l.fort {
            (Face::Gras, T_LIGNE)
        } else {
            (Face::Regulier, T_LIGNE)
        }
    }

    fn ligne(&mut self, l: &Ligne) {
        let h = self.hauteur_ligne(l);
        let haut = self.y;
        self.y += h;
        if l.vide {
            return;
        }
        let largeurs = grille(&CHIFFRES_GRILLE);
        let b = bords(&largeurs);
        let (face, taille) = Self::style(l);
        let gris = if l.note { GRIS } else { NOIR };

        let x_lbl = MARGE + RETRAIT;
        let larg = largeurs[0] - RETRAIT - 4.0;
        let lignes = self.decouper(&l.libelle, face, taille, larg);
        let base_y = |i: usize| haut + (i as f32 + 1.0) * if l.grand { H_GRAND } else { H_LIGNE } - 2.8;
        for (i, t) in lignes.iter().enumerate() {
            if l.centre {
                self.ecrire_centre(x_lbl, b[0] - 4.0, base_y(i), t, face, taille, gris);
            } else {
                self.ecrire(x_lbl, base_y(i), t, face, taille, gris);
            }
        }
        // Les montants s'alignent sur la DERNIÈRE ligne d'un libellé coupé.
        let y = base_y(lignes.len() - 1);
        // Le net à payer avant impôt s'imprime en grand ; ses voisins non.
        let (fv, tv) = if l.grand { (Face::Gras, T_GRAND) } else if l.fort { (Face::Gras, T_LIGNE) } else { (Face::Regulier, T_LIGNE) };
        let cols = [&l.base, &l.taux, &l.a_deduire, &l.a_payer, &l.base_pat, &l.taux_pat, &l.montant_pat];
        for (i, v) in cols.iter().enumerate() {
            self.ecrire_droite(b[i + 1] - 4.0, y, v, fv, tv, NOIR);
        }
    }

    fn grille(&mut self, b: &BulletinPdf) {
        let colonnes = if b.colonnes.is_empty() { vec![String::new(); 6] } else { b.colonnes.clone() };
        let mut haut = GRILLE_HAUT;
        self.cadre_grille(haut, &colonnes);
        self.y = haut + H_ENTETE + 6.0;
        let limite = GRILLE_BAS - 4.0;

        for l in &b.lignes {
            let h = self.hauteur_ligne(l);
            if self.y + h > limite {
                // Une ligne blanche ne justifie pas une page : on l'omet.
                if l.vide {
                    continue;
                }
                self.ecrire_centre(MARGE, MARGE + COL, GRILLE_BAS + 14.0, "Suite page suivante",
                    Face::Italique, T_PIED, GRIS);
                self.page_suivante();
                haut = GRILLE_HAUT_SUITE;
                self.titre_suite(b);
                self.cadre_grille(haut, &colonnes);
                self.y = haut + H_ENTETE + 6.0;
            }
            self.ligne(l);
        }
    }

    /// En-tête réduit des pages de suite : la période, pour qu'une feuille isolée
    /// se rattache à son bulletin.
    fn titre_suite(&mut self, b: &BulletinPdf) {
        self.ecrire(MARGE + 4.0, 30.0, &b.employeur, Face::Gras, T_TEXTE, NOIR);
        let t = format!("{} — Période : {} (suite)", b.titre, b.periode);
        self.ecrire_droite(MARGE + COL, 30.0, &t, Face::Regulier, T_TEXTE, NOIR);
    }

    // ── Le pied ───────────────────────────────────────────────────────────────

    fn rangee_entete(&mut self, y: f32, titres: &[String], larg: &[f32], jusqua: usize) {
        let b = bords(larg);
        self.aplat(MARGE, y, COL, H_PIED, BLEU);
        for (i, t) in titres.iter().enumerate().take(jusqua) {
            let gauche = if i == 0 { MARGE } else { b[i - 1] };
            self.ecrire_centre(gauche, b[i], y + H_PIED - 3.6, t, Face::Regulier, T_PIED, BLANC);
        }
        for x in &b[..b.len() - 1] {
            self.filet_v(*x, y, y + H_PIED, 0.4);
        }
    }

    fn rangee(&mut self, y: f32, l: &LignePied, larg: &[f32], jusqua: usize) {
        let b = bords(larg);
        let base = y + H_PIED - 3.6;
        self.ecrire(MARGE + 3.0, base, &l.l, Face::Regulier, T_PIED, NOIR);
        for (i, v) in l.v.iter().enumerate().take(jusqua.saturating_sub(1)) {
            self.ecrire_droite(b[i + 1] - 6.0, base, v, Face::Regulier, T_PIED, NOIR);
        }
        self.filet_h(y + H_PIED, MARGE, b[jusqua - 1], 0.4);
        for x in &b[..jusqua] {
            self.filet_v(*x, y, y + H_PIED, 0.4);
        }
        self.filet_v(MARGE, y, y + H_PIED, 0.4);
    }

    fn pied(&mut self, b: &BulletinPdf) {
        let larg = pied();
        let n = larg.len();
        let mut y = GRILLE_BAS;

        self.rangee_entete(y, &b.pied_entetes, &larg, n);
        y += H_PIED;
        for l in &b.pied_lignes {
            self.rangee(y, l, &larg, n);
            y += H_PIED;
        }

        // Compteurs de congés à gauche ; à droite, l'encadré du net payé.
        let haut_conges = y;
        self.rangee_entete(y, &b.conges_entetes, &larg, b.conges_entetes.len());
        y += H_PIED;
        for l in &b.conges_lignes {
            self.rangee(y, l, &larg, PIED_NET_DE);
            y += H_PIED;
        }
        let bx = bords(&larg);
        let x_net = bx[PIED_NET_DE - 1];
        let l_net = MARGE + COL - x_net;
        let haut_net = haut_conges + H_PIED;
        let bas_net = y;
        self.cadre(x_net, haut_net, l_net, bas_net - haut_net, 0.4);
        let separation = bas_net - H_PIED;
        self.filet_h(separation, x_net, x_net + l_net, 0.4);
        self.ecrire_centre(x_net, x_net + l_net, (haut_net + separation) / 2.0 + T_NET * 0.35,
            &b.net_paye, Face::Gras, T_NET, NOIR);
        self.ecrire_centre(x_net, x_net + l_net, bas_net - 3.6, &b.paiement, Face::Regulier, T_NOTE, NOIR);

        if !b.mention.is_empty() {
            let lignes = self.decouper(&b.mention, Face::Regulier, T_MENTION, COL);
            for (i, l) in lignes.iter().enumerate() {
                self.ecrire_centre(MARGE, MARGE + COL, y + 8.0 + i as f32 * 7.5, l, Face::Regulier, T_MENTION, NOIR);
            }
        }
    }

    // ── Annexe ────────────────────────────────────────────────────────────────

    fn annexe(&mut self, a: &Annexe) {
        self.page_suivante();
        let bas = PAGE_H - 30.0;
        self.y = 40.0;
        self.ecrire(MARGE, self.y, &a.titre, Face::Gras, 12.0, NOIR);
        self.filet_h(self.y + 4.0, MARGE, MARGE + COL, 0.8);
        self.y += 16.0;

        for para in &a.chapeau {
            for l in self.decouper(para, Face::Regulier, T_LIGNE, COL) {
                if self.y > bas {
                    self.page_suivante();
                    self.y = 40.0;
                }
                self.ecrire(MARGE, self.y, &l, Face::Regulier, T_LIGNE, NOIR);
                self.y += T_LIGNE + 2.6;
            }
            self.y += 4.0;
        }
        self.y += 4.0;

        let larg = grille(&CHIFFRES_ANNEXE);
        let b = bords(&larg);
        self.entete_annexe(a, &larg);
        for l in &a.lignes {
            // Le libellé laisse la place du code, logé au fer à droite de sa colonne ;
            // la référence légale, dessous, ne franchit pas le filet de la base.
            let l_code = self.p.largeur(Face::Italique, &l.code, T_NOTE);
            let lbl = self.decouper(&l.libelle, Face::Regulier, T_LIGNE, larg[0] - 14.0 - l_code);
            let refs = if l.reference.is_empty() {
                Vec::new()
            } else {
                self.decouper(&l.reference, Face::Italique, T_NOTE, larg[0] - 14.0)
            };
            let h = H_LIGNE * lbl.len() as f32 + refs.len() as f32 * (T_NOTE + 2.0) + 2.0;
            if self.y + h > bas {
                self.page_suivante();
                self.y = 40.0;
                self.entete_annexe(a, &larg);
            }
            let haut = self.y;
            for (i, t) in lbl.iter().enumerate() {
                self.ecrire(MARGE + 4.0, haut + H_LIGNE - 2.8 + i as f32 * H_LIGNE, t, Face::Regulier, T_LIGNE, NOIR);
            }
            let y = haut + H_LIGNE * lbl.len() as f32 - 2.8;
            // Le code se loge au fer à droite de la colonne du libellé, en grisé.
            self.ecrire_droite(b[0] - 4.0, haut + H_LIGNE - 2.8, &l.code, Face::Italique, T_NOTE, GRIS);
            let cols = [&l.base, &l.taux_sal, &l.montant_sal, &l.taux_pat, &l.montant_pat];
            for (i, v) in cols.iter().enumerate() {
                self.ecrire_droite(b[i + 1] - 4.0, y, v, Face::Regulier, T_LIGNE, NOIR);
            }
            for (i, r) in refs.iter().enumerate() {
                let yy = haut + H_LIGNE * lbl.len() as f32 + (i as f32 + 1.0) * (T_NOTE + 2.0) - 1.0;
                self.ecrire(MARGE + 8.0, yy, r, Face::Italique, T_NOTE, GRIS);
            }
            for x in &b[..b.len() - 1] {
                self.filet_v(*x, haut, haut + h, 0.3);
            }
            self.filet_h(haut + h, MARGE, MARGE + COL, 0.3);
            self.y = haut + h;
        }
    }

    fn entete_annexe(&mut self, a: &Annexe, larg: &[f32]) {
        let b = bords(larg);
        let haut = self.y;
        let h_groupe = if a.groupes.is_empty() { 0.0 } else { 11.0 };
        let h = h_groupe + 14.0;
        self.aplat(MARGE, haut, COL, h, BLEU);
        for g in a.groupes.iter().filter(|g: &&Groupe| g.de >= 1 && g.de <= g.a && g.a < b.len()) {
            self.ecrire_centre(b[g.de - 1], b[g.a], haut + 8.5, &g.titre, Face::Gras, T_PIED, BLANC);
        }
        let y = haut + h - 4.0;
        for (i, t) in a.colonnes.iter().enumerate().take(b.len()) {
            if i == 0 {
                self.ecrire(MARGE + 4.0, y, t, Face::Gras, T_PIED, BLANC);
            } else {
                self.ecrire_droite(b[i] - 4.0, y, t, Face::Gras, T_PIED, BLANC);
            }
        }
        self.y = haut + h;
    }
}

/// Compose le bulletin. Rend une page par élément.
pub fn composer(b: &BulletinPdf, p: &Polices) -> Vec<Vec<Dessin>> {
    let mut co = Composeur { p, pages: Vec::new(), cur: Vec::new(), y: 0.0 };
    co.entete(b);
    co.grille(b);
    co.pied(b);
    if let Some(a) = &b.annexe {
        if !a.lignes.is_empty() || !a.chapeau.is_empty() {
            co.annexe(a);
        }
    }
    co.page_suivante();
    habiller(&mut co.pages, b, p);
    co.pages
}

/// Filigrane et folio, ajoutés une fois le nombre de pages connu.
fn habiller(pages: &mut [Vec<Dessin>], b: &BulletinPdf, p: &Polices) {
    let total = pages.len();
    for (i, page) in pages.iter_mut().enumerate() {
        // Le filigrane passe PAR-DESSUS, en diagonale et translucide : les
        // aplats bleus le mangeraient par morceaux s'il passait derrière.
        if !b.filigrane.is_empty() {
            let large = p.largeur(Face::Gras, &b.filigrane, T_FILIGRANE);
            let rad = ANGLE_FILIGRANE.to_radians();
            page.push(Dessin::Filigrane {
                x: PAGE_L / 2.0 - large / 2.0 * rad.cos(),
                y: PAGE_H / 2.0 + large / 2.0 * rad.sin(),
                texte: b.filigrane.clone(),
                face: Face::Gras,
                taille: T_FILIGRANE,
                angle: ANGLE_FILIGRANE,
                gris: 0.0,
                alpha: ALPHA_FILIGRANE,
            });
        }
        if total > 1 {
            let folio = format!("Page {} / {total}", i + 1);
            let l = p.largeur(Face::Regulier, &folio, T_NOTE);
            page.push(Dessin::Texte {
                x: MARGE + COL - l,
                y: 14.0,
                texte: folio,
                face: Face::Regulier,
                taille: T_NOTE,
                gris: GRIS,
            });
        }
    }
}

/// Exposées pour les tests : les grilles doivent tenir dans la largeur utile.
/// Rend la largeur utile, les largeurs de la grille, du pied et de l'annexe,
/// la marge, et le bas de la grille.
pub fn metriques() -> (f32, [Vec<f32>; 3], f32, f32) {
    (COL, [grille(&CHIFFRES_GRILLE), pied(), grille(&CHIFFRES_ANNEXE)], MARGE, GRILLE_BAS)
}

