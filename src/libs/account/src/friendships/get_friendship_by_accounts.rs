use crate::Error;
use db::tables::friendship_requests::FriendshipRequests;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_friendship_by_accounts(
    account_id_1: &Uuid,
    account_id_2: &Uuid,
    db_pool: &Pool<Postgres>,
) -> Result<Option<FriendshipRequests>, Error> {
    let friendnships = sqlx::query_as!(
        FriendshipRequests,
        r#"
            SELECT
                *
            FROM FriendshipRequests 
            WHERE (account_in = $1 AND account_out = $2) OR (account_in = $2 AND account_out = $1);
        "#,
        account_id_1,
        account_id_2,
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(friendnships)
}
