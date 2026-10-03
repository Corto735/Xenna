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

/// Chine (Pékin), 12 000 ¥. Assiette entre 7 270 et 36 348 ¥ (07/2026) :
/// pension 8 % / 16 %, maladie 2 % / 9 % (fonds des grosses dépenses compris),
/// chômage 0,5 % / 0,5 %, accidents 0,4 %, maternité 0,8 %, fonds logement
/// 12 % / 12 %. IIT : (12 000 − 5 000 − 1 260 − 1 440) × 12 = 51 600 ¥ → 10 %
/// − 2 520 = 2 640 ¥/an, soit 220 ¥/mois en moyenne (la méthode cumulative
/// réelle commence à 3 % en janvier). Les 3 ¥ forfaitaires de grosses
/// dépenses du salarié ne sont pas modélisés.
#[tokio::test]
async fn golden_chine() {
    golden_regime(Pays::Chine, &[
        ("CN_YANGLAO",   "12000"  , "0.0800" , "960.00" , "0.1600" , "1920.00"),
        ("CN_YILIAO",    "12000"  , "0.0200" , "240.00" , "0.0900" , "1080.00"),
        ("CN_SHIYE",     "12000"  , "0.0050" , "60.00"  , "0.0050" , "60.00"  ),
        ("CN_GONGSHANG", "12000"  , "0"      , "0"      , "0.0040" , "48.00"  ),
        ("CN_SHENGYU",   "12000"  , "0"      , "0"      , "0.0080" , "96.00"  ),
        ("CN_GONGJIJIN", "12000"  , "0.1200" , "1440.00", "0.1200" , "1440.00"),
        ("CN_IIT",       "12000"  , "0.0183" , "220.00" , "0"      , "0"      ),
    ], "9080.00", "16644.00").await;
}

/// Pays-Bas, 4 000 €/mois sans pécule de vacances. Loonheffing 2026 :
/// 38 883 × 35,75 % + 9 117 × 37,56 % = 17 325,02 − algemene heffingskorting
/// 1 946,45 − arbeidskorting 5 528,24 = 9 850,33 €/an = 820,86 €/mois.
/// Employeur : Zvw 6,10 %, AWf 2,74 %, Aof (petit employeur) 6,27 %, Whk
/// indicatif 1,52 %, opslag kinderopvang 0,50 %.
#[tokio::test]
async fn golden_pays_bas() {
    golden_regime(Pays::PaysBas, &[
        ("NL_LOONHEFFING", "4000"   , "0.2052" , "820.86" , "0"      , "0"      ),
        ("NL_ZVW",         "4000"   , "0"      , "0"      , "0.0610" , "244.00" ),
        ("NL_AWF",         "4000"   , "0"      , "0"      , "0.0274" , "109.60" ),
        ("NL_AOF",         "4000"   , "0"      , "0"      , "0.0627" , "250.80" ),
        ("NL_WHK",         "4000"   , "0"      , "0"      , "0.0152" , "60.80"  ),
        ("NL_OPSLAG_KO",   "4000"   , "0"      , "0"      , "0.0050" , "20.00"  ),
    ], "3179.14", "4685.20").await;
}

/// Australie, 7 000 $ AU/mois, exercice 2026-27 (première tranche abaissée à
/// 15 % au 01/07/2026) : 18 200 / 45 000 / 135 000 $ ; 84 000 $ → 4 019,85 +
/// 39 000 × 30 % = 15 719,85 $/an = 1 310 $/mois ; Medicare 2 % ; super
/// guarantee 12 %.
#[tokio::test]
async fn golden_australie() {
    golden_regime(Pays::Australie, &[
        ("AU_INCOME_TAX", "7000"   , "0.1871" , "1310.00", "0"      , "0"      ),
        ("AU_MEDICARE",   "7000"   , "0.02"   , "140.00" , "0"      , "0"      ),
        ("AU_SUPER",      "7000"   , "0"      , "0"      , "0.1200" , "840.00" ),
    ], "5550.00", "7840.00").await;
}

