use crate::repository::{Authenticated, AuthenticationRepository};
use database::port::user;

use derive_more::{Deref, Into};
use garde::Validate;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Login
    #[error("login not found")]
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
    InvalidPassword(Box<dyn std::error::Error>),
    #[error("data source error: {0}")]
    Repository(Box<dyn std::error::Error>),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Clone, Deref, Into)]
pub struct Name(user::Name);

impl TryFrom<String> for Name {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        #[derive(Validate)]
        #[garde(transparent)]
        struct Username<'a>(#[garde(alphanumeric, length(chars, min = 2, max = 31))] &'a str);

        Username(&value).validate()?;
        Ok(Self(user::Name::try_from(value)?))
    }
}

#[derive(Clone, Deref, Into)]
pub struct Email(user::Email);
impl TryFrom<String> for Email {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        #[derive(Validate)]
        #[garde(transparent)]
        pub struct Email<'a>(#[garde(email, length(chars, max = 255))] &'a str);

        Email(&value).validate()?;
        Ok(Self(user::Email::try_from(value)?))
    }
}

// #[derive(Validate, Deref, From, Into)]
// #[garde(transparent)]
// pub struct Password(#[garde(length(chars, min = 8))] pub String);

pub enum LoginBy {
    Name(Name),
    Email(Email),
}

pub struct LoginData {
    login: LoginBy,
    password: String,
}

pub struct RegisterData {
    pub name: Name,
    pub email: Email,
    pub password: String,
}

pub trait AuthenticationContract {
    async fn name_available(&self, name: Name) -> Result;
    async fn email_available(&self, email: Email) -> Result;

    async fn login(&mut self, data: LoginData) -> Result<Authenticated>;
    async fn register(&mut self, data: RegisterData) -> Result<Authenticated>;
}

// Service

use crate::repository::Get;

use argon2::{
    Argon2,
    password_hash::{
        self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};

impl From<password_hash::Error> for Error {
    fn from(error: password_hash::Error) -> Self {
        match error {
            password_hash::Error::Password => Self::IncorrectPassword,
            _ => Self::InvalidPassword(error.into()),
        }
    }
}

pub struct AuthenticationService<R>
where
    R: AuthenticationRepository,
{
    repository: R,
}

impl<R> AuthenticationService<R>
where
    R: AuthenticationRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> AuthenticationContract for AuthenticationService<R>
where
    R: AuthenticationRepository,
{
    async fn name_available(&self, name: Name) -> Result {
        if self
            .repository
            .get_user(Get::Name(name.0))
            .await
            .map_err(Error::Repository)?
            .is_some()
        {
            return Err(Error::NameExists);
        };
        Ok(())
    }
    async fn email_available(&self, email: Email) -> Result {
        if self
            .repository
            .get_user(Get::Email(email.0))
            .await
            .map_err(Error::Repository)?
            .is_some()
        {
            return Err(Error::EmailExists);
        };
        Ok(())
    }

    async fn login(&mut self, data: LoginData) -> Result<Authenticated> {
        if data.password.chars().count() < 8 {
            return Err(Error::InvalidPassword(
                "password must be longer than 8 characters".into(),
            ));
        }

        let user = match data.login {
            LoginBy::Name(name) => self.repository.get_user(Get::Name(name.0)),
            LoginBy::Email(email) => self.repository.get_user(Get::Email(email.0)),
        }
        .await
        .map_err(Error::Repository)?
        .ok_or(Error::LoginNotFound)?;

        Argon2::default().verify_password(
            data.password.as_bytes(),
            &PasswordHash::new(user.password())?,
        )?;

        self.repository
            .start_session(user)
            .await
            .map_err(Error::Repository)
    }

    async fn register(&mut self, data: RegisterData) -> Result<Authenticated> {
        if data.password.chars().count() < 8 {
            return Err(Error::InvalidPassword(
                "password must be longer than 8 characters".into(),
            ));
        }

        self.name_available(data.name.clone()).await?;
        self.email_available(data.email.clone()).await?;

        // Get PHC string ($argon2id$v=19$...)
        let password = Argon2::default()
            .hash_password(data.password.as_bytes(), &SaltString::generate(&mut OsRng))?
            .to_string()
            .try_into()
            .map_err(|e| Error::InvalidPassword(Box::from(e)))?;

        let user = self
            .repository
            .create_user(user::New {
                name: data.name.0,
                email: data.email.0,
                password,
                last_used: None,
            })
            .await
            .map_err(Error::Repository)?;

        self.repository
            .start_session(user)
            .await
            .map_err(Error::Repository)
    }
}

