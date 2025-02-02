//! Low-level repository traits for unified data access.
//!
//! No data validation besides very basic one like length violation.
use crate::Result;

pub mod base;
pub mod package;
pub mod user;

pub trait CRUD<C> {
    type New;
    type Unique;
    type Update;
    type Existing;

    fn create(
        connection: &mut C,
        data: Self::New,
    ) -> impl Future<Output = crate::Result<Self::Existing>> + Send;
    fn read(
        connection: &C,
        data: Self::Unique,
    ) -> impl Future<Output = crate::Result<Option<Self::Existing>>> + Send;
    fn update(
        connection: &mut C,
        existing: &mut Self::Existing,
        data: Self::Update,
    ) -> impl Future<Output = crate::Result> + Send;
    fn delete(connection: &mut C, data: Self::Unique)
    -> impl Future<Output = crate::Result> + Send;
}

trait CharLength {
    fn length(&self) -> usize;
}
impl CharLength for String {
    fn length(&self) -> usize {
        self.chars().count()
    }
}
impl CharLength for Option<String> {
    fn length(&self) -> usize {
        self.as_ref().map_or(0, CharLength::length)
    }
}

trait MaxLength {
    type Inner: CharLength;
    const MAX_LENGTH: usize;

    fn validate(value: &Self::Inner) -> Result<(), &'static str> {
        if value.length() > Self::MAX_LENGTH {
            Err("too long")
        } else {
            Ok(())
        }
    }
}
