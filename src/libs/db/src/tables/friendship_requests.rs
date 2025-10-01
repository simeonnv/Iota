use sqlx::{Pool, Postgres, types::chrono::NaiveDateTime};
use uuid::Uuid;

use crate::Error;

#[derive(sqlx::FromRow, Debug)]
pub struct FriendshipRequests {
    pub friendship_request_id: Uuid,
    pub for_friendship_level: &'static str,
    pub account_from: Uuid,
    pub account_to: Uuid,
    pub created_at: NaiveDateTime,
}

pub async fn init_friendship_requests_table(pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS FriendshipRequests (
            friendship_request_id UUID PRIMARY KEY,
            account_from UUID NOT NULL,
            account_to UUID NOT NULL,
            for_friendship_level VARCHAR(64) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            
            FOREIGN KEY (account_from) REFERENCES Accounts(account_id) ON DELETE CASCADE,
            FOREIGN KEY (account_to) REFERENCES Accounts(account_id) ON DELETE CASCADE
        );
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE INDEX IF NOT EXISTS idx_friendship_requests_from ON FriendshipRequests (account_from);",
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE INDEX IF NOT EXISTS idx_friendship_requests_to ON FriendshipRequests (account_to);",
    )
    .execute(pool)
    .await?;

    Ok(())
}
