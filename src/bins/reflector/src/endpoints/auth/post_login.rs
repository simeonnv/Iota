use actix_web::{HttpResponse, post, web};

use auth::{
    account::get_account_by_credentials_db::get_account_by_credentials_db,
    jwt::create_jwt::create_jwt, refresh_token::create_refresh_token_db::create_refresh_token_db,
};
use error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use utils::insure_len;
use utoipa::ToSchema;

use crate::{
    config::{
        JWT_LIFETIME, MAX_PASS_LENGHT, MAX_USERNAME_LENGHT, MIN_PASS_LENGHT, MIN_USERNAME_LENGHT,
    },
    rolling_rsa::RollingRSA,
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
    rsa_key_pair: web::Data<RwLock<RollingRSA>>,
) -> Result<HttpResponse, Error> {
    insure_len(&body.username, MIN_USERNAME_LENGHT, MAX_USERNAME_LENGHT)?;
    insure_len(&body.password, MIN_PASS_LENGHT, MAX_PASS_LENGHT)?;

    let account = get_account_by_credentials_db(&body.username, &body.password, &db_pool).await?;
    let refresh_token =
        create_refresh_token_db(&account.account_id, &account.role, &db_pool).await?;

    let jwt = create_jwt(
        &account.account_id,
        &account.role,
        &JWT_LIFETIME,
        &rsa_key_pair.read().await.encode_key,
    )
    .await?;

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: DataRes { refresh_token, jwt },
    }));
}
