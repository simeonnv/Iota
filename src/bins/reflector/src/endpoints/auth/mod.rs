use actix_web::{
    Scope,
    web::{self, Data},
};
use chashmap::CHashMap;
use chrono::NaiveDateTime;

pub mod post_login;
pub mod post_refresh_session;
pub mod post_signup;

pub fn auth() -> Scope {
    // used for insuring that refresh token doesnt already have a jwt assigned to it
    // this is to prevent jwt DDoS attack
    let session_map: Data<CHashMap<String, NaiveDateTime>> = Data::new(CHashMap::new());

    web::scope("/auth")
        .service(post_signup::post_signup)
        .service(post_login::post_login)
        .app_data(session_map)
        .service(post_refresh_session::post_refresh_session)
}
