// Veille réglementaire : fraîcheur des barèmes, pays par pays.
//
// Un simulateur qui refuse d'inventer un chiffre ne doit pas non plus faire
// passer un chiffre périmé pour un chiffre à jour. Or une date de paie au-delà
// des barèmes intégrés ne déclenche aucune erreur : chaque calculateur retombe
// sur sa dernière branche (`_ =>`, `annee >= …`) ou sur la dernière période en
// base (`date_fin NULL`). Ce module DÉCLARE ce qui est réellement dans le code,
// pour que l'interface puisse le dire.
//
// Ce n'est PAS un calcul et rien ici n'est déduit automatiquement : une date de
// dernière modification en base ne prouve pas qu'un taux stable ait été
// revérifié, et les barèmes d'impôt vivent en Rust, hors de toute table. La
// vérité est donc un relevé, daté par `AUDIT_DU`, à refaire à chaque intégration
// de barèmes : quand un pays reçoit ses valeurs d'une nouvelle année, on relève
// `integre_jusqu_a` et on retire les lacunes comblées.
//
// Le `match` de `veille()` est exhaustif : un pays ajouté à l'enum sans sa
// déclaration de fraîcheur casse la compilation.

use serde::Serialize;

use crate::models::Pays;

/// Date du relevé (code et base relus pays par pays).
pub const AUDIT_DU: &str = "2026-09-25";

#[derive(Debug, Clone, Serialize)]
pub struct Veille {
    /// Dernière année civile dont TOUS les barèmes (taux, plafonds, impôt) sont
    /// intégrés. Au-delà, le calculateur prolonge les valeurs antérieures.
    pub integre_jusqu_a: i32,
    /// Ce qui manque au-delà de `integre_jusqu_a`, daté dans le texte même
    /// (une lacune peut ne porter que sur une partie de l'année).
    pub lacunes: &'static [&'static str],
    pub audit_du: &'static str,
    /// Date de la dernière évolution de taux du `JOURNAL` pour ce régime
    /// (entrées `taux: true`), s'il y en a une.
    pub derniere_maj: Option<&'static str>,
}

/// Une mise à jour de barèmes, datée et sourcée : ce que le visiteur lit pour
/// juger de la fraîcheur d'un régime. On n'y inscrit que ce qui a réellement
/// été intégré au code ou à la base, avec la source consultée.
#[derive(Debug, Clone, Serialize)]
pub struct MiseAJour {
    /// Date de l'intégration (AAAA-MM-JJ).
    pub date: &'static str,
    pub pays: Pays,
    /// Spécificité territoriale visée, si l'entrée ne porte que sur elle (le
    /// bandeau du bulletin ne l'affiche alors que si elle est cochée, et la date
    /// de dernière mise à jour du régime ne la compte pas).
    pub specificite: Option<Specificite>,
    /// Ce qui a changé, valeurs et date d'effet comprises.
    pub objet: &'static str,
    /// `true` si l'entrée fait évoluer un taux, un plafond ou un barème déjà
    /// modélisé (hausse, baisse, nouvelle année, correction d'une valeur
    /// fausse) ; `false` pour l'ajout d'un dispositif jusque-là absent ou un
    /// simple relevé « inchangé ». Seules les premières datent la « dernière
    /// évolution » affichée sous le bulletin ; le journal garde tout.
    pub taux: bool,
    /// Source(s) officielle(s) ou, à défaut, presse spécialisée concordante.
    pub sources: &'static [&'static str],
}

/// Spécificités territoriales journalisées à part de leur régime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Specificite {
    AlsaceMoselle,
    Esat,
    /// Convention collective des transports routiers (IDCC 0016).
    Idcc0016,
}