/// Nouvelle-Zélande, 6 000 $ NZ/mois : PAYE 10,5 / 17,5 / 30 % (seuils
/// 15 600 / 53 500 / 78 100 $) = 13 820,50 $/an = 1 151,71 $/mois ; ACC
/// 1,75 % (2026/27) ; KiwiSaver employeur 3,5 % (minimum relevé au
/// 01/04/2026), cotisation salariée et ESCT non modélisées.
#[tokio::test]
async fn golden_nouvelle_zelande() {
    golden_regime(Pays::NouvelleZelande, &[
        ("NZ_PAYE",          "6000"   , "0.1920" , "1151.71", "0"      , "0"      ),
        ("NZ_ACC",           "6000"   , "0.0175" , "105.00" , "0"      , "0"      ),
        ("NZ_KIWISAVER_EMP", "6000"   , "0"      , "0"      , "0.0350" , "210.00" ),
    ], "4743.29", "6210.00").await;
}

/// Pologne, 8 000 PLN. ZUS : emerytalne 9,76 % / 9,76 %, rentowe 1,5 % / 6,5 %,
/// chorobowe 2,45 %, wypadkowe 1,67 %, FP 2,45 %, FGŚP 0,10 % ; zdrowotne
/// 9 % de 6 903,20. Avance PIT : 8 000 − 1 096,80 − 250 = 6 653,20 → 6 653 ×
/// 12 % − 300 = 498,36 → 498 PLN (arrondis au złoty, Ordynacja art. 63).
#[tokio::test]
async fn golden_pologne() {
    golden_regime(Pays::Pologne, &[
        ("PL_EMERYTALNE", "8000"   , "0.0976" , "780.80" , "0.0976" , "780.80" ),
        ("PL_RENTOWE",    "8000"   , "0.0150" , "120.00" , "0.0650" , "520.00" ),
        ("PL_CHOROBOWE",  "8000"   , "0.0245" , "196.00" , "0"      , "0"      ),
        ("PL_WYPADKOWE",  "8000"   , "0"      , "0"      , "0.0167" , "133.60" ),
        ("PL_FP",         "8000"   , "0"      , "0"      , "0.0245" , "196.00" ),
        ("PL_FGSP",       "8000"   , "0"      , "0"      , "0.0010" , "8.00"   ),
        ("PL_ZDROWOTNE",  "6903.20", "0.09"   , "621.29" , "0"      , "0"      ),
        ("PL_PIT",        "8000"   , "0.0622" , "498"    , "0"      , "0"      ),
    ], "5783.91", "9638.40").await;
}

/// Corée du Sud, 3 500 000 ₩. Pension 4,75 % / 4,75 % (2026), santé 3,595 %
/// chacun, dépendance 13,14 % de la prime santé, emploi 0,9 % / 1,15 %,
/// accidents indicatif 0,7 %. Impôt annualisé : 42 M − 근로소득공제 11,55 M −
/// 기본공제 1,5 M − cotisations 4 081 296 = 24 868 704 ₩ → 2 470 306 −
/// crédit salarial 668 000 − crédit standard 130 000 = 1 672 306 ₩/an =
/// 139 359 ₩/mois ; taxe locale 10 %. Approximation de la table 간이세액표.
#[tokio::test]
async fn golden_coree() {
    golden_regime(Pays::CoreeDuSud, &[
        ("KR_NPS",        "3500000", "0.0475" , "166250" , "0.0475" , "166250" ),
        ("KR_NHI",        "3500000", "0.03595", "125825" , "0.03595", "125825" ),
        ("KR_LTC",        "125825" , "0.1314" , "16533"  , "0.1314" , "16533"  ),
        ("KR_EI",         "3500000", "0.009"  , "31500"  , "0.0115" , "40250"  ),
        ("KR_SANJAE",     "3500000", "0"      , "0"      , "0.007"  , "24500"  ),
        ("KR_INCOME_TAX", "3500000", "0.0398" , "139359" , "0"      , "0"      ),
        ("KR_LOCAL_TAX",  "139359" , "0.10"   , "13936"  , "0"      , "0"      ),
    ], "3006597", "3873358").await;
}

