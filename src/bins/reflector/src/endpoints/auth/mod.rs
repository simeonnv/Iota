use actix_web::{Scope, web};

pub mod login;
pub mod post_signup;

pub fn auth() -> Scope {
    web::scope("/auth").service(post_signup::post_signup)
}
