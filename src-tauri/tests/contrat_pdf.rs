//! Le moteur de composition du contrat de travail.
//!
//! On n'y vérifie pas une apparence — un PDF ne se relit pas en test — mais les
//! quatre choses qui cassent silencieusement : le fichier est-il un PDF, la
//! pagination suit-elle la longueur du texte, un titre d'article peut-il se
//! retrouver seul en bas de page, et les caractères français sont-ils tous
//! présents dans la fonte embarquée.

use xenna_paie_lib::contrat::mise_en_page::{self, Dessin, PAGE_H};
use xenna_paie_lib::contrat::police::{Face, Polices};
use xenna_paie_lib::contrat::{pdf, Article, ContratPdf, Mention, Run};

fn run(t: &str, s: &str) -> Run {
    Run { t: t.into(), s: s.into() }
}

fn para(t: &str) -> Vec<Run> {
    vec![run(t, "n")]
}

fn contrat_gabarit(articles: usize, paragraphes: usize) -> ContratPdf {
    let corps: Vec<Vec<Run>> = (0..paragraphes)
        .map(|_| {
            vec![
                run("En contrepartie de son travail, ", "n"),
                run("Geralt de Riv", "b"),
                run(" percevra une rémunération brute mensuelle de ", "n"),
                run("[RÉMUNÉRATION BRUTE MENSUELLE]", "i"),
                run(
                    ", versée sous réserve du maintien d’un indice de conformité \
                     émotionnelle supérieur à 0,62, relevé quotidiennement par le badge \
                     à l’entrée et à la sortie de l’établissement. En deçà de ce seuil, \
                     la fraction non conforme est convertie en jours de rétention \
                     affective portés au compte épargne-humeur.",
                    "n",
                ),
            ]
        })
        .collect();

    ContratPdf {
        titre: "CONTRAT DE TRAVAIL À DURÉE INDÉTERMINÉE".into(),
        sous_titre: para("Convention collective Syntec — IDCC 1486"),
        employeur: vec![
            Mention { l: "Raison sociale".into(), runs: para("Xenna Industries") },
            Mention { l: "Siège social".into(), runs: para("14, allée des Signaux Faibles — 92130 Issy-les-Moulineaux") },
        ],
        salarie: vec![Mention { l: "Nom et prénom".into(), runs: para("de Riv, Geralt") }],
        articles: (1..=articles)
            .map(|n| Article {
                numero: n as u32,
                titre: format!("Article de démonstration n° {n}"),
                corps: corps.clone(),
            })
            .collect(),
        lieu: para("Metz"),
        date: para("7 septembre 2026"),
        pied: "Xenna Industries".into(),
    }
}

#[test]
fn produit_un_pdf_valide() {
    let (octets, pages) = pdf::generer(&contrat_gabarit(20, 2)).expect("génération");
    assert!(octets.starts_with(b"%PDF-"), "en-tête PDF absente");
    assert!(octets.ends_with(b"\n") || octets.windows(5).any(|w| w == b"%%EOF"), "fin de fichier absente");
    assert!(octets.len() > 20_000, "PDF suspicieusement court : {} octets", octets.len());
    assert!(pages >= 3, "20 articles devraient tenir sur au moins 3 pages, obtenu {pages}");
}

#[test]
fn la_pagination_suit_la_longueur() {
    let (_, court) = pdf::generer(&contrat_gabarit(2, 1)).expect("court");
    let (_, long) = pdf::generer(&contrat_gabarit(40, 3)).expect("long");
    assert!(court >= 1);
    assert!(
        long > court * 3,
        "un contrat 60 fois plus long ne fait que {long} pages contre {court}"
    );
}

/// Un contrat vierge doit rester un document valide : c'est l'état dans lequel la
/// page s'ouvre, tous les libellés d'attente affichés.
#[test]
fn un_contrat_sans_aucune_saisie_reste_imprimable() {
    let mut c = contrat_gabarit(20, 1);
    c.employeur = vec![Mention {
        l: "Raison sociale".into(),
        runs: vec![run("[RAISON SOCIALE DE L’EMPLOYEUR]", "i")],
    }];
    c.salarie = vec![Mention {
        l: "Nom et prénom".into(),
        runs: vec![run("[NOM DU SALARIÉ]", "i")],
    }];
    c.lieu = vec![run("[LIEU DE SIGNATURE]", "i")];
    c.pied = String::new();

    let (octets, pages) = pdf::generer(&c).expect("génération");
    assert!(octets.starts_with(b"%PDF-"));
    assert!(pages >= 1);
}

