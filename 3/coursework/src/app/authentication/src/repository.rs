pub use database::port::user::*;

use derive_more::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
pub struct Authenticated(User);

#[allow(async_fn_in_trait)]
pub trait AuthenticationRepository {
    async fn get_user(&self, get: Get) -> Result<Option<User>>;
    async fn create_user(&self, new: New) -> Result<User>;
    async fn start_session(&self, user: User) -> Result<Authenticated>;
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

// Adapter

use database::connect::Connect;
use std::marker::PhantomData;

pub struct AuthenticationAdapter<D, C, UR>
where
    D: Connect<Connection = C>,
    UR: UserRepository<C>,
{
    driver: D,
    _user_repository: PhantomData<UR>,
}

impl<D, C, UR> AuthenticationAdapter<D, C, UR>
where
    D: Connect<Connection = C>,
    UR: UserRepository<C>,
{
    pub const fn new(driver: D) -> Self {
        Self {
            driver,
            _user_repository: PhantomData,
        }
    }
}

impl<D, C, UR> AuthenticationRepository for AuthenticationAdapter<D, C, UR>
where
    D: Connect<Connection = C>,
    UR: UserRepository<C>,
{
    async fn get_user(&self, get: Get) -> Result<Option<User>> {
        let c = self.driver.open_connection().await?;
        let user = UR::read(&c, get.into()).await?;
        D::close_connection(c).await?;

        Ok(user)
    }

    async fn create_user(&self, new: New) -> Result<User> {
        let mut c = self.driver.open_connection().await?;
        let user = UR::create(&mut c, new).await?;
        D::close_connection(c).await?;

        Ok(user)
    }

    async fn start_session(&self, mut user: User) -> Result<Authenticated> {
        let mut c = self.driver.open_connection().await?;
        UR::update(&mut c, &mut user, Field::LastUsed(Some(Utc::now()))).await?;
        D::close_connection(c).await?;

        Ok(Authenticated(user))
    }
}

