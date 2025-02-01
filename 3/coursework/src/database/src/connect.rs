type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Connect {
    type Connection;

    fn open_connection(&self) -> impl Future<Output = Result<Self::Connection>> + Send;
    fn close_connection(connection: Self::Connection) -> impl Future<Output = Result> + Send;
}

use sqlx::Connection;

#[derive(Clone)]
pub struct MySqlPool {
    pool: sqlx::MySqlPool,
}
impl Connect for MySqlPool {
    type Connection = sqlx::MySqlPool;

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

impl Connect for MySqlConnection {
    type Connection = sqlx::MySqlConnection;

    async fn open_connection(&self) -> Result<Self::Connection> {
        sqlx::MySqlConnection::connect(&self.link)
            .await
            .map_err(Box::from)
    }
    async fn close_connection(connection: Self::Connection) -> Result {
        connection.close().await?;
        Ok(())
    }
}
