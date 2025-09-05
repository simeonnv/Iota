use crate::api_docs;
use actix_web::{Scope, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod get_ping;

pub fn endpoints() -> Scope {
    web::scope("")
        .service(
            SwaggerUi::new("/swagger/{_:.*}")
                .url("/api-docs/openapi.json", api_docs::ApiDoc::openapi()),
        )
        .service(auth::auth())
        .service(get_ping::get_ping)
}
