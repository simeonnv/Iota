use actix_web::{HttpResponse, post, web};
use auth::{
    account::{create_account_db::create_account_db, does_account_exist_db::does_account_exist_db},
    jwt::create_jwt::create_jwt,
    refresh_token::create_refresh_token_db::create_refresh_token_db,
};
use chrono::Duration;
use error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use utils::insure_len::insure_len;
use utoipa::ToSchema;

use crate::rolling_rsa::RollingRSA;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Signup::Req)]
pub struct Req {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Signup::Res)]
struct Res {
    status: &'static str,
    data: DataRes,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Signup::Res::DataRes)]
struct DataRes {
    refresh_token: String,
    jwt: String,
}

#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = Req,
    responses(
        (status = 200, description = "Signup successful", body = Res, example = json!({
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
#[post("/signup")]
pub async fn post_signup(
    body: web::Json<Req>,
    db_pool: web::Data<Pool<Postgres>>,
    rsa_key_pair: web::Data<RwLock<RollingRSA>>,
) -> Result<HttpResponse, Error> {
    insure_len(&body.username, 5, 20)?;
    insure_len(&body.password, 5, 30)?;

    if does_account_exist_db(&body.username, &db_pool).await? {
        return Err(Error::Conflict("Account already exists".to_string()));
    }

    let account_id = create_account_db(&body.username, &body.password, "user", &db_pool).await?;

    let refresh_token = create_refresh_token_db(&account_id, &"user".to_string(), &db_pool).await?;
    let jwt = create_jwt(
        &account_id,
        &"user".into(),
        &Duration::minutes(30),
        &rsa_key_pair.read().await.encode_key,
    )
    .await?;

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: DataRes { refresh_token, jwt },
    }));
}
