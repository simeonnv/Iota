use error::Error;
use oqs::sig::{Algorithm, Sig};

pub fn sign_dilithium3(input: &Vec<u8>, private_key: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let sig_alg = Sig::new(Algorithm::Dilithium3)?;
    let private_key = match sig_alg.secret_key_from_bytes(private_key) {
        Some(e) => e,
        None => return Err(Error::Internal("invalid private key".into())),
    };
    let signature = sig_alg.sign(input, private_key)?;
    let signature = signature.into_vec();

    Ok(signature)
}
