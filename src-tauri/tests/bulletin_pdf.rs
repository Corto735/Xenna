//! Le moteur de composition du bulletin de paie.
//!
//! Un PDF ne se relit pas en test : on n'y vérifie donc pas une apparence, mais
//! ce qui casse en silence. Une grille à hauteur fixe suivie d'un pied fixe a
//! ses propres façons de mal tourner, différentes de celles d'un texte courant —
//! d'où un fichier distinct de `contrat_pdf.rs` :
//!
//!   - la somme des colonnes de chaque grille (bulletin, pied, annexe) doit
//!     valoir la largeur utile, sinon la dernière déborde d'un demi-millimètre
//!     que personne ne verra jamais en relecture ;
//!   - aucune ligne de la grille ne doit mordre sur le pied ;
//!   - une grille trop longue continue page suivante, avec son en-tête, et le
//!     pied (net payé) ne s'imprime qu'une fois, sur la dernière page ;
//!   - un document vide doit rester un document valide.

use xenna_paie_lib::paie_pdf::mise_en_page::{self, metriques};
use xenna_paie_lib::paie_pdf::modele::{
    Annexe, BulletinPdf, Champ, Groupe, Ligne, LigneAnnexe, LignePied,
};
use xenna_paie_lib::paie_pdf::pdf;
use xenna_paie_lib::pdf::police::{Face, Polices};
use xenna_paie_lib::pdf::rendu::{Dessin, PAGE_H, PAGE_L};

fn champ(l: &str, v: &str) -> Champ {
    Champ { l: l.into(), v: v.into() }
}

