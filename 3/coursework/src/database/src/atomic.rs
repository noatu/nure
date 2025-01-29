type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(async_fn_in_trait)]
pub trait Atomic {
    type Transaction<'a>;

    async fn start_transaction(&mut self) -> Result<Self::Transaction<'_>>;
    async fn abort_transaction(transaction: Self::Transaction<'_>) -> Result;
    async fn commit_transaction(transaction: Self::Transaction<'_>) -> Result;
}

use sqlx::Connection;

impl Atomic for sqlx::MySqlPool {
    type Transaction<'a> = sqlx::MySqlTransaction<'a>;

    async fn start_transaction(&mut self) -> Result<Self::Transaction<'_>> {
        self.begin().await.map_err(Box::from)
    }
    async fn abort_transaction(transaction: Self::Transaction<'_>) -> Result {
        transaction.rollback().await.map_err(Box::from)
    }
    async fn commit_transaction(transaction: Self::Transaction<'_>) -> Result {
        transaction.commit().await.map_err(Box::from)
    }
}

impl Atomic for sqlx::MySqlConnection {
    type Transaction<'a> = sqlx::MySqlTransaction<'a>;

    async fn start_transaction(&mut self) -> Result<Self::Transaction<'_>> {
        self.begin().await.map_err(Box::from)
    }
    async fn abort_transaction(transaction: Self::Transaction<'_>) -> Result {
        transaction.rollback().await.map_err(Box::from)
    }
    async fn commit_transaction(transaction: Self::Transaction<'_>) -> Result {
        transaction.commit().await.map_err(Box::from)
    }
}