/// Andorre, 2 500 €. CASS 6,5 % / 15,5 %. IRPF : 0 % jusqu'à 24 000 €/an,
/// 5 % jusqu'à 40 000 € : (30 000 − 24 000) × 5 % = 300 €/an = 25 €/mois.
/// Déductibilité des cotisations CASS non vérifiée (non appliquée).
#[tokio::test]
async fn golden_andorre() {
    golden_regime(Pays::Andorre, &[
        ("AD_CASS", "2500"   , "0.065"  , "162.50" , "0.155"  , "387.50" ),
        ("AD_IRPF", "2500"   , "0.01"   , "25.00"  , "0"      , "0"      ),
    ], "2312.50", "2887.50").await;
}

/// Monaco, 5 000 €, période d'octobre 2025 à septembre 2026 (Caisses sociales,
/// lettres aux employeurs) : CAR 6,85 % / 8,33 % (base 7,45 % + variable
/// 0,88 %) sous le plafond de 6 112 € ; CCSS 13,40 % sous 9 800 € ; chômage
/// 2,40 % / 4,00 % sous 15 700 € ; CMRC tranche A (3 971 €) 4,008 % / 6,012 %,
/// tranche B 9,716 % / 14,574 %. Pas d'impôt sur le revenu.
#[tokio::test]
async fn golden_monaco() {
    golden_regime(Pays::Monaco, &[
        ("MC_CAR",     "5000"   , "0.0685" , "342.50" , "0.0833" , "416.50" ),
        ("MC_CCSS",    "5000"   , "0"      , "0"      , "0.1340" , "670.00" ),
        ("MC_CHOM",    "5000"   , "0.024"  , "120.00" , "0.040"  , "200.00" ),
        ("MC_CMRC_TA", "3971"   , "0.04008", "159.16" , "0.06012", "238.74" ),
        ("MC_CMRC_TB", "1029"   , "0.09716", "99.98"  , "0.14574", "149.97" ),
    ], "4278.36", "6675.21").await;
}

/// Danemark, 40 000 DKK. AM-bidrag 8 % ; ATP temps plein 2026 99 / 198 DKK.
/// Impôt : bundskat 12,01 % sur 36 800 − 99 − personfradrag 4 508,33 =
/// 32 192,67 ; kommuneskat moyen 25,049 % sur la même base moins
/// beskæftigelsesfradrag (12,75 % × 480 000 = 61 200) et jobfradrag (3 100)
/// par mois = 26 834,34 → 3 866,34 + 6 721,73 = 10 588,07 DKK.
#[tokio::test]
async fn golden_danemark() {
    golden_regime(Pays::Danemark, &[
        ("DK_AM",           "40000"  , "0.08"   , "3200.00", "0"      , "0"      ),
        ("DK_ATP",          "40000"  , "0"      , "99.00"  , "0"      , "198.00" ),
        ("DK_INDKOMSTSKAT", "40000"  , "0.2647" , "10588.07", "0"      , "0"      ),
    ], "26112.93", "40198.00").await;
}

/// Finlande, 3 800 €. TyEL 7,30 % / 17,10 % ; chômage 0,89 % / 0,31 % ;
/// päiväraha 0,88 % ; sairaanhoito 1,10 % ; employeur 1,91 %. Impôt 2026 :
/// 45 600 − 750 − cotisations 4 135,92 = 40 714,08 ; État 7 267,73 (seuil
/// 22 000) ; communal 7,57 % = 3 082,06 ; työtulovähennys 3 430 − 2 % ×
/// 5 714,08 = 3 315,72 → 7 034,07 €/an = 586,17 €/mois.
#[tokio::test]
async fn golden_finlande() {
    golden_regime(Pays::Finlande, &[
        ("FI_TYEL",          "3800"   , "0.073"  , "277.40" , "0.1710" , "649.80" ),
        ("FI_TYOTTOMYYS",    "3800"   , "0.0089" , "33.82"  , "0.0031" , "11.78"  ),
        ("FI_PAIVARAHA",     "3800"   , "0.0088" , "33.44"  , "0"      , "0"      ),
        ("FI_SAIRAANHOITO",  "3800"   , "0.0110" , "41.80"  , "0"      , "0"      ),
        ("FI_TYONANTAJA_SV", "3800"   , "0"      , "0"      , "0.0191" , "72.58"  ),
        ("FI_TULOVERO",      "3800"   , "0.1543" , "586.17" , "0"      , "0"      ),
    ], "2827.37", "4534.16").await;
}

