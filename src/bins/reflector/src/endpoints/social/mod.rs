use crate::{funtional_middleware, middleware::auth_middleware};
use actix_web::{dev::HttpServiceFactory, web};

pub mod get_requests;
pub mod post_friend;

pub fn social() -> impl HttpServiceFactory {
    web::scope("/social")
        .wrap(funtional_middleware!(auth_middleware, None))
        .service(post_friend::post_friend)
        .service(get_requests::get_requests)
}
