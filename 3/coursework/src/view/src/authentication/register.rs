use crate::input::{Input, Validation};
use crate::widget::centerbox;

use iced::widget::{Space, button, checkbox, column, container, row, text};
use iced::{Length, Task, padding};

pub struct Register {
    state: State,
    name: Input,
    email: Input,
    password: Input,
    repeat: Input,
    show_password: bool,
}
enum State {
    None,
    Requesting,
    Error(String),
}

pub enum Request {
    SwitchToLogin,
    SimpleValidation(Field),
    Task(Task<Message>),
    Register {
        name: String,
        email: String,
        password: String,
    },
}
pub enum Field {
    Name(String),
    Email(String),
    Password(String),
}

pub enum RequestResult {
    Error(String),
    Validation(Field, Validation),
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    RepeatChanged(String),
    ShowPasswordToggled(bool),

    EmailSubmitted,
    NameSubmitted,
    PasswordSubmitted,
    RepeatSubmitted,

    RegisterPressed,
    LoginPressed,
}

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}

impl Register {
    pub const fn new() -> Self {
        Self {
            state: State::None,
            name: Input::new("register_name"),
            email: Input::new("register_email"),
            password: Input::new("register_password"),
            repeat: Input::new("register_repeat"),
            show_password: false,
        }
    }

    pub fn handle_result(&mut self, result: RequestResult) {
        match result {
            RequestResult::Error(e) => self.state = State::Error(e),
            RequestResult::Validation(field, validation) => match &field {
                Field::Name(s) => self.name.apply_if_eq(s, validation),
                Field::Email(s) => self.email.apply_if_eq(s, validation),
                Field::Password(s) => self.password.apply_if_eq(s, validation),
            },
        }
    }

    #[inline]
    fn check_passwords(&mut self) {
        if self.password.value() != self.repeat.value() {
            self.repeat.set_error("passwords are different".into());
        }
    }
    pub fn update(&mut self, message: Message) -> Option<Request> {
        Some(match message {
            Message::NameChanged(s) => {
                self.name.update(s.clone());
                Request::SimpleValidation(Field::Name(s))
            }
            Message::EmailChanged(s) => {
                self.email.update(s.clone());
                Request::SimpleValidation(Field::Email(s))
            }
            Message::PasswordChanged(s) => {
                self.password.update(s.clone());
                self.check_passwords();
                Request::SimpleValidation(Field::Password(s))
            }
            Message::RepeatChanged(s) => {
                self.repeat.update(s);
                self.check_passwords();
                return None;
            }

            Message::ShowPasswordToggled(b) => {
                self.show_password = b;
                return None;
            }

            Message::NameSubmitted if !self.name.submittable() => return None,
            Message::NameSubmitted => Request::Task(self.email.focus()),
            Message::EmailSubmitted if !self.email.submittable() => return None,
            Message::EmailSubmitted => Request::Task(self.password.focus()),
            Message::PasswordSubmitted if !self.password.submittable() => return None,
            Message::PasswordSubmitted => Request::Task(self.repeat.focus()),

            Message::RegisterPressed | Message::RepeatSubmitted => {
                if !self.name.submittable() {
                    Request::Task(self.name.focus())
                } else if !self.email.submittable() {
                    Request::Task(self.email.focus())
                } else if !self.password.submittable() {
                    Request::Task(self.password.focus())
                } else if !self.repeat.submittable() {
                    Request::Task(self.repeat.focus())
                } else {
                    self.state = State::Requesting;

                    Request::Register {
                        name: self.name.value().into(),
                        email: self.email.value().into(),
                        password: self.password.value().into(),
                    }
                }
            }

            Message::LoginPressed => Request::SwitchToLogin,
        })
    }

    pub fn view(&self) -> iced::Element<Message> {
        centerbox(
            column![
                container(text(self.title()).size(20))
                    .center_x(Length::Fill)
                    .padding(padding::bottom(10)),
                self.name
                    .view("Username")
                    .on_input(Message::NameChanged)
                    .on_submit(Message::NameSubmitted),
                self.email
                    .view("Email")
                    .on_input(Message::EmailChanged)
                    .on_submit(Message::EmailSubmitted),
                self.password
                    .view("Password")
                    .on_input(Message::PasswordChanged)
                    .on_submit(Message::PasswordSubmitted)
                    .secure(!self.show_password),
                self.repeat
                    .view("Repeat Password")
                    .on_input(Message::RepeatChanged)
                    .on_submit(Message::RepeatSubmitted)
                    .secure(!self.show_password),
                checkbox("Show password", self.show_password)
                    .on_toggle(Message::ShowPasswordToggled),
                row![
                    button(text("Login").center().size(18))
                        .on_press(Message::LoginPressed)
                        .style(button::secondary)
                        .width(Length::FillPortion(3))
                        .padding(10),
                    Space::with_width(Length::FillPortion(2)),
                    button(text("Register").center().size(18))
                        .on_press(Message::RegisterPressed)
                        .style(button::primary)
                        .width(Length::FillPortion(3))
                        .padding(10),
                ]
                .padding(padding::top(15)),
            ]
            .width(Length::Fixed(250.))
            .spacing(20),
        )
    }

    pub fn title(&self) -> std::borrow::Cow<str> {
        let errors = [
            self.name.error(),
            self.email.error(),
            self.password.error(),
            self.repeat.error(),
            self.name.warning(),
            self.email.warning(),
            self.password.warning(),
            self.repeat.warning(),
        ];
        let error = errors.into_iter().flatten().next();

        match &self.state {
            State::None => error.map_or_else(|| "Register".into(), Into::into),
            State::Requesting => "Requesting...".into(),
            State::Error(e) => e.into(),
        }
    }
}
