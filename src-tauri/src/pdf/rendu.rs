//! Le dernier étage, commun au contrat de travail et au bulletin de paie : des
//! tracés absolus deviennent des opérateurs PDF.
//!
//! Rien ici ne connaît le métier. Un module de composition produit une suite de
//! pages, chacune décrite par des `Dessin` ; `rendre` les traduit et rend les
//! octets. C'est la frontière entre la géométrie et le format de fichier.
//!
//! Chaque fragment de texte est émis dans sa propre section `BT … ET`. C'est
//! volontairement bavard, mais `BT` remet la matrice de texte à l'identité : le
//! curseur qui suit est donc une position **absolue**, et non un déplacement
//! relatif à la ligne précédente. Le placement calculé par le moteur est ainsi
//! reproduit tel quel, sans arithmétique de report.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use printpdf::{
    Color, ExtendedGraphicsState, FontId, Line, LinePoint, Mm, Op, PdfDocument, PdfFontHandle,
    PdfPage, PdfSaveOptions, Point, Pt, Rgb, TextItem, TextMatrix,
};

use crate::pdf::police::{Face, Polices};

// ── Géométrie de la page ──────────────────────────────────────────────────────
// Un point PDF vaut 1/72 de pouce ; A4 fait 210 × 297 mm.
pub const MM: f32 = 72.0 / 25.4;
pub const PAGE_L: f32 = 210.0 * MM;
pub const PAGE_H: f32 = 297.0 * MM;

/// Un tracé, dans le repère de la lecture : `y` est mesuré **depuis le haut de la
/// page**. La conversion vers le repère PDF (origine en bas à gauche) est faite
/// ici et nulle part ailleurs.
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
    /// Aplat plein — bandeaux de rubrique et lignes de total du bulletin.
    /// Tracé avant les textes de la même page : c'est l'ordre de la liste qui
    /// fait l'empilement, il n'y a pas de notion de plan.
    Pave {
        x: f32,
        y: f32,
        l: f32,
        h: f32,
        gris: f32,
    },
    /// Filigrane : texte tourné et TRANSLUCIDE, tracé PAR-DESSUS le reste.
    ///
    /// Il ne pouvait pas passer derrière : les aplats des bandeaux de rubrique
    /// et des lignes de total l'auraient mangé par morceaux, et un filigrane à
    /// moitié dévoré ressemble à un défaut d'impression, pas à une mise en
    /// garde. Il passe donc au-dessus, avec un canal alpha — c'est le seul
    /// tracé du moteur qui touche à l'état graphique étendu.
    Filigrane {
        /// Point d'ancrage de la ligne de base, avant rotation.
        x: f32,
        y: f32,
        texte: String,
        face: Face,
        taille: f32,
        /// Degrés, sens trigonométrique.
        angle: f32,
        gris: f32,
        /// 0 = invisible, 1 = opaque.
        alpha: f32,
    },
}

