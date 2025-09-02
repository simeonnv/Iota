use error::Error;
use log::info;
use sqlx::{Database, Executor, Pool};

use crate::tables::ACCOUNTS_INIT;

pub async fn init_tables<T: Database>(pool: &Pool<T>) -> Result<(), Error>
where
    T: Database,
    for<'c> &'c mut <T as Database>::Connection: Executor<'c, Database = T>,
    for<'a> <T as Database>::Arguments<'a>: sqlx::IntoArguments<'a, T>,
{
    let mut queries = Vec::new();

    // push all table schemas as needed
    {
        queries.push(ACCOUNTS_INIT);
    }

    for query in queries.into_iter() {
        sqlx::query(query).execute(pool).await?;
    }

    info!("Inited tables for the Database");

    Ok(())
}
