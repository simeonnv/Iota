use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use crypto::sign::{
    dilithium3::validate_dilithium3_sign::validate_dilithium3_sign,
    falcon512::validate_falcon512_sign::validate_falcon512_sign,
    rsa::validate_rsa_sign::validate_rsa_sign,
};
use error::Error;

use crate::jwt::{algorithm_type::AlgorithmType, jwt_claims::JWTClaims};

pub async fn decode_jwt(
    jwt: &String,
    alg_type: AlgorithmType,
    public_key: &Vec<u8>,
) -> Result<JWTClaims, Error> {
    let jwt_parts: Vec<&str> = jwt.splitn(3, '.').collect();

    if jwt_parts.len() != 3 {
        return Err(Error::Unauthorized("invalid jwt".into()));
    }

    let head_base64 = jwt_parts[0];
    let body_base64 = jwt_parts[1];
    let sign = BASE64_URL_SAFE_NO_PAD
        .decode(jwt_parts[2])
        .map_err(|_| Error::Unauthorized("invalid jwt".into()))?;

    let head_and_body = format!("{}.{}", head_base64, body_base64);

    match alg_type {
        AlgorithmType::Dilithium3 => {
            validate_dilithium3_sign(&head_and_body.into_bytes(), &sign, public_key)
        }
        AlgorithmType::Falcon512 => {
            validate_falcon512_sign(&head_and_body.into_bytes(), &sign, public_key)
        }
        AlgorithmType::Rsa => validate_rsa_sign(&head_and_body.into_bytes(), &sign, public_key),
    }
    .map_err(|_| Error::Unauthorized("invalid jwt".into()))?;

    let body = BASE64_URL_SAFE_NO_PAD.decode(body_base64)?;

    let claims: JWTClaims = serde_json::from_slice(&body)?;

    Ok(claims)
}
