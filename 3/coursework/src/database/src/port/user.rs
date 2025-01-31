pub use super::{CRUD, Result};

pub use chrono::{DateTime, Utc};
use derive_more::{Deref, Into};

#[allow(async_fn_in_trait)]
pub trait UserRepository<C>:
    CRUD<C, New = New, Update = Field, Unique = Unique, Existing = User>
{
}

#[derive(Clone, Deref, Into)]
pub struct Name(String);
impl TryFrom<String> for Name {
    type Error = &'static str;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        if value.chars().count() > 31 {
            Err(super::TOO_LONG)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Deref, Into)]
pub struct Email(String);
impl TryFrom<String> for Email {
    type Error = &'static str;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        if value.chars().count() > 255 {
            Err(super::TOO_LONG)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Deref, Into)]
pub struct Password(String);
impl TryFrom<String> for Password {
    type Error = &'static str;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        if value.chars().count() > 255 {
            Err(super::TOO_LONG)
        } else {
            Ok(Self(value))
        }
    }
}

pub enum Unique {
    Id(u64),
    Name(Name),
    Email(Email),
}

pub enum Field {
    Name(Name),
    Email(Email),
    Password(Password),
    LastUsed(Option<DateTime<Utc>>),
    CreatedAt(DateTime<Utc>),
    UpdatedAt(DateTime<Utc>),
}

pub struct New {
    pub name: Name,
    pub email: Email,
    pub password: Password,
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
