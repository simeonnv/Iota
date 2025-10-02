use db::init_postgres_db;
use envconfig::Envconfig;
use lazy_static::lazy_static;
use std::path::Path;

#[derive(Envconfig)]
pub struct EnvVars {
    #[envconfig(from = "POSTGRES_NAME")]
    pub postgres_name: String,

    #[envconfig(from = "POSTGRES_USER")]
    pub postgres_user: String,

    #[envconfig(from = "POSTGRES_PASSWORD")]
    pub postgres_password: String,

    #[envconfig(from = "DB_ADDRESS")]
    pub db_address: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,

    #[envconfig(from = "POOL_MAX_CONN", default = "5")]
    pub pool_max_conn: u32,
}

lazy_static! {
    pub static ref ENVVARS: EnvVars = load_env_vars();
}

pub fn load_env_vars() -> EnvVars {
    // if dotenv().ok().is_none() {
    let env_path = Path::new("./.env.dev");

    if let Err(e) = dotenv::from_path(env_path) {
        panic!("Failed to load {} file: {}", env_path.display(), e);
    }
    // }
    let env_vars = EnvVars::init_from_env();
    match env_vars {
        Ok(e) => return e,
        Err(e) => panic!("failed to serialize .env: {}", e),
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    init_postgres_db(
        &ENVVARS.postgres_user,
        &ENVVARS.postgres_password,
        &ENVVARS.db_address,
        ENVVARS.db_port,
        &ENVVARS.postgres_name,
        ENVVARS.pool_max_conn,
    )
    .await
    .expect("failed to init db in build.rs :3");
}
