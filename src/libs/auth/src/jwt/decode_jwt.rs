use error::Error;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use log::error;

use crate::jwt::jwt_claims::JWTClaims;

pub async fn decode_jwt(public_key: &DecodingKey, jwt: &String) -> Result<JWTClaims, Error> {
    let mut validation = Validation::new(Algorithm::RS256);

    validation.set_required_spec_claims(&["exp", "sub"]);

    let decoded = decode::<JWTClaims>(&jwt, &public_key, &validation);

    let decoded = match decoded {
        Ok(e) => e,
        Err(e) => {
            error!("invalid jwt: {}", e);
            return Err(Error::Unauthorized("invalid jwt".to_string()));
        }
    };
    Ok(decoded.claims)
}