/// Journal des mises à jour, du plus récent au plus ancien. Qui intègre des
/// barèmes y ajoute une ligne dans le même commit (voir `CLAUDE.md`).
pub const JOURNAL: &[MiseAJour] = &[
    MiseAJour {
        date: "2026-10-09",
        pays: Pays::France,
        specificite: None,
        objet: "Droits ouverts par le mois : alimentation du compte personnel de formation ajoutée — 500 € par an au moins à mi-temps (plafond 5 000 €), prorata en deçà, 800 € (plafond 8 000 €) en ESAT et pour le travailleur handicapé d'entreprise adaptée, mention des 800 € du salarié non qualifié ; un mois en vaut le douzième, arrondi au centime supérieur. Montants inchangés depuis 2019",
        taux: false,
        sources: &[
            "https://www.service-public.gouv.fr/particuliers/vosdroits/F10705",
            "https://www.centre-inffo.fr/site-droit-formation/actualites-droit/alimentation-du-cpf-publication-du-decret",
        ],
    },
    MiseAJour {
        date: "2026-10-09",
        pays: Pays::France,
        specificite: Some(Specificite::Idcc0016),
        objet: "Minimum conventionnel contrôlé sur le bulletin : avec un classement choisi dans Paramètres (branche, catégorie, coefficient), le salaire de base est comparé au taux ou au salaire garanti du coefficient, palier d'ancienneté et temps de travail compris (marchandises, voyageurs, déménagement, logistique, sanitaire — grilles déjà relevées au Chakrram, sans valeur nouvelle). Garanties annuelles et majorations DC du déménagement hors champ",
        taux: false,
        sources: &[
            "https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000049067154/?idConteneur=KALICONT000005635624",
            "https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000053788378",
            "https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000051927426",
        ],
    },
    MiseAJour {
        date: "2026-10-08",
        pays: Pays::France,
        specificite: None,
        objet: "SMIC 10,48 € / 1 589,47 € corrigé : applicable dès le 1er octobre 2021 (arrêté du 27/09/2021, revalorisation automatique de 2,2 %) et non au 1er août ; les paies d'août et septembre 2021 retrouvent 10,25 € / 1 554,58 €, réduction générale comprise",
        taux: true,
        sources: &[
            "https://www.editions-tissot.fr/actualite/droit-du-travail/smic-au-1er-octobre-2021-une-hausse-mecanique",
            "https://www.lafinancepourtous.com/2021/09/17/revalorisation-automatique-du-smic-au-1er-octobre-2021-22/",
        ],
    },
    MiseAJour {
        date: "2026-10-08",
        pays: Pays::France,
        specificite: None,
        objet: "Prélèvement à la source : grilles de taux par défaut d'outre-mer ajoutées (Guadeloupe, La Réunion, Martinique ; Guyane, Mayotte), 2019 à 2026, choisies selon le domicile fiscal ; abattement « contrats courts » (CDD ou mission ≤ 2 mois, moitié du SMIC mensuel net imposable : de 624 € en 2019 à 766 € au 01/06/2026) retranché de l'assiette avant lecture de la grille. Vaut aussi pour la FPT",
        taux: false,
        sources: &[
            "https://bofip.impots.gouv.fr/bofip/11255-PGP.html/identifiant=BOI-BAREME-000037-20260706",
            "https://bofip.impots.gouv.fr/bofip/11252-PGP.html/identifiant=BOI-IR-PAS-20-20-30-10-20230626",
        ],
    },
    MiseAJour {
        date: "2026-10-08",
        pays: Pays::France,
        specificite: None,
        objet: "SMIC 11,88 € / 1 801,80 € corrigé : applicable dès le 1er novembre 2024 (décret n° 2024-951 du 23/10/2024) et non au 1er décembre ; vaut aussi pour la réduction générale de novembre 2024, la revalorisation n'étant pas neutralisée en 2024",
        taux: true,
        sources: &[
            "https://www.info.gouv.fr/actualite/revalorisation-du-smic-au-1er-novembre-2024",
            "https://www.editions-tissot.fr/actualite/droit-du-travail/reduction-generale-des-cotisations-patronales-comment-prendre-en-compte-la-hausse-du-smic-au-1er-novembre-2024",
        ],
    },
    MiseAJour {
        date: "2026-10-08",
        pays: Pays::France,
        specificite: None,
        objet: "Ajout des droits à la retraite ouverts par le mois : points Agirc-Arrco (prix d'achat du point 2019 à 2026, de 17,0571 € à 20,1877 € ; valeur de service de 1,2588 € à 1,4386 € ; taux de calcul des points 6,20 % / 17 %) et trimestres (150 h × SMIC horaire du 1er janvier, art. R351-9 CSS). Ajout d'alertes de cohérence : salaire de base sous le SMIC (L3231-2), temps partiel sous 24 h (L3123-27)",
        taux: false,
        sources: &[
            "https://www.agirc-arrco.fr/storage/2024/10/Compilation_valeurs_de_point_novembre_2025.pdf",
            "https://www.legisocial.fr/reperes-sociaux/point-retraite-complementaire-arrco-agirc-2026.html",
        ],
    },
    MiseAJour {
        date: "2026-10-08",
        pays: Pays::FonctionPublique,
        specificite: None,
        objet: "Prélèvement à la source corrigé comme pour le privé : taux unique de la grille par défaut sur la totalité du net imposable (art. 204 H, III CGI), grilles 2019 à 2026 datées, taux personnalisé saisissable",
        taux: true,
        sources: &[
            "https://bofip.impots.gouv.fr/bofip/11255-PGP.html/identifiant=BOI-BAREME-000037-20260407",
        ],
    },
    MiseAJour {
        date: "2026-10-08",
        pays: Pays::France,
        specificite: None,
        objet: "Prélèvement à la source corrigé : le taux de la grille par défaut s'applique désormais à la totalité du net imposable (art. 204 H, III CGI) et non plus tranche par tranche, ce qui sous-estimait la retenue ; grilles métropole 2019 à 2026 mises en base et datées (2024 prolongée jusqu'au 30/04/2025, 2025 jusqu'au 30/04/2026, 2026 à compter du 01/05/2026) ; aucun PAS avant 2019 ; taux personnalisé saisissable. Vaut aussi pour la fonction publique territoriale",
        taux: true,
        sources: &[
            "https://www.legifrance.gouv.fr/codes/article_lc/LEGIARTI000049641730",
            "https://bofip.impots.gouv.fr/bofip/11255-PGP.html/identifiant=BOI-BAREME-000037-20260407",
            "https://bofip.impots.gouv.fr/bofip/11255-PGP.html/identifiant=BOI-BAREME-000037-20250410",
            "https://bofip.impots.gouv.fr/bofip/11255-PGP.html/identifiant=BOI-BAREME-000037-20240228",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Espagne,
        specificite: None,
        objet: "Retenue d'IRPF ajoutée (2025-2026), absente jusqu'ici : algorithme officiel de l'AEAT (réduction pour revenus du travail, frais de 2 000 €, minimum personnel de 5 550 €, barème de retenue 19 à 47 %, limite d'exonération de 15 876 € et plafond de 43 %), salarié sans charge de famille payé 12 fois",
        taux: true,
        sources: &[
            "https://sede.agenciatributaria.gob.es/static_files/Sede/Programas_ayuda/Retenciones/2026/ALGORITMO_2026.pdf",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Luxembourg,
        specificite: None,
        objet: "Impôt sur les salaires ajouté (2025-2026, classe 1), absent jusqu'ici : barème 0 à 42 % (tranche exonérée 13 230 €), contribution au fonds pour l'emploi 7 % / 9 %, forfaits de 540 et 480 €, crédit d'impôt salarié et crédit CO2 (192 € en 2025, 216 € en 2026)",
        taux: true,
        sources: &[
            "https://impotsdirects.public.lu/fr/az/t/tarif_pers.html",
            "https://impotsdirects.public.lu/fr/az/c/credit-impot-salaries/cis2026.html",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Autriche,
        specificite: None,
        objet: "Lohnsteuer : Werbungskostenpauschale 132 € et Verkehrsabsetzbetrag (487 € en 2025, 496 € en 2026) appliqués",
        taux: true,
        sources: &[
            "https://www.bmf.gv.at/themen/steuern/arbeitnehmerveranlagung/steuertarif-steuerabsetzbetraege/uebersicht-steuerabsetzbetraege.html",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Grece,
        specificite: None,
        objet: "Retenue ΦΜΥ annualisée sur 14 paies (et non 12) ; réduction salarié de 777 € dégressive de 20 € par 1 000 € au-delà de 12 000 €",
        taux: true,
        sources: &[
            "https://www.aade.gr/",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Slovenie,
        specificite: None,
        objet: "Cotisation dépendance 1 % / 1 % depuis le 01/07/2025 ; contribution santé forfaitaire (OZP) 35 €, 37,17 € (03/2025), 39,36 € (03/2026) ; abattement général 2025 5 260 €",
        taux: true,
        sources: &[
            "https://www.rtvslo.si/slovenija/z-marcem-visji-obvezni-zdravstveni-prispevek-sd-predlaga-zamrznitev-usklajevanja/774070",
            "https://www.racunovodstvo.net/tabelice/169/davcne-olajsave-v-letu-2025",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Bulgarie,
        specificite: None,
        objet: "Revenu maximal assurable 2 300 € dès le 01/08/2026 (2 111,64 € auparavant) ; taux inchangés, la hausse de 2 points de la pension n'a pas été adoptée",
        taux: true,
        sources: &[
            "https://www.innovires.com/blog/danaci-i-osigurovki.html",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::CoreeDuSud,
        specificite: None,
        objet: "Impôt : cotisations sociales salariales déduites du revenu imposable et crédit standard de 130 000 ₩ appliqués (impôt surestimé d'environ 45 % auparavant)",
        taux: true,
        sources: &[
            "https://www.nts.go.kr/english/main.do",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Pologne,
        specificite: None,
        objet: "Avance PIT : base et avance arrondies au złoty entier (Ordynacja podatkowa art. 63)",
        taux: false,
        sources: &[
            "https://isap.sejm.gov.pl/isap.nsf/DocDetails.xsp?id=WDU19970370601",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Monaco,
        specificite: None,
        objet: "D'octobre 2025 à septembre 2026 : CAR employeur 8,33 % (base 7,45 % + variable 0,88 %), plafonds CAR 6 112 €, CCSS 9 800 €, chômage 15 700 € ; retraite complémentaire CMRC ajoutée (TA 4,008 % / 6,012 %, TB 9,716 % / 14,574 %) ; dès octobre 2026 : CAR 8,39 %, plafonds 6 276 / 10 000 / 16 020 €, TA 4 067 €",
        taux: true,
        sources: &[
            "https://www.caisses-sociales.mc/content/download/3439/file/Lettre_information_aux_employeurs_octobre_2025.pdf",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Danemark,
        specificite: None,
        objet: "ATP 2026 : 99 / 198 DKK par mois ; beskæftigelsesfradrag (10,65 % / 12,30 % / 12,75 %, plafonds 45 100 / 55 600 / 63 300 DKK) et jobfradrag déduits de l'assiette de l'impôt communal",
        taux: true,
        sources: &[
            "https://www.borger.dk/pension-og-efterloen/atp-livslang-pension-oversigt/atp-bidraget/atp-satser-for-privat-virksomhed",
            "https://tax.dk/skat/beskaeftigelsesfradrag.htm",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Suede,
        specificite: None,
        objet: "Impôt selon Skatteverket (SKV 433 éd. 35 et 36) : grundavdrag, jobbskatteavdrag, réduction pour revenu d'activité et redevance public service, auparavant non modélisés (impôt surestimé d'environ 60 %)",
        taux: true,
        sources: &[
            "https://www.skatteverket.se/download/18.1522bf3f19aea8075ba55c/1766385913260/teknisk-beskrivning-skv-433-2026-utgava-36.pdf",
            "https://www.skatteverket.se/download/18.262c54c219391f2e9632607/1733849404498/teknisk-beskrivning-SKV433-2025-utgava-35.pdf",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Finlande,
        specificite: None,
        objet: "2026 : barème d'État (première tranche jusqu'à 22 000 €), impôt communal moyen 7,57 %, déduction de 750 €, työtulovähennys (18 %, plafond 3 430 €) et perusvähennys appliqués ; chômage employeur 0,31 % ; TyEL employeur moyen 17,10 %",
        taux: true,
        sources: &[
            "https://www.veronmaksajat.fi/neuvot/henkiloverotus/tyo-elake-ja-etuudet/verovahennykset/2026/ansiotulosta-tehtavat-vahennykset-2026/",
            "https://www.veronmaksajat.fi/neuvot/henkiloverotus/tyo-elake-ja-etuudet/ansiotulojen-verot-ja-maksut/2026/tyottomyysvakuutusmaksu-2026/",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Suisse,
        specificite: None,
        objet: "LPP : déduction de coordination 26 460 CHF en 2025-2026 (et non 27 225), 24 675 en 2015-2018 ; salaires coordonnés maximaux 59 925 / 60 435 / 60 945 / 62 475 / 64 260 CHF corrigés",
        taux: true,
        sources: &[
            "https://www.bsv.admin.ch/dam/bsv/fr/dokumente/bv/anleitungen/masszahlen-2025-2026.pdf.download.pdf/masszahlen-2025-2026.pdf",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Luxembourg,
        specificite: None,
        objet: "Pension 8,50 % salarié et employeur au 01/01/2026 (réforme des pensions) ; accidents 0,65 % (taux unique 2026) ; mutualité des employeurs classe 2 0,95 % ; dépendance sans plafond après abattement d'un quart du SSM ; plafond cotisable 13 518,68 € puis 13 856,63 € au 01/06/2026",
        taux: true,
        sources: &[
            "https://fedil.lu/fr/publications/parametres-sociaux-applicables-a-partir-du-1er-janvier-2026/",
            "https://fedil.lu/fr/publications/parametres-sociaux-applicables-a-partir-du-1er-juin-2026/",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::FonctionPublique,
        specificite: None,
        objet: "Titulaires CNRACL : maladie 9,88 % (11,50 % avant 2018) au lieu de 13 % ; ATIACL 0,40 % au lieu de l'AT/MP ; FNAL, contribution solidarité autonomie 0,30 % et CNFPT (1 %, 0,9 % de 2016 à 2021, 0,95 % en 2022) ajoutés ; part agent CNRACL 9,54 % (2015) à 11,10 % (2020+), décalée d'un an auparavant",
        taux: true,
        sources: &[
            "https://www.cdg44.fr/sites/default/files/content/Dossier%20RH/Cotisations%202026%20Agents%20CNRACL.pdf",
            "https://www.ipp.eu/baremes-ipp/prelevements-sociaux/prelevements_sociaux.cotisations_secteur_public.cnracl/table",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Italia,
        specificite: None,
        objet: "IRPEF sur le revenu net des cotisations INPS ; détraction salarié 1 910 + 1 190 × (28 000 − R) / 13 000 + 65 € ; coin fiscal 2025-2026 par tranches (7,1 / 5,3 / 4,8 %, puis 1 000 € dégressif de 32 000 à 40 000 €) ; exonération de cotisations 2022-2024 aux seuils mensuels 1 923 / 2 692 € et par semestre ; prime 2024 inexistante supprimée",
        taux: true,
        sources: &[
            "https://www.money.it/detrazioni-lavoro-dipendente-2026-calcolo",
            "https://www.assolombarda.it/servizi/lavoro-e-previdenza/informazioni/decontribuzione-2024-per-i-lavoratori-dipendenti-indicazioni-inps",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Canada,
        specificite: None,
        objet: "2026 : AE 1,63 % / 2,282 %, MGA 74 600 $, MGAP2 85 000 $, MAGA 68 900 $ ; retenue d'impôt selon la T4127 (déduction du RPC supplémentaire, crédits RPC/AE et montant pour emploi, réduction de l'Ontario)",
        taux: true,
        sources: &[
            "https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas/t4127-jan/t4127-jan-payroll-deductions-formulas-computer-programs.html",
            "https://www.canada.ca/en/employment-social-development/programs/ei/ei-list/ei-employers/premium-reduction-program/2026-maximum-insurable-earnings.html",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Quebec,
        specificite: None,
        objet: "2026 : RRQ 6,30 %, RQAP 0,430 % / 0,602 % (MRA 103 000 $), AE 1,30 % / 1,82 % ; historique RRQ 2016-2018, RQAP 2017-2020 et AE 2021, 2024 corrigé ; impôt du Québec après déduction pour travailleurs et RRQ supplémentaire, fédéral avec crédits RRQ/AE/RQAP",
        taux: true,
        sources: &[
            "https://www.revenuquebec.ca/fr/entreprises/retenues-et-cotisations/calculer-les-retenues-a-la-source-et-vos-cotisations-demployeur/regime-de-rentes-du-quebec/maximum-du-salaire-admissible-et-taux-de-cotisation/",
            "https://www.revenuquebec.ca/fr/entreprises/retenues-a-la-source-et-cotisations-de-lemployeur/calcul-des-retenues-et-des-cotisations/cotisations-au-rqap/maximum-de-revenus-assurables-et-taux-de-cotisation/",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Allemagne,
        specificite: None,
        objet: "Lohnsteuer : part maladie de la Vorsorgepauschale au taux réduit du PAP (7,0 % + moitié du Zusatzbeitrag) et non au taux réel",
        taux: false,
        sources: &[
            "https://www.bundesfinanzministerium.de/Content/DE/Downloads/Steuern/Steuerarten/Lohnsteuer/Programmablaufplan/",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Portugal,
        specificite: None,
        objet: "Retenue IRS 2026 selon la table officielle I du Continent (Despacho n.º 233-A/2026) ; contributions FCT (0,925 %) et FGCT (0,075 %) arrêtées au 01/05/2023",
        taux: true,
        sources: &[
            "https://www.doutorfinancas.pt/wp-content/uploads/2026/01/tabelas-retencao-trabalho-dependente.pdf",
            "https://www.plmj.com/xms/files/03_Novidades_legislativas/2023/dezembro/FCT_e_FGCT.pdf",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Belgique,
        specificite: None,
        objet: "Précompte professionnel calculé sur la rémunération imposable (brut − ONSS 13,07 %) et non sur le brut ; frais forfaitaires 2025 plafonnés à 5 930 €",
        taux: true,
        sources: &[
            "https://www.securex.be/getattachment/00e248e1-27eb-4cab-92ef-31d680e98826/Fiscoliste-janvier-2026.pdf",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Angleterre,
        specificite: None,
        objet: "National Insurance : seuils mensuels publiés par HMRC (1 048, 417, 4 189 £) au lieu de l'annuel ÷ 12",
        taux: false,
        sources: &[
            "https://www.gov.uk/guidance/rates-and-thresholds-for-employers-2026-to-2027",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::Japon,
        specificite: None,
        objet: "Maladie, dépendance, soutien à l'enfance et pension sur la rémunération mensuelle standard (標準報酬月額, 50 et 32 paliers) et non sur le salaire réel ; part salariale arrondie à l'entier, 0,50 ¥ et moins tronqués",
        taux: false,
        sources: &[
            "https://www.kyoukaikenpo.or.jp/g7/cat330/sb3150/r08/r8ryougakuhyou3gatukara/",
        ],
    },
    MiseAJour {
        date: "2026-10-03",
        pays: Pays::France,
        specificite: None,
        objet: "Taux corrigés après comparaison avec un bulletin réel : maladie patronale 13 % depuis 2018 (et non 13,13 % ; 12,80 / 12,84 / 12,89 % de 2015 à 2017) ; vieillesse déplafonnée patronale 2,02 % en 2025 et 2,11 % en 2026 (salariale 0,30 % en 2015, 0,35 % en 2016) ; allocations familiales 5,25 % (3,45 % n'était que le taux réduit) ; taux réduits maladie 7 % (≤ 2,5 SMIC de 2019 à 2024, ≤ 2,25 SMIC en 2025) et famille 3,45 % (≤ 1,6, 3,5 puis 3,3 SMIC), supprimés en 2026 ; chômage 4,00 % depuis le 01/05/2025 ; ligne AGS, en base mais jamais calculée, désormais sur le bulletin",
        taux: true,
        sources: &[
            "https://www.urssaf.fr/accueil/actualites/informations-nouvelle-annee.html",
            "https://www.urssaf.fr/accueil/outils-documentation/taux-baremes/taux-cotisations-secteur-prive.html",
            "https://www.legisocial.fr/reperes-sociaux/taux-cotisations-sociales-urssaf-2017.html",
            "https://www.legisocial.fr/actualites-sociales/1743-cotisations-urssaf-en-2016.html",
            "https://www.legisocial.fr/actualites-sociales/7292-taux-reduits-cotisations-assurance-maladie-allocations-familiales-comment-calculer-2025.html",
            "https://www.unedic.org/actualites/publication-de-la-circulaire-du-1er-mai-2025-relative-aux-contributions-d-assurance-chomage",
        ],
    },
    MiseAJour {
        date: "2026-09-30",
        pays: Pays::France,
        specificite: None,
        objet: "Avantages en nature (arrêté du 25/02/2025) : repas 5,50 € (5,45 € en 2025) ; logement, barème 2025 et 2026 en 8 tranches du PSS, 1 pièce ou par pièce ; véhicule 15 %/10 %, 20 %/15 % carburant compris, location 50 %/67 % (9 %/6 %, 12 %/9 %, 30 %/40 % pour une mise à disposition avant le 01/02/2025), abattement électrique 70 % plafonné à 4 641,60 € (4 582 € en 2025) ou 50 % plafonné à 2 026,30 € (2 000,30 €) ; NTIC 10 % ; avant 2025, forfaits non intégrés",
        taux: false,
        sources: &[
            "https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000051254024",
            "https://www.legisocial.fr/reperes-sociaux/avantage-en-nature-logement-2026.html",
            "https://www.legisocial.fr/reperes-sociaux/avantage-en-nature-repas-2026.html",
            "https://www.legisocial.fr/reperes-sociaux/avantage-en-nature-vehicule-2026.html",
            "https://www.urssaf.fr/accueil/outils-documentation/taux-baremes/avantages-en-nature.html",
        ],
    },
    MiseAJour {
        date: "2026-09-30",
        pays: Pays::France,
        specificite: Some(Specificite::Idcc0016),
        objet: "Indemnités de repas des ouvriers (protocole du 30/04/1974), versées en net : repas unique 10,07 €, repas unique de nuit 9,81 €, indemnité spéciale 4,42 €, casse-croûte 8,87 € au 01/01/2026 (av. n° 81) ; historique depuis le 01/12/2022 (av. n° 75, 77, 79) ; avant, aucun barème intégré",
        taux: false,
        sources: &[
            "https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000005678899/?idConteneur=KALICONT000005635624",
            "https://www.legifrance.gouv.fr/conv_coll/article/KALIARTI000053715766",
            "https://www.juristique.org/conventionnel/indemnites-ouvriers-transport-routier-2026",
            "https://unostra.fr/2025/02/21/revalorisation-des-indemnites-de-frais-de-deplacement-15-a-compter-du-1er-mars-2025/",
            "https://www.soluciaspj.fr/2023/12/22/remuneration-et-frais-de-deplacement-transport-routier-de-marchandises/",
            "https://www.juristique.org/conventionnel/indemnites-ouvriers-transport-routier-2023",
        ],
    },
    MiseAJour {
        date: "2026-09-30",
        pays: Pays::France,
        specificite: Some(Specificite::Esat),
        objet: "Travailleur d'ESAT : rémunération garantie de 55,7 % à 110,7 % du SMIC (55 % à 110 % avant 2018), aide au poste de l'État au plus 50,7 % (50 % avant 2018), dégressive au-delà d'une part ESAT de 20 % ; ni chômage, ni AGS, ni réduction générale ; compensation par l'État de toutes les cotisations patronales obligatoires dues sur l'aide au poste, Agirc-Arrco comprise (arrêté du 28/12/2006 ; FNAL, versement mobilité et médecine du travail non compensés)",
        taux: false,
        sources: &[
            "https://www.legifrance.gouv.fr/codes/section_lc/LEGITEXT000006074069/LEGISCTA000006190154/",
            "https://www.legifrance.gouv.fr/codes/id/LEGISCTA000006157600",
            "https://net-entreprises.custhelp.com/app/answers/detail/a_id/1884",
            "https://sante.gouv.fr/fichiers/bo/2008/08-09/ste_20080009_0100_0174.pdf",
            "https://www.directions.fr/Veille-juridique/dernieres-infos/ressources-humaines/2018/3/Nouvelle-formule-de-calcul-de-la-remuneration-garantie-2051086W/",
        ],
    },
    MiseAJour {
        date: "2026-09-29",
        pays: Pays::France,
        specificite: Some(Specificite::AlsaceMoselle),
        objet: "Régime local, maladie complémentaire : date de la baisse corrigée — 1,50 % du 01/01/2012 au 31/03/2022, 1,30 % depuis le 01/04/2022 (et non le 01/07/2018), maintenu en 2026 par le conseil d'administration du 19/12/2025",
        taux: true,
        sources: &[
            "https://regime-local.fr/cotisation/",
            "https://www.revue-fiduciaire.com/actualite/article/alsace-moselle-la-cotisation-d-assurance-maladie-du-regime-local-abaissee-a-1-30-en-avril-2022",
            "https://www.revue-fiduciaire.com/actualite/article/le-regime-local-d-assurance-maladie-d-alsace-moselle-maintient-son-taux-de-cotisation-a-1-30-pour-2026",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Chine,
        specificite: None,
        objet: "Pékin : assiette sociale 7 162-35 811 ¥ (07/2025-06/2026) puis 7 270-36 348 ¥ (dès 07/2026) ; fonds logement avec son propre plancher (salaire minimum, 2 540 ¥) ; maladie employeur 9 % avec le fonds des grosses dépenses (1 %), maternité 0,8 % à part",
        taux: true,
        sources: &[
            "https://www.beijing.gov.cn/zhengce/zhengcefagui/202509/t20250918_4205116.html",
            "https://www.beijing.gov.cn/zhengce/zcjd/202608/t20260821_4831683.html",
            "https://www.beijing.gov.cn/zhengce/zhengcefagui/202608/t20260824_4834975.html",
            "https://m.bjnews.com.cn/detail/161050449515293.html",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Japon,
        specificite: None,
        objet: "協会けんぽ Tokyo 2025 (9,91 % / 介護 1,59 %) et 2026 (9,85 % / 1,62 %) ; 雇用保険 2025 (0,55 % + 0,90 %) et 2026 (0,50 % + 0,85 %) ; 子ども・子育て支援金 0,23 % dès avril 2026 ; impôt : 基礎控除 et déduction d'emploi des réformes 2025 et 2026, cotisations sociales désormais déduites du revenu imposable",
        taux: true,
        sources: &[
            "https://www.kyoukaikenpo.or.jp/about/business/insurance_rate/rate_prefectures/r08/index.html",
            "https://www.mhlw.go.jp/content/001692566.pdf",
            "https://www.mof.go.jp/tax_policy/tax_reform/outline/fy2026/08taikou_gaiyou.pdf",
            "https://www.nta.go.jp/users/gensen/2026kiso/index.htm",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Emirats,
        specificite: None,
        objet: "GPSSA 2026 relevé : régime de la loi 7/1999 (salarié 5 %, employeur 12,5 %, État 2,5 %, plafond 50 000 AED) inchangé. Non modélisé : le régime du décret-loi 57/2023 pour les Émiratis entrés depuis le 31/10/2023 (11 % / 15 %, plafond 70 000 AED)",
        taux: false,
        sources: &[
            "https://www.zoho.com/en-ae/payroll/academy/compliance/gpssa-and-adpf-pension.html",
            "https://velmontcrest.ae/insights/gpssa-pension-uae-emirati-employee-contribution/",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Mexique,
        specificite: None,
        objet: "UMA au 1er février (113,14 $/j en 2025, 117,31 $/j en 2026) ; tarif ISR mensuel 2026 (Anexo 8 RMF 2026) ; subsidio al empleo 13,8 % de l'UMA en 2025 (≤ 10 171 $) et 15,02 % en 2026 (≤ 11 492,66 $) — le calcul appliquait les valeurs de 2024",
        taux: true,
        sources: &[
            "https://kpmg.com/mx/es/tendencias/2026/01/flash-inegi-valor-de-la-uma-para-2026.html",
            "https://www.sat.gob.mx/minisitio/NormatividadRMFyRGCE/documentos2026/rmf/anexos/Anexo-8-RMF-2026_DOF-28122025.pdf",
            "https://dof.gob.mx/nota_detalle.php?codigo=5777649&fecha=31%2F12%2F2025",
            "https://idconline.mx/fiscal-contable/2025/01/02/actualizan-estimulo-del-subsidio-al-empleo-para-2025",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Bresil,
        specificite: None,
        objet: "INSS 2026 (plancher 1 621 R$, plafond 8 475,55 R$) ; IRRF : table de mai 2025 (exonération jusqu'à 2 428,80 R$, desconto simplificado 607,20 R$) et réduction de la Lei 15.270/2025 dès 2026 (impôt nul jusqu'à 5 000 R$ de revenu, dégressif jusqu'à 7 350 R$)",
        taux: true,
        sources: &[
            "https://www.gov.br/previdencia/pt-br/assuntos/rpps/documentos/PortariaInterministerialMPSMF13de9dejaneirode2026.pdf",
            "https://calculabrasil.com/blog/tabelas-inss-irpf-2026",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Slovaquie,
        specificite: None,
        objet: "2026 : plafond social 16 764 €/mois ; assurance maladie salarié 5 % (hausse temporaire 2026-2027) ; nouvelles tranches d'impôt à 30 % et 35 % au-delà de 5 029,10 et 6 250,86 €/mois (3ᵉ paquet de consolidation)",
        taux: true,
        sources: &[
            "https://www.socpoist.sk/news/nove-vymeriavacie-zaklady-pre-platenie-poistneho-od-1-januara-2026",
            "https://www.vszp.sk/platitelia/platenie-poistneho/oznamenia-zmeny/zmeny-od-01-01.2026/",
            "https://www.podnikajte.sk/dan-z-prijmov/progresivne-zdanenie-prijmov-fyzickych-osob-od-2026",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Belgique,
        specificite: None,
        objet: "Revenus 2026 : tranches 16 720 / 29 510 / 51 070 €, quotité exemptée 11 180 €, forfait de frais professionnels plafonné à 6 070 € ; tranches 2025 corrigées (16 320 / 28 800 / 49 840 €)",
        taux: true,
        sources: &[
            "https://news.bloombergtax.com/daily-tax-report-international/belgium-mof-announces-automatic-indexation-for-2026-individual-income",
            "https://www.monsalaire-net.be/baremes-fiscaux-belgique-2026.html",
            "https://www.advice-me.be/2024/05/31/impot-personnes-physiques-belgique/",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Portugal,
        specificite: None,
        objet: "Barème IRS 2026 (OE 2026, Lei 73-A/2025) et barème 2025 rétroactivement abaissé (Lei 55-A/2025) ; dedução específica 8,54 × IAS (4 587,09 € en 2026) ; salaire minimum 920 €",
        taux: true,
        sources: &[
            "https://www.santander.pt/salto/escaloes-irs",
            "https://www.cgd.pt/Site/Saldo-Positivo/leis-e-impostos/Pages/novidades-IRS.aspx",
            "https://apcmc.pt/legislacao/ias-para-2026-fixado-em-e-53713/",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Espagne,
        specificite: None,
        objet: "2026 : base maximale 5 101,20 €, base minimale des groupes 4-7 1 424,40 € (le calcul prenait le SMI au lieu de la base minimale, corrigé depuis 2015), MEI 0,90 % (taux 2023-2025 corrigés) ; cotisation de solidarité au-delà de la base maximale ajoutée pour 2025 et 2026",
        taux: true,
        sources: &[
            "https://www.boe.es/diario_boe/txt.php?id=BOE-A-2026-7296",
            "https://www.cuatrecasas.com/es/spain/laboral/art/claves-orden-cotizacion-2026",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Allemagne,
        specificite: None,
        objet: "Lohnsteuer : tarif exact du §32a EStG pour 2023-2026 (Grundfreibetrag 2024 : 11 784 € ; 2026 : 12 348 € au lieu d'une estimation à 12 648 €) ; revenu imposable diminué des forfaits et de la Vorsorgepauschale (règles 2026) au lieu d'un double abattement ; splitting en classe III ; seuil du Soli par année (20 350 € en 2026)",
        taux: true,
        sources: &[
            "https://www.gesetze-im-internet.de/estg/__32a.html",
            "https://www.buzer.de/gesetz/4499/al210146-0.htm",
            "https://www.haufe.de/steuern/finanzverwaltung/vorsorgepauschale-im-lohnsteuerabzugsverfahren-ab-2026_164_658714.html",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Quebec,
        specificite: None,
        objet: "Table d'imposition du Québec 2025 (53 255 / 106 495 / 129 590 $, MPB 18 571 $) et 2026 (54 345 / 108 680 / 132 245 $, MPB 18 952 $) ; impôt fédéral 2025-2026 ; abattement du Québec de 16,5 % sur l'impôt fédéral, jusqu'ici omis",
        taux: true,
        sources: &[
            "https://cdn-contenu.quebec.ca/cdn-contenu/adm/min/finances/publications-adm/parametres/AUTFR_RegimeImpot2026.pdf",
            "https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas/t4127-jan/t4127-jan-payroll-deductions-formulas-computer-programs.html",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Canada,
        specificite: None,
        objet: "Impôt fédéral 2025 (1ʳᵉ tranche à 14,5 %, MPB 16 129 $) et 2026 (14 %, seuils 58 523 / 117 045 / 181 440 / 258 482 $, MPB 16 452 $) ; barèmes 2025 et 2026 des douze provinces et territoires hors Québec (dont la nouvelle tranche albertaine à 8 %) ; Ontario : surtaxe (2025+) et contribution-santé désormais comptées",
        taux: true,
        sources: &[
            "https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas/t4127-jan/t4127-jan-payroll-deductions-formulas-computer-programs.html",
            "https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/payroll-deductions-t4127-payroll-deductions-formulas/t4127-jul-121st-edition-effective-july-1-2025/t4127-jul-payroll-deductions-formulas.html",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::EtatsUnis,
        specificite: None,
        objet: "2026 : barème fédéral et déduction standard (16 100 $) de la Rev. Proc. 2025-32 ; plafond Social Security 184 500 $ ; California SDI 1,3 % ; New York : cinq premiers taux baissés de 0,1 point",
        taux: true,
        sources: &[
            "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill",
            "https://www.ssa.gov/news/en/cola/factsheets/2026.html",
            "https://edd.ca.gov/en/payroll_taxes/rates_and_withholding",
            "https://www.nerdwallet.com/taxes/learn/new-york-state-tax",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Inde,
        specificite: None,
        objet: "Exercice 2026-27 (dès le 01/04/2026) : barèmes, déduction standard (75 000 ₹) et rebate (revenu ≤ 12 lakh) inchangés par le Budget 2026-27 ; l'Income-tax Act 2025 remplace la loi de 1961",
        taux: false,
        sources: &[
            "https://www.businesstoday.in/personal-finance/tax/story/tax-slabs-fy-2026-27-what-budget-2026-changed-for-individual-taxpayers-and-which-regime-works-best-514044-2026-02-01",
            "https://cleartax.in/s/income-tax-slabs",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Australie,
        specificite: None,
        objet: "Exercice 2026-27 (dès le 01/07/2026) : 2ᵉ taux d'impôt ramené de 16 à 15 % ; maximum contribution base annuelle 270 830 $ ; calcul désormais par exercice (juillet-juin)",
        taux: true,
        sources: &[
            "https://www.ato.gov.au/about-ato/new-legislation/in-detail/individuals/personal-income-tax-new-tax-cuts-for-every-australian-taxpayer",
            "https://rest.com.au/super/learn/essentials/maximum-super-contribution-base",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Irlande,
        specificite: None,
        objet: "PRSI Class A : 4,2 % / 11,25 % dès le 01/10/2025 et 4,35 % / 11,40 % dès le 01/10/2026 (au lieu de 4,2 % / 11,15 % toute l'année) ; taux employeur réduit sous 552 €/semaine et crédit PRSI salarié désormais appliqués",
        taux: true,
        sources: &[
            "https://assets.gov.ie/static/documents/cb168977/PRSI_C20260116_Contribution_Rates_and_User_Guide_-_SW_14_-_English_Version_-_January_2026_.pdf-web.pdf",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::NouvelleZelande,
        specificite: None,
        objet: "Exercice 2026-27 (dès le 01/04/2026) : ACC earner's levy 1,75 % plafonné à 156 641 $ ; KiwiSaver employeur 3,5 % (4 % au 01/04/2028) ; barème PAYE inchangé ; calcul désormais par exercice (avril-mars)",
        taux: true,
        sources: &[
            "https://www.ird.govt.nz/kiwisaver-changes",
            "https://www.ird.govt.nz/updates/news-folder/2026/changes-to-the-kiwisaver-contribution-rate",
            "https://nztax.tools/tax-insights/acc-earner-levy-2026-27/",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Angleterre,
        specificite: None,
        objet: "Exercices 2025/26 et 2026/27 : NI employeur 15 % au-delà d'un Secondary Threshold de £5 000 (au lieu de 13,8 % / £9 100) ; seuils salariaux et d'impôt gelés ; calcul désormais par exercice fiscal (6 avril)",
        taux: true,
        sources: &[
            "https://www.gov.uk/guidance/rates-and-thresholds-for-employers-2026-to-2027",
            "https://www.gov.uk/guidance/rates-and-thresholds-for-employers-2025-to-2026",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::Italia,
        specificite: None,
        objet: "IRPEF 2026 : 2ᵉ tranche (28 000-50 000 €) ramenée de 35 à 33 % au 01/01/2026 — L. 199/2025 (Bilancio 2026)",
        taux: true,
        sources: &[
            "https://www.mef.gov.it/focus/Principali-misure-della-legge-di-bilancio-2026/",
        ],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::FonctionPublique,
        specificite: None,
        objet: "CNRACL : taux employeur 34,65 % (2025), 37,65 % (2026), 40,65 % (2027), 43,65 % (2028) — décret n° 2025-86 ; part agent inchangée (11,10 %)",
        taux: true,
        sources: &["https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000051070354"],
    },
    MiseAJour {
        date: "2026-09-28",
        pays: Pays::France,
        specificite: None,
        objet: "AGS : historique corrigé 2015-2026 — 0,30 % (2015), 0,25 % (2016), 0,20 % (1er sem. 2017), 0,15 % (07/2017-2023), 0,20 % (1er sem. 2024), 0,25 % depuis le 01/07/2024, maintenu en 2026",
        taux: true,
        sources: &[
            "https://entreprendre.service-public.gouv.fr/actualites/A17906",
            "https://www.legisocial.fr/actualites-sociales/1751-la-cotisation-ags-passe-025-au-1er-janvier-2016.html",
            "https://www.legisocial.fr/actualites-sociales/2262-diminution-de-la-cotisation-ags-qui-passe-015-au-1er-juillet-2017.html",
        ],
    },
];

/// Le journal complet (tableau « À propos » du front).
pub fn journal() -> &'static [MiseAJour] {
    JOURNAL
}

/// Veille d'un régime, étiquetée de son pays (tableau récapitulatif du front).
#[derive(Debug, Clone, Serialize)]
pub struct VeillePays {
    pub pays: Pays,
    #[serde(flatten)]
    pub veille: Veille,
}

/// La veille de tous les régimes, dans l'ordre de `Pays::TOUS`.
pub fn veille_tous() -> Vec<VeillePays> {
    Pays::TOUS.iter()
        .map(|p| VeillePays { pays: p.clone(), veille: veille(p) })
        .collect()
}

const fn v(integre_jusqu_a: i32, lacunes: &'static [&'static str]) -> Veille {
    Veille { integre_jusqu_a, lacunes, audit_du: AUDIT_DU, derniere_maj: None }
}

pub fn veille(pays: &Pays) -> Veille {
    let mut v = declaree(pays);
    // Les entrées propres à une spécificité (Alsace-Moselle) ne datent pas le
    // régime entier : le front les lit dans le journal quand elle est cochée.
    // Seule une évolution de taux compte, pas l'ajout d'un dispositif.
    v.derniere_maj = JOURNAL.iter()
        .filter(|m| m.pays == *pays && m.specificite.is_none() && m.taux)
        .map(|m| m.date).max();
    v
}

/// Fraîcheur déclarée d'un régime, relevée à la main (voir l'en-tête).
fn declaree(pays: &Pays) -> Veille {
    match pays {
        Pays::France => v(2026, &[]),
        Pays::FonctionPublique => v(2026, &[]),
        // ch_is.rs : table de 27 paliers « valeurs 2025 » et multiplicateurs de tarif —
        // une approximation, pas les fichiers de tarifs de l'AFC.
        Pays::Suisse => v(2025, &[
            "impôt à la source : taux A0 approchés sur 27 paliers et autres tarifs déduits par multiplicateur, et non les barèmes officiels de l'AFC ; barèmes cantonaux 2026 non intégrés",
        ]),
        Pays::Luxembourg => v(2026, &[]),
        Pays::Italia => v(2026, &[]),
        Pays::Canada => v(2026, &[]),
        Pays::Quebec => v(2026, &[]),
        Pays::Allemagne => v(2026, &[]),
        Pays::Espagne => v(2026, &[]),
        Pays::Portugal => v(2026, &[]),
        Pays::Belgique => v(2026, &[]),
        Pays::Angleterre => v(2026, &[]),
        Pays::Japon => v(2026, &[]),
        Pays::Chine => v(2026, &[]),
        Pays::PaysBas => v(2026, &[]),
        Pays::Australie => v(2026, &[]),
        Pays::NouvelleZelande => v(2026, &[]),
        Pays::Pologne => v(2026, &[]),
        Pays::CoreeDuSud => v(2026, &[]),
        Pays::Andorre => v(2026, &[]),
        Pays::Monaco => v(2026, &[]),
        Pays::Danemark => v(2026, &[]),
        Pays::Finlande => v(2026, &[]),
        Pays::Suede => v(2026, &[]),
        Pays::Estonie => v(2026, &[]),
        Pays::Lettonie => v(2026, &[]),
        Pays::Lituanie => v(2026, &[]),
        Pays::Autriche => v(2026, &[]),
        Pays::Tchequie => v(2026, &[]),
        Pays::Slovaquie => v(2026, &[]),
        Pays::Hongrie => v(2026, &[]),
        Pays::Slovenie => v(2026, &[]),
        Pays::Grece => v(2026, &[]),
        Pays::Chypre => v(2026, &[]),
        Pays::Malte => v(2026, &[]),
        Pays::Croatie => v(2026, &[]),
        Pays::Irlande => v(2026, &[]),
        Pays::Roumanie => v(2026, &[]),
        Pays::Bulgarie => v(2026, &[]),
        Pays::EtatsUnis => v(2025, &[
            "impôt d'État de Californie : barème 2026 indexé pas encore publié par la FTB (publication à l'automne) — barème 2025 appliqué",
        ]),
        Pays::Mexique => v(2026, &[]),
        Pays::Bresil => v(2026, &[]),
        Pays::Emirats => v(2026, &[]),
        Pays::Inde => v(2026, &[]),
    }
}
