use crate::middleware::auth_middleware;
use actix_web::{dev::HttpServiceFactory, middleware::from_fn, web};

pub mod get_me;

pub fn accounts() -> impl HttpServiceFactory {
    web::scope("/account")
        .wrap(from_fn(auth_middleware))
        .service(get_me::get_me)
}
