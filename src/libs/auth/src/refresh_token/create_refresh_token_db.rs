use crypto::rand_string;
use error::Error;
use log::debug;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_refresh_token_db(
    account_id: &Uuid,
    role: &String,
    db_pool: &Pool<Postgres>,
) -> Result<String, Error> {
    let token = rand_string(255);

    sqlx::query(
        r#"

        INSERT INTO RefreshTokens
            (refresh_token_id, role, refresh_token, account_id)
            VALUES ($1, $2, $3, $4)
        ;

    "#,
    )
    .bind(Uuid::new_v4())
    .bind(role)
    .bind(&token)
    .bind(account_id)
    .execute(db_pool)
    .await?;

    debug!(
        "created refresh token for: {} with role: {}",
        account_id, role
    );

    Ok(token)
}
