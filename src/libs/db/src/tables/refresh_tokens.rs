use sqlx::{Pool, Postgres, types::chrono::NaiveDateTime};
use uuid::Uuid;

use crate::Error;

#[derive(sqlx::FromRow, Debug)]
pub struct RefreshTokens {
    pub refresh_token_id: Uuid,
    pub account_id: Uuid,
    pub refresh_token: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

pub async fn init_refresh_tokens_table(pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS RefreshTokens (
            refresh_token_id UUID PRIMARY KEY,
            account_id UUID NOT NULL,
            refresh_token VARCHAR(256) NOT NULL UNIQUE,
            role VARCHAR(16) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (account_id) REFERENCES Accounts(account_id) ON DELETE CASCADE
        );
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE INDEX IF NOT EXISTS idx_refresh_tokens_refresh_token_id ON RefreshTokens (refresh_token_id);",
    )
    .execute(pool)
    .await?;

    Ok(())
}
