use crypto::hashing::argon2_hash;
use error::Error;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_account_db(
    username: &String,
    password: &String,
    role: &'static str,
    db_pool: &Pool<Postgres>,
) -> Result<Uuid, Error> {
    let hashed_password = argon2_hash(password).await?;
    let account_id = Uuid::new_v4();

    sqlx::query(
        r#"

        INSERT INTO Accounts
            (account_id, role, username, password)
            VALUES ($1, $2, $3, $4)
        ;

    "#,
    )
    .bind(&account_id)
    .bind(role)
    .bind(username)
    .bind(hashed_password)
    .execute(db_pool)
    .await?;

    Ok(account_id)
}
