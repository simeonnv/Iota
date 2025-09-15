use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use error::Error;
use oqs::kem::{Algorithm, Kem};

pub fn decrypt_ml_kem1024(
    ciphertext: &Vec<u8>,
    kem_ciphertext: &Vec<u8>,
    private_key: &Vec<u8>,
) -> Result<Vec<u8>, Error> {
    let kem_alg = Kem::new(Algorithm::MlKem1024)?;
    let private_key = match kem_alg.secret_key_from_bytes(private_key) {
        Some(e) => e,
        None => return Err(Error::Internal("invalid decryption key".into())),
    };
    let kem_ciphertext = match kem_alg.ciphertext_from_bytes(kem_ciphertext) {
        Some(e) => e,
        None => return Err(Error::Internal("invalid ciphertext".into())),
    };

    let shared_secret = kem_alg.decapsulate(private_key, kem_ciphertext)?;

    let key = Key::<Aes256Gcm>::from_slice(shared_secret.as_ref());
    let cipher = Aes256Gcm::new(key);

    if ciphertext.len() < 12 {
        return Err(Error::Internal("Invalid symmetric ciphertext".into()));
    }
    let nonce = Nonce::from_slice(&ciphertext[0..12]);
    let ciphertext = &ciphertext[12..];

    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    Ok(plaintext)
}
