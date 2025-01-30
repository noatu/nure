pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[allow(async_fn_in_trait)]
pub trait CRUD<C> {
    type Create;
    type Read;
    type Update;
    type Delete;
    type Existing;
    type Id;

    async fn create(connection: &mut C, data: Self::Create) -> Result<Self::Id>;
    async fn read(connection: &C, data: Self::Read) -> Result<Option<Self::Existing>>;
    async fn update(connection: &mut C, id: Self::Id, data: Self::Update) -> Result;
    async fn delete(connection: &mut C, data: Self::Delete) -> Result;
}

pub mod user;
pub mod base;
// pub mod package;
// pub mod session;
