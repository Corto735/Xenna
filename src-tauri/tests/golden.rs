//! Golden « ligne à ligne » — chaque ligne de cotisation figée au centime.
//!
//! `fiabilite.rs` vérifie des fourchettes : il attrape un calcul cassé, pas un
//! taux faux de 0,13 point (le 13,13 % de maladie y a survécu huit ans de
//! simulation). Ici, chaque ligne est comparée à une valeur de RÉFÉRENCE
//! externe : un vrai bulletin (France) ou le barème officiel du pays, relevé
//! et sourcé dans le commentaire du test. Toute ligne produite doit être
//! attendue, toute ligne attendue doit être produite : une cotisation qui
//! disparaît ou qui apparaît casse le test.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::SqlitePool;

use xenna_paie_lib::calculs::generer_bulletin;
use xenna_paie_lib::db::{init_db, ContextPaie};
use xenna_paie_lib::models::{Bulletin, Pays, Salarie, Statut};

// ────────────────────────────── Outils ──────────────────────────────

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

async fn base_test() -> (SqlitePool, PathBuf) {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("xenna_golden_{}_{}.db", std::process::id(), n));
    for suffixe in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{}", path.display(), suffixe));
    }
    let pool = init_db(&path).await.expect("les migrations doivent passer");
    (pool, path)
}

fn nettoyer(path: &PathBuf) {
    for suffixe in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{}", path.display(), suffixe));
    }
}

