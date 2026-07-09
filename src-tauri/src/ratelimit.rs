//! Limitation de débit en mémoire pour les endpoints d'authentification.
//! Politique : 5 échecs consécutifs sur une même clé → blocage 15 minutes.
//! Le compteur d'une clé expire 15 minutes après le dernier échec.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

const MAX_ECHECS: u32 = 5;
const DUREE_BLOCAGE: Duration = Duration::from_secs(15 * 60);

fn table() -> &'static Mutex<HashMap<String, (u32, Instant)>> {
    static T: OnceLock<Mutex<HashMap<String, (u32, Instant)>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Une tentative est-elle autorisée pour cette clé ?
pub fn tentative_autorisee(cle: &str) -> bool {
    let mut map = table().lock().unwrap();
    map.retain(|_, (_, dernier)| dernier.elapsed() < DUREE_BLOCAGE);
    !matches!(map.get(cle), Some((n, _)) if *n >= MAX_ECHECS)
}

/// À appeler après un échec d'authentification.
pub fn enregistrer_echec(cle: &str) {
    let mut map = table().lock().unwrap();
    let entree = map.entry(cle.to_string()).or_insert((0, Instant::now()));
    entree.0 += 1;
    entree.1 = Instant::now();
}

/// À appeler après une authentification réussie.
pub fn reinitialiser(cle: &str) {
    table().lock().unwrap().remove(cle);
}
