use crate::{funtional_middleware, middleware::auth_middleware};
use actix_web::{dev::HttpServiceFactory, web};

pub mod get_ping;
pub mod ws_subscribe;

pub fn nat_sync() -> impl HttpServiceFactory {
    web::scope("/nat_sync")
        .wrap(funtional_middleware!(auth_middleware, None))
        .service(ws_subscribe::ws_subscribe)
        .service(get_ping::get_ping)
}
