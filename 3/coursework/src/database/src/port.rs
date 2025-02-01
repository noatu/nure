pub type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub trait CRUD<C> {
    type New;
    type Unique;
    type Update;
    type Existing;

    fn create(
        connection: &mut C,
        data: Self::New,
    ) -> impl Future<Output = Result<Self::Existing>> + Send;
    fn read(
        connection: &C,
        data: Self::Unique,
    ) -> impl Future<Output = Result<Option<Self::Existing>>> + Send;
    fn update(
        connection: &mut C,
        existing: &mut Self::Existing,
        data: Self::Update,
    ) -> impl Future<Output = Result> + Send;
    fn delete(connection: &mut C, data: Self::Unique) -> impl Future<Output = Result> + Send;
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