fn date(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

fn d(s: &str) -> Decimal {
    s.parse().unwrap()
}

/// Salarié neutre : aucun champ régional renseigné, à chaque test de poser
/// ce dont son pays a besoin.
fn salarie(pays: Pays, brut: &str) -> Salarie {
    Salarie {
        nom: "Golden".into(),
        prenom: "Référence".into(),
        salaire_brut: d(brut),
        statut: Statut::NonCadre,
        alsace_moselle: false,
        pays,
        canton: None,
        tarif_is: None,
        assujetti_is: false,
        regione: None,
        contratto_termine: false,
        province: None,
        steuerklasse: None,
        kinderlos: None,
        land: None,
        kirchenmitglied: None,
        region_be: None,
        etp: 100.0,
        entreprise_adaptee: false,
        esat: false,
        convention_idcc: None,
        indemnites_repas: None,
        avantages_nature: Vec::new(),
        tranche_age_ea: None,
        heures_supp_25: 0.0,
        heures_supp_50: 0.0,
        heures_comp_10: 0.0,
        heures_comp_25: 0.0,
        salaire_base: None,
        effectif: Some("moins20".into()),
        anciennete: None,
        us_state: None,
        inde_regime: None,
        emirati_national: None,
    }
}

/// Ligne attendue : (code, base, taux sal., montant sal., taux pat., montant pat.).
type Attendu = (&'static str, &'static str, &'static str, &'static str, &'static str, &'static str);

/// Compare TOUTES les lignes du bulletin aux lignes attendues, au centime
/// (taux à 1e-6), et exige le même ensemble de codes des deux côtés.
fn verifier_lignes(b: &Bulletin, attendues: &[Attendu], contexte: &str) {
    let mut erreurs = Vec::new();
    for (code, base, ts, ms, tp, mp) in attendues {
        let Some(l) = b.cotisations.iter().find(|l| l.code == *code) else {
            erreurs.push(format!("{code} : attendue, absente du bulletin"));
            continue;
        };
        let champs = [
            ("base", l.base, d(base), d("0.01")),
            ("taux sal.", l.taux_sal, d(ts), d("0.000001")),
            ("montant sal.", l.montant_sal, d(ms), d("0.01")),
            ("taux pat.", l.taux_pat, d(tp), d("0.000001")),
            ("montant pat.", l.montant_pat, d(mp), d("0.01")),
        ];
        for (nom, obtenu, attendu, tol) in champs {
            if (obtenu - attendu).abs() > tol {
                erreurs.push(format!("{code} : {nom} = {obtenu}, attendu {attendu}"));
            }
        }
    }
    for l in &b.cotisations {
        if !attendues.iter().any(|(code, ..)| *code == l.code) {
            erreurs.push(format!(
                "{} : ligne inattendue (base {}, sal. {} = {}, pat. {} = {})",
                l.code, l.base, l.taux_sal, l.montant_sal, l.taux_pat, l.montant_pat
            ));
        }
    }
    assert!(erreurs.is_empty(), "[{contexte}]\n  {}", erreurs.join("\n  "));
}

// ─────────────────────────────── France ──────────────────────────────

/// Bulletin réel de septembre 2026 (anonymisé) : non-cadre, entreprise
/// adaptée de moins de 50 salariés, Haut-Rhin (régime local), 121,33 h
/// (80 %), brut 2 460,27 €, plafond SS proratisé 3 203,84 €.
///
/// Écarts connus avec le bulletin, chacun expliqué :
///   - SS plafonnée pat. 210,35 (bulletin 210,36) et famille 129,16
///     (129,15) : arrondi du logiciel de paie, au centime ;
///   - AT/MP : taux propre à l'entreprise (bulletin 3,34 %), défaut 2,35 % ;
///   - chômage 4,00 % + AGS 0,25 % : le bulletin porte 3,20 %, taux
///     bonus-malus de l'entreprise (secteur 5229B) AGS comprise ;
///   - CSG/CRDS : assiette 2 417,22 contre 2 452,21, le bulletin y réintègre
///     prévoyance (14,76) et mutuelle (20,23) patronales, non modélisées ;
///   - réduction générale 227,57 (bulletin 227,31) ;
///   - aide au poste EA : forfait 18 230 € × 80 % / 12 (bulletin : part État
///     1 259,27, à élucider).
/// Lignes du bulletin non modélisées : prévoyance, mutuelle, cotisation
/// conventionnelle, FNAL, contribution solidarité autonomie, dialogue
/// social, formation, taxe d'apprentissage.
#[tokio::test]
async fn golden_france_bulletin_reel_2026_09() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-09-01")).await.unwrap();
    let mut s = salarie(Pays::France, "2460.27");
    s.alsace_moselle = true;
    s.entreprise_adaptee = true;
    s.etp = 80.0;
    let b = generer_bulletin(s, &ctx, None);

    verifier_lignes(&b, &[
        // code                     base       t. sal    m. sal    t. pat    m. pat
        ("SS_MALADIE",             "2460.27", "0",      "0",      "0.13",   "319.84"),
        ("ALSACE_MOSELLE_MALADIE", "2460.27", "0.013",  "31.98",  "0",      "0"),
        ("SS_VIEILLESSE_PLAF",     "2460.27", "0.069",  "169.76", "0.0855", "210.35"),
        ("SS_VIEILLESSE_DEPLAF",   "2460.27", "0.004",  "9.84",   "0.0211", "51.91"),
        ("FAMILLE",                "2460.27", "0",      "0",      "0.0525", "129.16"),
        ("AT_MP",                  "2460.27", "0",      "0",      "0.0235", "57.82"),
        ("CHOMAGE",                "2460.27", "0",      "0",      "0.04",   "98.41"),
        ("AGS",                    "2460.27", "0",      "0",      "0.0025", "6.15"),
        // Retraite complémentaire T1 : le bulletin regroupe 4,01 / 6,01 %.
        ("AGIRC_ARRCO_T1",         "2460.27", "0.0315", "77.50",  "0.0472", "116.12"),
        ("AGIRC_ARRCO_CEG_T1",     "2460.27", "0.0086", "21.16",  "0.0129", "31.74"),
        ("CSG_DEDUCTIBLE",         "2417.22", "0.068",  "164.37", "0",      "0"),
        ("CSG_NON_DEDUCTIBLE",     "2417.22", "0.024",  "58.01",  "0",      "0"),
        ("CRDS",                   "2417.22", "0.005",  "12.09",  "0",      "0"),
        ("REDUCTION_FILLON",       "2460.27", "0",      "0",      "-0.0925","-227.57"),
        ("AIDE_POSTE_EA",          "2460.27", "0",      "0",      "0",      "-1215.33"),
    ], "France, bulletin réel 09/2026");

    // Net : bulletin 1 876,56. L'écart est exactement prévoyance + mutuelle +
    // cotisation conventionnelle (14,76 + 20,23 + 0,62) et la CSG/CRDS sur leur
    // réintégration (237,86 − 234,47 = 3,39).
    assert_eq!(b.net_a_payer, d("1915.56"));
    assert_eq!(b.net_a_payer - d("14.76") - d("20.23") - d("0.62") - d("3.39"), d("1876.56"));

    nettoyer(&path);
}

