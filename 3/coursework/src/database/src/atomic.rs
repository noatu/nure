type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Atomic {
    type Transaction<'a>;

    fn start_transaction(&mut self) -> impl Future<Output = Result<Self::Transaction<'_>>> + Send;
    fn abort_transaction(transaction: Self::Transaction<'_>) -> impl Future<Output = Result> + Send;
    fn commit_transaction(transaction: Self::Transaction<'_>) -> impl Future<Output = Result> + Send;
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
