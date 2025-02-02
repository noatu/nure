mod widget;

mod input;
pub use input::Validation;

pub mod authentication;
pub use authentication::{Authentication, login, register};
