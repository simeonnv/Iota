use actix_web::{HttpResponse, post, web};

use auth::{
    jwt::create_jwt::create_jwt,
    refresh_token::get_refresh_token_data_db::get_refresh_token_data_db,
};
use chrono::{NaiveDateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use utoipa::ToSchema;

use crate::{Error, config::JWT_LIFETIME, rolling_rsa::RollingKeyPair};

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::RefreshSession::Req)]
pub struct Req {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::RefreshSession::Res)]
struct Res {
    status: &'static str,
    data: DataRes,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::RefreshSession::Res::DataRes)]
struct DataRes {
    jwt: String,
}

#[utoipa::path(
    post,
    path = "/auth/refresh_session",
    request_body = Req,
    responses(
        (status = 200, description = "Login successful", body = Res, example = json!({
            "status": "success",
            "data": {
                "jwt": "abc123xyz456"
            }
        })),
        (status = 401, description = "Unauthorized", body = Res, example = json!({
            "status": "incorrect credential",
            "data": ""
        })),
        (status = 409, description = "Conflict", body = Res, example = json!({
            "status": "account already exists",
            "data": ""
        }))
    ),
    security(),
    tag = "Auth"
)]
#[post("/refresh_session")]
pub async fn post_refresh_session(
    body: web::Json<Req>,
    db_pool: web::Data<Pool<Postgres>>,
    rolling_key_pair: web::Data<RwLock<RollingKeyPair>>,
    session_map: web::Data<DashMap<String, NaiveDateTime>>,
) -> Result<HttpResponse, Error> {
    dbg!(&session_map);
    let last_session = session_map.get(&body.refresh_token);
    let now = Utc::now().naive_utc();

    if let Some(last_session) = last_session
        && *last_session < now + JWT_LIFETIME
    {
        return Err(Error::Conflict(
            "there is aleady a jwt in use for your session!".into(),
        ));
    }

    let token_data = get_refresh_token_data_db(&body.refresh_token, &db_pool).await?;
    let jwt = {
        let rolling_key_pair_read_lock = rolling_key_pair.read().await;
        create_jwt(
            token_data.account_id,
            token_data.role,
            JWT_LIFETIME,
            &rolling_key_pair_read_lock.key_pair.private_key,
        )
        .await?
    };

    session_map.insert(body.refresh_token.clone(), now);

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: DataRes { jwt },
    }));
}
