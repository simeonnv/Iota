use account::friendships::accept_friend_request;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use auth::jwt::jwt_claims::JWTClaims;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::Error;

#[derive(Serialize, ToSchema)]
#[schema(as = Post::Social::Requests::RequestId::Res)]
pub struct Res {
    status: &'static str,
}

#[utoipa::path(
    get,
    path = "/social/requests/{request_id}/accept",
    responses(),
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("request_id" = Uuid, Path, description = "the friend request id"),
    ),
    tag = "Social"
)]
#[get("/requests/{request_id}/accept")]
pub async fn get_requests_request_id_accept(
    req: HttpRequest,
    path: web::Path<(Uuid,)>,
    db_pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let token_data = match extensions.get::<JWTClaims>() {
        None => return Err(Error::Unauthorized("Unauthorized access".to_string())),
        Some(e) => e,
    };
    let friendship_request_id = path.into_inner().0;

    accept_friend_request(&token_data.sub, &friendship_request_id, &db_pool).await?;

    return Ok(HttpResponse::Ok().json(Res { status: "success" }));
}
