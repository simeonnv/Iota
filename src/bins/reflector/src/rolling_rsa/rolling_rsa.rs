use std::sync::Arc;

use auth::rsa_key_pair::rsa_key_pair::RsaKeyPair;
use chrono::Duration;
use tokio::sync::RwLock;

pub static RSA_EXPIRATION_TIME: Duration = Duration::days(7);

#[derive(Debug)]
pub struct RollingRSA {
    pub rsa_key_pair: Arc<RwLock<RsaKeyPair>>,
}
