use chrono::Utc;
use error::Error;
use openssl::rsa::Rsa;

use crate::rsa_key_pair::rsa_key_pair::RsaKeyPair;

pub fn generate_rsa_key_pair() -> Result<RsaKeyPair, Error> {
    let rsa = Rsa::generate(2048)?;
    let private_key = rsa.private_key_to_pem()?;
    let public_key = rsa.public_key_to_pem()?;

    let now = Utc::now().naive_utc();

    Ok(RsaKeyPair {
        private_key,
        public_key,
        creation_time: now,
    })
}
