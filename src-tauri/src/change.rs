// ── Cours de change — conversion du montant saisi d'un pays à l'autre ────────
//
// Le simulateur ne convertit rien dans un bulletin : chaque régime calcule dans
// sa devise. Ce module sert le seul formulaire. Le visiteur saisit un salaire
// dans une devise ; en changeant de pays, le front reconvertit ce montant
// d'origine dans la devise du nouveau régime.
//
// Source : cours de référence de la Banque centrale européenne, publiés chaque
// jour ouvré vers 16 h (heure de Francfort), servis sans clé par Frankfurter
// (https://frankfurter.dev, logiciel libre). Ce n'est pas un cours en temps réel :
// aucune source gratuite n'en donne, et c'est le cours de référence qui convient
// à une conversion de salaire.
//
// Le dirham (AED) n'est pas coté par la BCE. Il est ancré au dollar depuis 1997
// à 3,6725 AED pour 1 USD (Banque centrale des Émirats) : on le déduit du cours
// EUR→USD. Le lev (BGN) ne l'est plus depuis l'entrée de la Bulgarie dans la zone
// euro au 1er janvier 2026 : on le pose au taux de conversion irrévocable,
// 1 € = 1,95583 BGN, fixé par le Conseil de l'UE en juillet 2025 (c'est aussi
// l'ancienne parité du currency board bulgare). Il ne sert plus qu'aux
// paies bulgares de 2025.
//
// Un seul appel pour tous les visiteurs : le résultat est gardé en mémoire
// FRAICHEUR, puis redemandé. Si Frankfurter ne répond pas, on sert le dernier
// cours obtenu ; sa date dit au visiteur de quel jour il est.

use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

const URL_BCE: &str = "https://api.frankfurter.dev/v1/latest?base=EUR";
/// Les cours changent une fois par jour ouvré ; six heures suffisent pour
/// prendre celui du jour dans l'après-midi sans interroger à chaque visite.
const FRAICHEUR: Duration = Duration::from_secs(6 * 3600);
const DELAI: Duration = Duration::from_secs(8);
/// Parité fixe du dirham des Émirats : 3,6725 AED pour 1 USD.
const AED_PAR_USD: f64 = 3.6725;
/// Taux de conversion irrévocable du lev bulgare : 1,95583 BGN pour 1 €.
const BGN_PAR_EUR: f64 = 1.95583;

/// Cours du jour, base euro : `taux["JPY"]` = nombre de yens pour 1 €.
#[derive(Debug, Clone, Serialize)]
pub struct CoursChange {
    /// Date de publication du cours par la BCE (AAAA-MM-JJ).
    pub date:   String,
    pub source: &'static str,
    pub taux:   BTreeMap<String, f64>,
}

#[derive(Deserialize)]
struct ReponseFrankfurter {
    date:  String,
    rates: BTreeMap<String, f64>,
}

fn cache() -> &'static Mutex<Option<(Instant, CoursChange)>> {
    static CACHE: OnceLock<Mutex<Option<(Instant, CoursChange)>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(None))
}

async fn interroger_bce() -> Result<CoursChange, String> {
    let client = reqwest::Client::builder()
        .timeout(DELAI)
        .user_agent(concat!("xenna-paie/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|e| e.to_string())?;
    let r: ReponseFrankfurter = client.get(URL_BCE).send().await
        .and_then(|r| r.error_for_status())
        .map_err(|e| format!("Cours BCE indisponible : {e}"))?
        .json().await
        .map_err(|e| format!("Cours BCE illisible : {e}"))?;

    let mut taux = r.rates;
    taux.insert("EUR".into(), 1.0);
    taux.insert("BGN".into(), BGN_PAR_EUR);
    if let Some(usd) = taux.get("USD").copied() {
        taux.insert("AED".into(), usd * AED_PAR_USD);
    }
    Ok(CoursChange { date: r.date, source: "BCE (cours de référence, via Frankfurter)", taux })
}

/// Cours du jour, depuis le cache s'il est frais, sinon redemandé à la BCE.
/// Un échec n'efface pas le cache : le dernier cours connu est servi tel quel.
pub async fn cours_du_jour() -> Result<CoursChange, String> {
    let mut garde = cache().lock().await;
    if let Some((quand, cours)) = garde.as_ref() {
        if quand.elapsed() < FRAICHEUR {
            return Ok(cours.clone());
        }
    }
    match interroger_bce().await {
        Ok(cours) => {
            *garde = Some((Instant::now(), cours.clone()));
            Ok(cours)
        }
        Err(e) => match garde.as_ref() {
            Some((_, ancien)) => {
                tracing::warn!("{e} — dernier cours connu ({}) servi", ancien.date);
                Ok(ancien.clone())
            }
            None => Err(e),
        },
    }
}
