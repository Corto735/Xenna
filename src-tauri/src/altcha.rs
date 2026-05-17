use base64::Engine;
use digest::KeyInit as DigestKeyInit;
use hmac::{Hmac, Mac};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

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
    let salt = hex::encode(salt_bytes);

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

    // Vérifier challenge = sha256(salt + ":" + number)
    let expected_challenge =
        hex::encode(Sha256::digest(format!("{}:{}", payload.salt, payload.number).as_bytes()));
    if expected_challenge != payload.challenge {
        return false;
    }

    // Vérifier signature = HMAC-SHA256(challenge, secret)
    let mut mac = <HmacSha256 as DigestKeyInit>::new_from_slice(secret.as_bytes()).expect("HMAC key error");
    mac.update(payload.challenge.as_bytes());
    let expected_sig = hex::encode(mac.finalize().into_bytes());
    expected_sig == payload.signature
}
