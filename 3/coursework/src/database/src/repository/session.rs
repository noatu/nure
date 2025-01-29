use super::user::User;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use chrono::{DateTime, Utc};
use derive_more::{Deref, DerefMut};
use sqlx::{Executor, MySql};

#[allow(async_fn_in_trait)]
pub trait SessionRepository<C> {
    async fn start(connection: &mut C, user: User) -> Result<Session>;
    async fn end(connection: &mut C, session: Session) -> Result<User>;
}

#[derive(DerefMut, Deref)]
pub struct Session {
    #[deref]
    #[deref_mut]
    user: User,

    start: DateTime<Utc>,
}

impl Session {
    pub const fn start(&self) -> DateTime<Utc> {
        self.start
    }
}

#[derive(Debug)]
pub struct SessionAdapter;

impl<E> SessionRepository<E> for SessionAdapter
where
    for<'a> &'a E: Executor<'a, Database = MySql>,
{
    async fn start(connection: &mut E, user: User) -> Result<Session> {
        let start = Utc::now();
        sqlx::query!(
            "UPDATE Users SET last_used = ? WHERE id = ?",
            start,
            *user.id()
        )
        .execute(&*connection)
        .await?;
        Ok(Session { user, start })
    }

    async fn end(_: &mut E, session: Session) -> Result<User> {
        Ok(session.user)
    }
}
