use data::Result;
use data::user::{Email, Name, New, Unique, User};

use derive_more::{Deref, DerefMut};

#[derive(Deref, DerefMut, Debug)]
pub struct Authenticated(pub(super) User);

pub trait AuthenticationRepository {
    fn get_user(&self, get: Get) -> impl Future<Output = Result<Option<User>>> + Send;
    fn create_user(&self, new: New) -> impl Future<Output = Result<User>> + Send;
    fn start_session(&self, user: User) -> impl Future<Output = Result<Authenticated>> + Send;
}

pub enum Get {
    Name(Name),
    Email(Email),
}
impl From<Get> for Unique {
    fn from(value: Get) -> Self {
        match value {
            Get::Name(s) => Self::Name(s),
            Get::Email(s) => Self::Email(s),
        }
    }
}
