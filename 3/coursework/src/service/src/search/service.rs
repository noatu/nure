use super::{Data, Result, SearchContract, SearchRepository};
use data::search;

pub struct SearchService<R>
where
    R: SearchRepository,
{
    pub(crate) repository: R,
}

impl<R> SearchService<R>
where
    R: SearchRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> SearchContract for SearchService<R>
where
    R: SearchRepository + Send + Sync,
{
    async fn search(&self, data: Data) -> Result<Vec<search::Entry>> {
        self.repository.search(data.into()).await
    }
}
