use crate::{api_docs, funtional_middleware, middleware::auth_middleware};
use actix_web::{Scope, dev::HttpServiceFactory, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod get_ping;
pub mod ws_subscribe;

pub fn nat_sync() -> impl HttpServiceFactory {
    web::scope("/nat_sync")
        .wrap(funtional_middleware!(auth_middleware, None))
        .service(get_ping::get_ping)
}
