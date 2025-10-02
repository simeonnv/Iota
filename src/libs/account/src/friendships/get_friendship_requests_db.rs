use crate::Error;
use serde::Serialize;
use sqlx::{Pool, Postgres, types::chrono::NaiveDateTime};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct FriendshipRequest {
    pub friendship_request_id: Uuid,
    pub for_friendship_level: String,
    pub account_in: Uuid,
    pub account_out: Uuid,
    pub created_at: NaiveDateTime,
}

pub async fn get_friendship_requests_db(
    account_id: &Uuid,
    db_pool: &Pool<Postgres>,
) -> Result<Vec<FriendshipRequest>, Error> {
    let friendnships: Vec<FriendshipRequest> = sqlx::query_as!(
        FriendshipRequest,
        r#"
            SELECT
                f.friendship_request_id,
                f.for_friendship_level,
                f.created_at,
                f.account_in,
                f.account_out
            FROM FriendshipRequests f
            WHERE f.account_in = $1 OR f.account_out = $1;
        "#,
        account_id
    )
    .fetch_all(db_pool)
    .await?
    .into();

    Ok(friendnships)
}
