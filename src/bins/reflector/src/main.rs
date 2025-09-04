use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web::Data};
use db::init_postgres_db;
use env_logger::Env;
use log::info;

use crate::{env::ENVVARS, rolling_rsa::RollingRSA};

pub mod env;
pub mod rolling_rsa;

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
    let rsa_key_pair = RollingRSA::init(db_pool.clone().into_inner()).await?;
    let rsa_key_pair = Data::new(rsa_key_pair);

    info!(
        "Server listening on {}:{}",
        ENVVARS.db_address, ENVVARS.db_port,
    );

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .app_data(rsa_key_pair.clone())
            .service(hello)
    })
    .bind(("0.0.0.0", 25025))?
    .run()
    .await
}

#[get("/")]
async fn hello(req: HttpRequest) -> impl Responder {
    dbg!(req.peer_addr());

    HttpResponse::Ok().body("Hello world!")
}
