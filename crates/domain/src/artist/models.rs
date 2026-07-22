use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::user::models::UserId;

pub type ArtistId = String;

#[derive(Debug, FromRow)]
pub struct ArtistModel {
    pub id: ArtistId,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<UserId>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<UserId>,
    pub title: String,
}
