use std::sync::Arc;

use error::Error;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn create_postgres_pool(
    postgres_user: &String,
    postgres_password: &String,
    db_address: &String,
    db_port: &String,
    postgres_name: &String,
    max_conn: u32,
) -> Result<Arc<Pool<Postgres>>, Error> {
    let db_url: String = format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres_user, postgres_password, db_address, db_port, postgres_name
    );

    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&db_url)
        .await?;
    let pool = Arc::new(pool);

    Ok(pool)
}
