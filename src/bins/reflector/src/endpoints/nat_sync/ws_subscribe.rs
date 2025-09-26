use crate::nat_subscriber::NatSubsciber;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use actix_ws::Message;
use auth::jwt::jwt_claims::JWTClaims;
use error::Error;
use log::{error, info};
use tokio::sync::RwLock;

#[utoipa::path(
    get,
    path = "/nat_sync/subscribe",
    responses(
        // (status = 200, description = "account details successful", example = json!({
        //     "status": "success",
        //     "data": {
        //         "username": "XxCoolGamerXDxX",
        //         "id": "3b31ffd1-a47b-4b6e-930e-d6b906ee55f3"
        //     }
        // })),
        // (status = 401, description = "Unauthorized", body = Res, example = json!({
        //     "status": "Unauthorized access",
        //     "data": ""
        // })),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Account"
)]
#[get("/subscribe")]
pub async fn ws_subscribe(
    req: HttpRequest,
    stream: web::Payload,
    nat_subscriber: web::Data<RwLock<NatSubsciber>>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)
        .map_err(|e| Error::BadRequest(format!("unable to create ws: {}", e)))?;

    let extensions = req.extensions();
    let token_data = match extensions.get::<JWTClaims>() {
        None => return Err(Error::Unauthorized("Unauthorized access".to_string())),
        Some(e) => e,
    };

    let mut receiver = {
        if let Some(sender) = nat_subscriber.read().await.subscribe(&token_data.sub) {
            sender.clone()
        } else {
            let mut subscriber = nat_subscriber.write().await;
            subscriber.init_nat(token_data.sub).receiver
        }
    };

    actix_web::rt::spawn(async move {
        loop {
            tokio::select! {
                Some(Ok(msg)) = msg_stream.recv() => {
                    match msg {
                        Message::Close(_) => {
                            info!("subscription ws closed");
                            let _ = session.close(None).await;
                            break;
                        },
                        _ => {}
                    }
                }

                changed = receiver.changed() => {
                    if changed.is_err() {
                        let _ = session.close(None).await;
                        error!("sender got dropped smh");
                        break;
                    }

                    let nat = receiver.borrow_and_update();
                    let nat = nat.as_ref();
                    let nat = match nat {
                        Some(e) => e,
                        None => continue
                    };


                    let serialized_nat = serde_json::to_string(nat);
                    let serialized_nat = match serialized_nat {
                        Ok(e) => e,
                        Err(err) => {
                            error!("failed to serialize nat smh, {}", err);
                            continue;
                        }
                    };

                    if let Err(e) = session.text(serialized_nat).await {
                        eprintln!("failed to send ws message: {e}");
                        break;
                    }
                }
            }
        }
    });

    Ok(response)
}
