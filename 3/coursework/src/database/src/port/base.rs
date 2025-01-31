pub use super::{CRUD, Result};

pub use chrono::{DateTime, Utc};
use derive_more::{Deref, Into};

#[allow(async_fn_in_trait)]
pub trait BaseRepository<C>:
    CRUD<C, New = New, Unique = u64, Update = Field, Existing = Base>
{
}

// #[derive(Deref, Into, Clone, Copy)]
// pub struct Id(pub(crate) u64);

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
pub struct Description(Option<String>);
impl TryFrom<Option<String>> for Description {
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

pub enum Field {
    Name(Name),
    Description(Description),
    CreatedAt(DateTime<Utc>),
    UpdatedAt(DateTime<Utc>),
}

pub struct New {
    pub name: Name,
    pub description: Description,
}

pub struct Base {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl Base {
    pub const fn id(&self) -> u64 {
        self.id
    }
    pub const fn name(&self) -> &String {
        &self.name
    }
    pub const fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub const fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
