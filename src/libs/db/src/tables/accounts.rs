use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::Error;

#[derive(sqlx::FromRow, Debug)]
pub struct Accounts {
    pub account_id: Uuid,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

pub async fn init_accounts_table(pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS Accounts (
            account_id UUID PRIMARY KEY,
            username VARCHAR(64) NOT NULL UNIQUE,
            password VARCHAR(256) NOT NULL,
            role VARCHAR(32) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query!(r#"CREATE INDEX IF NOT EXISTS idx_accounts_account_id ON Accounts (account_id);"#,)
        .execute(pool)
        .await?;

    sqlx::query!(r#"CREATE INDEX IF NOT EXISTS idx_accounts_username ON Accounts (username);"#,)
        .execute(pool)
        .await?;

    Ok(())
}