/// Brut mensuel courant, en monnaie locale, d'un salarié du privé à temps
/// plein (ordre de grandeur du salaire médian) : les goldens ne valent que
/// sur des montants réalistes (4 000 ¥ ou 4 000 ₩ n'ont aucun sens).
fn brut_type(pays: &Pays) -> &'static str {
    match pays {
        Pays::France | Pays::Allemagne | Pays::PaysBas | Pays::Belgique => "4000",
        Pays::FonctionPublique => "2800",
        Pays::Suisse => "6500",
        Pays::Luxembourg | Pays::Monaco => "5000",
        Pays::Italia | Pays::Espagne | Pays::Andorre | Pays::Chypre => "2500",
        Pays::Canada | Pays::Quebec | Pays::EtatsUnis | Pays::Bresil => "5000",
        Pays::Portugal => "1600",
        Pays::Angleterre => "3000",
        Pays::Japon => "350000",
        Pays::Chine => "12000",
        Pays::Australie => "7000",
        Pays::NouvelleZelande => "6000",
        Pays::Pologne => "8000",
        Pays::CoreeDuSud => "3500000",
        Pays::Danemark | Pays::Suede => "40000",
        Pays::Finlande => "3800",
        Pays::Estonie => "2000",
        Pays::Lettonie => "1700",
        Pays::Lituanie => "2200",
        Pays::Autriche => "3500",
        Pays::Tchequie => "45000",
        Pays::Slovaquie | Pays::Croatie => "1800",
        Pays::Hongrie => "650000",
        Pays::Slovenie => "2400",
        Pays::Grece => "1500",
        Pays::Malte => "2000",
        Pays::Irlande => "4500",
        Pays::Roumanie => "8000",
        Pays::Bulgarie => "1200",
        Pays::Mexique => "15000",
        Pays::Emirats => "15000",
        Pays::Inde => "50000",
    }
}

// ───────────────────────── Autres régimes ────────────────────────────
//
// Un test par régime, au 01/09/2026, sur le brut de `brut_type` et les
// régions par défaut de `regionaliser`. Chaque taux est confronté au barème
// officiel cité ; chaque montant a été recalculé à la main.

/// Régions par défaut des régimes qui en ont besoin.
fn regionaliser(s: &mut Salarie) {
    s.canton = Some("GE".into());
    s.regione = Some("LO".into());
    s.province = Some("ON".into());
    s.steuerklasse = Some(1);
    s.kinderlos = Some(false);
    s.land = Some("BY".into());
    s.kirchenmitglied = Some(false);
    s.region_be = Some("bruxelles".into());
}

async fn golden_regime(pays: Pays, attendues: &[Attendu], net: &str, cout: &str) {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-09-01")).await.unwrap();
    let mut s = salarie(pays.clone(), brut_type(&pays));
    regionaliser(&mut s);
    let b = generer_bulletin(s, &ctx, None);
    let contexte = format!("{pays:?} 09/2026 @ {}", brut_type(&pays));
    verifier_lignes(&b, attendues, &contexte);
    assert_eq!(b.net_a_payer, d(net), "[{contexte}] net");
    assert_eq!(b.cout_total_employeur, d(cout), "[{contexte}] coût employeur");
    nettoyer(&path);
}

