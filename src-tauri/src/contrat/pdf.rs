//! Assemblage final : les tracés calculés par `mise_en_page` deviennent des
//! opérateurs PDF.
//!
//! Chaque fragment de texte est émis dans sa propre section `BT … ET`. C'est
//! volontairement bavard, mais `BT` remet la matrice de texte à l'identité : le
//! curseur qui suit est donc une position **absolue**, et non un déplacement
//! relatif à la ligne précédente. Le placement calculé par le moteur est ainsi
//! reproduit tel quel, sans arithmétique de report.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use printpdf::{
    Color, FontId, Greyscale, Line, LinePoint, Mm, Op, PdfDocument, PdfFontHandle, PdfPage,
    PdfSaveOptions, Point, Pt, TextItem,
};

use crate::contrat::mise_en_page::{self, Dessin, PAGE_H, PAGE_L};
use crate::contrat::modele::ContratPdf;
use crate::contrat::police::{Face, Polices};

/// Rend les octets du PDF et le nombre de pages.
///
/// Tout se joue en mémoire : rien n'est écrit sur disque, rien n'est enregistré
/// en base, et le contenu du contrat n'est pas journalisé.
pub fn generer(c: &ContratPdf) -> Result<(Vec<u8>, usize), String> {
    let polices = Polices::charger()?;
    let pages = mise_en_page::composer(c, &polices);

    let titre = if c.titre.is_empty() { "Contrat de travail" } else { &c.titre };
    let mut doc = PdfDocument::new(titre);
    let reg = doc.add_font(polices.face(Face::Regulier));
    let gras = doc.add_font(polices.face(Face::Gras));
    let ital = doc.add_font(polices.face(Face::Italique));
    let id = |f: Face| -> &FontId {
        match f {
            Face::Regulier => &reg,
            Face::Gras => &gras,
            Face::Italique => &ital,
        }
    };

    let pages_pdf: Vec<PdfPage> = pages
        .iter()
        .map(|dessins| {
            let mut ops = Vec::with_capacity(dessins.len() * 6);
            for d in dessins {
                match d {
                    Dessin::Texte { x, y, texte, face, taille, gris } => {
                        ops.push(Op::StartTextSection);
                        ops.push(Op::SetFillColor { col: gris_en_couleur(*gris) });
                        ops.push(Op::SetFont {
                            font: PdfFontHandle::External(id(*face).clone()),
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
                                points: vec![
                                    point(*x1, *y1),
                                    point(*x2, *y2),
                                ],
                                is_closed: false,
                            },
                        });
                    }
                }
            }
            PdfPage::new(Mm(PAGE_L / (72.0 / 25.4)), Mm(PAGE_H / (72.0 / 25.4)), ops)
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
pub fn generer_base64(c: &ContratPdf) -> Result<(String, usize), String> {
    let (octets, pages) = generer(c)?;
    Ok((STANDARD.encode(octets), pages))
}

fn gris_en_couleur(g: f32) -> Color {
    // Même convention des deux côtés : 0 = noir, 1 = blanc.
    Color::Greyscale(Greyscale { percent: g.clamp(0.0, 1.0), icc_profile: None })
}

fn point(x: f32, y: f32) -> LinePoint {
    LinePoint { p: Point { x: Pt(x), y: Pt(PAGE_H - y) }, bezier: false }
}
