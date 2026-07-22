use thiserror::Error;

pub mod album;
pub mod artist;
pub mod auth;
pub mod track;
pub mod user;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("sql error")]
    SqlError(#[from] sqlx::Error),
}

pub type DbResult<T> = Result<T, DbError>;
