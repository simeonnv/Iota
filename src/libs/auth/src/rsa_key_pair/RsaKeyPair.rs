use chrono::NaiveDateTime;
use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct RsaKeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub creation_time: NaiveDateTime,
}
