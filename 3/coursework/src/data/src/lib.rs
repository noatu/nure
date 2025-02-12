//! Data access for the application.
pub mod adapter;
pub mod atomic;
pub mod connect;
pub mod port;

// Don't want to handle errors for dynamic mess.
pub type BoxDynError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T = (), E = BoxDynError> = std::result::Result<T, E>;

pub use chrono::Utc;

pub use adapter::mysql::base::BaseAdapter as MySqlBaseAdapter;
pub use adapter::mysql::package::PackageAdapter as MySqlPackageAdapter;
pub use adapter::mysql::user::UserAdapter as MySqlUserAdapter;
pub use adapter::mysql::search::SearchAdapter as MySqlSearchAdapter;
pub use atomic::Atomic;
pub use connect::*;
pub use port::base::{Base, BaseRepository};
pub use port::package::{Package, PackageRepository};
pub use port::search::{Search, SearchRepository};
pub use port::user::{User, UserRepository};
pub use port::*;
