//! Le moteur de composition du bulletin de paie.
//!
//! Un PDF ne se relit pas en test : on n'y vérifie donc pas une apparence, mais
//! ce qui casse en silence. Une grille de six colonnes a ses propres façons de
//! mal tourner, différentes de celles d'un texte courant — d'où un fichier
//! distinct de `contrat_pdf.rs` :
//!
//!   - la somme des colonnes doit valoir la largeur utile, sinon la dernière
//!     déborde d'un demi-millimètre que personne ne verra jamais en relecture ;
//!   - un bandeau de rubrique ne doit jamais être le dernier tracé d'une page ;
//!   - l'en-tête des colonnes doit se répéter sur chaque page de grille, sans
//!     quoi les chiffres de la page 2 ne veulent plus rien dire ;
//!   - un bulletin sans cotisation — cas de plusieurs pays, et d'un brut nul —
//!     doit rester un document valide.

use xenna_paie_lib::paie_pdf::mise_en_page::{self, metriques};
use xenna_paie_lib::paie_pdf::modele::{
    Annexe, BulletinPdf, Champ, Ligne, LigneAnnexe, Rubrique, Total,
};
use xenna_paie_lib::paie_pdf::pdf;
use xenna_paie_lib::pdf::police::{Face, Polices};
use xenna_paie_lib::pdf::rendu::{Dessin, PAGE_H, PAGE_L};

fn champ(l: &str, v: &str) -> Champ {
    Champ { l: l.into(), v: v.into() }
}

fn ligne(libelle: &str) -> Ligne {
    Ligne {
        libelle: libelle.into(),
        base: "3 925,00".into(),
        taux_sal: "2,450 %".into(),
        montant_sal: "96,16".into(),
        taux_pat: "7,300 %".into(),
        montant_pat: "286,53".into(),
        fort: false,
        note: false,
    }
}

