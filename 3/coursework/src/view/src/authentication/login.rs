use crate::input::{Input, Validation};
use crate::widget::centerbox;

use iced::widget::{Space, button, checkbox, column, container, row, text};
use iced::{Length, Task, padding};

pub struct Login {
    state: State,
    login: Input,
    password: Input,
    show_password: bool,
}
enum State {
    None,
    Requesting,
    Error(String),
}

pub enum Request {
    SwitchToRegister,
    SimpleValidation(Field),
    Task(Task<Message>),
    Login { login: String, password: String },
}
pub enum Field {
    Login(String),
    Password(String),
}

pub enum RequestResult {
    Error(String),
    Validation(Field, Validation),
}

#[derive(Debug, Clone)]
pub enum Message {
    LoginChanged(String),
    PasswordChanged(String),
    ShowPasswordToggled(bool),

    LoginSubmitted,
    PasswordSubmitted,

    LoginPressed,
    RegisterPressed,
}

impl Default for Login {
    fn default() -> Self {
        Self::new()
    }
}

impl Login {
    pub const fn new() -> Self {
        Self {
            state: State::None,
            login: Input::new("login_name"),
            password: Input::new("login_password"),
            show_password: false,
        }
    }

    pub fn handle_result(&mut self, result: RequestResult) {
        match result {
            RequestResult::Error(e) => self.state = State::Error(e),
            RequestResult::Validation(field, validation) => match &field {
                Field::Login(s) => self.login.apply_if_eq(s, validation),
                Field::Password(s) => self.password.apply_if_eq(s, validation),
            },
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Request> {
        Some(match message {
            Message::LoginChanged(s) => {
                self.login.update(s.clone());
                Request::SimpleValidation(Field::Login(s))
            }
            Message::PasswordChanged(s) => {
                self.password.update(s.clone());
                Request::SimpleValidation(Field::Password(s))
            }

            Message::ShowPasswordToggled(b) => {
                self.show_password = b;
                return None;
            }

            Message::LoginSubmitted if !self.login.submittable() => return None,
            Message::LoginSubmitted => Request::Task(self.login.focus()),

            Message::LoginPressed | Message::PasswordSubmitted => {
                if !self.login.submittable() {
                    Request::Task(self.login.focus())
                } else if !self.password.submittable() {
                    Request::Task(self.password.focus())
                } else {
                    self.state = State::Requesting;

                    Request::Login {
                        login: self.login.value().into(),
                        password: self.password.value().into(),
                    }
                }
            }

            Message::RegisterPressed => Request::SwitchToRegister,
        })
    }

    pub fn view(&self) -> iced::Element<Message> {
        centerbox(
            column![
                container(text(self.title()).size(20))
                    .center_x(Length::Fill)
                    .padding(padding::bottom(10)),
                self.login
                    .view("Email or Username")
                    .on_input(Message::LoginChanged)
                    .on_submit(Message::LoginSubmitted),
                self.password
                    .view("Password")
                    .on_input(Message::PasswordChanged)
                    .on_submit(Message::PasswordSubmitted)
                    .secure(!self.show_password),
                checkbox("Show password", self.show_password)
                    .on_toggle(Message::ShowPasswordToggled),
                row![
                    button(text("Register").center().size(18))
                        .on_press(Message::RegisterPressed)
                        .style(button::secondary)
                        .width(Length::FillPortion(3))
                        .padding(10),
                    Space::with_width(Length::FillPortion(2)),
                    button(text("Login").center().size(18))
                        .on_press(Message::LoginPressed)
                        .style(button::primary)
                        .width(Length::FillPortion(3))
                        .padding(10)
                ]
                .padding(padding::top(15)),
            ]
            .width(Length::Fixed(250.))
            .spacing(20),
        )
    }

    pub fn title(&self) -> std::borrow::Cow<str> {
        let errors = [
            self.login.error(),
            self.password.error(),
            self.login.warning(),
            self.password.warning(),
        ];
        let error = errors.into_iter().flatten().next();

        match &self.state {
            State::None => error.map_or_else(|| "Login".into(), Into::into),
            State::Requesting => "Requesting...".into(),
            State::Error(e) => e.into(),
        }
    }
}
