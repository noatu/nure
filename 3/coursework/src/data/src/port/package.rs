use super::Validatable;
use crate::Base;

use chrono::{DateTime, Utc};
use derive_more::{Deref, Into};

pub trait PackageRepository<C>:
    super::Crud<C, New = New, Update = Field, Unique = Unique, Existing = Package>
{
}

#[derive(Clone, Deref, Into)]
pub struct Name(String);
impl Validatable for Name {
    type Inner = String;
    const MAX_LENGTH: usize = 127;
    fn encapsulate(value: Self::Inner) -> Self {
        Self(value)
    }
}

#[derive(Clone, Deref, Into)]
pub struct Version(String);
impl Validatable for Version {
    type Inner = String;
    const MAX_LENGTH: usize = 127;
    fn encapsulate(value: Self::Inner) -> Self {
        Self(value)
    }
}

#[derive(Clone, Deref, Into)]
pub struct Description(Option<String>);
impl Validatable for Description {
    type Inner = Option<String>;
    const MAX_LENGTH: usize = 255;
    fn encapsulate(value: Self::Inner) -> Self {
        Self(value)
    }
}

#[derive(Clone, Deref, Into)]
pub struct Url(Option<String>);
impl Validatable for Url {
    type Inner = Option<String>;
    const MAX_LENGTH: usize = 510;
    fn encapsulate(value: Self::Inner) -> Self {
        Self(value)
    }
}

pub enum Unique {
    Id(u64),
    Name(Name),
}

pub enum Field {
    PackageBase(Base),
    Name(Name),
    Version(Version),
    Description(Description),
    Url(Url),
    FlaggedAt(Option<DateTime<Utc>>),
    CreatedAt(DateTime<Utc>),
    UpdatedAt(DateTime<Utc>),
}

pub struct New {
    pub package_base: Base,
    pub name: Name,
    pub version: Version,
    pub description: Description,
    pub url: Url,
    pub flagged_at: Option<DateTime<Utc>>,
}

pub struct Package {
    pub(crate) id: u64,
    pub(crate) base: u64,
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
        self.base
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
