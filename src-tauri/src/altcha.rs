use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use digest::KeyInit as DigestKeyInit;
use hmac::{Hmac, Mac};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

/// Durée de validité d'un challenge (le widget résout en quelques secondes).
const VALIDITE_SECS: u64 = 600;

fn maintenant() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

// Challenges déjà consommés (anti-rejeu). Purgés à chaque passage une fois
// leur date d'expiration atteinte — la table reste donc bornée.
fn deja_utilises() -> &'static Mutex<HashMap<String, u64>> {
    static T: OnceLock<Mutex<HashMap<String, u64>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

/// true si le challenge n'avait jamais été consommé (et le marque consommé).
fn marquer_consomme(challenge: &str, expires: u64) -> bool {
    let mut map = deja_utilises().lock().unwrap();
    let now = maintenant();
    map.retain(|_, exp| *exp > now);
    map.insert(challenge.to_string(), expires).is_none()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AltchaChallenge {
    pub algorithm:  &'static str,
    pub challenge:  String,
    pub max_number: u64,
    pub salt:       String,
    pub signature:  String,
}

#[derive(Debug, Deserialize)]
struct AltchaPayload {
    algorithm: String,
    challenge: String,
    number:    u64,
    salt:      String,
    signature: String,
}

pub fn generate_challenge(secret: &str) -> AltchaChallenge {
    let mut salt_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut salt_bytes);
    // L'expiration voyage dans le salt (format standard Altcha `?expires=`) :
    // elle est couverte par la signature via le challenge, donc infalsifiable.
    let salt = format!("{}?expires={}", hex::encode(salt_bytes), maintenant() + VALIDITE_SECS);

    let max_number: u64 = 100_000;
    let number = rand::random::<u64>() % max_number;

    let challenge = hex::encode(Sha256::digest(format!("{salt}:{number}").as_bytes()));

    let mut mac = <HmacSha256 as DigestKeyInit>::new_from_slice(secret.as_bytes()).expect("HMAC key error");
    mac.update(challenge.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    AltchaChallenge { algorithm: "SHA-256", challenge, max_number, salt, signature }
}

pub fn verify_solution(payload_b64: &str, secret: &str) -> bool {
    let json_bytes = match base64::engine::general_purpose::STANDARD.decode(payload_b64) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let payload: AltchaPayload = match serde_json::from_slice(&json_bytes) {
        Ok(v) => v,
        Err(_) => return false,
    };
    if payload.algorithm != "SHA-256" {
        return false;
    }

    // Expiration : embarquée dans le salt, donc couverte par la signature.
    let expires = match payload
        .salt
        .split_once("?expires=")
        .and_then(|(_, e)| e.parse::<u64>().ok())
    {
        Some(e) if e >= maintenant() => e,
        _ => return false,
    };

    // Vérifier challenge = sha256(salt + ":" + number)
    let expected_challenge =
        hex::encode(Sha256::digest(format!("{}:{}", payload.salt, payload.number).as_bytes()));
    if expected_challenge != payload.challenge {
        return false;
    }

    // Vérifier signature = HMAC-SHA256(challenge, secret) — en temps constant.
    let sig_bytes = match hex::decode(&payload.signature) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let mut mac = <HmacSha256 as DigestKeyInit>::new_from_slice(secret.as_bytes()).expect("HMAC key error");
    mac.update(payload.challenge.as_bytes());
    if mac.verify_slice(&sig_bytes).is_err() {
        return false;
    }

    // Anti-rejeu : un challenge résolu ne peut servir qu'une fois.
    marquer_consomme(&payload.challenge, expires)
}
