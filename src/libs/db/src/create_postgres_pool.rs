use error::Error;
use log::info;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn create_postgres_pool(
    postgres_user: &String,
    postgres_password: &String,
    db_address: &String,
    db_port: u16,
    postgres_name: &String,
    max_conn: u32,
) -> Result<Pool<Postgres>, Error> {
    let db_url: String = format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres_user, postgres_password, db_address, db_port, postgres_name
    );

    info!("creating a connection with db: {}", postgres_name);

    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&db_url)
        .await?;

    Ok(pool)
}
