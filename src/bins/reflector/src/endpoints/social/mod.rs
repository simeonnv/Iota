use crate::{funtional_middleware, middleware::auth_middleware};
use actix_web::{dev::HttpServiceFactory, web};

pub mod get_friends;
pub mod get_requests;
pub mod get_requests_request_id_accept;
pub mod post_friends;

pub fn social() -> impl HttpServiceFactory {
    web::scope("/social")
        .wrap(funtional_middleware!(auth_middleware, None))
        .service(post_friends::post_friends)
        .service(get_requests::get_requests)
        .service(get_requests_request_id_accept::get_requests_request_id_accept)
        .service(get_friends::get_friends)
}
