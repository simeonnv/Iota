use account::query::get_account_by_credentials_db;
use actix_web::{HttpResponse, post, web};

use auth::{
    jwt::create_jwt::create_jwt, refresh_token::create_refresh_token_db::create_refresh_token_db,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use utils::insure_len;
use utoipa::ToSchema;

use crate::{
    Error,
    config::{
        JWT_LIFETIME, MAX_PASS_LENGHT, MAX_USERNAME_LENGHT, MIN_PASS_LENGHT, MIN_USERNAME_LENGHT,
    },
    rolling_rsa::RollingKeyPair,
};

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Login::Req)]
pub struct Req {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Login::Res)]
struct Res {
    status: &'static str,
    data: DataRes,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Login::Res::DataRes)]
struct DataRes {
    refresh_token: String,
    jwt: String,
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = Req,
    responses(
        (status = 200, description = "Login successful", body = Res, example = json!({
            "status": "success",
            "data": {
                "refresh_token": "abc123xyz456",
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
#[post("/login")]
pub async fn post_login(
    body: web::Json<Req>,
    db_pool: web::Data<Pool<Postgres>>,
    rolling_key_pair: web::Data<RwLock<RollingKeyPair>>,
) -> Result<HttpResponse, Error> {
    insure_len(&body.username, MIN_USERNAME_LENGHT, MAX_USERNAME_LENGHT)
        .map_err(|e| Error::BadRequest(e))?;
    insure_len(&body.password, MIN_PASS_LENGHT, MAX_PASS_LENGHT)
        .map_err(|e| Error::BadRequest(e))?;

    let account = get_account_by_credentials_db(&body.username, &body.password, &db_pool).await?;
    let refresh_token =
        create_refresh_token_db(&account.account_id, &account.role, &db_pool).await?;

    let jwt = {
        let rolling_key_pair_read_lock = rolling_key_pair.read().await;
        create_jwt(
            account.account_id,
            account.role,
            JWT_LIFETIME,
            &rolling_key_pair_read_lock.key_pair.private_key,
        )
        .await?
    };

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: DataRes { refresh_token, jwt },
    }));
}
