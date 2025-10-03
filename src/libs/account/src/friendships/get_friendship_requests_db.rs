use crate::Error;
use db::tables::friendship_requests::FriendshipRequests;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_friendship_requests_db(
    account_id: &Uuid,
    db_pool: &Pool<Postgres>,
) -> Result<Vec<FriendshipRequests>, Error> {
    let friendnships: Vec<FriendshipRequests> = sqlx::query_as!(
        FriendshipRequests,
        r#"
            SELECT
                *
            FROM FriendshipRequests 
            WHERE account_in = $1 OR account_out = $1;
        "#,
        account_id
    )
    .fetch_all(db_pool)
    .await?
    .into();

    Ok(friendnships)
}
