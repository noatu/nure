use garde::{Report, Unvalidated, Valid, Validate};

pub trait IntoValid: Validate {
    fn into_valid(self) -> Result<Valid<Self>, Report>
    where
        Self: Sized,
        <Self as Validate>::Context: Default,
    {
        Unvalidated::new(self).validate()
    }
}
impl<T: garde::Validate> IntoValid for T {}

pub mod adapter;
pub mod atomic;
pub mod connect;
pub mod port;
