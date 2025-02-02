use crate::widget::text_input::{error, success, warning};

use iced::widget::{TextInput, text_input, text_input::default};

pub struct Input {
    id: &'static str,
    value: String,
    warning: Option<String>,
    state: State,
}
enum State {
    None,
    Valid,
    Invalid(String),
}

pub enum Validation {
    Valid,
    Warning(String),
    Invalid(String),
}

impl Input {
    pub const fn new(id: &'static str) -> Self {
        Self {
            id,
            value: String::new(),
            warning: None,
            state: State::None,
        }
    }

    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
    pub fn error(&self) -> Option<&str> {
        match &self.state {
            State::Invalid(e) => Some(e.as_ref()),
            _ => None,
        }
    }
    pub fn warning(&self) -> Option<&str> {
        self.warning.as_ref().map(AsRef::as_ref)
    }
    // pub fn submit(&self) -> Result<String, &str> {
    //     match &self.state {
    //         State::Invalid(e) => Err(e.as_ref()),
    //         State::None | State::Valid => Ok(self.value.clone()),
    //     }
    // }
    pub const fn submittable(&self) -> bool {
        !matches!(self.state, State::Invalid(_))
    }

    pub fn update(&mut self, value: String) {
        self.value = value;
        self.warning = None;
        self.state = State::None;
    }
    pub fn set_error(&mut self, value: String) {
        self.state = State::Invalid(value);
    }
    pub fn set_warning(&mut self, value: String) {
        self.warning = Some(value);
    }
    pub fn apply(&mut self, validation: Validation) {
        match validation {
            Validation::Valid => self.state = State::Valid,
            Validation::Warning(w) => self.warning = Some(w),
            Validation::Invalid(e) => self.state = State::Invalid(e),
        }
    }
    pub fn apply_if_eq(&mut self, value: &str, validation: Validation) {
        if self.value == value {
            self.apply(validation);
        }
    }

    pub fn focus<Message>(&self) -> iced::Task<Message> {
        iced::widget::text_input::focus(self.id)
    }
    pub fn view<Message: Clone>(&self, placeholder: &str) -> TextInput<Message> {
        text_input(placeholder, &self.value)
            .id(self.id)
            .padding(12)
            .style(match self.state {
                State::None if self.warning.is_none() => default,
                State::Valid if self.warning.is_none() => success,
                State::Invalid(_) => error,
                _ => warning,
            })
    }
}
