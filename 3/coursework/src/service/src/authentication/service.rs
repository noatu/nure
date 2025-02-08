use super::{
    Authenticated, AuthenticationContract, AuthenticationRepository, Email, Error, Get, Login,
    LoginData, Name, RegisterData, Result, Validation,
};

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
    pub(crate) repository: R,
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
    R: AuthenticationRepository + Send + Sync,
{
    async fn name_available(&self, name: Name) -> Result {
        if self
            .repository
            .get_user(Get::Name(name.into()))
            .await
            .map_err(Error::Repository)?
            .is_some()
        {
            return Err(Error::NameExists);
        }
        Ok(())
    }
    async fn email_available(&self, email: Email) -> Result {
        if self
            .repository
            .get_user(Get::Email(email.into()))
            .await
            .map_err(Error::Repository)?
            .is_some()
        {
            return Err(Error::EmailExists);
        }
        Ok(())
    }

    async fn login(&self, data: LoginData) -> Result<Authenticated> {
        let user = match data.login {
            Login::Name(name) => self.repository.get_user(Get::Name(name.into())),
            Login::Email(email) => self.repository.get_user(Get::Email(email.into())),
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
        self.name_available(data.name.clone()).await?;
        self.email_available(data.email.clone()).await?;

        // Get PHC string ($argon2id$v=19$...)
        let phc = Argon2::default()
            .hash_password(data.password.as_bytes(), &SaltString::generate(&mut OsRng))?
            .to_string();
        let password = data::user::Password::new(phc)
            .map_err(|(_, e)| Error::InvalidPassword(e))?;

        let user = self
            .repository
            .create_user(data::user::New {
                name: data.name.into(),
                email: data.email.into(),
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
