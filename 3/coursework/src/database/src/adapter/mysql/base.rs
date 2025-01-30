pub use crate::port::base::*;

use sqlx::{Executor, MySql};

pub struct BaseAdapter;

impl<E> BaseRepository<E> for BaseAdapter where for<'a> &'a E: Executor<'a, Database = MySql> {}
impl<E> crate::port::CRUD<E> for BaseAdapter
where
    for<'a> &'a E: Executor<'a, Database = MySql>,
{
    type New = New;
    type Unique = u64;
    type Update = Field;
    type Existing = Base;

    async fn create(connection: &mut E, data: Self::New) -> Result<Self::Existing> {
        let created_at = Utc::now();
        let id = sqlx::query!(
            "INSERT INTO PackageBases (name, description, created_at, updated_at) VALUES (?, ?, ?, ?)",
            data.name.0,
            data.description.0,
            created_at, created_at,
        )
        .execute(&*connection)
        .await?
        .last_insert_id();

        Ok(Self::Existing {
            id,
            name: data.name.into_inner().0,
            description: data.description.into_inner().0,
            created_at,
            updated_at: created_at,
        })
    }

    async fn read(connection: &E, data: Self::Unique) -> Result<Option<Self::Existing>> {
        Ok(
            sqlx::query_as!(Base, "SELECT * FROM PackageBases WHERE id = ?", data)
                .fetch_optional(connection)
                .await?,
        )
    }

    async fn update(
        connection: &mut E,
        existing: &mut Self::Existing,
        data: Self::Update,
    ) -> Result {
        match &data {
            Field::Name(name) => {
                sqlx::query!(
                    "UPDATE PackageBases SET name = ? WHERE id = ?",
                    name.0,
                    existing.id
                )
            }
            Field::Description(description) => {
                sqlx::query!(
                    "UPDATE PackageBases SET description = ? WHERE id = ?",
                    description.0,
                    existing.id
                )
            }
            Field::CreatedAt(date_time) => sqlx::query!(
                "UPDATE PackageBases SET created_at = ? WHERE id = ?",
                date_time,
                existing.id
            ),
            Field::UpdatedAt(date_time) => sqlx::query!(
                "UPDATE PackageBases SET updated_at = ? WHERE id = ?",
                date_time,
                existing.id
            ),
        }
        .execute(&*connection)
        .await?;

        match data {
            Field::Name(valid) => existing.name = valid.into_inner().0,
            Field::Description(valid) => existing.description = valid.into_inner().0,
            Field::CreatedAt(date_time) => existing.created_at = date_time,
            Field::UpdatedAt(date_time) => existing.updated_at = date_time,
        }

        Ok(())
    }

    async fn delete(connection: &mut E, data: Self::Unique) -> Result {
        sqlx::query!("DELETE FROM PackageBases WHERE id = ?", data)
            .execute(&*connection)
            .await?;

        Ok(())
    }
}
