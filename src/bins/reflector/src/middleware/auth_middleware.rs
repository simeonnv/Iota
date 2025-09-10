use crate::rolling_rsa::RollingRSA;
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
) -> Result<ServiceResponse<impl MessageBody>, ActixError> {
    let rsa_key_pair = req
        .app_data::<web::Data<RwLock<RollingRSA>>>()
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
        return Err(Error::Unauthorized("Invalid token syntax".to_string()).into());
    }

    let jwt = String::from(&auth_header["Bearer ".len()..]);

    let claims = decode_jwt(&rsa_key_pair.read().await.decode_key, &jwt).await?;

    req.extensions_mut().insert(claims);

    next.call(req).await
}
