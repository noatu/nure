//! Low-level repository traits for unified data access.
//!
//! Very mild argument validation.
use crate::{BoxDynError, Result};

pub mod base;
pub mod package;
pub mod search;
pub mod user;

pub trait Crud<C> {
    type New;
    type Unique;
    type Update;
    type Existing;

    fn create(
        connection: &mut C,
        data: Self::New,
    ) -> impl Future<Output = Result<Self::Existing>> + Send;
    fn read(
        connection: &C,
        data: Self::Unique,
    ) -> impl Future<Output = Result<Option<Self::Existing>>> + Send;
    fn update(
        connection: &mut C,
        existing: &mut Self::Existing,
        data: Self::Update,
    ) -> impl Future<Output = Result> + Send;
    fn delete(connection: &mut C, data: Self::Unique) -> impl Future<Output = Result> + Send;
}

pub trait CharLength {
    fn length(&self) -> usize;
}
impl CharLength for &str {
    fn length(&self) -> usize {
        self.chars().count()
    }
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

trait Validatable {
    type Inner: CharLength;
    const MAX_LENGTH: usize;
    fn encapsulate(value: Self::Inner) -> Self;
}

#[allow(private_bounds)] // don't expose the impl details
pub trait Validation<T>: Validatable
where
    T: CharLength + Into<Self::Inner>,
{
    fn valid(value: &T) -> Result<(), String> {
        if value.length() > Self::MAX_LENGTH {
            Err(format!(
                "too long (length: {}, max length: {})",
                value.length(),
                Self::MAX_LENGTH
            ))
        } else {
            Ok(())
        }
    }

    fn new(value: T) -> Result<Self, (T, BoxDynError)>
    where
        Self: Sized,
    {
        match Self::valid(&value) {
            Ok(()) => Ok(Self::encapsulate(value.into())),
            Err(e) => Err((value, e.into())),
        }
    }
}

impl<T, U> Validation<U> for T
where
    T: Validatable,
    U: CharLength + Into<T::Inner>,
{
}
