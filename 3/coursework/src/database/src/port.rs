pub type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

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

trait CharLength {
    fn length(&self) -> usize;
}
impl CharLength for String {
    fn length(&self) -> usize {
        self.chars().count()
    }
}
impl CharLength for Option<String> {
    fn length(&self) -> usize {
        self.as_ref().map_or(0, CharLength::length)
    }
}

trait MaxLength {
    type Inner: CharLength;
    const MAX_LENGTH: usize;

    fn validate(value: &Self::Inner) -> Result<(), &'static str> {
        if value.length() > Self::MAX_LENGTH {
            Err("too long")
        } else {
            Ok(())
        }
    }
}

// const TOO_LONG: &str = "too long";

pub mod base;
pub mod package;
pub mod user;
