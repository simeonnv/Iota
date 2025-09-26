use actix_web::{HttpRequest, HttpResponse, get};
use error::Error;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::NatSync::Res)]
pub struct Res {
    ip: String,
}

#[utoipa::path(
    get,
    path = "/nat_sync/ping",
    responses(
        (status = 200, description = "account details successful", body = Res, example = json!({
            "ip": "123.23.12.323:54321"
        })),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "NAT Sync"
)]
#[get("/ping")]
pub async fn get_ping(req: HttpRequest) -> Result<HttpResponse, Error> {
    let peer_addr = req.peer_addr();

    let peer_addr = match peer_addr {
        Some(e) => e,
        None => return Err(Error::BadRequest("invalid peer address".into())),
    };

    return Ok(HttpResponse::Ok().json(Res {
        ip: peer_addr.to_string(),
    }));
}
