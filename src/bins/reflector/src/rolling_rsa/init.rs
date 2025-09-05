use auth::rsa_key_pair::{
    generate_rsa_key_pair::generate_rsa_key_pair,
    get_latest_rsa_key_pair_db::get_latest_rsa_key_pair_db,
    save_rsa_key_pair_db::save_rsa_key_pair_db,
};
use chrono::Utc;
use error::Error;
use jsonwebtoken::{DecodingKey, EncodingKey};
use log::{error, info};
use sqlx::{Pool, Postgres};
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};

use crate::rolling_rsa::{RSA_EXPIRATION_TIME, RollingRSA};

impl RollingRSA {
    pub async fn init(db_pool: Arc<Pool<Postgres>>) -> Result<Arc<RwLock<Self>>, Error> {
        let rsa_pair = get_latest_rsa_key_pair_db(&db_pool).await?;

        let now = Utc::now().naive_utc();
        let rsa_pair = match rsa_pair {
            Some(e) if now > e.creation_time + RSA_EXPIRATION_TIME => None,
            Some(e) => Some(e),
            None => None,
        };

        let rsa_pair = match rsa_pair {
            Some(e) => e,
            None => {
                let new_rsa_pair = generate_rsa_key_pair()?;
                save_rsa_key_pair_db(&new_rsa_pair, &db_pool).await?;
                new_rsa_pair
            }
        };

        let decode_key = DecodingKey::from_rsa_pem(&rsa_pair.public_key)?;
        let encode_key = EncodingKey::from_rsa_pem(&rsa_pair.private_key)?;

        let rolling_rsa = Self {
            rsa_key_pair: rsa_pair,
            decode_key,
            encode_key,
        };

        let rolling_rsa = Arc::new(RwLock::new(rolling_rsa));

        // creates a background thread that checks once per hour if the rsa key has expired,
        // if yes it will make a new one and replace it
        // it locks RsaKeyPair when replacing it
        {
            let rolling_rsa = rolling_rsa.clone();
            tokio::spawn(async move {
                info!("spawned background rsa key pair expiration checker");
                loop {
                    let now = Utc::now().naive_utc();
                    info!("checking rsa key pair expiration");
                    let needs_update = {
                        let rsa_pair_guard = &rolling_rsa.read().await.rsa_key_pair;
                        now > rsa_pair_guard.creation_time + RSA_EXPIRATION_TIME
                    };

                    if needs_update {
                        let new_rsa_pair = match generate_rsa_key_pair() {
                            Ok(e) => e,
                            Err(err) => {
                                error!(
                                    "FAILED TO GENERATE A NEW RSA KEY PAIR IN BACKGROUND TASK, TRYING AGAIN: {err}"
                                );
                                sleep(Duration::from_secs(5)).await;
                                continue;
                            }
                        };
                        if let Err(err) = save_rsa_key_pair_db(&new_rsa_pair, &db_pool).await {
                            error!(
                                "FAILED TO SAVE RSA KEY PAIR IN DB FROM THE BACKGROUND TASK, TRYING AGAIN: {err}"
                            );
                            sleep(Duration::from_secs(5)).await;
                            continue;
                        };
                        let decode_key = DecodingKey::from_rsa_pem(&new_rsa_pair.public_key)
                            .expect("invalid rsa pub key in background checker thread");
                        let encode_key = EncodingKey::from_rsa_pem(&new_rsa_pair.private_key)
                            .expect("invalid rsa priv key in background checker thread");

                        let mut rsa_pair_guard = rolling_rsa.write().await;
                        rsa_pair_guard.rsa_key_pair = new_rsa_pair;
                        rsa_pair_guard.decode_key = decode_key;
                        rsa_pair_guard.encode_key = encode_key;
                    }
                    sleep(Duration::from_secs(3600)).await;
                }
            });
        }

        Ok(rolling_rsa)
    }
}
