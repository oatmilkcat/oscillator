use chrono::{DateTime, Utc};
use sqlx::FromRow;

pub type UserId = String;

#[derive(Debug, FromRow)]
pub struct UserModel {
    id: UserId,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    display_name: Option<String>,
    username: Option<String>,
    email: Option<String>,
    password_hash: Option<String>,
    password_reset_token: Option<String>,
}
