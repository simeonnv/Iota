use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres, types::chrono};
use uuid::Uuid;

use crate::Error;

#[derive(sqlx::FromRow, Debug)]
pub struct KeyPairs {
    pub key_pair_id: Uuid,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub created_at: NaiveDateTime,
}

pub async fn init_key_pairs_table(pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS KeyPairs (
            key_pair_id UUID PRIMARY KEY,
            private_key BYTEA NOT NULL,
            public_key BYTEA NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
    "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
