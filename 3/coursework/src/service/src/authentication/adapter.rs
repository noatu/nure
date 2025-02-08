use super::{Authenticated, AuthenticationRepository, Get};
use data::user::{Field, New, User, UserRepository};
use data::{Connect, Result};

use std::marker::PhantomData;

pub struct AuthenticationAdapter<D, C, UR>
where
    C: Send,
    D: Connect<Connection = C> + Sync,
    UR: UserRepository<C> + Sync,
{
    driver: D,
    // connection: Option<C>,
    _user_repository: PhantomData<UR>,
}

impl<D, C, UR> AuthenticationAdapter<D, C, UR>
where
    C: Send,
    D: Connect<Connection = C> + Sync,
    UR: UserRepository<C> + Sync,
{
    pub const fn new(driver: D) -> Self {
        Self {
            driver,
            // connection: None,
            _user_repository: PhantomData,
        }
    }

    // async fn connection(&mut self) -> Result<&C> {
    //     if let Some(ref c) = self.connection {
    //         Ok(c)
    //     } else {
    //         self.connection = Some(self.driver.open_connection().await?);
    //         self.connection().await
    //     }
    // }
}

impl<D, C, UR> AuthenticationRepository for AuthenticationAdapter<D, C, UR>
where
    C: Send, //+ Sync,
    D: Connect<Connection = C> + Sync,
    UR: UserRepository<C> + Sync,
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
        UR::update(&mut c, &mut user, Field::LastUsed(Some(data::Utc::now()))).await?;
        D::close_connection(c).await?;

        Ok(Authenticated(user))
    }
}
