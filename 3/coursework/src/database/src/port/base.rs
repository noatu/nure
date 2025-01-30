pub use super::{CRUD, Result};

pub use chrono::{DateTime, Utc};
use derive_more::{Deref, From, Into};
use garde::{Valid, Validate};

#[allow(async_fn_in_trait)]
pub trait BaseRepository<C>:
    super::CRUD<
        C,
        Create = New,
        Read = Id,
        Update = Field,
        Delete = Id,
        Existing = Base,
        Id = Id,
    >
{
    async fn update_base(connection: &mut C, base: &mut Base, data: Self::Update) -> Result {
        Self::update(connection, base.id, data.clone()).await?;
        match data {
            Field::Name(valid) => base.name = valid.into_inner(),
            Field::Description(valid) => base.description = valid.into_inner(),
            Field::CreatedAt(date_time) => base.created_at = date_time,
            Field::UpdatedAt(date_time) => base.updated_at = date_time,
        }
        Ok(())
    }
}

#[derive(Deref, Into, Clone, Copy)]
pub struct Id(pub(crate) u64);

#[derive(Validate, Deref, From, Clone)]
#[garde(transparent)]
pub struct Name(#[garde(alphanumeric, length(min = 2, max = 127))] pub String);

#[derive(Validate, Deref, From, Clone)]
#[garde(transparent)]
pub struct Description(#[garde(length(max = 510))] pub Option<String>);

pub struct New {
    pub name: Name,
    pub description: Description,
}

#[derive(Clone)]
pub enum Field {
    Name(Valid<Name>),
    Description(Valid<Description>),
    CreatedAt(DateTime<Utc>),
    UpdatedAt(DateTime<Utc>),
}

pub struct Base {
    pub(crate) id: Id,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl Base {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn description(&self) -> &Description {
        &self.description
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
