use account::{
    friendships::{
        create_friendship_request_db, get_friend_request_by_in_out_db, get_friendship_by_accounts,
    },
    query::get_account_by_id,
};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, post, web};
use auth::jwt::jwt_claims::JWTClaims;
use db::tables::friendships::FriendshipLevel;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::Error;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Social::Friends::Req)]
pub struct Req {
    pub account_uuid: Uuid,
    pub for_friendship_level: FriendshipLevel,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Social::Friends::Res)]
struct Res {
    status: &'static str,
}

#[utoipa::path(
    post,
    path = "/social/friends",
    request_body = Req,
    description = "create a friend request to someone",
    responses(),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Social"
)]
#[post("/friends")]
pub async fn post_friends(
    body: web::Json<Req>,
    db_pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let token_data = match extensions.get::<JWTClaims>() {
        None => return Err(Error::Unauthorized("Unauthorized access".to_string())),
        Some(e) => e,
    };

    get_account_by_id(body.account_uuid, &db_pool).await?;

    let maybe_existing_friendrequest =
        get_friend_request_by_in_out_db(&token_data.sub, &body.account_uuid, &db_pool).await?;

    if maybe_existing_friendrequest.is_some() {
        return Ok(HttpResponse::Conflict().body("friendrequest already exists!"));
    }

    let friendship =
        get_friendship_by_accounts(&token_data.sub, &body.account_uuid, &db_pool).await?;

    if let Some(friendship) = friendship {
        let friendship_level: FriendshipLevel = friendship.for_friendship_level.into();
        match (friendship_level, body.for_friendship_level) {
            (FriendshipLevel::Trusted, FriendshipLevel::Trusted)
            | (FriendshipLevel::Normal, FriendshipLevel::Normal) => {
                return Ok(
                    HttpResponse::Conflict().body("friendship already exists with such level!")
                );
            }
            (FriendshipLevel::Trusted, FriendshipLevel::Normal) => {
                return Ok(
                    HttpResponse::Conflict().body("friendship already exists with a higher level!")
                );
            }
            _ => {}
        }
    }

    create_friendship_request_db(
        &token_data.sub,
        &body.account_uuid,
        body.for_friendship_level.clone(),
        &db_pool,
    )
    .await?;
    Ok(HttpResponse::Ok().json(Res { status: "success" }))
}
