pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(async_fn_in_trait)]
pub trait CRUD<C> {
    type New;
    type Unique;
    type Update;
    type Existing;

    async fn create(connection: &mut C, data: Self::New) -> Result<Self::Existing>;
    async fn read(connection: &C, data: Self::Unique) -> Result<Option<Self::Existing>>;
    async fn update(
        connection: &mut C,
        existing: &mut Self::Existing,
        data: Self::Update,
    ) -> Result;
    async fn delete(connection: &mut C, data: Self::Unique) -> Result;
}

const TOO_LONG: &str = "too long";

pub mod base;
pub mod package;
pub mod user;
