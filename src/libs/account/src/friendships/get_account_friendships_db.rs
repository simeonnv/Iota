// SELECT
//     f.friendship_id,
//     f.friendship_level,
//     f.created_at,
//     CASE
//         WHEN f.account_in = $1 THEN f.account_out
//         ELSE f.account_in
//     END AS friend_account_id
// FROM Friendships f
// WHERE f.account_in = $1 OR f.account_out = $1;

use crate::Error;
use sqlx::{Pool, Postgres, types::chrono::NaiveDateTime};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct Friendship {
    pub friendship_id: Uuid,
    pub friendship_level: String,
    pub created_at: NaiveDateTime,
    pub friend_account_id: Uuid,
}

pub async fn get_account_friendships_db(
    account_id: &Uuid,
    db_pool: &Pool<Postgres>,
) -> Result<Vec<Friendship>, Error> {
    let friendnships: Vec<Friendship> = sqlx::query_as(
        r#"
            SELECT
                f.friendship_id,
                f.friendship_level,
                f.created_at,
                CASE
                    WHEN f.account_in = $1 THEN f.account_out
                    ELSE f.account_in
                END AS friend_account_id
            FROM Friendships f
            WHERE f.account_in = $1 OR f.account_out = $1;
    "#,
    )
    .bind(account_id)
    .fetch_all(db_pool)
    .await?
    .into();

    Ok(friendnships)
}
