use super::{Result, package_base::Base};

use derive_more::{Deref, From, Into};
use garde::{Valid, Validate};
use sqlx::{Executor, MySql};

// pub enum GetBy

#[allow(async_fn_in_trait)]
pub trait PackageRepository<C> {
    async fn get_by_id(connection: &C, id: Id) -> Result<Option<Package>>;
    async fn get_by_name(connection: &C, name: &Valid<Name>) -> Result<Option<Package>>;

    async fn change_name(connection: &mut C, package: &mut Package, name: Valid<Name>) -> Result;
    async fn change_base(connection: &mut C, package: &mut Package, base: &Base) -> Result;
    async fn change_version(connection: &mut C, package: &mut Package, version: Valid<Version>) -> Result;

    async fn create(connection: &mut C, data: Valid<PackageData>) -> Result<Package>;
}

#[derive(Deref, Into, Clone, Copy)]
pub struct Id(u32);

pub type BaseId = super::package_base::Id;

#[derive(Validate, Deref)]
#[garde(transparent)]
pub struct Name(#[garde(alphanumeric, length(min = 2, max = 127))] pub String);

#[derive(Validate, Deref)]
#[garde(transparent)]
pub struct Version(#[garde(alphanumeric, length(min = 1, max = 127))] pub String);

#[derive(Validate, Deref)]
#[garde(transparent)]
pub struct Description(#[garde(ascii, length(max = 255))] pub Option<String>);

#[derive(Validate)]
#[garde(transparent)]
pub struct URL(#[garde(url, length(max = 510))] pub Option<String>);

#[derive(Validate)]
pub struct PackageData {
    #[garde(dive)]
    pub name: Name,
    #[garde(dive)]
    pub description: Description,
}

#[derive(Deref)]
pub struct Package {
    id: Id,
    #[deref]
    data: PackageData,
}
impl Package {
    pub const fn id(&self) -> Id {
        self.id
    }
}

// pub struct UserAdapter;
//
// struct QueryUser {
//     id: u32,
//     name: String,
//     email: String,
//     password: String,
// }
// impl From<QueryUser> for Package {
//     fn from(value: QueryUser) -> Self {
//         Self {
//             id: Id(value.id),
//             data: PackageData {
//                 name: Name(value.name),
//                 description: Description(value.email),
//             },
//         }
//     }
// }
//
// impl<E> PackageRepository<E> for UserAdapter
// where
//     for<'a> &'a E: Executor<'a, Database = MySql>,
// {
//     async fn get_by_id(connection: &E, id: Id) -> Result<Option<Package>> {
//         Ok(sqlx::query_as!(
//             QueryUser,
//             "SELECT id, name, email, password FROM Users WHERE id = ?",
//             id.0
//         )
//         .fetch_optional(connection)
//         .await?
//         .map(Into::into))
//     }
//     async fn get_by_name(connection: &E, name: &Valid<Name>) -> Result<Option<Package>> {
//         Ok(sqlx::query_as!(
//             QueryUser,
//             "SELECT id, name, email, password FROM Users WHERE name = ?",
//             name.0
//         )
//         .fetch_optional(connection)
//         .await?
//         .map(Into::into))
//     }
//     async fn get_by_email(connection: &E, email: &Valid<Description>) -> Result<Option<Package>> {
//         Ok(sqlx::query_as!(
//             QueryUser,
//             "SELECT id, name, email, password FROM Users WHERE email = ?",
//             email.0
//         )
//         .fetch_optional(connection)
//         .await?
//         .map(Into::into))
//     }
//
//     async fn change_name(connection: &mut E, user: &mut Package, name: Valid<Name>) -> Result {
//         sqlx::query!("UPDATE Users SET name = ? WHERE id = ?", name.0, user.id.0)
//             .execute(&*connection)
//             .await?;
//         Ok(())
//     }
//     async fn change_email(
//         connection: &mut E,
//         user: &mut Package,
//         email: Valid<Description>,
//     ) -> Result {
//         sqlx::query!(
//             "UPDATE Users SET email = ? WHERE id = ?",
//             email.0,
//             user.id.0
//         )
//         .execute(&*connection)
//         .await?;
//         Ok(())
//     }
//     async fn change_password(
//         connection: &mut E,
//         user: &mut Package,
//         password: Valid<Password>,
//     ) -> Result {
//         sqlx::query!(
//             "UPDATE Users SET password = ? WHERE id = ?",
//             password.0,
//             user.id.0
//         )
//         .execute(&*connection)
//         .await?;
//         Ok(())
//     }
//
//     async fn create(connection: &mut E, data: Valid<PackageData>) -> Result<Package> {
//         let id = sqlx::query!(
//             "INSERT INTO Users (name, email, password) VALUES (?, ?, ?)",
//             data.name.0,
//             data.description.0,
//             data.password.0
//         )
//         .execute(&*connection)
//         .await?
//         .last_insert_id() as u32;
//
//         Ok(Package {
//             id: Id(id),
//             data: data.into_inner(),
//         })
//     }
// }
