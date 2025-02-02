use super::Authenticated;
use data::user;

use derive_more::{Deref, Into};
use garde::Validate;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub trait AuthenticationContract {
    fn name_available(&self, name: Name) -> impl Future<Output = Result> + Send;
    fn email_available(&self, email: Email) -> impl Future<Output = Result> + Send;

    fn login(&self, data: LoginData) -> impl Future<Output = Result<Authenticated>> + Send;
    fn register(
        &mut self,
        data: RegisterData,
    ) -> impl Future<Output = Result<Authenticated>> + Send;
}

pub struct LoginData {
    pub login: Login,
    pub password: Password,
}

pub struct RegisterData {
    pub name: Name,
    pub email: Email,
    pub password: Password,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Login
    #[error("login was not found")]
    LoginNotFound,
    #[error("incorrect password")]
    IncorrectPassword,
    // Register
    #[error("username is taken")]
    NameExists,
    #[error("email is already in use")]
    EmailExists,
    // Shared
    #[error("invalid password: {0}")]
    InvalidPassword(data::BoxDynError),
    #[error("data source error: {0}")]
    Repository(data::BoxDynError),
}

#[derive(Clone)]
pub enum Login {
    Name(Name),
    Email(Email),
}
impl TryFrom<String> for Login {
    type Error = (String, &'static str);

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = match Email::try_from(value) {
            Ok(x) => return Ok(Self::Email(x)),
            Err((s, _)) => s,
        };
        match Name::try_from(value) {
            Ok(x) => Ok(Self::Name(x)),
            Err((s, _)) => Err((s, "login is invalid")),
        }
    }
}

#[derive(Clone, Deref, Into)]
pub struct Name(user::Name);
impl TryFrom<String> for Name {
    type Error = (String, Box<dyn std::error::Error>);

    fn try_from(value: String) -> Result<Self, Self::Error> {
        #[derive(Validate)]
        #[garde(transparent)]
        struct Username<'a>(#[garde(alphanumeric, length(chars, min = 2, max = 31))] &'a str);

        if let Err(e) = Username(&value).validate() {
            return Err((value, e.into()));
        }
        match user::Name::try_from(value) {
            Ok(x) => Ok(Self(x)),
            Err((s, e)) => Err((s, e.into())),
        }
    }
}

#[derive(Clone, Deref, Into)]
pub struct Email(user::Email);
impl TryFrom<String> for Email {
    type Error = (String, Box<dyn std::error::Error>);

    fn try_from(value: String) -> Result<Self, Self::Error> {
        #[derive(Validate)]
        #[garde(transparent)]
        pub struct Email<'a>(#[garde(email, length(chars, max = 255))] &'a str);

        if let Err(e) = Email(&value).validate() {
            return Err((value, e.into()));
        }
        match user::Email::try_from(value) {
            Ok(x) => Ok(Self(x)),
            Err((s, e)) => Err((s, e.into())),
        }
    }
}

#[derive(Clone, Deref, Into)]
pub struct Password(String);
impl TryFrom<String> for Password {
    type Error = (String, &'static str);

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() >= 8 {
            Ok(Self(value))
        } else {
            Err((value, "password must be 8 characters or more"))
        }
    }
}
