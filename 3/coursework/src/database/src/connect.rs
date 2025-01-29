type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(async_fn_in_trait)]
pub trait Connect {
    type Connection;

    async fn open_connection(&self) -> Result<Self::Connection>;
    async fn close_connection(connection: Self::Connection) -> Result;
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
