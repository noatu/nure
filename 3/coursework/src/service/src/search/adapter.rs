use data::search::*;
use data::{Connect, Result};

use std::marker::PhantomData;

pub struct SearchAdapter<D, C, UR>
where
    C: Send,
    D: Connect<Connection = C> + Sync,
    UR: SearchRepository<C> + Sync,
{
    driver: D,
    _search_repository: PhantomData<UR>,
}

impl<D, C, UR> SearchAdapter<D, C, UR>
where
    C: Send,
    D: Connect<Connection = C> + Sync,
    UR: SearchRepository<C> + Sync,
{
    pub const fn new(driver: D) -> Self {
        Self {
            driver,
            _search_repository: PhantomData,
        }
    }
}

impl<D, C, SR> super::SearchRepository for SearchAdapter<D, C, SR>
where
    C: Send, //+ Sync,
    D: Connect<Connection = C> + Sync,
    SR: SearchRepository<C> + Sync,
{
    async fn search(&self, data: Data) -> Result<Vec<Entry>> {
        let c = self.driver.open_connection().await?;
        let result = SR::search(&c, data).await?;
        D::close_connection(c).await?;

        Ok(result)
    }
}
