use utoipa::Modify;
use utoipa::OpenApi;

use crate::endpoints;

use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

struct BearerAuthAddon;
impl Modify for BearerAuthAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        endpoints::auth::post_signup::post_signup,
        endpoints::auth::post_login::post_login,
        endpoints::auth::post_refresh_session::post_refresh_session,

        endpoints::account::get_me::get_me,

        endpoints::nat_sync::get_ping::get_ping,

        endpoints::social::post_friend::post_friend,
        endpoints::social::get_requests::get_requests
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Account", description = "Account managment, and account data fetching"),
        (name = "NAT Sync", description = "Everything doing with synchronizing nats and obtaining nats mappings"),
        (name = "Social", description = "Endpoints for handling your friends")
    ),
    modifiers(&BearerAuthAddon),
    security(
        ("bearer_auth" = [])
    )

)]
pub struct ApiDoc;