/// Suède, 40 000 SEK, moins de 66 ans (Skatteverket, SKV 433 éd. 36) :
/// arbetsgivaravgifter 31,42 % ; grundavdrag 0,293 PBB → 17 400 ; impôt
/// communal moyen 32,38 % sur 462 600 = 149 789 ; jobbskatteavdrag (3,027 PBB
/// − 17 400) × 32,38 % = 52 390 ; réduction pour revenu d'activité 1 500 ;
/// public service 1 184 → 97 083 SEK/an = 8 090,25 SEK/mois.
#[tokio::test]
async fn golden_suede() {
    golden_regime(Pays::Suede, &[
        ("SE_ARBETSGIVARAVGIFT", "40000"  , "0"      , "0"      , "0.3142" , "12568.00"),
        ("SE_SKATT",             "40000"  , "0.2023" , "8090.25", "0"      , "0"      ),
    ], "31909.75", "52568.00").await;
}

/// Estonie, 2 000 €. Chômage 1,6 % / 0,8 %, 2ᵉ pilier 2 %, taxe sociale 33 %.
/// Impôt 22 % sur 2 000 − 32 − 40 − 700 (abattement uniforme 2026) = 270,16 €.
#[tokio::test]
async fn golden_estonie() {
    golden_regime(Pays::Estonie, &[
        ("EE_TOOTUS",         "2000"   , "0.016"  , "32.00"  , "0.008"  , "16.00"  ),
        ("EE_KOGUMISPENSION", "2000"   , "0.02"   , "40.00"  , "0"      , "0"      ),
        ("EE_SOTSIAALMAKS",   "2000"   , "0"      , "0"      , "0.33"   , "660.00" ),
        ("EE_TULUMAKS",       "2000"   , "0.1351" , "270.16" , "0"      , "0"      ),
    ], "1657.84", "2676.00").await;
}

/// Lettonie, 1 700 €. VSAOI 10,5 % / 23,59 % ; IIN 25,5 % sur 1 700 − 178,50
/// − minimum non imposable 550 € = 971,50 → 247,73 €.
#[tokio::test]
async fn golden_lettonie() {
    golden_regime(Pays::Lettonie, &[
        ("LV_VSAOI", "1700"   , "0.105"  , "178.50" , "0.2359" , "401.03" ),
        ("LV_IIN",   "1700"   , "0.1457" , "247.73" , "0"      , "0"      ),
    ], "1273.77", "2101.03").await;
}

/// Lituanie, 2 200 €. Sodra 19,5 % / 1,77 % ; NPD 2026 = 747 − 0,49 × (2 200 −
/// 1 153) = 233,97 ; GPM 20 % × (2 200 − 233,97) = 393,21 €.
#[tokio::test]
async fn golden_lituanie() {
    golden_regime(Pays::Lituanie, &[
        ("LT_SODRA", "2200"   , "0.195"  , "429.00" , "0.0177" , "38.94"  ),
        ("LT_GPM",   "2200"   , "0.1787" , "393.21" , "0"      , "0"      ),
    ], "1377.79", "2238.94").await;
}

/// Autriche, 3 500 €, mois ordinaire (13ᵉ et 14ᵉ mois non modélisés). SV
/// 18,07 % / 21,03 % ; Lohnsteuer 2026 : (3 500 − 632,45) × 12 − 132 =
/// 34 278,60 → 1 690,60 + 30 % × 12 286,60 = 5 376,58 − Verkehrsabsetzbetrag
/// 496 = 4 880,58 €/an = 406,72 €/mois.
#[tokio::test]
async fn golden_autriche() {
    golden_regime(Pays::Autriche, &[
        ("AT_SV",         "3500"   , "0.1807" , "632.45" , "0.2103" , "736.05" ),
        ("AT_LOHNSTEUER", "3500"   , "0.1162" , "406.72" , "0"      , "0"      ),
    ], "2460.83", "4236.05").await;
}

