use super::Authenticated;
pub use data::Validation;
use data::{BoxDynError, user};

use derive_more::{Deref, Into};
use garde::Validate;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub trait AuthenticationContract: Send {
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

    #[error(transparent)]
    Other(data::BoxDynError),
}

pub type ReturnError<T = String> = (T, BoxDynError);

#[derive(Clone)]
pub enum Login {
    Name(Name),
    Email(Email),
}
impl AsRef<str> for Login {
    fn as_ref(&self) -> &str {
        match self {
            Self::Name(name) => name.as_ref(),
            Self::Email(email) => email.as_ref(),
        }
    }
}
impl TryFrom<String> for Login {
    type Error = ReturnError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = match Email::try_from(value) {
            Ok(x) => return Ok(Self::Email(x)),
            Err((v, _)) => v,
        };
        match Name::try_from(value) {
            Ok(x) => Ok(Self::Name(x)),
            Err((v, _)) => Err((v, "login is invalid".into())),
        }
    }
}
impl From<Login> for String {
    fn from(val: Login) -> Self {
        match val {
            Login::Name(name) => name.0.into(),
            Login::Email(email) => email.0.into(),
        }
    }
}

#[derive(Clone, Deref, Into)]
pub struct Name(user::Name);
impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
impl TryFrom<String> for Name {
    type Error = ReturnError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        #[derive(Validate)]
        #[garde(transparent)]
        struct Username<'a>(#[garde(ascii, length(chars, min = 2, max = 31))] &'a str);

        match Username(value.as_str()).validate() {
            Ok(()) => (),
            Err(e) => return Err((value, e.into())),
        }
        Ok(Self(user::Name::new(value)?))
    }
}

#[derive(Clone, Deref, Into)]
pub struct Email(user::Email);
impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
impl TryFrom<String> for Email {
    type Error = ReturnError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        #[derive(Validate)]
        #[garde(transparent)]
        pub struct Email<'a>(#[garde(email, length(chars, max = 255))] &'a str);

        match Email(value.as_str()).validate() {
            Ok(()) => (),
            Err(e) => return Err((value, e.into())),
        }
        Ok(Self(user::Email::new(value)?))
    }
}

#[derive(Clone, Deref, Into)]
pub struct Password(String);
impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
impl TryFrom<String> for Password {
    type Error = ReturnError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.chars().count() > 7 {
            Ok(Self(value))
        } else {
            Err((value, "password must be longer than 7 characters".into()))
        }
    }
}
