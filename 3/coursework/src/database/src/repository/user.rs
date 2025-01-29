use derive_more::{Deref, From, Into};
use garde::{Valid, Validate};
use sqlx::{Executor, MySql};

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(async_fn_in_trait)]
pub trait UserRepository<C> {
    async fn get_by_id(connection: &C, id: Id) -> Result<Option<User>>;
    async fn get_by_name(connection: &C, name: &Valid<Username>) -> Result<Option<User>>;
    async fn get_by_email(connection: &C, email: &Valid<Email>) -> Result<Option<User>>;

    async fn change_name(connection: &mut C, user: &mut User, name: Valid<Username>) -> Result;
    async fn change_email(connection: &mut C, user: &mut User, email: Valid<Email>) -> Result;
    async fn change_password(
        connection: &mut C,
        user: &mut User,
        password: Valid<Password>,
    ) -> Result;

    async fn create(connection: &mut C, user: Valid<UserData>) -> Result<User>;
}

#[derive(Deref, Into, Clone, Copy)]
pub struct Id(u32);

#[derive(Validate, Deref)]
#[garde(transparent)]
pub struct Username(#[garde(alphanumeric, length(min = 2, max = 31))] pub String);

#[derive(Validate, Deref)]
#[garde(transparent)]
pub struct Email(#[garde(email, length(max = 255))] pub String);

#[derive(Validate, Deref)]
#[garde(transparent)]
pub struct Password(#[garde(ascii, length(max = 255))] pub String);

#[derive(Validate)]
pub struct UserData {
    #[garde(dive)]
    pub name: Username,
    #[garde(dive)]
    pub email: Email,
    #[garde(dive)]
    pub password: Password,
}

#[derive(Deref)]
pub struct User {
    id: Id,
    #[deref]
    data: UserData,
}
impl User {
    pub const fn id(&self) -> Id {
        self.id
    }
}

pub struct UserAdapter;

struct QueryUser {
    id: u32,
    name: String,
    email: String,
    password: String,
}
impl From<QueryUser> for User {
    fn from(value: QueryUser) -> Self {
        Self {
            id: Id(value.id),
            data: UserData {
                name: Username(value.name),
                email: Email(value.email),
                password: Password(value.password),
            },
        }
    }
}

impl<E> UserRepository<E> for UserAdapter
where
    for<'a> &'a E: Executor<'a, Database = MySql>,
{
    async fn get_by_id(connection: &E, id: Id) -> Result<Option<User>> {
        Ok(sqlx::query_as!(
            QueryUser,
            "SELECT id, name, email, password FROM Users WHERE id = ?",
            id.0
        )
        .fetch_optional(connection)
        .await?
        .map(Into::into))
    }
    async fn get_by_name(connection: &E, name: &Valid<Username>) -> Result<Option<User>> {
        Ok(sqlx::query_as!(
            QueryUser,
            "SELECT id, name, email, password FROM Users WHERE name = ?",
            name.0
        )
        .fetch_optional(connection)
        .await?
        .map(Into::into))
    }
    async fn get_by_email(connection: &E, email: &Valid<Email>) -> Result<Option<User>> {
        Ok(sqlx::query_as!(
            QueryUser,
            "SELECT id, name, email, password FROM Users WHERE email = ?",
            email.0
        )
        .fetch_optional(connection)
        .await?
        .map(Into::into))
    }

    async fn change_name(connection: &mut E, user: &mut User, name: Valid<Username>) -> Result {
        sqlx::query!("UPDATE Users SET name = ? WHERE id = ?", name.0, user.id.0)
            .execute(&*connection)
            .await?;
        Ok(())
    }
    async fn change_email(connection: &mut E, user: &mut User, email: Valid<Email>) -> Result {
        sqlx::query!(
            "UPDATE Users SET email = ? WHERE id = ?",
            email.0,
            user.id.0
        )
        .execute(&*connection)
        .await?;
        Ok(())
    }
    async fn change_password(
        connection: &mut E,
        user: &mut User,
        password: Valid<Password>,
    ) -> Result {
        sqlx::query!(
            "UPDATE Users SET password = ? WHERE id = ?",
            password.0,
            user.id.0
        )
        .execute(&*connection)
        .await?;
        Ok(())
    }

    async fn create(connection: &mut E, user: Valid<UserData>) -> Result<User> {
        let id = sqlx::query!(
            "INSERT INTO Users (name, email, password) VALUES (?, ?, ?)",
            user.name.0,
            user.email.0,
            user.password.0
        )
        .execute(&*connection)
        .await?
        .last_insert_id() as u32;

        Ok(User {
            id: Id(id),
            data: user.into_inner(),
        })
    }
}
