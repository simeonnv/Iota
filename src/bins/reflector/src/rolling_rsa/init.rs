use auth::{
    jwt::algorithm_type::AlgorithmType,
    key_pair_db::{
        get_latest_key_pair_db::get_latest_key_pair_db, save_key_pair_db::save_key_pair_db,
    },
};
use chrono::Utc;
use crypto::sign::{
    dilithium3::generate_dilithium3_key_pair::generate_dilithium3_key_pair,
    falcon512::generate_falcon512_key_pair::generate_falcon512_key_pair,
    rsa::generate_rsa_key_pair::generate_rsa_key_pair,
};
use error::Error;
use log::{error, info};
use sqlx::{Pool, Postgres};
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};

use crate::rolling_rsa::{RSA_EXPIRATION_TIME, RollingKeyPair};

impl RollingKeyPair {
    pub async fn init(
        db_pool: Arc<Pool<Postgres>>,
        sign_alg: AlgorithmType,
    ) -> Result<Arc<RwLock<Self>>, Error> {
        let key_pair = get_latest_key_pair_db(&db_pool).await?;

        let now = Utc::now().naive_utc();
        let key_pair = match key_pair {
            Some(e) if now > e.creation_time + RSA_EXPIRATION_TIME => None,
            Some(e) => Some(e),
            None => None,
        };

        let key_pair = match key_pair {
            Some(e) => e,
            None => {
                let new_key_pair = match sign_alg {
                    AlgorithmType::Dilithium3 => generate_dilithium3_key_pair()?,
                    AlgorithmType::Rsa => generate_rsa_key_pair()?,
                    AlgorithmType::Falcon512 => generate_falcon512_key_pair()?,
                };
                save_key_pair_db(&new_key_pair, &db_pool).await?;
                new_key_pair
            }
        };

        let rolling_key_pair = Self {
            key_pair: key_pair,
            sign_alg: sign_alg,
        };

        let rolling_key_pair = Arc::new(RwLock::new(rolling_key_pair));

        // creates a background thread that checks once per hour if the rsa key has expired,
        // if yes it will make a new one and replace it
        // it locks RsaKeyPair when replacing it
        {
            let rolling_key_pair = rolling_key_pair.clone();
            tokio::spawn(async move {
                info!("spawned background rsa key pair expiration checker");
                loop {
                    let now = Utc::now().naive_utc();
                    info!("checking rsa key pair expiration");
                    let needs_update = {
                        let rsa_pair_guard = &rolling_key_pair.read().await.key_pair;
                        now > rsa_pair_guard.creation_time + RSA_EXPIRATION_TIME
                    };

                    if needs_update {
                        let key_pair_res = match sign_alg {
                            AlgorithmType::Dilithium3 => generate_dilithium3_key_pair(),
                            AlgorithmType::Rsa => generate_rsa_key_pair(),
                            AlgorithmType::Falcon512 => generate_falcon512_key_pair(),
                        };
                        let key_rsa_pair = match key_pair_res {
                            Ok(e) => e,
                            Err(err) => {
                                error!(
                                    "FAILED TO GENERATE A NEW RSA KEY PAIR IN BACKGROUND TASK, TRYING AGAIN: {err}"
                                );
                                sleep(Duration::from_secs(5)).await;
                                continue;
                            }
                        };
                        if let Err(err) = save_key_pair_db(&key_rsa_pair, &db_pool).await {
                            error!(
                                "FAILED TO SAVE RSA KEY PAIR IN DB FROM THE BACKGROUND TASK, TRYING AGAIN: {err}"
                            );
                            sleep(Duration::from_secs(5)).await;
                            continue;
                        };

                        let mut key_pair_guard = rolling_key_pair.write().await;
                        key_pair_guard.key_pair = key_rsa_pair;
                    }
                    sleep(Duration::from_secs(3600)).await;
                }
            });
        }

        Ok(rolling_key_pair)
    }
}
