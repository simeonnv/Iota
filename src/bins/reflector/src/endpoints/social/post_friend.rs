use account::query::get_account_by_credentials_db;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};

use auth::{
    jwt::{create_jwt::create_jwt, jwt_claims::JWTClaims},
    refresh_token::create_refresh_token_db::create_refresh_token_db,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use utils::insure_len;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    Error,
    config::{
        JWT_LIFETIME, MAX_PASS_LENGHT, MAX_USERNAME_LENGHT, MIN_PASS_LENGHT, MIN_USERNAME_LENGHT,
    },
    rolling_rsa::RollingKeyPair,
};

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Social::Friend::Req)]
pub struct Req {
    pub account_uuid: Uuid,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Social::Friend::Res)]
struct Res {
    status: &'static str,
}

#[utoipa::path(
    post,
    path = "/social/friend",
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
    tag = "Social"
)]
#[post("/friend")]
pub async fn post_friend(
    body: web::Json<Req>,
    db_pool: web::Data<Pool<Postgres>>,
    rolling_key_pair: web::Data<RwLock<RollingKeyPair>>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let token_data = match extensions.get::<JWTClaims>() {
        None => return Err(Error::Unauthorized("Unauthorized access".to_string())),
        Some(e) => e,
    };

    return Ok(HttpResponse::Ok().json(Res { status: "success" }));
}
