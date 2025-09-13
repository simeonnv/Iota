use crate::rolling_rsa::RollingKeyPair;
use actix_web::{
    Error as ActixError, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use auth::jwt::decode_jwt::decode_jwt;
use error::Error;
use tokio::sync::RwLock;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
    auth_level: Option<&str>,
) -> Result<ServiceResponse<impl MessageBody>, ActixError> {
    let rolling_key_pair = req
        .app_data::<web::Data<RwLock<RollingKeyPair>>>()
        .ok_or_else(|| Error::Internal("RSA key pair not found".to_string()))?;

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|auth| auth.to_string());
    let auth_header = match auth_header {
        Some(e) => Ok(e),
        None => Err(Error::Unauthorized("Invalid or missing token".into())),
    }?;

    if !auth_header.starts_with("Bearer ") {
        return Err(Error::Unauthorized("Invalid token syntax".into()).into());
    }

    let jwt = String::from(&auth_header["Bearer ".len()..]);

    let claims = {
        let rolling_key_pair_read_lock = rolling_key_pair.read().await;
        decode_jwt(
            &jwt,
            rolling_key_pair_read_lock.sign_alg,
            &rolling_key_pair_read_lock.key_pair.public_key,
        )
        .await?
    };

    if let Some(auth_level) = auth_level {
        if claims.role != auth_level {
            return Err(Error::Unauthorized("Invalid authorization level!".into()).into());
        }
    }

    req.extensions_mut().insert(claims);

    next.call(req).await
}
