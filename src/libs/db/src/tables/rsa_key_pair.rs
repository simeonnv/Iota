use chrono::NaiveDateTime;
use sqlx::types::chrono;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct RsaKeyPairs {
    pub key_pair_id: Uuid,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub created_at: NaiveDateTime,
}

pub const INIT_RSAKEYPAIR_TABLE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS KeyPairs (
        key_pair_id UUID PRIMARY KEY,
        private_key BYTEA NOT NULL,
        public_key BYTEA NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );
"#;