/// Suisse (Genève), 6 500 CHF. AVS 4,35 %, AI 0,70 %, APG 0,25 % (8,70 %,
/// 1,40 %, 0,50 % au total) ; AC 1,10 % jusqu'à 148 200 CHF/an ; LPP sur le
/// salaire coordonné 6 500 − 26 460 / 12 = 4 295 (OFAS, montants limites
/// 2025-2026), 5 % + 5 % (bonification 35-44 ans). AANP, AAP, IJM : taux
/// indicatifs d'assureur. Impôt à la source non retenu (salarié non assujetti).
#[tokio::test]
async fn golden_suisse() {
    golden_regime(Pays::Suisse, &[
        ("CH_AVS",  "6500"   , "0.0435" , "282.75" , "0.0435" , "282.75" ),
        ("CH_AI",   "6500"   , "0.0070" , "45.50"  , "0.0070" , "45.50"  ),
        ("CH_APG",  "6500"   , "0.0025" , "16.25"  , "0.0025" , "16.25"  ),
        ("CH_AC",   "6500"   , "0.0110" , "71.50"  , "0.0110" , "71.50"  ),
        ("CH_AANP", "6500"   , "0.0100" , "65.00"  , "0"      , "0"      ),
        ("CH_AAP",  "6500"   , "0"      , "0"      , "0.0100" , "65.00"  ),
        ("CH_IJM",  "6500"   , "0.0075" , "48.75"  , "0.0075" , "48.75"  ),
        ("CH_LPP",  "4295.00", "0.0500" , "214.75" , "0.0500" , "214.75" ),
    ], "5755.50", "7244.50").await;
}

/// Luxembourg, 5 000 €. Pension 8,50 % + 8,50 % (réforme au 01/01/2026),
/// maladie 3,05 % + 3,05 %, dépendance 1,40 % sur 5 000 − 2 771,33 / 4 =
/// 4 307,17 (abattement d'un quart du SSM, indice 992,24 depuis juin 2026),
/// accidents 0,65 % (taux unique 2026), mutualité classe 2 0,95 % (FEDIL,
/// paramètres sociaux au 01/06/2026). Aucun impôt modélisé.
#[tokio::test]
async fn golden_luxembourg() {
    golden_regime(Pays::Luxembourg, &[
        ("LU_AP", "5000"   , "0.0850" , "425.00" , "0.0850" , "425.00" ),
        ("LU_AM", "5000"   , "0.0305" , "152.50" , "0.0305" , "152.50" ),
        ("LU_AD", "4307.17", "0.0140" , "60.30"  , "0"      , "0"      ),
        ("LU_AA", "5000"   , "0"      , "0"      , "0.0065" , "32.50"  ),
        ("LU_ME", "5000"   , "0"      , "0"      , "0.0095" , "47.50"  ),
    ], "4362.20", "5657.50").await;
}

/// Fonctionnaire territorial titulaire CNRACL, 2 800 € de traitement.
/// Maladie 9,88 %, CNRACL 11,10 % / 37,65 %, famille 5,25 %, ATIACL 0,40 %,
/// FNAL 0,10 % (≤ PMSS, moins de 50 agents), CSA 0,30 %, CNFPT 1 %, CSG/CRDS
/// sur 98,25 % (CDG 44, « Cotisations 2026 — régime spécial CNRACL »).
#[tokio::test]
async fn golden_fonction_publique() {
    golden_regime(Pays::FonctionPublique, &[
        ("FPT_MALADIE",        "2800"   , "0"      , "0"      , "0.0988" , "276.64" ),
        ("FPT_CNRACL",         "2800"   , "0.1110" , "310.80" , "0.3765" , "1054.20"),
        ("FAMILLE",            "2800"   , "0"      , "0"      , "0.0525" , "147.00" ),
        ("FPT_ATIACL",         "2800"   , "0"      , "0"      , "0.0040" , "11.20"  ),
        ("FPT_FNAL",           "2800"   , "0"      , "0"      , "0.0010" , "2.80"   ),
        ("FPT_CSA",            "2800"   , "0"      , "0"      , "0.0030" , "8.40"   ),
        ("FPT_CNFPT",          "2800"   , "0"      , "0"      , "0.0100" , "28.00"  ),
        ("CSG_DEDUCTIBLE",     "2751.00", "0.0680" , "187.07" , "0"      , "0"      ),
        ("CSG_NON_DEDUCTIBLE", "2751.00", "0.0240" , "66.02"  , "0"      , "0"      ),
        ("CRDS",               "2751.00", "0.0050" , "13.76"  , "0"      , "0"      ),
    ], "2222.35", "4328.24").await;
}

