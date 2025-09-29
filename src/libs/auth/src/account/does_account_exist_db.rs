use sqlx::{Pool, Postgres};

use crate::Error;

pub async fn does_account_exist_db(
    username: &String,
    db_pool: &Pool<Postgres>,
) -> Result<bool, Error> {
    let account_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) AS count
            FROM Accounts
            WHERE username = $1
        ;
    "#,
    )
    .bind(username)
    .fetch_one(db_pool)
    .await?;

    Ok(account_count > 0)
}
