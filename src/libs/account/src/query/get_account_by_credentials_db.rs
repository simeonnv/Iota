use crypto::hashing::compare_argon2_hash;
use db::tables::Accounts;
use serde::{Deserialize, Serialize};

use sqlx::{Pool, Postgres};

use crate::Error;

#[derive(Serialize, Deserialize)]
struct Res {
    status: String,
    data: &'static str,
}

pub async fn get_account_by_credentials_db(
    username: &String,
    password: &String,
    db_pool: &Pool<Postgres>,
) -> Result<Accounts, Error> {
    let db_res: Option<Accounts> = sqlx::query_as(
        r#"
            SELECT * FROM Accounts
                WHERE username = $1
            ;
        "#,
    )
    .bind(username)
    .fetch_optional(db_pool)
    .await?;

    let account = match db_res {
        Some(value) => value,
        None => {
            return Err(Error::InvalidCredentials(
                "Invalid username or password!".into(),
            ));
        }
    };

    match compare_argon2_hash(password, &account.password).await? {
        true => Ok(account),
        false => Err(Error::InvalidCredentials(
            "Invalid username or password!".to_string(),
        )),
    }
}
