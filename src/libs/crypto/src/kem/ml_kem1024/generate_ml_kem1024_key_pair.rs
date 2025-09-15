use chrono::Utc;
use error::Error;
use oqs::kem::{Algorithm, Kem};

use crate::sign::key_pair::KeyPair;

pub fn generate_ml_kem1024_key_pair() -> Result<KeyPair, Error> {
    let kem_alg = Kem::new(Algorithm::MlKem1024)?;
    let (public_key, private_key) = kem_alg.keypair()?;
    let now = Utc::now().naive_utc();
    // kem_alg.
    let keypair = KeyPair {
        public_key: public_key.into_vec(),
        private_key: private_key.into_vec(),
        creation_time: now,
    };

    Ok(keypair)
}
