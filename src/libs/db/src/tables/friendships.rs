use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, types::chrono::NaiveDateTime};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::Error;

#[derive(Serialize, Deserialize, ToSchema, Clone, Copy, PartialEq)]
pub enum FriendshipLevel {
    Normal,
    Trusted,
}

impl<'a> From<&'a str> for FriendshipLevel {
    fn from(a: &'a str) -> Self {
        match a {
            "normal" => Self::Normal,
            "trusted" => Self::Trusted,
            _ => Self::Normal,
        }
    }
}

impl From<String> for FriendshipLevel {
    fn from(a: String) -> Self {
        match a.as_str() {
            "normal" => Self::Normal,
            "trusted" => Self::Trusted,
            _ => Self::Normal,
        }
    }
}

impl FriendshipLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Trusted => "trusted",
        }
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct Friendships {
    pub friendship_id: Uuid,
    pub friendship_level: String,
    pub account_in: Uuid,
    pub account_out: Uuid,
    pub created_at: NaiveDateTime,
}

pub async fn init_friendships_table(pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS Friendships (
            friendship_id UUID PRIMARY KEY,
            account_in UUID NOT NULL,
            account_out UUID NOT NULL,
            friendship_level VARCHAR(64) NOT NULL CHECK (status IN ('normal', 'trusted')),
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY (account_in) REFERENCES Accounts(account_id) ON DELETE CASCADE,
            FOREIGN KEY (account_out) REFERENCES Accounts(account_id) ON DELETE CASCADE,

            CONSTRAINT unique_request UNIQUE (account_in, account_out)
        );
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query!("CREATE INDEX IF NOT EXISTS idx_friendships_in ON Friendships (account_in);",)
        .execute(pool)
        .await?;

    sqlx::query!("CREATE INDEX IF NOT EXISTS idx_friendships_in ON Friendships (account_in);",)
        .execute(pool)
        .await?;

    Ok(())
}
