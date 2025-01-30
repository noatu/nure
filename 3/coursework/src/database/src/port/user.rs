pub use super::{CRUD, Result};

pub use chrono::{DateTime, Utc};
use derive_more::{Deref, From, Into};
use garde::{Valid, Validate};

#[allow(async_fn_in_trait)]
pub trait UserRepository<C>:
    super::CRUD<
        C,
        Create = New,
        Read = Unique,
        Update = Field,
        Delete = Unique,
        Existing = User,
        Id = Id,
    >
{
    async fn update_user(connection: &mut C, user: &mut User, data: Self::Update) -> Result {
        Self::update(connection, user.id, data.clone()).await?;
        match data {
            Field::Name(valid) => user.name = valid.into_inner(),
            Field::Email(valid) => user.email = valid.into_inner(),
            Field::Password(valid) => user.password = valid.into_inner(),
            Field::LastUsed(date_time) => user.last_used = date_time,
            Field::CreatedAt(date_time) => user.created_at = date_time,
            Field::UpdatedAt(date_time) => user.updated_at = date_time,
        }
        Ok(())
    }
}

#[derive(Deref, Into, Clone, Copy)]
pub struct Id(pub(crate) u64);

// TODO: is this the right layer for requirements (email) validatoin?

#[derive(Validate, Deref, From, Clone)]
#[garde(transparent)]
pub struct Name(#[garde(alphanumeric, length(min = 2, max = 31))] pub String);

#[derive(Validate, Deref, From, Clone)]
#[garde(transparent)]
pub struct Email(#[garde(email, length(max = 255))] pub String);

#[derive(Validate, Deref, From, Clone)]
#[garde(transparent)]
pub struct Password(#[garde(ascii, length(max = 255))] pub String);

pub struct New {
    pub name: Valid<Name>,
    pub email: Valid<Email>,
    pub password: Valid<Password>,
    pub last_used: Option<DateTime<Utc>>,
}

pub enum Unique {
    Id(Id),
    Name(Valid<Name>),
    Email(Valid<Email>),
}

#[derive(Clone)]
pub enum Field {
    Name(Valid<Name>),
    Email(Valid<Email>),
    Password(Valid<Password>),
    LastUsed(Option<DateTime<Utc>>),
    CreatedAt(DateTime<Utc>),
    UpdatedAt(DateTime<Utc>),
}

pub struct User {
    pub(crate) id: Id,
    pub(crate) name: Name,
    pub(crate) email: Email,
    pub(crate) password: Password,
    pub(crate) last_used: Option<DateTime<Utc>>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl User {
    pub const fn id(&self) -> Id {
        self.id
    }
    pub const fn name(&self) -> &Name {
        &self.name
    }
    pub const fn email(&self) -> &Email {
        &self.email
    }
    pub const fn password(&self) -> &Password {
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
