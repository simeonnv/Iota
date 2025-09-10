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

// auth level is the authorization needed to enter the endpoint
// if auth level is none => the only thing required to ender the endpoint is
// a valid session => you can enter no matter auth level
pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
    auth_level: Option<&str>,
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
        return Err(Error::Unauthorized("Invalid token syntax".into()).into());
    }

    let jwt = String::from(&auth_header["Bearer ".len()..]);

    let claims = decode_jwt(&rsa_key_pair.read().await.decode_key, &jwt).await?;

    if let Some(auth_level) = auth_level
        && claims.role != auth_level
    {
        return Err(Error::Unauthorized("Invalid autoriztion level!".into()).into());
    }

    req.extensions_mut().insert(claims);

    next.call(req).await
}
