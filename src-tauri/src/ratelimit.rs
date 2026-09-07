//! Limitation de débit en mémoire.
//!
//! Deux mécanismes distincts :
//!
//! * **compteur d'échecs** (`tentative_autorisee` / `enregistrer_echec`) —
//!   5 échecs consécutifs sur une même clé (un compte) → blocage 15 minutes.
//!   Protège un compte donné du bourrinage.
//! * **quota par fenêtre** (`quota_autorise`) — N requêtes par fenêtre glissante
//!   pour une clé (une IP). Le compteur d'échecs seul se contournait en changeant
//!   de compte à chaque essai : 30 logins sur 30 identifiants différents ne
//!   déclenchaient rien, tout en consommant 30 hachages Argon2id (~19 Mio chacun).
//!
//! Portée : un processus. Derrière plusieurs instances, chacune a sa table —
//! il faudra un magasin partagé (Redis) le jour où l'app est répliquée.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

const MAX_ECHECS: u32 = 5;
const DUREE_BLOCAGE: Duration = Duration::from_secs(15 * 60);

/// Récupère le contenu d'un `Mutex` même empoisonné : ces tables ne sont que
/// des compteurs, une panique ailleurs ne doit pas condamner définitivement
/// l'authentification (chaque `.lock().unwrap()` paniquerait à son tour).
fn verrouiller<T>(m: &'static Mutex<T>) -> MutexGuard<'static, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

// ── Compteur d'échecs par compte ──────────────────────────────────────────────

fn table() -> &'static Mutex<HashMap<String, (u32, Instant)>> {
    static T: OnceLock<Mutex<HashMap<String, (u32, Instant)>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Une tentative est-elle autorisée pour cette clé ?
pub fn tentative_autorisee(cle: &str) -> bool {
    let mut map = verrouiller(table());
    map.retain(|_, (_, dernier)| dernier.elapsed() < DUREE_BLOCAGE);
    !matches!(map.get(cle), Some((n, _)) if *n >= MAX_ECHECS)
}

/// À appeler après un échec d'authentification.
pub fn enregistrer_echec(cle: &str) {
    let mut map = verrouiller(table());
    let entree = map.entry(cle.to_string()).or_insert((0, Instant::now()));
    entree.0 += 1;
    entree.1 = Instant::now();
}

/// À appeler après une authentification réussie.
pub fn reinitialiser(cle: &str) {
    verrouiller(table()).remove(cle);
}

// ── Quota par fenêtre glissante ───────────────────────────────────────────────

fn table_quota() -> &'static Mutex<HashMap<String, (u32, Instant)>> {
    static T: OnceLock<Mutex<HashMap<String, (u32, Instant)>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Consomme une unité de quota pour `cle`. `false` = quota dépassé.
/// La fenêtre repart à zéro dès qu'elle est écoulée.
pub fn quota_autorise(cle: &str, max: u32, fenetre: Duration) -> bool {
    let mut map = verrouiller(table_quota());
    // Purge des fenêtres expirées : la table reste bornée par le trafic d'une
    // fenêtre, sans quoi une IP par requête la ferait croître indéfiniment.
    map.retain(|_, (_, debut)| debut.elapsed() < fenetre);

    let entree = map.entry(cle.to_string()).or_insert((0, Instant::now()));
    if entree.1.elapsed() >= fenetre {
        *entree = (0, Instant::now());
    }
    entree.0 += 1;
    entree.0 <= max
}
