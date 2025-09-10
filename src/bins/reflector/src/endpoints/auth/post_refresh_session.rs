use actix_web::{HttpResponse, post, web};

use auth::{
    jwt::create_jwt::create_jwt,
    refresh_token::get_refresh_token_data_db::get_refresh_token_data_db,
};
use chashmap::CHashMap;
use chrono::{NaiveDateTime, Utc};
use error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use utoipa::ToSchema;

use crate::{config::JWT_LIFETIME, rolling_rsa::RollingRSA};

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
    rsa_key_pair: web::Data<RwLock<RollingRSA>>,
    session_map: web::Data<CHashMap<String, NaiveDateTime>>,
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
    let jwt = create_jwt(
        &token_data.account_id,
        &token_data.role,
        &JWT_LIFETIME,
        &rsa_key_pair.read().await.encode_key,
    )
    .await?;

    // session_map.insert(body.refresh_token.clone(), now);

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: DataRes { jwt },
    }));
}
