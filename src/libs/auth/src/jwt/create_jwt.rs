use chrono::{Duration, Utc};
use error::Error;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use uuid::Uuid;

use crate::jwt::jwt_claims::JWTClaims;

pub async fn create_jwt(
    user_id: &Uuid,
    user_role: &String,
    jwt_lifetime: &Duration,
    private_key: &EncodingKey,
) -> Result<String, Error> {
    let header = Header::new(Algorithm::RS256);
    let now = Utc::now().naive_utc();
    let jwt_claims: JWTClaims = JWTClaims {
        sub: user_id.clone(),
        exp: (now + *jwt_lifetime).and_utc().timestamp() as usize,
        role: user_role.to_owned(),
    };

    let token = encode(&header, &jwt_claims, private_key)?;

    Ok(token)
}
