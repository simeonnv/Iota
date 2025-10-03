use crate::{Error, friendships::create_friendship_db};
use db::tables::friendship_requests::FriendshipRequests;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn accept_friend_request(
    account_id: &Uuid,
    friendship_request_id: &Uuid,
    db_pool: &Pool<Postgres>,
) -> Result<(), Error> {
    let friendship_request: Option<FriendshipRequests> = sqlx::query_as!(
        FriendshipRequests,
        r#"         
        SELECT * FROM FriendshipRequests WHERE friendship_request_id = $1 AND account_out = $2;
    "#,
        friendship_request_id,
        account_id,
    )
    .fetch_optional(db_pool)
    .await?;

    let friendship_request = friendship_request.ok_or(Error::FriendRequestDoesntExist())?;

    sqlx::query!(
        r#"
        DELETE FROM FriendshipRequests WHERE friendship_request_id = $1;
        "#,
        friendship_request_id
    )
    .execute(db_pool)
    .await?;

    create_friendship_db(
        &friendship_request.account_in,
        &friendship_request.account_out,
        friendship_request.for_friendship_level,
        db_pool,
    )
    .await?;

    Ok(())
}
