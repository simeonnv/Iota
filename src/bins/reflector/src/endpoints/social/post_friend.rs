use account::friendships::create_friendship_request_db;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};

use auth::jwt::jwt_claims::JWTClaims;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::Error;

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
    description = "create a friendrequest to smb",
    responses(),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Social"
)]
#[post("/friend")]
pub async fn post_friend(
    body: web::Json<Req>,
    db_pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let token_data = match extensions.get::<JWTClaims>() {
        None => return Err(Error::Unauthorized("Unauthorized access".to_string())),
        Some(e) => e,
    };

    create_friendship_request_db(&token_data.sub, &body.account_uuid, &db_pool).await?;
    return Ok(HttpResponse::Ok().json(Res { status: "success" }));
}
