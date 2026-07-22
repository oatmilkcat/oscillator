use std::ops::Deref;

use db::Connection;

use super::models::ArtistModel;
use crate::DbResult;

#[derive(Debug)]
pub struct ArtistRepo<'a> {
    conn: &'a Connection,
}

impl<'a> ArtistRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub async fn get_by_id(&self, id: &str) -> DbResult<Option<ArtistModel>> {
        let maybe_artist = sqlx::query_as::<_, ArtistModel>(
            r#"
                SELECT artists.* 
                FROM artists
                WHERE artists.id = $1;
            "#,
        )
        .bind(id)
        .fetch_optional(self.conn.deref())
        .await?;

        Ok(maybe_artist)
    }
}
