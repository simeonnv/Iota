use crate::{funtional_middleware, middleware::auth_middleware};
use actix_web::{dev::HttpServiceFactory, web};

pub mod get_me;

pub fn accounts() -> impl HttpServiceFactory {
    web::scope("/account")
        .wrap(funtional_middleware!(auth_middleware, None))
        .service(get_me::get_me)
}
