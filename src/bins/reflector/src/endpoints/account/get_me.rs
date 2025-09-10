use actix_web::{HttpMessage, HttpRequest, HttpResponse, get, web};
use auth::{account::get_account_by_id::get_account_by_id, jwt::jwt_claims::JWTClaims};
use error::Error;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Debug, ToSchema)]
#[schema(as = Get::Auth::Account::Me::Res)]
struct Res {
    status: &'static str,
    data: AccountData,
}
#[derive(Serialize, Debug, ToSchema)]
#[schema(as = Get::Auth::Account::Me::Res::AccountData)]
struct AccountData {
    username: String,
    id: Uuid,
}

#[utoipa::path(
    get,
    path = "/account/me",
    responses(
        (status = 200, description = "account details successful", body = Res, example = json!({
            "status": "success",
            "data": {
                "username": "XxCoolGamerXDxX",
                "id": "3b31ffd1-a47b-4b6e-930e-d6b906ee55f3"
            }
        })),
        (status = 401, description = "Unauthorized", body = Res, example = json!({
            "status": "Unauthorized access",
            "data": ""
        })),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Account"
)]
#[get("/me")]
pub async fn get_me(
    req: HttpRequest,
    db_pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let token_data = match extensions.get::<JWTClaims>() {
        None => return Err(Error::Unauthorized("Unauthorized access".to_string())),
        Some(e) => e,
    };

    let account = get_account_by_id(token_data.sub, &db_pool).await?;

    Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: AccountData {
            username: account.username,
            id: account.account_id,
        },
    }))
}
