//! Driver to manage a connection which is passed to adapters.
use crate::Result;

pub trait Connect {
    type Connection;

    fn open_connection(&self) -> impl Future<Output = Result<Self::Connection>> + Send;
    fn close_connection(connection: Self::Connection) -> impl Future<Output = Result> + Send;
}

use sqlx::Connection;
pub use sqlx::MySqlConnection as SqlxConnection;
pub use sqlx::MySqlPool as SqlxPool;

#[derive(Clone)]
pub struct MySqlPool {
    pool: SqlxPool,
}
impl MySqlPool {
    pub const fn new(pool: SqlxPool) -> Self {
        Self { pool }
    }
}
impl Connect for MySqlPool {
    type Connection = SqlxPool;

    async fn open_connection(&self) -> Result<Self::Connection> {
        Ok(self.pool.clone())
    }
    async fn close_connection(_: Self::Connection) -> Result {
        Ok(())
    }
}

pub struct MySqlConnection {
    link: String,
}
impl MySqlConnection {
    pub const fn new(link: String) -> Self {
        Self { link }
    }
}
impl Connect for MySqlConnection {
    type Connection = SqlxConnection;

    async fn open_connection(&self) -> Result<Self::Connection> {
        SqlxConnection::connect(&self.link).await.map_err(Box::from)
    }
    async fn close_connection(connection: Self::Connection) -> Result {
        connection.close().await?;
        Ok(())
    }
}
