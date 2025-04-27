use super::Validatable;

use chrono::{DateTime, Utc};
use derive_more::{Deref, Into};

pub trait BaseRepository<C>:
    super::Crud<C, New = New, Unique = u64, Update = Field, Existing = Base>
{
}

// #[derive(Deref, Into, Clone, Copy)]
// pub struct Id(pub(crate) u64);

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
pub struct Description(Option<String>);
impl Validatable for Description {
    type Inner = Option<String>;
    const MAX_LENGTH: usize = 510;
    fn encapsulate(value: Self::Inner) -> Self {
        Self(value)
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