/// Un bulletin de gabarit : `rubriques` rubriques de `lignes` lignes chacune.
fn gabarit(rubriques: usize, lignes: usize) -> BulletinPdf {
    BulletinPdf {
        titre: "BULLETIN DE PAIE".into(),
        sous_titre: "Modèle adapté — art. R. 3243-2 du code du travail".into(),
        filigrane: "SPÉCIMEN".into(),
        avertissement: "SPÉCIMEN — sortie d’un simulateur de paie, sans valeur de bulletin \
                        de paie. L’employeur et ses identifiants sont fictifs."
            .into(),
        employeur: vec![
            champ("Raison sociale", "Gormenghast Logistique"),
            champ("Adresse", "14, allée des Contrevents — 78412 Lud-en-Brume"),
            champ("SIRET", "412 908 335 00047"),
            champ("Convention collective", "Porteurs de fardeaux et gardiens de seuils (IDCC 4471)"),
        ],
        salarie: vec![
            champ("Nom et prénom", "de Riv Geralt"),
            champ("Matricule", "XN-042"),
            champ("Emploi", "Contrôleur des Vents Contraires"),
            champ("Classification", "Cadre — niveau III — coefficient 410"),
        ],
        periode: vec![
            champ("Période d’emploi", "du 01 au 30 septembre 2026"),
            champ("Date de paiement", "30/09/2026"),
            champ("Mode de paiement", "Virement"),
        ],
        colonnes: [
            "Libellé", "Base", "Taux salarial", "Part salarié", "Taux patronal", "Part employeur",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
        rubriques: (0..rubriques)
            .map(|r| Rubrique {
                titre: format!("RUBRIQUE RÉGLEMENTAIRE N° {}", r + 1),
                lignes: (0..lignes)
                    .map(|l| ligne(&format!("Cotisation de démonstration {}.{}", r + 1, l + 1)))
                    .collect(),
            })
            .collect(),
        totaux: vec![
            Total { libelle: "Montant net social".into(), valeur: "3 012,44".into(), poids: 1, note:
                "Montant à déclarer pour le RSA et la prime d’activité.".into() },
            Total { libelle: "Net à payer avant impôt sur le revenu".into(), valeur: "3 012,44".into(), poids: 1, note: String::new() },
            Total { libelle: "Net imposable".into(), valeur: "3 218,90".into(), poids: 0, note: String::new() },
            Total { libelle: "NET PAYÉ EN EUROS".into(), valeur: "2 780,12".into(), poids: 2, note: String::new() },
        ],
        cumuls: vec![
            champ("Période", "septembre 2026"),
            champ("Brut du mois", "3 925,00 €"),
            champ("Net imposable du mois", "3 218,90 €"),
        ],
        mentions: vec![
            "Dans votre intérêt et pour vous aider à faire valoir vos droits, conservez ce \
             bulletin de paie sans limitation de durée."
                .into(),
            "Pour toute information complémentaire sur le bulletin de paie : www.service-public.fr"
                .into(),
        ],
        annexe: Some(Annexe {
            titre: "ANNEXE — DÉTAIL DES COTISATIONS ET CONTRIBUTIONS".into(),
            chapeau: "Le modèle réglementaire regroupe les cotisations par risque couvert. \
                      Cette annexe les redonne ligne à ligne."
                .into(),
            colonnes: [
                "Cotisation", "Base", "Taux salarial", "Part salarié", "Taux patronal",
                "Part employeur",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            lignes: (0..rubriques * lignes)
                .map(|i| LigneAnnexe {
                    libelle: format!("Cotisation de démonstration n° {}", i + 1),
                    code: format!("DEMO_{i}"),
                    base: "3 925,00".into(),
                    taux_sal: "2,450 %".into(),
                    montant_sal: "96,16".into(),
                    taux_pat: "7,300 %".into(),
                    montant_pat: "286,53".into(),
                    reference: "CSS art. L241-13, D241-7 — BOSS, chapitre 4 § 670".into(),
                })
                .collect(),
        }),
        pied: "Gormenghast Logistique".into(),
    }
}

#[test]
fn produit_un_pdf_valide() {
    let (octets, pages) = pdf::generer(&gabarit(7, 3)).expect("génération");
    assert!(octets.starts_with(b"%PDF-"), "en-tête PDF absente");
    assert!(
        octets.windows(5).any(|w| w == b"%%EOF"),
        "fin de fichier absente"
    );
    assert!(octets.len() > 20_000, "PDF suspicieusement court : {} octets", octets.len());
    // Le bulletin tient sur une page, l'annexe en occupe au moins une autre.
    assert!(pages >= 2, "bulletin + annexe devraient faire au moins 2 pages, obtenu {pages}");
}

/// La somme des six colonnes doit valoir exactement la largeur utile. C'est la
/// seule chose qui garantisse que la colonne « Part employeur » finit au bord
/// droit du cadre et non un millimètre plus loin.
#[test]
fn les_colonnes_remplissent_exactement_la_largeur_utile() {
    let (col, largeurs, _, _) = metriques();
    let somme: f32 = largeurs.iter().sum();
    assert!(
        (somme - col).abs() < 0.5,
        "les six colonnes font {somme:.1} pt pour une largeur utile de {col:.1} pt"
    );
}

/// Rien ne doit déborder du cadre, ni à droite (les montants, calés au fer à
/// droite, sont les premiers à sortir) ni à gauche, ni hors de la page.
#[test]
fn rien_ne_deborde_du_cadre() {
    let polices = Polices::charger().expect("polices");
    let (col, _, marge_g, _) = metriques();
    for (r, l) in [(3usize, 2usize), (7, 4), (12, 6)] {
        for page in mise_en_page::composer(&gabarit(r, l), &polices) {
            for d in page {
                match d {
                    // Le filigrane traverse la page en diagonale : il n'est
                    // délibérément pas soumis à la colonne de texte.
                    Dessin::Filigrane { .. } => {}
                    Dessin::Texte { x, y, texte, face, taille, .. } => {
                        let droite = x + polices.largeur(face, &texte, taille);
                        assert!(
                            droite <= marge_g + col + 0.5,
                            "« {texte} » déborde à droite ({droite:.1} pt) — {r}×{l}"
                        );
                        assert!(x >= marge_g - 0.5, "« {texte} » déborde à gauche ({x:.1} pt)");
                        assert!(y > 0.0 && y < PAGE_H, "« {texte} » hors page ({y:.1} pt)");
                    }
                    Dessin::Pave { x, l: larg, .. } => {
                        assert!(
                            x >= marge_g - 0.5 && x + larg <= marge_g + col + 0.5,
                            "un aplat déborde du cadre ({x:.1} → {:.1} pt)",
                            x + larg
                        );
                    }
                    Dessin::Filet { x1, x2, .. } => {
                        assert!(x1 >= 0.0 && x2 <= PAGE_L, "un filet sort de la page");
                    }
                }
            }
        }
    }
}

/// Le garde-fou des veuves, version grille : un bandeau de rubrique annonce des
/// lignes. S'il est le dernier tracé de la page, il annonce le vide.
#[test]
fn aucun_bandeau_de_rubrique_ne_reste_seul_en_bas_de_page() {
    let polices = Polices::charger().expect("polices");
    for lignes in 1..=5 {
        for rubriques in [4, 8, 13, 21] {
            let b = gabarit(rubriques, lignes);
            let pages = mise_en_page::composer(&b, &polices);
            for (i, page) in pages.iter().enumerate() {
                let dernier = page
                    .iter()
                    .filter_map(|d| match d {
                        // Le folio et le bandeau de pied vivent sous cette
                        // limite : on ne regarde que la zone de composition.
                        Dessin::Texte { y, texte, .. } if *y < PAGE_H - 50.0 => {
                            Some((*y, texte.clone()))
                        }
                        _ => None,
                    })
                    .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                if let Some((_, texte)) = dernier {
                    assert!(
                        !texte.starts_with("RUBRIQUE RÉGLEMENTAIRE"),
                        "page {} de {rubriques} rubriques × {lignes} lignes : le bandeau \
                         « {texte} » est seul en bas de page",
                        i + 1
                    );
                }
            }
        }
    }
}

/// Une grille qui déborde sur une deuxième page doit y remontrer ses en-têtes.
/// Sans cela, la colonne « Part salarié » de la page 2 n'est plus identifiable.
#[test]
fn l_entete_des_colonnes_se_repete_sur_chaque_page_de_grille() {
    let polices = Polices::charger().expect("polices");
    let b = gabarit(14, 5);
    let pages = mise_en_page::composer(&b, &polices);
    assert!(pages.len() >= 3, "gabarit trop court pour éprouver la répétition");

    // La dernière page est celle de l'annexe, qui a ses propres en-têtes ; on
    // regarde les pages de la grille du bulletin.
    let porte_entete = |page: &Vec<Dessin>| {
        page.iter().any(|d| matches!(d, Dessin::Texte { texte, .. } if texte == "Part salarié"))
    };
    for (i, page) in pages.iter().enumerate() {
        let a_des_lignes = page.iter().any(
            |d| matches!(d, Dessin::Texte { texte, .. } if texte.starts_with("Cotisation de démonstration")),
        );
        if a_des_lignes {
            assert!(
                porte_entete(page),
                "page {} porte des cotisations sans en-tête de colonnes",
                i + 1
            );
        }
    }
}

/// Le filigrane doit passer PAR-DESSUS — les aplats des bandeaux et des totaux
/// le mangeraient par morceaux s'il passait derrière —, être assez translucide
/// pour ne pas gêner la lecture, et se trouver sur chacune des pages : un
/// spécimen dont la deuxième page n'est pas marquée n'est pas un spécimen.
#[test]
fn le_filigrane_couvre_chaque_page_par_dessus_et_translucide() {
    let polices = Polices::charger().expect("polices");
    for (i, page) in mise_en_page::composer(&gabarit(14, 5), &polices).iter().enumerate() {
        let pos = page
            .iter()
            .position(|d| matches!(d, Dessin::Filigrane { .. }))
            .unwrap_or_else(|| panic!("page {} sans filigrane", i + 1));
        // Aucun aplat après lui : rien ne peut plus le recouvrir.
        assert!(
            !page[pos + 1..].iter().any(|d| matches!(d, Dessin::Pave { .. })),
            "page {} : un aplat est tracé après le filigrane et le mangerait",
            i + 1
        );
        let Dessin::Filigrane { texte, taille, alpha, angle, .. } = &page[pos] else {
            unreachable!()
        };
        assert_eq!(texte, "SPÉCIMEN");
        assert!(*taille > 40.0, "filigrane trop petit pour être un filigrane");
        assert!(*alpha > 0.0 && *alpha < 0.25, "opacité {alpha} : illisible ou envahissant");
        assert!(*angle > 10.0 && *angle < 80.0, "un filigrane à {angle}° n'est pas en diagonale");
    }
}

/// Un bulletin sans aucune cotisation reste un document : c'est le cas des
/// Émirats pour un expatrié, et celui d'un brut nul.
#[test]
fn un_bulletin_sans_cotisation_reste_imprimable() {
    let mut b = gabarit(0, 0);
    b.rubriques.clear();
    b.annexe = None;
    let (octets, pages) = pdf::generer(&b).expect("génération");
    assert!(octets.starts_with(b"%PDF-"));
    assert_eq!(pages, 1);
}

/// Un document vide de bout en bout ne doit pas faire paniquer le moteur : c'est
/// ce qu'on reçoit si le front envoie une structure par défaut.
#[test]
fn un_document_vide_ne_fait_pas_paniquer_le_moteur() {
    let b = BulletinPdf {
        titre: String::new(),
        sous_titre: String::new(),
        filigrane: String::new(),
        avertissement: String::new(),
        employeur: vec![],
        salarie: vec![],
        periode: vec![],
        colonnes: vec![],
        rubriques: vec![],
        totaux: vec![],
        cumuls: vec![],
        mentions: vec![],
        annexe: None,
        pied: String::new(),
    };
    let (octets, pages) = pdf::generer(&b).expect("génération");
    assert!(octets.starts_with(b"%PDF-"));
    assert_eq!(pages, 1);
}

/// La linéale embarquée doit couvrir tout ce qu'un bulletin français écrit —
/// l'euro, l'apostrophe typographique, les accents et l'exposant ordinal des
/// échelons. Un glyphe manquant s'imprimerait en blanc, sans la moindre erreur.
#[test]
fn la_fonte_lineale_couvre_le_francais_typographique() {
    let polices = Polices::charger().expect("polices");
    let echantillon = "àâäéèêëîïôöùûüÿçÀÂÄÉÈÊËÎÏÔÖÙÛÜŸÇœŒæÆ€«»’—…°²ᵉ";
    for face in [Face::Sans, Face::SansGras, Face::SansItalique] {
        for c in echantillon.chars() {
            let police = polices.face(face);
            if let Some(gid) = police.lookup_glyph_index(c as u32) {
                assert!(gid != 0, "glyphe .notdef pour « {c} » ({face:?})");
            }
        }
        assert!(polices.largeur(face, "", 7.6) == 0.0);
        assert!(polices.largeur(face, "€", 7.6) > 0.0);
        assert!(polices.largeur(face, "cotisation", 7.6) > polices.largeur(face, "coti", 7.6));
    }
}

/// Les six faces doivent être distinctes : si l'index de `Face` glissait d'un
/// cran, tout compilerait et le bulletin sortirait en romaine.
#[test]
fn les_six_faces_sont_bien_distinctes() {
    let p = Polices::charger().expect("polices");
    let mesure = |f| p.largeur(f, "Rémunération brute mensuelle", 10.0);
    let serif = mesure(Face::Regulier);
    let sans = mesure(Face::Sans);
    assert!((serif - sans).abs() > 0.5, "romaine et linéale mesurent pareil : index suspect");
    assert!(
        mesure(Face::SansGras) > mesure(Face::Sans),
        "la linéale grasse n'est pas plus large que la maigre"
    );
    assert!(
        (mesure(Face::Gras) - mesure(Face::SansGras)).abs() > 0.5,
        "les deux grasses mesurent pareil : index suspect"
    );
}

/// Écrit un PDF de démonstration pour inspection à l'œil — la seule chose qu'un
/// test ne sait pas faire. Ignoré par défaut :
/// `cargo test --test bulletin_pdf -- --ignored --nocapture`
#[test]
#[ignore]
fn ecrire_un_exemple() {
    let dest =
        std::env::var("BULLETIN_PDF_OUT").unwrap_or_else(|_| "/tmp/bulletin_exemple.pdf".into());
    let (octets, pages) = pdf::generer(&gabarit(7, 3)).expect("génération");
    std::fs::write(&dest, &octets).expect("écriture");
    println!("{} — {} pages, {} octets", dest, pages, octets.len());
}
