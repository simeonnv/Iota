use log::info;
use sqlx::{Pool, Postgres};

use crate::{
    Error,
    tables::{
        accounts::init_accounts_table, friendship_requests::init_friendship_requests_table,
        friendships::init_friendships_table, key_pair::init_key_pairs_table,
        refresh_tokens::init_refresh_tokens_table,
    },
};

pub async fn init_tables(pool: &Pool<Postgres>) -> Result<(), Error> {
    init_accounts_table(pool).await?;
    init_key_pairs_table(pool).await?;
    init_refresh_tokens_table(pool).await?;
    init_friendships_table(pool).await?;
    init_friendship_requests_table(pool).await?;

    info!("Inited tables for the Database");

    Ok(())
}
