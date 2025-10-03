use crate::Error;
use db::tables::friendships::FriendshipLevel;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_friendship_request_db(
    account_from: &Uuid,
    account_to: &Uuid,
    friendship_level: FriendshipLevel,
    db_pool: &Pool<Postgres>,
) -> Result<(), Error> {
    let friendship_request_id = Uuid::new_v4();

    sqlx::query!(
        r#"         
        INSERT INTO FriendshipRequests
            (friendship_request_id, account_in, account_out, for_friendship_level)
            VALUES ($1, $2, $3, $4)
        ;
    "#,
        friendship_request_id,
        account_from,
        account_to,
        friendship_level.as_str(),
    )
    .execute(db_pool)
    .await?;

    Ok(())
}