/// Italie (Lombardie), 2 500 €. IVS 9,19 % / 23,81 % ; imponibile IRPEF =
/// 2 500 − 229,75 = 2 270,25, soit 27 243 €/an (art. 51 TUIR) ; IRPEF 2026
/// 23 % jusqu'à 28 000 € (L. 199/2025) = 6 265,89 € ; détraction salarié
/// 1 910 + 1 190 × 757 / 13 000 + 65 = 2 044,29 € (art. 13 TUIR) ; net
/// 4 221,60 €/an = 351,80 €/mois. Coin fiscal : 1 000 € entre 20 000 et
/// 32 000 € (L. 207/2024) = 83,33 €/mois. Addition régionale au taux de base
/// lombard 1,23 % (tranches progressives non reproduites). Charges patronales
/// INPS d'un profil industrie générique, INAIL indicatif.
#[tokio::test]
async fn golden_italie() {
    golden_regime(Pays::Italia, &[
        ("IT_IVS",            "2500"   , "0.0919" , "229.75" , "0.2381" , "595.25" ),
        ("IT_NASPI",          "2500"   , "0"      , "0"      , "0.0161" , "40.25"  ),
        ("IT_MALATTIA",       "2500"   , "0"      , "0"      , "0.0222" , "55.50"  ),
        ("IT_MATERNITA",      "2500"   , "0"      , "0"      , "0.0046" , "11.50"  ),
        ("IT_FONDO_GARANZIA", "2500"   , "0"      , "0"      , "0.0020" , "5.00"   ),
        ("IT_INAIL",          "2500"   , "0"      , "0"      , "0.0065" , "16.25"  ),
        ("IT_TFR",            "2500"   , "0"      , "0"      , "0.0691" , "172.75" ),
        ("IT_IRPEF",          "2270.25", "0.1407" , "351.80" , "0"      , "0"      ),
        ("IT_BONUS_CUNEO",    "2500"   , "-0.033332", "-83.33" , "0"      , "0"      ),
        ("IT_ADD_REG_LO",     "2270.25", "0.0123" , "27.92"  , "0"      , "0"      ),
    ], "1973.86", "3223.75").await;
}

/// Canada (Ontario), 5 000 $ CA. RPC 5,95 % sur 5 000 − 3 500 / 12 ;
/// AE 1,63 % / 2,282 % (EDSC 2026). Impôt selon l'ARC, T4127 122ᵉ édition :
/// A = 60 000 − 565,01 (RPC supplémentaire) = 59 434,99 ; fédéral 14 % jusqu'à
/// 58 523 $ puis 20,5 % = 8 380,18 − K1 2 303,28 − K2 528,47 − K4 210,14 =
/// 5 338,29 $/an = 444,86 $/mois ; Ontario 5,05 % / 9,15 % = 3 228,77 − K1P
/// 655,94 − K2P 190,63 = 2 382,20, sans surtaxe ni réduction, + contribution
/// santé 600 $ = 2 982,20 $/an = 248,52 $/mois.
#[tokio::test]
async fn golden_canada() {
    golden_regime(Pays::Canada, &[
        ("CA_RPC",        "4708.33", "0.0595" , "280.15" , "0.0595" , "280.15" ),
        ("CA_AE",         "5000"   , "0.0163" , "81.50"  , "0.02282", "114.10" ),
        ("CA_IMPOT_FED",  "5000"   , "0.0890" , "444.86" , "0"      , "0"      ),
        ("ON_IMPOT_PROV", "5000"   , "0.0497" , "248.52" , "0"      , "0"      ),
    ], "3944.97", "5394.25").await;
}

