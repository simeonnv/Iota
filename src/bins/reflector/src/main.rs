use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    middleware::Logger,
    web::{Data, PayloadConfig},
};

use auth::{account::create_account_db::create_account_db, jwt::algorithm_type::AlgorithmType};
use db::init_postgres_db;
use env_logger::Env;
use log::info;

use crate::{env::ENVVARS, rolling_rsa::RollingKeyPair};

pub mod api_docs;
pub mod config;
pub mod endpoints;
pub mod env;
pub mod middleware;
pub mod rolling_rsa;

use endpoints::endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = ENVVARS.rust_log;
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Server starting up");

    let db_pool = init_postgres_db(
        &ENVVARS.postgres_user,
        &ENVVARS.postgres_password,
        &ENVVARS.db_address,
        ENVVARS.db_port,
        &ENVVARS.postgres_name,
        ENVVARS.pool_max_conn,
    )
    .await?;

    let db_pool = Data::new(db_pool);
    let rsa_key_pair =
        RollingKeyPair::init(db_pool.clone().into_inner(), AlgorithmType::Falcon512).await?;
    let rsa_key_pair = Data::from(rsa_key_pair);

    let _ = create_account_db(&"admin".into(), &"admin".into(), "admin", &db_pool).await;
    info!(
        "Server listening on {}:{}",
        ENVVARS.db_address, ENVVARS.db_port,
    );

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i %T"))
            .app_data(PayloadConfig::new(1 * 1024 * 1024)) // 1 mb max upload
            .app_data(db_pool.clone())
            .app_data(rsa_key_pair.clone())
            .service(endpoints())
    })
    .bind(("0.0.0.0", 25025))?
    .run()
    .await
}