/// Tchéquie, 45 000 CZK. Sociální 7,1 % / 24,8 %, zdravotní 4,5 % / 9 % ;
/// impôt 15 % × 45 000 − sleva na poplatníka 2 570 = 4 180 CZK.
#[tokio::test]
async fn golden_tchequie() {
    golden_regime(Pays::Tchequie, &[
        ("CZ_SOCIAL",    "45000"  , "0.071"  , "3195.00", "0.248"  , "11160.00"),
        ("CZ_ZDRAVOTNI", "45000"  , "0.045"  , "2025.00", "0.09"   , "4050.00"),
        ("CZ_DAN",       "45000"  , "0.0929" , "4180.00", "0"      , "0"      ),
    ], "35600.00", "60210.00").await;
}

/// Slovaquie, 1 800 €. Santé 5 % / 11 %, social 9,4 % / 25,2 % ; impôt 19 % ×
/// (1 800 − 259,20 − NČZD 497,23) = 198,28 €.
#[tokio::test]
async fn golden_slovaquie() {
    golden_regime(Pays::Slovaquie, &[
        ("SK_ZDRAVOTNE", "1800"   , "0.05"   , "90.00"  , "0.11"   , "198.00" ),
        ("SK_SOCIALNE",  "1800"   , "0.094"  , "169.20" , "0.252"  , "453.60" ),
        ("SK_DAN",       "1800"   , "0.1102" , "198.28" , "0"      , "0"      ),
    ], "1342.52", "2451.60").await;
}

/// Hongrie, 650 000 HUF. TB 18,5 %, SZJA 15 %, szocho 13 % (sans allocation
/// familiale ni exonération des moins de 25 ans).
#[tokio::test]
async fn golden_hongrie() {
    golden_regime(Pays::Hongrie, &[
        ("HU_TB",     "650000" , "0.185"  , "120250.00", "0"      , "0"      ),
        ("HU_SZOCHO", "650000" , "0"      , "0"      , "0.13"   , "84500.00"),
        ("HU_SZJA",   "650000" , "0.15"   , "97500.00", "0"      , "0"      ),
    ], "432250.00", "734500.00").await;
}

/// Slovénie, 2 400 €. Prispevki 22,1 % / 16,1 % ; dépendance 1 % / 1 %
/// (depuis le 01/07/2025) ; dohodnina 2026 : (2 400 − 530,40 − 24) × 12 −
/// 5 551,93 = 16 595,27 → 3 342,63 €/an = 278,55 €/mois ; contribution santé
/// forfaitaire 39,36 € (dès le 01/03/2026).
#[tokio::test]
async fn golden_slovenie() {
    golden_regime(Pays::Slovenie, &[
        ("SI_PRISPEVKI",   "2400"   , "0.221"  , "530.40" , "0.161"  , "386.40" ),
        ("SI_DOLGOTRAJNA", "2400"   , "0.01"   , "24.00"  , "0.01"   , "24.00"  ),
        ("SI_DOHODNINA",   "2400"   , "0.1161" , "278.55" , "0"      , "0"      ),
        ("SI_OZP",         "2400"   , "0"      , "39.36"  , "0"      , "0"      ),
    ], "1527.69", "2810.40").await;
}

/// Grèce, 1 500 €/mois versés 14 fois. EFKA 13,37 % / 21,79 % ; ΦΜΥ 2026 :
/// (1 500 − 200,55) × 14 = 18 192,30 → 900 + 20 % × 8 192,30 = 2 538,46 −
/// réduction (777 − 20 × 6,19) 653,15 = 1 885,31 €/an = 134,66 € par paie.
#[tokio::test]
async fn golden_grece() {
    golden_regime(Pays::Grece, &[
        ("GR_EFKA",  "1500"   , "0.1337" , "200.55" , "0.2179" , "326.85" ),
        ("GR_FOROS", "1500"   , "0.0898" , "134.66" , "0"      , "0"      ),
    ], "1164.79", "1826.85").await;
}