/// Québec, 5 000 $ CA. RRQ 6,30 % (5,30 % de base + 1 %), AE 1,30 % / 1,82 %,
/// RQAP 0,430 % / 0,602 % (Revenu Québec 2026) ; FSS au taux indicatif 2,05 %,
/// CNT 0,06 %. Fédéral (T4127) : A = 59 435,01, 8 380,18 − 2 303,28 − K2Q
/// 564,54 (RRQ de base, AE, RQAP) − 210,14 = 5 302,22, moins l'abattement du
/// Québec de 16,5 % = 4 427,35 $/an. Québec (TP-1015.F) : 60 000 − 564,99 −
/// déduction pour travailleurs 1 450 = 57 985,01 ; 14 % / 19 % = 8 299,90 −
/// 14 % × 18 952 = 5 646,62 $/an = 470,55 $/mois.
#[tokio::test]
async fn golden_quebec() {
    golden_regime(Pays::Quebec, &[
        ("QC_RRQ",        "4708.33", "0.0630" , "296.62" , "0.0630" , "296.62" ),
        ("QC_AE",         "5000"   , "0.0130" , "65.00"  , "0.01820", "91.00"  ),
        ("QC_RQAP",       "5000"   , "0.00430", "21.50"  , "0.00602", "30.10"  ),
        ("QC_FSS",        "5000"   , "0"      , "0"      , "0.0205" , "102.50" ),
        ("QC_CNT",        "5000"   , "0"      , "0"      , "0.0006" , "3.00"   ),
        ("CA_IMPOT_FED",  "5000"   , "0.0738" , "368.95" , "0"      , "0"      ),
        ("QC_IMPOT_PROV", "5000"   , "0.0941" , "470.55" , "0"      , "0"      ),
    ], "3777.38", "5523.22").await;
}

/// Allemagne (Bavière), 4 000 €, Steuerklasse I, un enfant, sans Église.
/// KV 14,6 % + Zusatzbeitrag moyen 2,9 % (8,75 % chacun), RV 18,6 %, AV 2,6 %,
/// PV 3,6 % (avec enfant) ; UV patronale indicative 1,3 %. Lohnsteuer (PAP
/// 2026) : Vorsorgepauschale 4 464 (RV) + 12 × (338 + 72) (KV au taux réduit
/// 7,0 % + 1,45 %, PV) = 9 384 ; zvE = 48 000 − 1 230 − 36 − 9 384 = 37 350 ;
/// §32a 2026 zone 3 : (173,10 z + 2 397) z + 1 034,87 avec z = 1,9551 →
/// 6 382 €/an = 531,83 €/mois ; Soli nul (seuil 20 350 €).
#[tokio::test]
async fn golden_allemagne() {
    golden_regime(Pays::Allemagne, &[
        ("DE_KRANKENVERSICHERUNG",      "4000"   , "0.0875" , "350.00" , "0.0875" , "350.00" ),
        ("DE_RENTENVERSICHERUNG",       "4000"   , "0.0930" , "372.00" , "0.0930" , "372.00" ),
        ("DE_ARBEITSLOSENVERSICHERUNG", "4000"   , "0.0130" , "52.00"  , "0.0130" , "52.00"  ),
        ("DE_PFLEGEVERSICHERUNG",       "4000"   , "0.0180" , "72.00"  , "0.0180" , "72.00"  ),
        ("DE_UNFALLVERSICHERUNG",       "4000"   , "0"      , "0"      , "0.0130" , "52.00"  ),
        ("DE_LOHNSTEUER",               "4000"   , "0.1330" , "531.83" , "0"      , "0"      ),
    ], "2622.17", "4898.00").await;
}

/// Espagne, 2 500 €, CDI. Contingences communes 4,70 % / 23,60 %, chômage
/// 1,55 % / 5,50 %, FOGASA 0,20 %, formation 0,10 % / 0,60 %, MEI 0,15 % /
/// 0,75 % (Orden PJC/297/2026) ; base entre 1 424,40 et 5 101,20 €. Aucune
/// retenue IRPF modélisée : le net est avant impôt.
#[tokio::test]
async fn golden_espagne() {
    golden_regime(Pays::Espagne, &[
        ("ES_CC",        "2500"   , "0.0470" , "117.50" , "0.2360" , "590.00" ),
        ("ES_DESEMPLEO", "2500"   , "0.0155" , "38.75"  , "0.0550" , "137.50" ),
        ("ES_FOGASA",    "2500"   , "0"      , "0"      , "0.0020" , "5.00"   ),
        ("ES_FP",        "2500"   , "0.0010" , "2.50"   , "0.0060" , "15.00"  ),
        ("ES_MEI",       "2500"   , "0.0015" , "3.75"   , "0.0075" , "18.75"  ),
    ], "2337.50", "3266.25").await;
}

