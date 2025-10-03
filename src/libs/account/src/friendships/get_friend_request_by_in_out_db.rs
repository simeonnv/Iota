use crate::Error;
use db::tables::friendships::Friendships;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_friend_request_by_in_out_db(
    account_in_id: &Uuid,
    account_out_id: &Uuid,
    db_pool: &Pool<Postgres>,
) -> Result<Option<Friendships>, Error> {
    let friendnships = sqlx::query_as!(
        Friendships,
        r#"
            SELECT
                *
            FROM Friendships
            WHERE account_in = $1 AND account_out = $2;
    "#,
        account_in_id,
        account_out_id
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(friendnships)
}