/// Chypre, 2 500 €. Sécurité sociale 8,8 % / 8,8 %, GESY 2,65 % / 2,90 % ;
/// impôt 2026 (seuil 22 000 €) : (30 000 − 3 435) − 22 000 = 4 565 × 20 % =
/// 913 €/an = 76,08 €/mois.
#[tokio::test]
async fn golden_chypre() {
    golden_regime(Pays::Chypre, &[
        ("CY_SI",    "2500"   , "0.088"  , "220.00" , "0.088"  , "220.00" ),
        ("CY_GESY",  "2500"   , "0.0265" , "66.25"  , "0.029"  , "72.50"  ),
        ("CY_FOROS", "2500"   , "0.0304" , "76.08"  , "0"      , "0"      ),
    ], "2137.67", "2792.50").await;
}

/// Malte, 2 000 €. SSC 10 % / 10 % ; impôt célibataire : 0 % jusqu'à 12 000 €,
/// 15 % à 16 000 €, 25 % au-delà = 2 600 €/an = 216,67 €/mois.
#[tokio::test]
async fn golden_malte() {
    golden_regime(Pays::Malte, &[
        ("MT_SSC", "2000"   , "0.10"   , "200.00" , "0.10"   , "200.00" ),
        ("MT_TAX", "2000"   , "0.1083" , "216.67" , "0"      , "0"      ),
    ], "1583.33", "2200.00").await;
}

/// Croatie, 1 800 €. Pension 20 % (15 % + 5 %), santé employeur 16,5 % ;
/// impôt 20 % (taux bas indicatif, variable par commune) × (1 800 − 360 −
/// abattement 600) = 168 €.
#[tokio::test]
async fn golden_croatie() {
    golden_regime(Pays::Croatie, &[
        ("HR_MIROVINSKO",  "1800"   , "0.20"   , "360.00" , "0"      , "0"      ),
        ("HR_ZDRAVSTVENO", "1800"   , "0"      , "0"      , "0.165"  , "297.00" ),
        ("HR_POREZ",       "1800"   , "0.0933" , "168.00" , "0"      , "0"      ),
    ], "1272.00", "2097.00").await;
}

/// Irlande, 4 500 €. PRSI 4,20 % / 11,25 % (jusqu'au 30/09/2026) ; USC 2026
/// 0,5 / 2 / 3 % (12 012 / 28 700 €) = 96,07 € ; PAYE 20 % jusqu'à 44 000 €
/// puis 40 % = 12 800 − crédits 4 000 = 733,33 €/mois.
#[tokio::test]
async fn golden_irlande() {
    golden_regime(Pays::Irlande, &[
        ("IE_PRSI", "4500"   , "0.0420" , "189.00" , "0.1125" , "506.25" ),
        ("IE_USC",  "4500"   , "0.0213" , "96.07"  , "0"      , "0"      ),
        ("IE_PAYE", "4500"   , "0.1630" , "733.33" , "0"      , "0"      ),
    ], "3481.60", "5006.25").await;
}

/// Roumanie, 8 000 RON. CAS 25 %, CASS 10 %, CAM 2,25 % ; impôt 10 % ×
/// (8 000 − 2 800) = 520 RON (pas de déduction personnelle au-delà du salaire
/// minimum + 2 000).
#[tokio::test]
async fn golden_roumanie() {
    golden_regime(Pays::Roumanie, &[
        ("RO_CAS",     "8000"   , "0.25"   , "2000.00", "0"      , "0"      ),
        ("RO_CASS",    "8000"   , "0.10"   , "800.00" , "0"      , "0"      ),
        ("RO_CAM",     "8000"   , "0"      , "0"      , "0.0225" , "180.00" ),
        ("RO_IMPOZIT", "8000"   , "0.0650" , "520.00" , "0"      , "0"      ),
    ], "4680.00", "8180.00").await;
}

