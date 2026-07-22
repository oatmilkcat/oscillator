use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::user::models::UserId;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "album_type")]
#[sqlx(rename_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    ExtendedPlay,
    Compilation,
    Other,
}

pub type AlbumId = String;

#[derive(Debug, FromRow)]
pub struct AlbumModel {
    pub id: AlbumId,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<UserId>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<UserId>,
    pub album_type: AlbumType,
    pub title: String,
    pub release_date: DateTime<Utc>,
    #[sqlx(default)]
    pub images: Vec<AlbumImageModel>,
}

#[derive(Debug)]
pub struct InsertAlbumModel {
    pub created_at: DateTime<Utc>,
    pub created_by: Option<UserId>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<UserId>,
    pub album_type: AlbumType,
    pub title: String,
    pub release_date: DateTime<Utc>,
}

pub type AlbumImageId = String;

#[derive(Debug, FromRow, sqlx::Type)]
pub struct AlbumImageModel {
    pub id: AlbumImageId,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<UserId>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<UserId>,
    pub width: i32,
    pub height: i32,
    pub url: Vec<String>,
}
