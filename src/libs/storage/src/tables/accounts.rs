use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Accounts {
    pub account_id: Uuid,
    pub username: String,
    pub role: String,
    pub logged_in_at: NaiveDateTime,
}
