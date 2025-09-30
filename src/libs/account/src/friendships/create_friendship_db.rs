use crate::Error;
use db::tables::FriendshipLevel;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_friendship_db(
    account_in: &Uuid,
    account_out: &Uuid,
    db_pool: &Pool<Postgres>,
) -> Result<(), Error> {
    let friendship_id = Uuid::new_v4();

    sqlx::query(
        r#"

        INSERT INTO Friendships
            (friendship_id, account_in, account_out, friendship_level)
            VALUES ($1, $2, $3, $4)
        ;

    "#,
    )
    .bind(friendship_id)
    .bind(account_in)
    .bind(account_out)
    .bind(FriendshipLevel::Normal.as_str())
    .execute(db_pool)
    .await?;

    Ok(())
}
