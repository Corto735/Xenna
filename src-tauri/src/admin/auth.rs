use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::FromRequestParts, http::{request::Parts, StatusCode}};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("hash_password failed")
        .to_string()
}

/// Hash factice : vérifié quand le compte n'existe pas, pour que le temps de
/// réponse ne révèle pas si un identifiant est valide (anti-énumération).
pub fn dummy_hash() -> &'static str {
    static H: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    H.get_or_init(|| hash_password("xenna_dummy_password_egalisation_timing"))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok()
}

pub fn generate_jwt(username: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 8 * 3600;
    let claims = Claims { sub: username.to_string(), exp };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data.claims)
}

pub fn jwt_secret() -> String {
    std::env::var("ADMIN_JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("ADMIN_JWT_SECRET non définie — utilisation d'un secret de développement INSÉCURISÉ");
        "dev_jwt_secret_insecure".into()
    })
}

// ── Extractor Axum pour les routes admin protégées ────────────────────────────

pub struct AdminAuth(pub Claims);

impl<S: Send + Sync> FromRequestParts<S> for AdminAuth {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = jwt_secret();

        let auth = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Authorization header manquant"))?;

        let token = auth
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Format attendu : Bearer <token>"))?;

        let claims = validate_jwt(token, &secret)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Token invalide ou expiré"))?;

        Ok(AdminAuth(claims))
    }
}
