pub use crate::port::package::*;

use sqlx::{Executor, MySql};

pub struct PackageAdapter;

impl<E> PackageRepository<E> for PackageAdapter
where
    E: Send,
    for<'a> &'a E: Executor<'a, Database = MySql>,
{
}
impl<E> crate::port::CRUD<E> for PackageAdapter
where
    E: Send,
    for<'a> &'a E: Executor<'a, Database = MySql>,
{
    type New = New;
    type Update = Field;
    type Unique = Unique;
    type Existing = Package;

    async fn create(connection: &mut E, data: Self::New) -> Result<Self::Existing> {
        let created_at = Utc::now();
        let id = sqlx::query!(
            "INSERT INTO Packages \
            (package_base, name, version, description, url, flagged_at, created_at, updated_at) \
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            data.package_base.id,
            data.name.as_str(),
            data.version.as_str(),
            data.description.as_ref(),
            data.url.as_ref(),
            data.flagged_at,
            created_at,
            created_at,
        )
        .execute(&*connection)
        .await?
        .last_insert_id();

        Ok(Self::Existing {
            id,
            package_base: data.package_base.id,
            name: data.name.into(),
            version: data.version.into(),
            description: data.description.into(),
            url: data.url.into(),
            flagged_at: data.flagged_at,
            created_at,
            updated_at: created_at,
        })
    }

    async fn read(connection: &E, data: Self::Unique) -> Result<Option<Self::Existing>> {
        Ok(match data {
            Unique::Id(id) => {
                sqlx::query_as!(Package, "SELECT * FROM Packages WHERE id = ?", id)
                    .fetch_optional(connection)
                    .await
            }
            Unique::Name(name) => {
                sqlx::query_as!(
                    Package,
                    "SELECT * FROM Packages WHERE name = ?",
                    name.as_str()
                )
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
                    "UPDATE Packages SET name = ? WHERE id = ?",
                    name.as_str(),
                    existing.id
                )
            }
            Field::PackageBase(package_base) => {
                sqlx::query!(
                    "UPDATE Packages SET package_base = ? WHERE id = ?",
                    package_base.id,
                    existing.id
                )
            }
            Field::Version(version) => {
                sqlx::query!(
                    "UPDATE Packages SET version = ? WHERE id = ?",
                    version.as_str(),
                    existing.id
                )
            }
            Field::Description(description) => {
                sqlx::query!(
                    "UPDATE Packages SET description = ? WHERE id = ?",
                    description.as_ref(),
                    existing.id
                )
            }
            Field::URL(url) => {
                sqlx::query!(
                    "UPDATE Packages SET url = ? WHERE id = ?",
                    url.as_ref(),
                    existing.id
                )
            }
            Field::FlaggedAt(date_time) => sqlx::query!(
                "UPDATE Packages SET flagged_at = ? WHERE id = ?",
                date_time,
                existing.id
            ),
            Field::CreatedAt(date_time) => sqlx::query!(
                "UPDATE Packages SET created_at = ? WHERE id = ?",
                date_time,
                existing.id
            ),
            Field::UpdatedAt(date_time) => sqlx::query!(
                "UPDATE Packages SET updated_at = ? WHERE id = ?",
                date_time,
                existing.id
            ),
        }
        .execute(&*connection)
        .await?;

        match data {
            Field::Name(s) => existing.name = s.into(),
            Field::PackageBase(s) => existing.package_base = s.id,
            Field::Version(s) => existing.version = s.into(),
            Field::Description(o) => existing.description = o.into(),
            Field::URL(o) => existing.url = o.into(),
            Field::FlaggedAt(date_time) => existing.flagged_at = date_time,
            Field::CreatedAt(date_time) => existing.created_at = date_time,
            Field::UpdatedAt(date_time) => existing.updated_at = date_time,
        }

        Ok(())
    }

    async fn delete(connection: &mut E, data: Self::Unique) -> Result {
        match data {
            Unique::Id(id) => sqlx::query!("DELETE FROM Packages WHERE id = ?", id),
            Unique::Name(name) => {
                sqlx::query!("DELETE FROM Packages WHERE name = ?", name.as_str())
            }
        }
        .execute(&*connection)
        .await?;

        Ok(())
    }
}
