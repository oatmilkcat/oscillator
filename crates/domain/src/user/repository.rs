use std::ops::Deref;

use db::Connection;

use super::models::UserModel;
use crate::DbResult;

#[derive(Debug)]
pub struct UserRepo<'a> {
    conn: &'a Connection,
}

impl<'a> UserRepo<'a> {
    pub async fn get_by_id(&self, id: &str) -> DbResult<Option<UserModel>> {
        let user = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(self.conn.deref())
            .await?;

        Ok(user)
    }

    pub async fn get_by_username(&self, username: &str) -> DbResult<Option<UserModel>> {
        let user = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(self.conn.deref())
            .await?;

        Ok(user)
    }

    pub async fn get_by_email(&self, email: &str) -> DbResult<Option<UserModel>> {
        let user = sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(self.conn.deref())
            .await?;

        Ok(user)
    }
}