/// Bulgarie, 1 200 €. Cotisations 13,78 % / 18,92 % sous le plafond (2 300 €
/// depuis le 01/08/2026) ; impôt 10 % × (1 200 − 165,36) = 103,46 €.
#[tokio::test]
async fn golden_bulgarie() {
    golden_regime(Pays::Bulgarie, &[
        ("BG_OSIG",  "1200"   , "0.1378" , "165.36" , "0.1892" , "227.04" ),
        ("BG_DANAK", "1200"   , "0.0862" , "103.46" , "0"      , "0"      ),
    ], "931.18", "1427.04").await;
}

/// États-Unis (Texas, pas d'impôt d'État), 5 000 $/mois, célibataire. Social
/// Security 6,2 % / 6,2 %, Medicare 1,45 % / 1,45 % ; FUTA 0,6 % sur 7 000 $
/// répartis sur l'année (583,33 $/mois). Fédéral 2026 : 60 000 − abattement
/// standard 16 100 = 43 900 → 10 % × 12 400 + 12 % × 31 500 = 5 020 $/an =
/// 418,33 $/mois.
#[tokio::test]
async fn golden_etats_unis() {
    golden_regime(Pays::EtatsUnis, &[
        ("US_SS",        "5000"   , "0.062"  , "310.00" , "0.062"  , "310.00" ),
        ("US_MEDICARE",  "5000"   , "0.0145" , "72.50"  , "0.0145" , "72.50"  ),
        ("US_FUTA",      "583.33" , "0"      , "0"      , "0.006"  , "3.50"   ),
        ("US_IMPOT_FED", "5000"   , "0.0837" , "418.33" , "0"      , "0"      ),
    ], "4199.17", "5386.00").await;
}

/// Mexique, 15 000 MXN. ISR 2026 (Anexo 8 RMF 2026) : 1 339,14 + 17,92 % ×
/// (15 000 − 14 644,65) = 1 402,82 MXN, sans subsidio (revenu > 11 492,66) ;
/// IMSS ouvrier 2,375 % et excédent de 3 UMA 0,40 % ; INFONAVIT 5 %, retiro 2 %.
#[tokio::test]
async fn golden_mexique() {
    golden_regime(Pays::Mexique, &[
        ("MX_IMSS",      "15000"  , "0.02375", "356.25" , "0"      , "0"      ),
        ("MX_IMSS_EXC",  "4301.34", "0.004"  , "17.21"  , "0"      , "0"      ),
        ("MX_ISR",       "15000"  , "0.0935" , "1402.82", "0"      , "0"      ),
        ("MX_INFONAVIT", "15000"  , "0"      , "0"      , "0.05"   , "750.00" ),
        ("MX_RETIRO",    "15000"  , "0"      , "0"      , "0.02"   , "300.00" ),
    ], "13223.72", "16050.00").await;
}

/// Brésil, 5 000 R$. INSS 2026 progressif (7,5 / 9 / 12 / 14 %) = 501,51 R$ ;
/// IRRF nul jusqu'à 5 000 R$ de revenu mensuel (Lei 15.270/2025) ; patronal
/// INSS 20 % et FGTS 8 % (RAT et contributions de tiers non modélisés).
#[tokio::test]
async fn golden_bresil() {
    golden_regime(Pays::Bresil, &[
        ("BR_INSS",     "5000"   , "0.1003" , "501.51" , "0"      , "0"      ),
        ("BR_IRRF",     "4392.80", "0"      , "0.00"   , "0"      , "0"      ),
        ("BR_INSS_PAT", "5000"   , "0"      , "0"      , "0.20"   , "1000.00"),
        ("BR_FGTS",     "5000"   , "0"      , "0"      , "0.08"   , "400.00" ),
    ], "4498.49", "6400.00").await;
}

