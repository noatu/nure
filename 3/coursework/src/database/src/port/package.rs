pub use super::{CRUD, Result, base::Base};

pub use chrono::{DateTime, Utc};
use derive_more::{Deref, Into};

#[allow(async_fn_in_trait)]
pub trait PackageRepository<C>:
    CRUD<C, New = New, Update = Field, Unique = Unique, Existing = Package>
{
}

#[derive(Clone, Deref, Into)]
pub struct Name(String);
impl TryFrom<String> for Name {
    type Error = &'static str;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        if value.chars().count() > 127 {
            Err(super::TOO_LONG)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Deref, Into)]
pub struct Version(String);
impl TryFrom<String> for Version {
    type Error = &'static str;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        if value.chars().count() > 127 {
            Err(super::TOO_LONG)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Deref, Into)]
pub struct Description(Option<String>);
impl TryFrom<Option<String>> for Description {
    type Error = &'static str;

    fn try_from(value: Option<String>) -> std::result::Result<Self, Self::Error> {
        if let Some(x) = &value {
            if x.chars().count() > 255 {
                return Err(super::TOO_LONG);
            }
        }
        Ok(Self(value))
    }
}

#[derive(Clone, Deref, Into)]
pub struct URL(Option<String>);
impl TryFrom<Option<String>> for URL {
    type Error = &'static str;

    fn try_from(value: Option<String>) -> std::result::Result<Self, Self::Error> {
        if let Some(x) = &value {
            if x.chars().count() > 510 {
                return Err(super::TOO_LONG);
            }
        }
        Ok(Self(value))
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
    URL(URL),
    FlaggedAt(Option<DateTime<Utc>>),
    CreatedAt(DateTime<Utc>),
    UpdatedAt(DateTime<Utc>),
}

pub struct New {
    pub package_base: Base,
    pub name: Name,
    pub version: Version,
    pub description: Description,
    pub url: URL,
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
