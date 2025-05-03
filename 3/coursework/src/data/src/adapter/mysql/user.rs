use crate::Result;
use crate::port::user::{Field, New, Unique, User, UserRepository};

use chrono::Utc;
use sqlx::{Executor, MySql};

pub struct UserAdapter;

impl<E> UserRepository<E> for UserAdapter
where
    E: Send,
    for<'a> &'a E: Executor<'a, Database = MySql>,
{
}
impl<E> crate::port::Crud<E> for UserAdapter
where
    E: Send,
    for<'a> &'a E: Executor<'a, Database = MySql>,
{
    type New = New;
    type Update = Field;
    type Unique = Unique;
    type Existing = User;

    async fn create(connection: &mut E, data: Self::New) -> Result<Self::Existing> {
        let created_at = Utc::now();
        let id = sqlx::query!(
            "INSERT INTO Users (name, email, password, last_used, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
            data.name.as_str(),
            data.email.as_str(),
            data.password.as_str(),
            data.last_used,
            created_at,
            created_at,
        )
        .execute(&*connection)
        .await?
        .last_insert_id();

        Ok(Self::Existing {
            id,
            name: data.name.into(),
            email: data.email.into(),
            password: data.password.into(),
            last_used: data.last_used,
            created_at,
            updated_at: created_at,
        })
    }

    async fn read(connection: &E, data: Self::Unique) -> Result<Option<Self::Existing>> {
        Ok(match data {
            Unique::Id(id) => {
                sqlx::query_as!(User, "SELECT * FROM Users WHERE id = ?", id)
                    .fetch_optional(connection)
                    .await
            }
            Unique::Name(name) => {
                sqlx::query_as!(User, "SELECT * FROM Users WHERE name = ?", name.as_str())
                    .fetch_optional(connection)
                    .await
            }
            Unique::Email(email) => {
                sqlx::query_as!(User, "SELECT * FROM Users WHERE email = ?", email.as_str())
                    .fetch_optional(connection)
                    .await
            }
        }?)
    }

    async fn update(
        connection: &mut E,
        existing: &mut Self::Existing,
        data: Self::Update,
    ) -> Result {
        match &data {
            Field::Name(name) => {
                sqlx::query!(
                    "UPDATE Users SET name = ? WHERE id = ?",
                    name.as_str(),
                    existing.id
                )
            }
            Field::Email(email) => {
                sqlx::query!(
                    "UPDATE Users SET email = ? WHERE id = ?",
                    email.as_str(),
                    existing.id
                )
            }
            Field::Password(password) => {
                sqlx::query!(
                    "UPDATE Users SET password = ? WHERE id = ?",
                    password.as_str(),
                    existing.id
                )
            }
            Field::LastUsed(date_time) => {
                sqlx::query!(
                    "UPDATE Users SET last_used = ? WHERE id = ?",
                    date_time,
                    existing.id
                )
            }
            Field::CreatedAt(date_time) => sqlx::query!(
                "UPDATE Users SET created_at = ? WHERE id = ?",
                date_time,
                existing.id
            ),
            Field::UpdatedAt(date_time) => sqlx::query!(
                "UPDATE Users SET updated_at = ? WHERE id = ?",
                date_time,
                existing.id
            ),
        }
        .execute(&*connection)
        .await?;

        match data {
            Field::Name(valid) => existing.name = valid.into(),
            Field::Email(valid) => existing.email = valid.into(),
            Field::Password(valid) => existing.password = valid.into(),
            Field::LastUsed(date_time) => existing.last_used = date_time,
            Field::CreatedAt(date_time) => existing.created_at = date_time,
            Field::UpdatedAt(date_time) => existing.updated_at = date_time,
        }

        Ok(())
    }

    async fn delete(connection: &mut E, data: Self::Unique) -> Result {
        match data {
            Unique::Id(id) => sqlx::query!("DELETE FROM Users WHERE id = ?", id),
            Unique::Name(name) => {
                sqlx::query!("DELETE FROM Users WHERE name = ?", name.as_str())
            }
            Unique::Email(email) => {
                sqlx::query!("DELETE FROM Users WHERE email = ?", email.as_str())
            }
        }
        .execute(&*connection)
        .await?;

        Ok(())
    }
}
