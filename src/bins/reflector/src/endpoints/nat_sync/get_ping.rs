use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use auth::jwt::jwt_claims::JWTClaims;
use chrono::Utc;
use error::Error;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use utoipa::ToSchema;

use crate::nat_subscriber::{NatBody, NatSubsciber};

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
pub async fn get_ping(
    req: HttpRequest,
    nat_subscriber: web::Data<RwLock<NatSubsciber>>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let token_data = match extensions.get::<JWTClaims>() {
        None => return Err(Error::Unauthorized("Unauthorized access".to_string())),
        Some(e) => e,
    };

    let peer_addr = req.peer_addr();
    let peer_addr = match peer_addr {
        Some(e) => e,
        None => return Err(Error::BadRequest("invalid peer address".into())),
    };
    let peer_addr = peer_addr.to_string();
    // let sen = {
    //     let subscriber = nat_subscriber.read().await;
    //     if let Some(sender) = subscriber.nats.get(&token_data.sub) {
    //         sender.clone()
    //     } else {
    //         let mut subscriber = nat_subscriber.write().await;
    //         subscriber.get_sender_or_init(&token_data.sub)
    //     }
    // };

    let sender = {
        if let Some(sender) = nat_subscriber.read().await.get_sender(&token_data.sub) {
            sender.clone()
        } else {
            let mut subscriber = nat_subscriber.write().await;
            subscriber.init_nat(token_data.sub).sender
        }
    };

    sender
        .send(Some(NatBody {
            ip: peer_addr.clone(),
            last_updated: Utc::now().naive_utc(),
        }))
        .map_err(|e| Error::Internal(format!("Nat Broadcast error: {}", e)))?;

    return Ok(HttpResponse::Ok().json(Res { ip: peer_addr }));
}
