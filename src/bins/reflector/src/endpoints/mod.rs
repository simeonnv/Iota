use crate::api_docs;
use actix_web::{Scope, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod account;
pub mod auth;
pub mod get_ping;
pub mod nat_sync;

pub fn endpoints() -> Scope {
    web::scope("")
        .service(
            SwaggerUi::new("/swagger/{_:.*}")
                .url("/api-docs/openapi.json", api_docs::ApiDoc::openapi()),
        )
        .service(web::redirect("/swagger", "/swagger/"))
        .service(get_ping::get_ping)
        .service(auth::auth())
        .service(account::accounts())
        .service(nat_sync::nat_sync())
}
