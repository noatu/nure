pub use super::{CRUD, Result};

pub use chrono::{DateTime, Utc};
use derive_more::{Deref, From, Into};
use garde::{Valid, Validate};

#[allow(async_fn_in_trait)]
pub trait UserRepository<C>:
    CRUD<C, New = New, Update = Field, Unique = Unique, Existing = User>
{
}

#[derive(Validate, Deref, From, Into)]
#[garde(transparent)]
pub struct Name(#[garde(length(chars, max = 31))] pub String);

#[derive(Validate, Deref, From, Into)]
#[garde(transparent)]
pub struct Email(#[garde(length(chars, max = 255))] pub String);

#[derive(Validate, Deref, From, Into)]
#[garde(transparent)]
pub struct Password(#[garde(length(chars, max = 255))] pub String);

pub enum Unique {
    Id(u64),
    Name(Valid<Name>),
    Email(Valid<Email>),
}

pub enum Field {
    Name(Valid<Name>),
    Email(Valid<Email>),
    Password(Valid<Password>),
    LastUsed(Option<DateTime<Utc>>),
    CreatedAt(DateTime<Utc>),
    UpdatedAt(DateTime<Utc>),
}

pub struct New {
    pub name: Valid<Name>,
    pub email: Valid<Email>,
    pub password: Valid<Password>,
    pub last_used: Option<DateTime<Utc>>,
}

pub struct User {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) last_used: Option<DateTime<Utc>>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl User {
    pub const fn id(&self) -> u64 {
        self.id
    }
    pub const fn name(&self) -> &String {
        &self.name
    }
    pub const fn email(&self) -> &String {
        &self.email
    }
    pub const fn password(&self) -> &String {
        &self.password
    }
    pub const fn last_used(&self) -> Option<DateTime<Utc>> {
        self.last_used
    }
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub const fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
