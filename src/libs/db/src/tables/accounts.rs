use chrono::NaiveDateTime;
use sqlx::types::chrono;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct Accounts {
    pub account_id: Uuid,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

pub const INIT_ACCOUNTS_TABLE: &'static str = r#"
    CREATE TABLE IF NOT EXISTS Accounts (
        account_id UUID PRIMARY KEY,
        username VARCHAR(64) NOT NULL UNIQUE,
        password VARCHAR(256) NOT NULL,
        role VARCHAR(32) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );
"#;

pub const INIT_ACCOUNTS_INDEXES: &'static str = r#"
    CREATE INDEX idx_accounts_account_id ON Accounts (account_id);
"#;
