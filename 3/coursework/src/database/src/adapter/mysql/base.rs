pub use crate::port::base::*;

use sqlx::{Executor, MySql};

pub struct BaseAdapter;

struct DatabaseBase {
    id: u64,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
impl From<DatabaseBase> for Base {
    fn from(value: DatabaseBase) -> Self {
        Self {
            id: Id(value.id),
            name: value.name.into(),
            description: value.description.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl<E> BaseRepository<E> for BaseAdapter where for<'a> &'a E: Executor<'a, Database = MySql> {}
impl<E> crate::port::CRUD<E> for BaseAdapter
where
    for<'a> &'a E: Executor<'a, Database = MySql>,
{
    type Create = New;
    type Read = Id;
    type Update = Field;
    type Delete = Id;
    type Existing = Base;
    type Id = Id;

    async fn create(connection: &mut E, data: Self::Create) -> Result<Self::Id> {
        Ok(Id(sqlx::query!(
            "INSERT INTO PackageBases (name, description) VALUES (?, ?)",
            data.name.0,
            data.description.0,
        )
        .execute(&*connection)
        .await?
        .last_insert_id()))
    }

    async fn read(connection: &E, data: Self::Read) -> Result<Option<Self::Existing>> {
        Ok(sqlx::query_as!(
            DatabaseBase,
            "SELECT * FROM PackageBases WHERE id = ?",
            data.0
        )
        .fetch_optional(connection)
        .await?
        .map(Into::into))
    }

    async fn update(connection: &mut E, id: Self::Id, data: Self::Update) -> Result {
        match data {
            Field::Name(valid) => {
                sqlx::query!(
                    "UPDATE PackageBases SET name = ? WHERE id = ?",
                    valid.0,
                    id.0
                )
            }
            Field::Description(valid) => {
                sqlx::query!(
                    "UPDATE PackageBases SET description = ? WHERE id = ?",
                    valid.0,
                    id.0
                )
            }
            Field::CreatedAt(date_time) => sqlx::query!(
                "UPDATE PackageBases SET created_at = ? WHERE id = ?",
                date_time,
                id.0
            ),
            Field::UpdatedAt(date_time) => sqlx::query!(
                "UPDATE PackageBases SET updated_at = ? WHERE id = ?",
                date_time,
                id.0
            ),
        }
        .execute(&*connection)
        .await?;
        Ok(())
    }

    async fn delete(connection: &mut E, data: Self::Delete) -> Result {
        sqlx::query!("DELETE FROM PackageBases WHERE id = ?", data.0)
            .execute(&*connection)
            .await?;
        Ok(())
    }
}
