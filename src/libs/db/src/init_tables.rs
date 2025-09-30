use log::{info, warn};
use sqlx::{Database, Executor, Pool};

use crate::{
    Error,
    tables::{
        INIT_ACCOUNTS_INDEX_ACCOUNT_ID, INIT_ACCOUNTS_INDEX_USERNAME, INIT_ACCOUNTS_TABLE,
        INIT_FRIENDSHIPS_INDEX_IN, INIT_FRIENDSHIPS_INDEX_OUT, INIT_FRIENDSHIPS_TABLE,
        INIT_REFRESH_TOKEN_INDEX_REFRESH_TOKEN_ID, INIT_REFRESH_TOKEN_TABLE, INIT_RSAKEYPAIR_TABLE,
    },
};

pub async fn init_tables<T: Database>(pool: &Pool<T>) -> Result<(), Error>
where
    T: Database,
    for<'c> &'c mut <T as Database>::Connection: Executor<'c, Database = T>,
    for<'a> <T as Database>::Arguments<'a>: sqlx::IntoArguments<'a, T>,
{
    let mut queries = Vec::new();

    // push all table schemas as needed
    {
        // accounts
        queries.push(INIT_ACCOUNTS_TABLE);
        queries.push(INIT_ACCOUNTS_INDEX_ACCOUNT_ID);
        queries.push(INIT_ACCOUNTS_INDEX_USERNAME);

        // rsa key pair
        queries.push(INIT_RSAKEYPAIR_TABLE);

        // refresh token
        queries.push(INIT_REFRESH_TOKEN_TABLE);
        queries.push(INIT_REFRESH_TOKEN_INDEX_REFRESH_TOKEN_ID);

        // friendships
        queries.push(INIT_FRIENDSHIPS_TABLE);
        queries.push(INIT_FRIENDSHIPS_INDEX_IN);
        queries.push(INIT_FRIENDSHIPS_INDEX_OUT);
    }

    for query in queries.into_iter() {
        match sqlx::query(query).execute(pool).await {
            Ok(e) => e,
            Err(err) => {
                warn!("database init error: {}", err);
                continue;
            }
        };
    }

    info!("Inited tables for the Database");

    Ok(())
}