fn s(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn cotisation(libelle: &str) -> Ligne {
    Ligne {
        libelle: libelle.into(),
        base: "2 460.27".into(),
        taux: "6.9000".into(),
        a_deduire: "169.76".into(),
        base_pat: "2 460.27".into(),
        taux_pat: "8.5500".into(),
        montant_pat: "210.36".into(),
        ..Default::default()
    }
}

/// Un bulletin de gabarit portant `n` lignes de cotisation.
fn gabarit(n: usize) -> BulletinPdf {
    let mut lignes = vec![
        Ligne { libelle: "Salaire de base".into(), base: "151.67".into(), taux: "16.1403".into(),
                a_payer: "2 448.00".into(), ..Default::default() },
        Ligne { libelle: "Salaire brut".into(), a_payer: "2 460.27".into(), fort: true, ..Default::default() },
        Ligne { vide: true, ..Default::default() },
        Ligne { libelle: "Retraite".into(), fort: true, ..Default::default() },
    ];
    lignes.extend((0..n).map(|i| cotisation(&format!("Cotisation de démonstration {}", i + 1))));
    lignes.push(Ligne { libelle: "Net à payer avant impôt sur le revenu".into(),
                        a_payer: "1 876.56".into(), grand: true, ..Default::default() });
    lignes.push(Ligne { libelle: "Taux neutre (barème DGFiP)".into(), note: true, centre: true,
                        ..Default::default() });
    lignes.push(Ligne { libelle: "Net payé".into(), a_payer: "1 809.65".into(), fort: true,
                        ..Default::default() });

    BulletinPdf {
        titre: "BULLETIN DE SALAIRE".into(),
        periode: "Septembre 2026".into(),
        reference: String::new(),
        filigrane: "SPÉCIMEN".into(),
        employeur: "Gormenghast Logistique".into(),
        employeur_adresse: s(&["14, allée des Contrevents", "78412 Lud-en-Brume"]),
        identifiants: vec![
            vec![champ("Siret", "41290833500047"), champ("Code Naf", "5229B")],
            vec![champ("Urssaf/Msa", "117 000 004 512 34")],
        ],
        blocs: vec![
            vec![champ("Matricule", "XN-042")],
            vec![champ("Emploi", "Contrôleur des Vents Contraires"), champ("Statut", "Non-cadre"),
                 champ("Echelon", "2"), champ("Niveau", "III"), champ("Coefficient", "157,5")],
            vec![champ("Entrée", "01/02/2024"), champ("Ancienneté", "2 ans et 7 mois  01/02/2024")],
        ],
        convention: Some(champ("Convention collective",
            "Transports routiers et activités auxiliaires du transport (IDCC 0016)")),
        destinataire: s(&["DE RIV Geralt", "6, place du Mont-Brumeux", "68270 Wittenheim"]),
        colonnes: s(&["Eléments de paie", "Base", "Taux", "A déduire", "A payer", "Charges patronales"]),
        lignes,
        pied_entetes: s(&["", "Heures", "Heures suppl.", "Brut", "Plafond S.S.", "Net imposable",
                          "Ch. patronales", "Coût Global", "Total versé", "Allègements"]),
        pied_lignes: vec![
            LignePied { l: "Mensuel".into(), v: s(&["151.67", "", "2 460.27", "4 005.00", "1 967.90",
                "888.45", "3 348.72", "3 348.72", "227.31"]) },
            LignePied { l: "Annuel".into(), v: vec![] },
        ],
        conges_entetes: s(&["", "Congés N-1", "Congés N"]),
        conges_lignes: vec![
            LignePied { l: "Acquis".into(), v: s(&["", "10.00"]) },
            LignePied { l: "Pris".into(), v: s(&["5.00", ""]) },
            LignePied { l: "Solde".into(), v: vec![] },
        ],
        net_paye: "Net payé : 1 809.65 euros".into(),
        paiement: "Paiement le 30/09/2026 par Virement".into(),
        mention: "Dans votre intérêt, et pour vous aider à faire valoir vos droits, conservez ce \
                  bulletin de paie sans limitation de durée. Informations complémentaires : \
                  www.service-public.fr".into(),
        annexe: Some(Annexe {
            titre: "ANNEXE — DÉTAIL DES COTISATIONS ET CONTRIBUTIONS".into(),
            chapeau: vec!["Le bulletin regroupe les cotisations par risque couvert. Cette annexe \
                           les redonne ligne à ligne.".into()],
            colonnes: s(&["Cotisation", "Base", "Taux", "Montant", "Taux", "Montant"]),
            groupes: vec![
                Groupe { titre: "PART SALARIÉ".into(), de: 2, a: 3 },
                Groupe { titre: "PART EMPLOYEUR".into(), de: 4, a: 5 },
            ],
            lignes: (0..n)
                .map(|i| LigneAnnexe {
                    libelle: format!("Cotisation d'annexe n° {}", i + 1),
                    code: format!("DEMO_{i}"),
                    base: "2 460.27".into(),
                    taux_sal: "6.9000".into(),
                    montant_sal: "169.76".into(),
                    taux_pat: "8.5500".into(),
                    montant_pat: "210.36".into(),
                    reference: "CSS art. L241-13, D241-7 — BOSS, chapitre 4 § 670".into(),
                })
                .collect(),
        }),
    }
}

fn textes(page: &[Dessin]) -> Vec<(f32, String)> {
    page.iter()
        .filter_map(|d| match d {
            Dessin::Texte { y, texte, .. } => Some((*y, texte.clone())),
            _ => None,
        })
        .collect()
}

#[test]
fn produit_un_pdf_valide() {
    let (octets, pages) = pdf::generer(&gabarit(15)).expect("génération");
    assert!(octets.starts_with(b"%PDF-"), "en-tête PDF absente");
    assert!(octets.windows(5).any(|w| w == b"%%EOF"), "fin de fichier absente");
    assert!(octets.len() > 20_000, "PDF suspicieusement court : {} octets", octets.len());
    // Le bulletin tient sur une page, l'annexe en occupe une autre.
    assert_eq!(pages, 2, "bulletin + annexe devraient faire 2 pages, obtenu {pages}");
}

/// La somme des colonnes de chaque grille doit valoir exactement la largeur
/// utile : c'est ce qui garantit que les charges patronales et l'encadré du net
/// payé finissent au bord droit du cadre et non un millimètre plus loin.
#[test]
fn les_colonnes_remplissent_exactement_la_largeur_utile() {
    let (col, grilles, _, _) = metriques();
    for (nom, largeurs) in ["bulletin", "pied", "annexe"].iter().zip(grilles.iter()) {
        let somme: f32 = largeurs.iter().sum();
        assert!((somme - col).abs() < 0.5,
            "grille {nom} : {somme:.1} pt pour une largeur utile de {col:.1} pt");
    }
    let libelle = grilles[0][0];
    assert!(libelle > col * 0.35, "le libellé n'a plus que {libelle:.1} pt");
}

/// Rien ne doit déborder du cadre, ni à droite (les montants, calés au fer à
/// droite, sont les premiers à sortir) ni à gauche, ni hors de la page.
#[test]
fn rien_ne_deborde_du_cadre() {
    let polices = Polices::charger().expect("polices");
    let (col, _, marge, _) = metriques();
    for n in [3usize, 30, 90] {
        for page in mise_en_page::composer(&gabarit(n), &polices) {
            for d in page {
                match d {
                    // Le filigrane traverse la page en diagonale : il n'est
                    // délibérément pas soumis à la colonne de texte.
                    Dessin::Filigrane { .. } => {}
                    Dessin::Texte { x, y, texte, face, taille, .. } => {
                        let droite = x + polices.largeur(face, &texte, taille);
                        assert!(droite <= marge + col + 0.5,
                            "« {texte} » déborde à droite ({droite:.1} pt) — {n} lignes");
                        assert!(x >= marge - 0.5, "« {texte} » déborde à gauche ({x:.1} pt)");
                        assert!(y > 0.0 && y < PAGE_H, "« {texte} » hors page ({y:.1} pt)");
                    }
                    Dessin::Pave { x, l, .. } | Dessin::Aplat { x, l, .. } => {
                        assert!(x >= marge - 0.5 && x + l <= marge + col + 0.5,
                            "un aplat déborde du cadre ({x:.1} → {:.1} pt)", x + l);
                    }
                    Dessin::Filet { x1, x2, y1, y2, .. } => {
                        assert!(x1 >= 0.0 && x2 <= PAGE_L, "un filet sort de la page");
                        assert!(y1 >= 0.0 && y2 <= PAGE_H, "un filet sort de la page");
                    }
                }
            }
        }
    }
}

/// Le pied est à hauteur fixe : aucune ligne de la grille ne doit descendre
/// dessous, même quand la grille est pleine à ras bord.
#[test]
fn aucune_ligne_ne_mord_sur_le_pied() {
    let polices = Polices::charger().expect("polices");
    let (_, _, _, bas_grille) = metriques();
    for n in 1..=70 {
        let pages = mise_en_page::composer(&gabarit(n), &polices);
        for (i, page) in pages.iter().enumerate() {
            for (y, t) in textes(page) {
                if t.starts_with("Cotisation de démonstration") || t == "Net payé" || t == "Retraite" {
                    assert!(y < bas_grille - 1.0,
                        "{n} lignes, page {} : « {t} » à {y:.1} pt mord sur le pied ({bas_grille:.1})",
                        i + 1);
                }
            }
        }
    }
}

/// Une grille trop longue continue page suivante, en-tête compris ; le pied et
/// son net payé ne s'impriment qu'une fois, sur la dernière page de grille.
#[test]
fn une_grille_trop_longue_continue_avec_son_entete_et_un_seul_pied() {
    let polices = Polices::charger().expect("polices");
    let pages = mise_en_page::composer(&gabarit(90), &polices);
    let grille: Vec<_> = pages.iter()
        .filter(|p| textes(p).iter().any(|(_, t)| t.starts_with("Cotisation de démonstration")))
        .collect();
    assert!(grille.len() >= 2, "90 lignes devraient déborder sur une deuxième page");
    for (i, p) in grille.iter().enumerate() {
        let t = textes(p);
        for attendu in ["Eléments de paie", "Charges patronales", "A payer"] {
            assert!(t.iter().any(|(_, x)| x == attendu), "page de grille {} sans « {attendu} »", i + 1);
        }
        let pied = t.iter().any(|(_, x)| x.starts_with("Net payé :"));
        assert_eq!(pied, i + 1 == grille.len(),
            "page de grille {} : le pied doit être sur la seule dernière page", i + 1);
    }
    assert!(textes(grille[0]).iter().any(|(_, x)| x == "Suite page suivante"));
}

/// Le filigrane doit passer PAR-DESSUS — les aplats anthracite le mangeraient par
/// morceaux s'il passait derrière —, être assez translucide pour ne pas gêner
/// la lecture, et se trouver sur chacune des pages.
#[test]
fn le_filigrane_couvre_chaque_page_par_dessus_et_translucide() {
    let polices = Polices::charger().expect("polices");
    for (i, page) in mise_en_page::composer(&gabarit(90), &polices).iter().enumerate() {
        let pos = page.iter()
            .position(|d| matches!(d, Dessin::Filigrane { .. }))
            .unwrap_or_else(|| panic!("page {} sans filigrane", i + 1));
        assert!(!page[pos + 1..].iter().any(|d| matches!(d, Dessin::Pave { .. } | Dessin::Aplat { .. })),
            "page {} : un aplat est tracé après le filigrane et le mangerait", i + 1);
        let Dessin::Filigrane { texte, taille, alpha, angle, .. } = &page[pos] else { unreachable!() };
        assert_eq!(texte, "SPÉCIMEN");
        assert!(*taille > 40.0, "filigrane trop petit pour être un filigrane");
        assert!(*alpha > 0.0 && *alpha < 0.25, "opacité {alpha} : illisible ou envahissant");
        assert!(*angle > 10.0 && *angle < 80.0, "un filigrane à {angle}° n'est pas en diagonale");
    }
}

/// Un bulletin sans cotisation ni annexe tient sur une page.
#[test]
fn un_bulletin_sans_cotisation_reste_imprimable() {
    let mut b = gabarit(0);
    b.annexe = None;
    let (octets, pages) = pdf::generer(&b).expect("génération");
    assert!(octets.starts_with(b"%PDF-"));
    assert_eq!(pages, 1);
}

/// Un document vide de bout en bout ne doit pas faire paniquer le moteur : c'est
/// ce qu'on reçoit si le front envoie une structure par défaut.
#[test]
fn un_document_vide_ne_fait_pas_paniquer_le_moteur() {
    let (octets, pages) = pdf::generer(&BulletinPdf::default()).expect("génération");
    assert!(octets.starts_with(b"%PDF-"));
    assert_eq!(pages, 1);
}

/// La romaine embarquée doit couvrir tout ce qu'un bulletin français écrit —
/// l'euro, l'apostrophe typographique, les accents et l'exposant ordinal des
/// échelons. Un glyphe manquant s'imprimerait en blanc, sans la moindre erreur.
#[test]
fn la_fonte_romaine_couvre_le_francais_typographique() {
    let polices = Polices::charger().expect("polices");
    let echantillon = "àâäéèêëîïôöùûüÿçÀÂÄÉÈÊËÎÏÔÖÙÛÜŸÇœŒæÆ€«»’—…°²ᵉ";
    for face in [Face::Regulier, Face::Gras, Face::Italique] {
        for c in echantillon.chars() {
            if let Some(gid) = polices.face(face).lookup_glyph_index(c as u32) {
                assert!(gid != 0, "glyphe .notdef pour « {c} » ({face:?})");
            }
        }
        assert!(polices.largeur(face, "", 7.2) == 0.0);
        assert!(polices.largeur(face, "€", 7.2) > 0.0);
        assert!(polices.largeur(face, "cotisation", 7.2) > polices.largeur(face, "coti", 7.2));
    }
}

/// Les six faces doivent être distinctes : si l'index de `Face` glissait d'un
/// cran, tout compilerait et le bulletin sortirait dans la mauvaise fonte.
#[test]
fn les_six_faces_sont_bien_distinctes() {
    let p = Polices::charger().expect("polices");
    let mesure = |f| p.largeur(f, "Rémunération brute mensuelle", 10.0);
    assert!((mesure(Face::Regulier) - mesure(Face::Sans)).abs() > 0.5,
        "romaine et linéale mesurent pareil : index suspect");
    assert!(mesure(Face::Gras) > mesure(Face::Regulier), "la romaine grasse n'est pas plus large");
    assert!((mesure(Face::Gras) - mesure(Face::SansGras)).abs() > 0.5,
        "les deux grasses mesurent pareil : index suspect");
}

/// Écrit un PDF de démonstration pour inspection à l'œil — la seule chose qu'un
/// test ne sait pas faire. Ignoré par défaut :
/// `cargo test --test bulletin_pdf -- --ignored --nocapture`
#[test]
#[ignore]
fn ecrire_un_exemple() {
    let dest =
        std::env::var("BULLETIN_PDF_OUT").unwrap_or_else(|_| "/tmp/bulletin_exemple.pdf".into());
    let (octets, pages) = pdf::generer(&gabarit(15)).expect("génération");
    std::fs::write(&dest, &octets).expect("écriture");
    println!("{} — {} pages, {} octets", dest, pages, octets.len());
}

/// Annexe : un libellé long ne doit pas chevaucher le code de cotisation calé
/// au fer à droite de la même colonne.
#[test]
fn l_annexe_ne_fait_pas_chevaucher_libelle_et_code() {
    let polices = Polices::charger().expect("polices");
    let mut b = gabarit(1);
    let a = b.annexe.as_mut().unwrap();
    a.lignes[0].libelle = "Maladie complémentaire Alsace-Moselle (régime local) — cotisation salariale".into();
    a.lignes[0].code = "ALSACE_MOSELLE_MALADIE".into();
    let pages = mise_en_page::composer(&b, &polices);
    let page = pages.last().unwrap();
    let boite = |t: &str| page.iter().find_map(|d| match d {
        Dessin::Texte { x, y, texte, face, taille, .. } if texte == t =>
            Some((*x, *x + polices.largeur(*face, texte, *taille), *y)),
        _ => None,
    });
    let (cx, _, cy) = boite("ALSACE_MOSELLE_MALADIE").expect("code imprimé");
    for d in page {
        if let Dessin::Texte { x, y, texte, face, taille, .. } = d {
            if (y - cy).abs() < 0.5 && texte != "ALSACE_MOSELLE_MALADIE" && *x < cx {
                let droite = x + polices.largeur(*face, texte, *taille);
                assert!(droite < cx, "« {texte} » chevauche le code ({droite:.1} > {cx:.1})");
            }
        }
    }
}

/// Le bulletin est MONOCHROME, pour s'imprimer fidèlement en noir et blanc :
/// chaque aplat doit être un gris neutre, sans la moindre teinte.
#[test]
fn le_bulletin_est_monochrome() {
    let polices = Polices::charger().expect("polices");
    for page in mise_en_page::composer(&gabarit(30), &polices) {
        for d in page {
            if let Dessin::Aplat { rvb: [r, v, b], .. } = d {
                assert!((r - v).abs() < 1e-6 && (v - b).abs() < 1e-6,
                    "aplat teinté ({r}, {v}, {b}) : le bulletin doit rester en niveaux de gris");
            }
        }
    }
}