/// Portugal (Continent), 1 600 €, non marié sans personne à charge. Segurança
/// Social 11 % / 23,75 % ; assurance accidents du travail indicative 1,75 % ;
/// FCT et FGCT supprimés depuis le 01/05/2023. Retenue IRS selon la table
/// officielle I 2026 (Despacho n.º 233-A/2026) : 1 600 × 24,10 % − 193,33 =
/// 192,27 €.
#[tokio::test]
async fn golden_portugal() {
    golden_regime(Pays::Portugal, &[
        ("PT_SS",     "1600"   , "0.1100" , "176.00" , "0.2375" , "380.00" ),
        ("PT_AT_SEG", "1600"   , "0"      , "0"      , "0.0175" , "28.00"  ),
        ("PT_IRS",    "1600"   , "0.1202" , "192.27" , "0"      , "0"      ),
    ], "1231.73", "2008.00").await;
}

/// Belgique (Bruxelles), 4 000 €. ONSS travailleur 13,07 % ; ONSS patronal
/// 25,92 % (taux indicatif de Xenna, non confirmé : l'ONSS publie 24,92 %
/// globalisé, hors fonds selon l'effectif). Précompte professionnel (barèmes
/// revenus 2026, Fiscoliste Securex janvier 2026) : (4 000 − 522,80) × 12 =
/// 41 726,40 − frais forfaitaires 6 070 = 35 656,40 ; 25 % / 40 % / 45 %
/// = 12 061,88 − quotité exemptée 11 180 × 25 % = 9 266,88 €/an = 772,24 €.
#[tokio::test]
async fn golden_belgique() {
    golden_regime(Pays::Belgique, &[
        ("BE_ONSS_SAL", "4000"   , "0.1307" , "522.80" , "0"      , "0"      ),
        ("BE_ONSS_PAT", "4000"   , "0"      , "0"      , "0.2592" , "1036.80"),
        ("BE_PP",       "4000"   , "0.1931" , "772.24" , "0"      , "0"      ),
    ], "2704.96", "5036.80").await;
}

/// Royaume-Uni (Angleterre), 3 000 £/mois, code 1257L, exercice 2026/27.
/// NI salarié 8 % entre 1 048 et 4 189 £/mois (seuils mensuels HMRC) =
/// 156,16 £ ; NI employeur 15 % au-delà de 417 £/mois = 387,45 £ ; impôt
/// (36 000 − 12 570) × 20 % / 12 = 390,50 £ (GOV.UK, rates and thresholds
/// 2026 to 2027).
#[tokio::test]
async fn golden_royaume_uni() {
    golden_regime(Pays::Angleterre, &[
        ("UK_NI_SAL",     "3000"   , "0.0521" , "156.16" , "0"      , "0"      ),
        ("UK_NI_PAT",     "3000"   , "0"      , "0"      , "0.1292" , "387.45" ),
        ("UK_INCOME_TAX", "3000"   , "0.1302" , "390.50" , "0"      , "0"      ),
    ], "2453.34", "3387.45").await;
}

