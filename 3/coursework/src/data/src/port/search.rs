use super::Validatable;
use crate::Result;

use chrono::{DateTime, Utc};
use derive_more::{Deref, Into};

pub trait SearchRepository<C> {
    fn search(connection: &C, data: Data) -> impl Future<Output = Result<Vec<Entry>>> + Send;
}

#[derive(Clone, Deref, Into)]
pub struct Search(String);
impl Validatable for Search {
    type Inner = String;
    const MAX_LENGTH: usize = 255;
    fn encapsulate(value: Self::Inner) -> Self {
        Self(value)
    }
}

pub struct Data {
    pub mode: Mode,
    pub order: Order,
    pub search: Search,

    pub limit: u8,
    pub exact: bool,
    pub ascending: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Entry {
    pub id: u64,
    pub name: Box<str>,
    pub version: Box<str>,
    pub base_id: u64,
    pub base_name: Box<str>,
    pub url: Option<Box<str>>,
    pub description: Box<str>,
    pub submitter_id: u64,
    pub submitter_name: Box<str>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Url,
    Name,
    PackageBase,
    Description,
    BaseDescription,
    NameAndDescription,
    User,
    Flagger,
    Packager,
    Submitter,
    Maintainer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Name,
    Version,
    BaseName,
    // Submitter,
    UpdatedAt,
    CreatedAt,
}
