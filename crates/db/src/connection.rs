use std::ops::{Deref, DerefMut};

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

/// A wrapper type which dereferences into a [`Pool<Postgres>`]
#[derive(Debug)]
pub struct Connection(Pool<Postgres>);

impl Deref for Connection {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Maximum number of connections in the pool
const MAX_CONNECTIONS: u32 = 5;

/// Constructs a connection pool with the provided connection string and returns it
pub async fn get_connection(conn_str: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(conn_str)
        .await
        .unwrap()
}
