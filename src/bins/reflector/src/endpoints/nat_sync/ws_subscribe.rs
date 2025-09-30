use crate::{Error, nat_subscriber::NatSubsciber};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use actix_ws::Message;
use auth::jwt::jwt_claims::JWTClaims;
use log::{error, info};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::NatSync::Subscribe::Req)]
pub struct Req {
    uuid: Uuid,
}

#[utoipa::path(
    get,
    path = "/nat_sync/subscribe",
    request_body(content = Req, description = "UUID to subscribe to updates", content_type = "application/json"),
    responses(
        (status = 200, description = "WebSocket connection established"),
        (status = 401, description = "Unauthorized", body = String, example = "Unauthorized access"),
        (status = 400, description = "Bad Request", body = String, example = "Invalid UUID or WebSocket error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "WebSocket"
)]
#[get("/subscribe")]
pub async fn ws_subscribe(
    req: HttpRequest,
    stream: web::Payload,
    nat_subscriber: web::Data<RwLock<NatSubsciber>>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)
        .map_err(|e| Error::BadRequest(format!("Unable to establish WebSocket: {}", e)))?;

    let extensions = req.extensions();
    let token_data = extensions
        .get::<JWTClaims>()
        .ok_or_else(|| Error::Unauthorized("Unauthorized access".to_string()))?;

    // Spawn async task to handle WebSocket messages
    actix_web::rt::spawn(async move {
        // Receive UUID from initial WebSocket message
        let uuid = match msg_stream.recv().await {
            Some(Ok(Message::Text(text))) => match serde_json::from_str::<Req>(&text) {
                Ok(req) => req.uuid,
                Err(e) => {
                    let _ = session.text(format!("Invalid UUID format: {}", e)).await;
                    return;
                }
            },
            _ => {
                let _ = session.text("Expected UUID in initial message").await;
                return;
            }
        };

        // Subscribe to NAT updates
        let mut receiver = {
            let subscriber = nat_subscriber.read().await;
            if let Some(sender) = subscriber.subscribe(&uuid) {
                sender.clone()
            } else {
                let mut subscriber = nat_subscriber.write().await;
                subscriber.init_nat(uuid).receiver
            }
        };

        // Main loop to handle WebSocket messages and NAT updates
        loop {
            tokio::select! {
                Some(Ok(msg)) = msg_stream.recv() => {
                    match msg {
                        Message::Close(_) => {
                            info!("WebSocket connection closed for UUID: {}", uuid);
                            let _ = session.close(None).await;
                            break;
                        }
                        _ => {} // Ignore other message types
                    }
                }
                changed = receiver.changed() => {
                    if changed.is_err() {
                        error!("Receiver dropped for UUID: {}", uuid);
                        let _ = session.close(None).await;
                        break;
                    }

                    let nat = receiver.borrow_and_update();
                    let nat = match nat.as_ref() {
                        Some(e) => e,
                        None => continue,
                    };

                    let serialized_nat = match serde_json::to_string(nat) {
                        Ok(data) => data,
                        Err(err) => {
                            error!("Failed to serialize data for UUID {}: {}", uuid, err);
                            continue;
                        }
                    };

                    if let Err(e) = session.text(serialized_nat).await {
                        error!("Failed to send WebSocket message for UUID {}: {}", uuid, e);
                        break;
                    }
                }
            }
        }
    });

    // Return the WebSocket response immediately
    Ok(response)
}
