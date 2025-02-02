//! Data access for the application.
pub mod adapter;
pub mod atomic;
pub mod connect;
pub mod port;

// Don't want to handle errors for dynamic mess.
pub type BoxDynError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T = (), E = BoxDynError> = std::result::Result<T, E>;

pub use chrono::Utc;

pub use atomic::Atomic;
pub use connect::*;

pub use adapter::mysql::base::BaseAdapter as MySqlBaseAdapter;
pub use adapter::mysql::package::PackageAdapter as MySqlPackageAdapter;
pub use adapter::mysql::user::UserAdapter as MySqlUserAdapter;
pub use port::base::{self, Base, BaseRepository};
pub use port::package::{self, Package, PackageRepository};
pub use port::user::{self, User, UserRepository};
