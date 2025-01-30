pub use super::{CRUD, Result};

pub use chrono::{DateTime, Utc};
use derive_more::{Deref, From, Into};
use garde::{Valid, Validate};

#[allow(async_fn_in_trait)]
pub trait BaseRepository<C>:
    CRUD<C, New = New, Unique = u64, Update = Field, Existing = Base>
{
}

// #[derive(Deref, Into, Clone, Copy)]
// pub struct Id(pub(crate) u64);

#[derive(Validate, Deref, From, Into)]
#[garde(transparent)]
pub struct Name(#[garde(length(chars, max = 127))] pub String);

#[derive(Validate, Deref, From, Into)]
#[garde(transparent)]
pub struct Description(#[garde(length(chars, max = 510))] pub Option<String>);

pub enum Field {
    Name(Valid<Name>),
    Description(Valid<Description>),
    CreatedAt(DateTime<Utc>),
    UpdatedAt(DateTime<Utc>),
}

pub struct New {
    pub name: Valid<Name>,
    pub description: Valid<Description>,
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
