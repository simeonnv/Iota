use serde::Serialize;
use sqlx::{Pool, Postgres, types::chrono::NaiveDateTime};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::Error;

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct FriendshipRequests {
    pub friendship_request_id: Uuid,
    pub for_friendship_level: String,
    pub account_in: Uuid,
    pub account_out: Uuid,
    pub created_at: NaiveDateTime,
}

pub async fn init_friendship_requests_table(pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS FriendshipRequests (
            friendship_request_id UUID PRIMARY KEY,
            account_in UUID NOT NULL,
            account_out UUID NOT NULL,
            for_friendship_level VARCHAR(64) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            
            FOREIGN KEY (account_in) REFERENCES Accounts(account_id) ON DELETE CASCADE,
            FOREIGN KEY (account_out) REFERENCES Accounts(account_id) ON DELETE CASCADE
        );
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE INDEX IF NOT EXISTS idx_friendship_requests_in ON FriendshipRequests (account_in);",
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "CREATE INDEX IF NOT EXISTS idx_friendship_requests_out ON FriendshipRequests (account_out);",
    )
    .execute(pool)
    .await?;

    Ok(())
}