/// Japon (Tokyo, 40-64 ans), 350 000 ¥. Maladie, dépendance et soutien à
/// l'enfance sur la rémunération standard de 360 000 ¥ (palier 350 000 -
/// 370 000) : Kyokai Kenpo Tokyo 9,85 % (R8), 介護 1,62 %, 子ども・子育て支援金
/// 0,23 % ; pension 18,3 % sur 360 000 ; assurance emploi 0,5 % / 0,85 %
/// (R8) et accidents 0,3 % sur le salaire réel ; part salariale arrondie à
/// l'entier, 0,50 ¥ et moins tronqués. Impôt sur le revenu et taxe résidentielle
/// estimés à l'année (barème R8, non par la table mensuelle 源泉徴収税額表).
#[tokio::test]
async fn golden_japon() {
    golden_regime(Pays::Japon, &[
        ("JP_KENPO",      "360000" , "0.04925", "17730"  , "0.04925", "17730"  ),
        ("JP_KAIGO",      "360000" , "0.0081" , "2916"   , "0.0081" , "2916"   ),
        ("JP_KODOMO",     "360000" , "0.00115", "414"    , "0.00115", "414"    ),
        ("JP_KOSEI",      "360000" , "0.0915" , "32940"  , "0.0915" , "32940"  ),
        ("JP_KOYO",       "350000" , "0.0050" , "1750"   , "0.0085" , "2975"   ),
        ("JP_ROUSAI",     "350000" , "0"      , "0"      , "0.0030" , "1050"   ),
        ("JP_SHOTOKUZEI", "350000" , "0.0147" , "5152"   , "0"      , "0"      ),
        ("JP_JUMINZEI",   "350000" , "0.0434" , "15175"  , "0"      , "0"      ),
    ], "273923", "408025").await;
}

// ────────────────────────────── Relevé ───────────────────────────────

/// Outil, pas un test : imprime toutes les lignes de chaque régime pour un
/// brut mensuel courant, de quoi relever les valeurs à confronter aux
/// barèmes officiels avant d'écrire un golden.
/// `cargo test --test golden releve -- --ignored --nocapture`
#[tokio::test]
#[ignore]
async fn releve_tous_pays() {
    let (pool, path) = base_test().await;
    let ctx = ContextPaie::charger(&pool, date("2026-09-01")).await.unwrap();
    for pays in Pays::TOUS.iter().cloned() {
        let mut s = salarie(pays.clone(), brut_type(&pays));
        s.canton = Some("GE".into());
        s.regione = Some("LO".into());
        s.province = Some("ON".into());
        s.steuerklasse = Some(1);
        s.kinderlos = Some(false);
        s.land = Some("BY".into());
        s.kirchenmitglied = Some(false);
        s.region_be = Some("bruxelles".into());
        let b = generer_bulletin(s, &ctx, None);
        println!("\n=== {pays:?} ({}) brut {} net {} coût {}", b.devise, b.brut, b.net_a_payer, b.cout_total_employeur);
        for l in &b.cotisations {
            println!("  {:<30} base {:>10} | sal {:>9} {:>10} | pat {:>9} {:>10}",
                l.code, l.base, l.taux_sal, l.montant_sal, l.taux_pat, l.montant_pat);
        }
    }
    nettoyer(&path);
}

// ──────────────────────────── Bornes de période ─────────────────────────────

/// Une paie datée du dernier jour d'une période de taux doit encore trouver le
/// taux : le moteur lit les dates de fin comme exclusives, et une date de fin
/// inclusive (« 2026-02-28 ») faisait disparaître la cotisation ce jour-là.
#[tokio::test]
async fn aucun_taux_perdu_en_fin_de_periode() {
    let (pool, path) = base_test().await;
    let trous: Vec<(String, String)> = sqlx::query_as(
        "SELECT c.code, a.date_fin FROM cotisation_taux a JOIN cotisation c ON c.id = a.cotisation_id
          WHERE a.date_fin IS NOT NULL AND strftime('%d', date(a.date_fin, '+1 day')) = '01'
         UNION ALL
         SELECT code, date_fin FROM plafond_reference
          WHERE date_fin IS NOT NULL AND strftime('%d', date(date_fin, '+1 day')) = '01'")
        .fetch_all(&pool).await.unwrap();
    assert!(trous.is_empty(), "dates de fin inclusives : {trous:?}");

    // Japon au 28/02/2026 : maladie au taux R7 (9,91 %), présente.
    let ctx = ContextPaie::charger(&pool, date("2026-02-28")).await.unwrap();
    assert_eq!(ctx.taux_sal("JP_KENPO"), d("0.04955"));
    nettoyer(&path);
}
