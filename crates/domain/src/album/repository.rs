use std::ops::Deref;

use db::Connection;

use super::models::AlbumModel;
use crate::{DbResult, album::models::InsertAlbumModel};

#[derive(Debug)]
pub struct AlbumRepo<'a> {
    conn: &'a Connection,
}

impl<'a> AlbumRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub async fn get_by_id(&self, id: &str) -> DbResult<Option<AlbumModel>> {
        let maybe_album = sqlx::query_as::<_, AlbumModel>(
            r#"
                SELECT albums.* 
                FROM albums 
                    LEFT OUTER JOIN album_images ON album_images.album_id = albums.id
                    LEFT OUTER JOIN images ON images.id = album_images.image_id
                WHERE albums.id = $1;
            "#,
        )
        .bind(id)
        .fetch_optional(self.conn.deref())
        .await?;

        Ok(maybe_album)
    }

    pub async fn insert_one(
        &self,
        InsertAlbumModel {
            created_at,
            created_by,
            updated_at,
            updated_by,
            album_type,
            title,
            release_date,
        }: InsertAlbumModel,
    ) -> DbResult<AlbumModel> {
        let id = cuid2::create_id();

        let album = sqlx::query_as::<_, AlbumModel>(
            r#"
                INSERT INTO albums
                    (id, created_at, created_by, updated_at, updated_by, album_type, title, release_date)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING *
            "#,
        )
        .bind(&id)
        .bind(&created_at)
        .bind(&created_by)
        .bind(&updated_at)
        .bind(&updated_by)
        .bind(&album_type)
        .bind(&title)
        .bind(&release_date)
        .fetch_one(self.conn.deref())
        .await?;

        Ok(album)
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use tokio;

    use super::*;
    use db::connection::get_connection;

    #[tokio::test]
    async fn test_get_by_id() {
        let conn = get_connection(&std::env::var("POSTGRES_URL").unwrap_or_default()).await;
        let repo = AlbumRepo::new(&conn);

        let new_album = repo
            .insert_one(InsertAlbumModel {
                created_at: DateTime::default(),
                created_by: None,
                updated_at: None,
                updated_by: None,
                album_type: crate::album::models::AlbumType::Album,
                title: "Test".into(),
                release_date: DateTime::default(),
            })
            .await
            .unwrap();
        println!("{:?}", new_album);

        println!("{:?}", repo.get_by_id(&new_album.id).await.unwrap());
    }
}
