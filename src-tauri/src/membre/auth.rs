use axum::{extract::FromRequestParts, http::{request::Parts, StatusCode}};
use crate::admin::auth::{validate_jwt, Claims};

pub fn member_jwt_secret() -> String {
    std::env::var("MEMBER_JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("MEMBER_JWT_SECRET non définie — secret de dev INSÉCURISÉ");
        "dev_member_jwt_secret_insecure".into()
    })
}

pub struct MemberAuth(pub Claims);

impl<S: Send + Sync> FromRequestParts<S> for MemberAuth {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = member_jwt_secret();

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

        Ok(MemberAuth(claims))
    }
}

pub fn member_profile_id(auth: &MemberAuth) -> Result<i64, (StatusCode, &'static str)> {
    auth.0.sub.parse::<i64>()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Token malformé"))
}
