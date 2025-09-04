use error::Error;
use log::{info, warn};
use sqlx::{Database, Executor, Pool};

use crate::tables::{
    INIT_ACCOUNTS_INDEXES, INIT_ACCOUNTS_TABLE, INIT_REFRESH_TOKEN_INDEXES,
    INIT_REFRESH_TOKEN_TABLE, INIT_RSAKEYPAIR_TABLE,
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
        queries.push(INIT_ACCOUNTS_TABLE);
        queries.push(INIT_ACCOUNTS_INDEXES);

        queries.push(INIT_RSAKEYPAIR_TABLE);

        queries.push(INIT_REFRESH_TOKEN_TABLE);
        queries.push(INIT_REFRESH_TOKEN_INDEXES);
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
