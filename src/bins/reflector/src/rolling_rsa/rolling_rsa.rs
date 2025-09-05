use auth::rsa_key_pair::rsa_key_pair::RsaKeyPair;
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey};

pub static RSA_EXPIRATION_TIME: Duration = Duration::days(7);

pub struct RollingRSA {
    pub rsa_key_pair: RsaKeyPair,
    pub decode_key: DecodingKey,
    pub encode_key: EncodingKey,
}
