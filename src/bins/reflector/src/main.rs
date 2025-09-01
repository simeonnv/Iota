use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get};
use error::Error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
