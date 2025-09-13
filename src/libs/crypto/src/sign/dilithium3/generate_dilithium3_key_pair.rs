use chrono::Utc;
use error::Error;
use oqs::sig::{self, Sig};

use crate::sign::key_pair::KeyPair;

pub fn generate_dilithium3_key_pair() -> Result<KeyPair, Error> {
    let now = Utc::now().naive_utc();
    let sig_alg = Sig::new(sig::Algorithm::Dilithium3)?;
    let (public_key, private_key) = sig_alg.keypair()?;
    let (public_key, private_key) = (public_key.into_vec(), private_key.into_vec());

    // dbg!(
    //     String::from_utf8(public_key.to_owned()).unwrap(),
    //     String::from_utf8(private_key.to_owned()).unwrap(),
    // );

    Ok(KeyPair {
        private_key,
        public_key,
        creation_time: now,
    })
}
