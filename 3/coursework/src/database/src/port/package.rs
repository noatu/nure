pub use super::{CRUD, Result, base::Base};

pub use chrono::{DateTime, Utc};
use derive_more::{Deref, From, Into};
use garde::{Valid, Validate};

#[allow(async_fn_in_trait)]
pub trait PackageRepository<C>:
    CRUD<C, New = New, Update = Field, Unique = Unique, Existing = Package>
{
}

#[derive(Validate, Deref, From, Into)]
#[garde(transparent)]
pub struct Name(#[garde(length(chars, max = 127))] pub String);

#[derive(Validate, Deref, From, Into)]
#[garde(transparent)]
pub struct Version(#[garde(length(chars, max = 127))] pub String);

#[derive(Validate, Deref, From, Into)]
#[garde(transparent)]
pub struct Description(#[garde(length(chars, max = 255))] pub Option<String>);

#[derive(Validate, Deref, From, Into)]
#[garde(transparent)]
pub struct URL(#[garde(length(chars, max = 510))] pub Option<String>);

pub enum Unique {
    Id(u64),
    Name(Valid<Name>),
}

pub enum Field {
    PackageBase(Base),
    Name(Valid<Name>),
    Version(Valid<Version>),
    Description(Valid<Description>),
    URL(Valid<URL>),
    FlaggedAt(Option<DateTime<Utc>>),
    CreatedAt(DateTime<Utc>),
    UpdatedAt(DateTime<Utc>),
}

pub struct New {
    pub package_base: Base,
    pub name: Valid<Name>,
    pub version: Valid<Version>,
    pub description: Valid<Description>,
    pub url: Valid<URL>,
    pub flagged_at: Option<DateTime<Utc>>,
}

pub struct Package {
    pub(crate) id: u64,
    pub(crate) package_base: u64,
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) description: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) flagged_at: Option<DateTime<Utc>>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl Package {
    pub const fn id(&self) -> u64 {
        self.id
    }
    pub const fn package_base(&self) -> u64 {
        self.package_base
    }
    pub const fn name(&self) -> &String {
        &self.name
    }
    pub const fn version(&self) -> &String {
        &self.version
    }
    pub const fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub const fn url(&self) -> Option<&String> {
        self.url.as_ref()
    }
    pub const fn flagged_at(&self) -> Option<DateTime<Utc>> {
        self.flagged_at
    }
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub const fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
