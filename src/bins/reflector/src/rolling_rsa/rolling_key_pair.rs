use chrono::Duration;
use crypto::sign::key_pair::KeyPair;

pub static RSA_EXPIRATION_TIME: Duration = Duration::days(7);

pub struct RollingKeyPair {
    pub key_pair: KeyPair,
}
