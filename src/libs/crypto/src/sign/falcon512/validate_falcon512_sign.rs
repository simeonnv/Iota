use error::Error;
use log::debug;
use oqs::sig::{Algorithm, Sig};

pub fn validate_falcon512_sign(
    input: &Vec<u8>,
    signature: &Vec<u8>,
    public_key: &Vec<u8>,
) -> Result<(), Error> {
    let sig_alg = Sig::new(Algorithm::Falcon512)?;
    let public_key = match sig_alg.public_key_from_bytes(public_key) {
        Some(e) => e,
        None => return Err(Error::Internal("invalid public key".into())),
    };
    let signature = match sig_alg.signature_from_bytes(&signature) {
        Some(e) => e,
        None => return Err(Error::Unauthorized("invalid signature".into())),
    };

    sig_alg.verify(input, signature, public_key).map_err(|e| {
        debug!("validating falcon512 signature failed: {}", e);
        Error::Unauthorized("invalid signature".into())
    })?;

    Ok(())
}