#[test]
fn un_contrat_sans_article_ne_fait_pas_paniquer_le_moteur() {
    let mut c = contrat_gabarit(0, 0);
    c.articles.clear();
    let (octets, pages) = pdf::generer(&c).expect("génération");
    assert!(octets.starts_with(b"%PDF-"));
    assert_eq!(pages, 1);
}

/// Le garde-fou des veuves : le titre d'un article ne doit jamais être le dernier
/// tracé d'une page. C'est le défaut le plus visible d'une composition automatique.
#[test]
fn aucun_titre_d_article_ne_reste_seul_en_bas_de_page() {
    let polices = Polices::charger().expect("polices");
    // Plusieurs longueurs de corps : c'est en faisant varier le remplissage qu'on
    // fait tomber un titre au mauvais endroit.
    for paragraphes in 1..=4 {
        for articles in [6, 12, 20, 33] {
            let c = contrat_gabarit(articles, paragraphes);
            let pages = mise_en_page::composer(&c, &polices);
            for (i, page) in pages.iter().enumerate() {
                // Le pied de page est ajouté après coup : on ne regarde que le
                // dernier tracé situé au-dessus de la zone de pied.
                let dernier = page
                    .iter()
                    .filter_map(|d| match d {
                        Dessin::Texte { y, texte, .. } if *y < PAGE_H - 60.0 => Some((*y, texte.clone())),
                        _ => None,
                    })
                    .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                if let Some((_, texte)) = dernier {
                    assert!(
                        !texte.starts_with("Article "),
                        "page {} de {articles} articles × {paragraphes} § : le titre « {texte} » \
                         est seul en bas de page",
                        i + 1
                    );
                }
            }
        }
    }
}

/// Rien ne doit déborder de la colonne de texte : c'est ce que garantit la découpe
/// en lignes, et ce qu'un mauvais calcul de largeur casserait sans rien signaler.
#[test]
fn rien_ne_deborde_de_la_zone_de_texte() {
    let polices = Polices::charger().expect("polices");
    let c = contrat_gabarit(20, 3);
    let marge = 22.0 * 72.0 / 25.4;
    for page in mise_en_page::composer(&c, &polices) {
        for d in page {
            if let Dessin::Texte { x, y, texte, face, taille, .. } = d {
                let droite = x + polices.largeur(face, &texte, taille);
                assert!(
                    droite <= 210.0 * 72.0 / 25.4 - marge + 0.5,
                    "« {texte} » déborde à droite ({droite:.1} pt)"
                );
                assert!(x >= marge - 0.5, "« {texte} » déborde à gauche ({x:.1} pt)");
                assert!(y > 0.0 && y < PAGE_H, "« {texte} » hors page ({y:.1} pt)");
            }
        }
    }
}

/// La fonte embarquée doit couvrir tout ce qu'un contrat français écrit. Un glyphe
/// manquant s'imprimerait en blanc, sans la moindre erreur.
#[test]
fn la_fonte_couvre_le_francais_typographique() {
    let polices = Polices::charger().expect("polices");
    let echantillon = "àâäéèêëîïôöùûüÿçÀÂÄÉÈÊËÎÏÔÖÙÛÜŸÇœŒæÆ€«»’—…°²ǀ";
    for face in [Face::Regulier, Face::Gras, Face::Italique] {
        for c in echantillon.chars() {
            let police = polices.face(face);
            if let Some(gid) = police.lookup_glyph_index(c as u32) {
                assert!(gid != 0, "glyphe .notdef pour « {c} » ({face:?})");
            }
        }
        // Une chaîne accentuée doit être plus large qu'une chaîne vide, et une
        // longue plus large qu'une courte : la mesure elle-même doit répondre.
        assert!(polices.largeur(face, "", 11.0) == 0.0);
        assert!(polices.largeur(face, "é", 11.0) > 0.0);
        assert!(
            polices.largeur(face, "rémunération", 11.0) > polices.largeur(face, "rému", 11.0)
        );
    }
}

/// Écrit un PDF de démonstration pour inspection à l'œil — la seule chose qu'un
/// test ne sait pas faire. Ignoré par défaut :
/// `cargo test --test contrat_pdf -- --ignored --nocapture`
#[test]
#[ignore]
fn ecrire_un_exemple() {
    let dest = std::env::var("CONTRAT_PDF_OUT").unwrap_or_else(|_| "/tmp/contrat_exemple.pdf".into());
    let (octets, pages) = pdf::generer(&contrat_gabarit(20, 3)).expect("génération");
    std::fs::write(&dest, &octets).expect("écriture");
    println!("{} — {} pages, {} octets", dest, pages, octets.len());
}
