use actix_web::{
    Error as ActixError,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};
use error::Error;
use gateway::rate_limiter::leaky_bucket::leaky_bucket_rate_limiter::LeakyBucketRateLimiter;
use std::{net::IpAddr, str::FromStr, sync::Arc};

pub async fn rate_limiter_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
    rate_limiter: Arc<LeakyBucketRateLimiter>,
) -> Result<ServiceResponse<impl MessageBody>, ActixError> {
    let ip = {
        let conn_info = req.connection_info();
        let ip = conn_info.realip_remote_addr();
        let ip = match ip {
            Some(e) => e,
            None => return Err(Error::BadRequest("Invalid ip address".into()).into()),
        };
        IpAddr::from_str(ip).map_err(|_| Error::BadRequest("invalid ip address".into()))?
    };

    let rate_limited = rate_limiter.check_rate_limit(ip).await;

    if rate_limited {
        return Err(Error::ErrorTooManyRequests("Rate limited".into()).into());
    }

    next.call(req).await
}
