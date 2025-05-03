use data::{BoxDynError, search};
pub use data::{
    Result, Validation,
    search::{Mode, Order, Entry},
};

use derive_more::{Deref, Into};
use garde::Validate;

pub trait SearchContract: Send {
    fn search(&self, data: Data) -> impl Future<Output = Result<Vec<Entry>>> + Send;
}

pub struct Data {
    pub mode: Mode,
    pub order: Order,
    pub search: Search,

    pub limit: u16,
    pub exact: bool,
    pub ascending: bool,
}

impl From<Data> for search::Data {
    fn from(value: Data) -> Self {
        Self {
            mode: value.mode,
            order: value.order,
            search: value.search.into(),
            limit: value.limit,
            exact: value.exact,
            ascending: value.ascending,
        }
    }
}

pub type ReturnError<T = String> = (T, BoxDynError);

#[derive(Clone, Deref, Into)]
pub struct Search(search::Search);
impl AsRef<str> for Search {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
impl TryFrom<String> for Search {
    type Error = ReturnError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        #[derive(Validate)]
        #[garde(transparent)]
        struct Check<'a>(#[garde(ascii, length(chars, min = 1, max = 255))] &'a str);

        match Check(value.as_str()).validate() {
            Ok(()) => (),
            Err(e) => return Err((value, e.into())),
        }
        Ok(Self(search::Search::new(value)?))
    }
}
