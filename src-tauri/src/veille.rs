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
