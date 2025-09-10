use std::sync::Arc;

use crate::config::{AUTH_RATE_LIMIT_CAP, AUTH_RATE_LIMIT_LEAK};
use crate::funtional_middleware;
use crate::middleware::rate_limiter_middleware;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::{self, Data};
use chashmap::CHashMap;
use chrono::NaiveDateTime;
use gateway::rate_limiter::leaky_bucket::leaky_bucket_rate_limiter::LeakyBucketRateLimiter;
use lazy_static::lazy_static;

pub mod post_login;
pub mod post_refresh_session;
pub mod post_signup;

lazy_static! {
    // used for insuring that refresh token doesnt already have a jwt assigned to it
    // this is to prevent jwt DDoS attack
    pub static ref SESSION_MAP: Data<CHashMap<String, NaiveDateTime>> = Data::new(CHashMap::new());
    pub static ref AUTH_RATE_LIMITER: Arc<LeakyBucketRateLimiter> = Arc::new(
        LeakyBucketRateLimiter::new(AUTH_RATE_LIMIT_CAP, AUTH_RATE_LIMIT_LEAK)
    );

}

pub fn auth() -> impl HttpServiceFactory {
    web::scope("/auth")
        .wrap(funtional_middleware!(
            rate_limiter_middleware,
            AUTH_RATE_LIMITER.clone()
        ))
        .app_data(SESSION_MAP.clone())
        .service(post_signup::post_signup)
        .service(post_login::post_login)
        .service(post_refresh_session::post_refresh_session)
}