/// Rend les octets du PDF et le nombre de pages.
///
/// Tout se joue en mémoire : rien n'est écrit sur disque, rien n'est enregistré
/// en base, et le contenu du document n'est pas journalisé.
///
/// Seules les fontes réellement tracées sont incluses : sans sous-ensemblage
/// (cf. `Cargo.toml`), chaque face embarquée pèse quelque 400 Ko, et il serait
/// absurde qu'un bulletin en linéale traîne les trois romaines du contrat.
pub fn rendre(pages: &[Vec<Dessin>], titre: &str, p: &Polices) -> Result<(Vec<u8>, usize), String> {
    let mut doc = PdfDocument::new(titre);

    let utilisee = |f: &Face| {
        pages.iter().flatten().any(|d| match d {
            Dessin::Texte { face, .. } | Dessin::Filigrane { face, .. } => face == f,
            _ => false,
        })
    };
    let utilisees: Vec<Face> = Face::TOUTES.into_iter().filter(utilisee).collect();

    // Un seul état graphique étendu par niveau d'opacité rencontré : les
    // filigranes d'un même document en partagent un.
    let mut alphas: Vec<(u8, printpdf::ExtendedGraphicsStateId)> = Vec::new();
    for d in pages.iter().flatten() {
        if let Dessin::Filigrane { alpha, .. } = d {
            let cle = (alpha.clamp(0.0, 1.0) * 100.0).round() as u8;
            if !alphas.iter().any(|(c, _)| *c == cle) {
                let gs = ExtendedGraphicsState::default()
                    .with_current_fill_alpha(f32::from(cle) / 100.0)
                    .with_current_stroke_alpha(f32::from(cle) / 100.0);
                let id = doc.add_graphics_state(gs);
                alphas.push((cle, id));
            }
        }
    }
    let ids: Vec<(Face, FontId)> = utilisees
        .into_iter()
        .map(|f| (f, doc.add_font(p.face(f))))
        .collect();
    let id = |f: Face| -> Option<&FontId> {
        ids.iter().find(|(g, _)| *g == f).map(|(_, i)| i)
    };

    let pages_pdf: Vec<PdfPage> = pages
        .iter()
        .map(|dessins| {
            let mut ops = Vec::with_capacity(dessins.len() * 6);
            for d in dessins {
                match d {
                    Dessin::Texte { x, y, texte, face, taille, gris } => {
                        let Some(font) = id(*face) else { continue };
                        ops.push(Op::StartTextSection);
                        ops.push(Op::SetFillColor { col: gris_en_couleur(*gris) });
                        ops.push(Op::SetFont {
                            font: PdfFontHandle::External(font.clone()),
                            size: Pt(*taille),
                        });
                        ops.push(Op::SetTextCursor {
                            pos: Point { x: Pt(*x), y: Pt(PAGE_H - *y) },
                        });
                        ops.push(Op::ShowText { items: vec![TextItem::Text(texte.clone())] });
                        ops.push(Op::EndTextSection);
                    }
                    Dessin::Filet { x1, y1, x2, y2, ep, gris } => {
                        ops.push(Op::SetOutlineColor { col: gris_en_couleur(*gris) });
                        ops.push(Op::SetOutlineThickness { pt: Pt(*ep) });
                        ops.push(Op::DrawLine {
                            line: Line {
                                points: vec![point(*x1, *y1), point(*x2, *y2)],
                                is_closed: false,
                            },
                        });
                    }
                    Dessin::Pave { x, y, l, h, gris } => {
                        ops.push(Op::SetFillColor { col: gris_en_couleur(*gris) });
                        ops.push(Op::DrawPolygon {
                            polygon: printpdf::Polygon {
                                rings: vec![printpdf::PolygonRing {
                                    points: vec![
                                        point(*x, *y),
                                        point(*x + *l, *y),
                                        point(*x + *l, *y + *h),
                                        point(*x, *y + *h),
                                    ],
                                }],
                                mode: printpdf::PaintMode::Fill,
                                winding_order: printpdf::WindingOrder::NonZero,
                            },
                        });
                        // Le remplissage courant reste actif pour l'opérateur
                        // suivant : on rend la main en noir, sinon le premier
                        // texte tracé après un pavé en hériterait le gris.
                        ops.push(Op::SetFillColor { col: gris_en_couleur(0.0) });
                    }
                    Dessin::Filigrane { x, y, texte, face, taille, angle, gris, alpha } => {
                        let Some(font) = id(*face) else { continue };
                        let cle = (alpha.clamp(0.0, 1.0) * 100.0).round() as u8;
                        let Some((_, gs)) = alphas.iter().find(|(c, _)| *c == cle) else { continue };
                        // L'état graphique est empilé puis dépilé : l'opacité ne
                        // doit fuir sur aucun tracé suivant.
                        ops.push(Op::SaveGraphicsState);
                        ops.push(Op::LoadGraphicsState { gs: gs.clone() });
                        ops.push(Op::StartTextSection);
                        ops.push(Op::SetFillColor { col: gris_en_couleur(*gris) });
                        ops.push(Op::SetFont {
                            font: PdfFontHandle::External(font.clone()),
                            size: Pt(*taille),
                        });
                        ops.push(Op::SetTextMatrix {
                            matrix: TextMatrix::TranslateRotate(Pt(*x), Pt(PAGE_H - *y), *angle),
                        });
                        ops.push(Op::ShowText { items: vec![TextItem::Text(texte.clone())] });
                        ops.push(Op::EndTextSection);
                        ops.push(Op::RestoreGraphicsState);
                    }
                }
            }
            PdfPage::new(Mm(PAGE_L / MM), Mm(PAGE_H / MM), ops)
        })
        .collect();

    let n = pages_pdf.len();
    doc.with_pages(pages_pdf);

    let mut avertissements = Vec::new();
    let octets = doc.save(&PdfSaveOptions::default(), &mut avertissements);
    if octets.is_empty() {
        return Err("le moteur PDF n'a produit aucun octet".into());
    }
    Ok((octets, n))
}

/// Encodage prêt à traverser le pont Tauri comme la route HTTP : les deux
/// renvoient exactement la même forme.
pub fn rendre_base64(
    pages: &[Vec<Dessin>],
    titre: &str,
    p: &Polices,
) -> Result<(String, usize), String> {
    let (octets, n) = rendre(pages, titre, p)?;
    Ok((STANDARD.encode(octets), n))
}

fn gris_en_couleur(g: f32) -> Color {
    // Même convention des deux côtés : 0 = noir, 1 = blanc.
    // Les aplats passent par du RVB : certains lecteurs rendent mal un
    // remplissage en niveaux de gris sans profil ICC déclaré.
    let v = g.clamp(0.0, 1.0);
    Color::Rgb(Rgb { r: v, g: v, b: v, icc_profile: None })
}

fn point(x: f32, y: f32) -> LinePoint {
    LinePoint { p: Point { x: Pt(x), y: Pt(PAGE_H - y) }, bezier: false }
}
