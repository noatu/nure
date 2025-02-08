use crate::widget::text_input::{error, success, warning};

use iced::widget::{TextInput, text_input, text_input::default};

/// A smarter [`text_input`].to avoid boilerplate.
pub struct Input<T> {
    id: &'static str,
    value: Value<T>,
    warning: Option<String>,
}

// use std::ops::Deref;
// impl<T> Deref for Input<T> {
//     type Target = Value<T>;
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }

impl<T: AsRef<str>> AsRef<str> for Input<T> {
    fn as_ref(&self) -> &str {
        self.value.as_ref()
    }
}

impl<T> Input<T> {
    pub const fn new(id: &'static str) -> Self {
        Self {
            id,
            value: Value::None,
            warning: None,
        }
    }

    pub fn focus<Message>(&self) -> iced::Task<Message> {
        iced::widget::text_input::focus(self.id)
    }

    pub fn warning(&self) -> Option<&str> {
        self.warning.as_ref().map(AsRef::as_ref)
    }
    pub fn error(&self) -> Option<&str> {
        match &self.value {
            Value::Invalid { error, .. } => Some(error.as_ref()),
            _ => None,
        }
    }

    pub fn set_value(&mut self, value: Value<T>) {
        self.value = value;
    }
    pub fn set_warning(&mut self, value: &impl ToString) {
        self.warning = Some(value.to_string());
    }
}

impl<T: AsRef<str>> Input<T> {
    pub fn set_error(&mut self, value: &impl ToString) {
        self.value.set_error(value.to_string());
    }

    pub fn view<Message: Clone>(&self, placeholder: &str) -> TextInput<Message> {
        text_input(placeholder, self.value.as_ref())
            .id(self.id)
            .padding(12)
            .style(match self.value {
                Value::Invalid { .. } => error,
                Value::None | Value::Valid(_) if self.warning.is_some() => warning,
                Value::Valid(_) => success,
                Value::None => default,
            })
    }
}

impl<T, E> Input<T>
where
    E: ToString,
    T: TryFrom<String, Error = (String, E)>,
{
    pub fn update(&mut self, value: String) {
        self.value.update(value);
        self.warning = None;
    }

    pub fn value(&mut self) -> Result<&T, &str> {
        match self.value {
            Value::None => {
                self.value.update(String::new());
                self.value()
            }
            Value::Valid(ref x) => Ok(x),
            Value::Invalid { ref error, .. } => Err(error),
        }
    }
    pub fn submittable(&mut self) -> bool {
        self.value().is_ok()
    }
    pub fn critical(&mut self) -> bool {
        self.value().is_err()
    }
}

impl<T, E> Input<T>
where
    E: ToString,
    T: TryFrom<String, Error = (String, E)> + Clone,
{
    pub fn submit<Message>(&mut self) -> Result<T, iced::Task<Message>> {
        match self.value() {
            Ok(x) => Ok(x.clone()),
            Err(_) => Err(self.focus()),
        }
    }
}

#[derive(Default)]
pub enum Value<T> {
    #[default]
    None,
    Valid(T),
    Invalid {
        value: String,
        error: String,
    },
}

impl<T: AsRef<str>> AsRef<str> for Value<T> {
    fn as_ref(&self) -> &str {
        match self {
            Self::None => "",
            Self::Valid(x) => x.as_ref(),
            Self::Invalid { value, .. } => value.as_ref(),
        }
    }
}

impl<T: AsRef<str>> Value<T> {
    fn set_error(&mut self, error: String) {
        match self {
            Self::None => {
                *self = Self::Invalid {
                    value: String::new(),
                    error,
                }
            }
            Self::Valid(x) => {
                *self = Self::Invalid {
                    value: x.as_ref().to_string(),
                    error,
                }
            }
            Self::Invalid { error: e, .. } => *e = error,
        }
    }
}

impl<T, E> Value<T>
where
    E: ToString,
    T: TryFrom<String, Error = (String, E)>,
{
    fn update(&mut self, value: String) {
        *self = match T::try_from(value) {
            Ok(x) => Self::Valid(x),
            Err((s, e)) => Self::Invalid {
                value: s,
                error: e.to_string(),
            },
        };
    }
}
