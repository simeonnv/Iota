use error::Error;
use log::info;
use sqlx::{Pool, Postgres};

use crate::{create_postgres_pool::create_postgres_pool, init_tables::init_tables};

pub async fn init_postgres_db(
    postgres_user: &String,
    postgres_password: &String,
    db_address: &String,
    db_port: u16,
    postgres_name: &String,
    max_conn: u32,
) -> Result<Pool<Postgres>, Error> {
    let pool = create_postgres_pool(
        postgres_user,
        postgres_password,
        db_address,
        db_port,
        postgres_name,
        max_conn,
    )
    .await?;

    init_tables(&pool).await?;

    info!("successfully inited db connection");

    Ok(pool)
}
