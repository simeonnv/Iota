use error::Error;
use openssl::{hash::MessageDigest, pkey::PKey, sign::Signer};

pub fn sign_rsa(input: &Vec<u8>, private_key: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let private_key = PKey::private_key_from_pem(private_key)?;

    let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;
    signer.update(input)?;
    let signature = signer.sign_to_vec()?;

    Ok(signature)
}
