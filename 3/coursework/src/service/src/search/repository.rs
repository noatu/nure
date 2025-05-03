use data::Result;
use data::search::{Data, Entry};

pub trait SearchRepository {
    fn search(&self, data: Data) -> impl Future<Output = Result<Vec<Entry>>> + Send;
}
