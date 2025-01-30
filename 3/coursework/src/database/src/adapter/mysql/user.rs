pub use crate::port::user::*;

use sqlx::{Executor, MySql};

pub struct UserAdapter;

struct DatabaseUser {
    id: u64,
    name: String,
    email: String,
    password: String,
    last_used: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<DatabaseUser> for User {
    fn from(value: DatabaseUser) -> Self {
        Self {
            id: Id(value.id),
            name: value.name.into(),
            email: value.email.into(),
            password: value.password.into(),
            last_used: value.last_used,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl<E> UserRepository<E> for UserAdapter where for<'a> &'a E: Executor<'a, Database = MySql> {}
impl<E> crate::port::CRUD<E> for UserAdapter
where
    for<'a> &'a E: Executor<'a, Database = MySql>,
{
    type Create = New;
    type Read = Unique;
    type Update = Field;
    type Delete = Unique;
    type Existing = User;
    type Id = Id;

    async fn create(connection: &mut E, data: Self::Create) -> Result<Self::Id> {
        Ok(Id(sqlx::query!(
            "INSERT INTO Users (name, email, password, last_used) VALUES (?, ?, ?, ?)",
            data.name.0,
            data.email.0,
            data.password.0,
            data.last_used,
        )
        .execute(&*connection)
        .await?
        .last_insert_id()))
    }

    async fn read(connection: &E, data: Self::Read) -> Result<Option<Self::Existing>> {
        Ok(match data {
            Unique::Id(id) => {
                sqlx::query_as!(DatabaseUser, "SELECT * FROM Users WHERE id = ?", id.0)
                    .fetch_optional(connection)
                    .await
            }
            Unique::Name(name) => {
                sqlx::query_as!(DatabaseUser, "SELECT * FROM Users WHERE name = ?", name.0)
                    .fetch_optional(connection)
                    .await
            }
            Unique::Email(email) => {
                sqlx::query_as!(DatabaseUser, "SELECT * FROM Users WHERE email = ?", email.0)
                    .fetch_optional(connection)
                    .await
            }
        }?
        .map(Into::into))
    }

    async fn update(connection: &mut E, id: Self::Id, data: Self::Update) -> Result {
        match data {
            Field::Name(valid) => {
                sqlx::query!("UPDATE Users SET name = ? WHERE id = ?", valid.0, id.0)
            }
            Field::Email(valid) => {
                sqlx::query!("UPDATE Users SET email = ? WHERE id = ?", valid.0, id.0)
            }
            Field::Password(valid) => {
                sqlx::query!("UPDATE Users SET password = ? WHERE id = ?", valid.0, id.0)
            }
            Field::LastUsed(date_time) => {
                sqlx::query!(
                    "UPDATE Users SET last_used = ? WHERE id = ?",
                    date_time,
                    id.0
                )
            }
            Field::CreatedAt(date_time) => sqlx::query!(
                "UPDATE Users SET created_at = ? WHERE id = ?",
                date_time,
                id.0
            ),
            Field::UpdatedAt(date_time) => sqlx::query!(
                "UPDATE Users SET updated_at = ? WHERE id = ?",
                date_time,
                id.0
            ),
        }
        .execute(&*connection)
        .await?;
        Ok(())
    }

    async fn delete(connection: &mut E, data: Self::Delete) -> Result {
        match data {
            Unique::Id(id) => sqlx::query!("DELETE FROM Users WHERE id = ?", id.0),
            Unique::Name(name) => {
                sqlx::query!("DELETE FROM Users WHERE name = ?", name.0)
            }
            Unique::Email(email) => {
                sqlx::query!("DELETE FROM Users WHERE email = ?", email.0)
            }
        }
        .execute(&*connection)
        .await?;
        Ok(())
    }
}
