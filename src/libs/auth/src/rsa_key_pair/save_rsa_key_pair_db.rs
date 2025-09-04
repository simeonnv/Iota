use crate::rsa_key_pair::rsa_key_pair::RsaKeyPair;
use error::Error;
use log::info;
use sqlx::{Pool, Postgres, types::Uuid};

pub async fn save_rsa_key_pair_db(
    rsa_key_pair: &RsaKeyPair,
    pool: &Pool<Postgres>,
) -> Result<(), Error> {
    sqlx::query(
        r#"
            INSERT INTO KeyPairs
                (key_pair_id, private_key, public_key)
            VALUES ($1, $2, $3);
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(rsa_key_pair.private_key.clone())
    .bind(rsa_key_pair.public_key.clone())
    .execute(pool)
    .await?;

    info!("succesfully saved rsa key pair into db!");

    Ok(())
}
