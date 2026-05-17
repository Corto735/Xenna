use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::Engine;
use digest::KeyInit as DigestKeyInit;
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn encrypt_email(email: &str, key: &[u8; 32]) -> String {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, email.as_bytes())
        .expect("AES-GCM encryption failure");
    format!("{}:{}", hex::encode(nonce_bytes), hex::encode(ciphertext))
}

pub fn decrypt_email(enc: &str, key: &[u8; 32]) -> Result<String, String> {
    let (nonce_hex, ct_hex) = enc
        .split_once(':')
        .ok_or_else(|| "format email_enc invalide".to_string())?;
    let nonce_bytes = hex::decode(nonce_hex).map_err(|e| e.to_string())?;
    let ciphertext = hex::decode(ct_hex).map_err(|e| e.to_string())?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(&nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| e.to_string())?;
    String::from_utf8(plaintext).map_err(|e| e.to_string())
}

pub fn email_hash(email: &str, key: &[u8; 32]) -> String {
    let mut mac = <HmacSha256 as DigestKeyInit>::new_from_slice(key).expect("HMAC key error");
    mac.update(email.to_lowercase().as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

pub fn parse_encryption_key() -> [u8; 32] {
    match std::env::var("ENCRYPTION_KEY") {
        Ok(b64) => {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(&b64)
                .expect("ENCRYPTION_KEY base64 invalide");
            bytes.try_into().expect("ENCRYPTION_KEY doit faire 32 bytes (openssl rand -base64 32)")
        }
        Err(_) => {
            tracing::warn!("ENCRYPTION_KEY non définie — utilisation d'une clé de développement INSÉCURISÉE");
            [0u8; 32]
        }
    }
}