/// Émirats arabes unis, 15 000 AED, salarié expatrié : ni cotisation sociale
/// (la GPSSA ne couvre que les nationaux) ni impôt sur le revenu.
#[tokio::test]
async fn golden_emirats() {
    golden_regime(Pays::Emirats, &[
        ("AE_EXPAT", "15000"  , "0"      , "0"      , "0"      , "0"      ),
    ], "15000", "15000").await;
}

/// Inde (Karnataka), 50 000 ₹. EPF 12 % / 12 % sur le plafond de 15 000 ₹ ;
/// ESI non applicable au-delà de 21 000 ₹ ; Professional Tax 200 ₹ ; impôt nul
/// (nouveau régime : rabais jusqu'à 12 lakh de revenu imposable).
#[tokio::test]
async fn golden_inde() {
    golden_regime(Pays::Inde, &[
        ("IN_EPF",   "15000"  , "0.12"   , "1800.00", "0.12"   , "1800.00"),
        ("IN_PT",    "50000"  , "0"      , "200"    , "0"      , "0"      ),
        ("IN_IMPOT", "50000"  , "0"      , "0"      , "0"      , "0"      ),
    ], "48000.00", "51800.00").await;
}

/// Espagne, 2 500 €, CDI. Contingences communes 4,70 % / 23,60 %, chômage
/// 1,55 % / 5,50 %, FOGASA 0,20 %, formation 0,10 % / 0,60 %, MEI 0,15 % /
/// 0,75 % (Orden PJC/297/2026). Retenue IRPF (algorithme AEAT 2026, situation
/// 3, sans enfant, 12 paies) : 30 000 − cotisations 1 950 = 28 050 − 2 000 =
/// base 26 050 (pas de réduction au-delà de 19 747,50) ; barème 5 980,50 −
/// minimum personnel 1 054,50 = 4 926 ; taux 16,42 % → 410,50 €.
#[tokio::test]
async fn golden_espagne() {
    golden_regime(Pays::Espagne, &[
        ("ES_CC",        "2500"   , "0.0470" , "117.50" , "0.2360" , "590.00" ),
        ("ES_DESEMPLEO", "2500"   , "0.0155" , "38.75"  , "0.0550" , "137.50" ),
        ("ES_FOGASA",    "2500"   , "0"      , "0"      , "0.0020" , "5.00"   ),
        ("ES_FP",        "2500"   , "0.0010" , "2.50"   , "0.0060" , "15.00"  ),
        ("ES_MEI",       "2500"   , "0.0015" , "3.75"   , "0.0075" , "18.75"  ),
        ("ES_IRPF",      "2500"   , "0.1642" , "410.50" , "0"      , "0"      ),
    ], "1927.00", "3266.25").await;
}

/// Luxembourg, 5 000 €, classe d'impôt 1. Pension 8,50 % + 8,50 % (réforme au
/// 01/01/2026), maladie 3,05 % + 3,05 %, dépendance 1,40 % sur 5 000 −
/// 2 771,33 / 4 = 4 307,17, accidents 0,65 %, mutualité classe 2 0,95 %
/// (FEDIL, 01/06/2026). Impôt (ACD) : 60 000 − 6 930 − 540 − 480 = 52 050 →
/// barème 8 084,40 × 1,07 (fonds pour l'emploi) = 8 650,31 − CIS 300 −
/// CI-CO2 108 = 8 242,31 €/an = 686,86 €/mois.
#[tokio::test]
async fn golden_luxembourg() {
    golden_regime(Pays::Luxembourg, &[
        ("LU_AP",    "5000"   , "0.0850" , "425.00" , "0.0850" , "425.00" ),
        ("LU_AM",    "5000"   , "0.0305" , "152.50" , "0.0305" , "152.50" ),
        ("LU_AD",    "4307.17", "0.0140" , "60.30"  , "0"      , "0"      ),
        ("LU_AA",    "5000"   , "0"      , "0"      , "0.0065" , "32.50"  ),
        ("LU_ME",    "5000"   , "0"      , "0"      , "0.0095" , "47.50"  ),
        ("LU_IMPOT", "5000"   , "0.1374" , "686.86" , "0"      , "0"      ),
    ], "3675.34", "5657.50").await;
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
