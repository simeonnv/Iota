use account::friendships::FriendshipRequest;
use account::friendships::get_friendship_requests_db;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use auth::jwt::jwt_claims::JWTClaims;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::Error;

#[derive(Serialize, ToSchema)]
#[schema(as = Post::NatSync::Ping::Res)]
pub struct Res {
    status: &'static str,
    data: Vec<FriendshipRequest>,
}

#[utoipa::path(
    get,
    path = "/social/requests",
    responses(),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Social"
)]
#[get("/requests")]
pub async fn get_requests(
    req: HttpRequest,
    db_pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let token_data = match extensions.get::<JWTClaims>() {
        None => return Err(Error::Unauthorized("Unauthorized access".to_string())),
        Some(e) => e,
    };

    let friendship_requests = get_friendship_requests_db(&token_data.sub, &db_pool).await?;

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: friendship_requests,
    }));
}
