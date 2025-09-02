use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get};
use db::init_postgres_db;

use crate::env::ENVVARS;

pub mod env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = init_postgres_db(
        &ENVVARS.postgres_user,
        &ENVVARS.postgres_password,
        &ENVVARS.db_address,
        ENVVARS.db_port,
        &ENVVARS.postgres_name,
        ENVVARS.pool_max_conn,
    )
    .await?;

    println!(
        "Server listening on {}:{}",
        ENVVARS.db_address, ENVVARS.db_port
    );

    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 25025))?
        .run()
        .await
}

#[get("/")]
async fn hello(req: HttpRequest) -> impl Responder {
    dbg!(req.peer_addr());

    HttpResponse::Ok().body("Hello world!")
}
