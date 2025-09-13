use error::Error;
use openssl::{hash::MessageDigest, pkey::PKey, sign::Verifier};

pub fn validate_rsa_sign(
    input: &Vec<u8>,
    signature: &Vec<u8>,
    public_key: &Vec<u8>,
) -> Result<(), Error> {
    let public_key = PKey::public_key_from_pem(public_key)?;

    let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key)?;
    verifier.update(input)?;
    let verified = verifier.verify(signature)?;

    if !verified {
        return Err(Error::Unauthorized("invalid signiture".into()));
    }

    Ok(())
}
