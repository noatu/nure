use crate::input::Input;
use crate::widget::centerbox;
use service::{
    Authenticated, AuthenticationContract,
    authentication::{self, Error, LoginData, Result},
};

use iced::futures::lock::Mutex;
use iced::widget::{Space, button, checkbox, column, container, row, text};
use iced::{Length, Task, padding};
use std::sync::Arc;

pub struct Login<S> {
    login: Input<authentication::Login>,
    password: Input<authentication::Password>,
    show_password: bool,

    state: State,
    service: Arc<Mutex<S>>,
}
enum State {
    None,
    Requesting,
    Success,
    Error(String),
}

pub enum Event {
    SwitchToRegister,
    Task(Task<Message>),
    Authenticated(Authenticated),
}
impl From<Task<Message>> for Event {
    fn from(value: Task<Message>) -> Self {
        Self::Task(value)
    }
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

    RequestResult(Arc<Result<Authenticated>>),
}

impl<S: AuthenticationContract + 'static> Login<S> {
    pub fn new(service: Arc<Mutex<S>>) -> Self {
        Self {
            login: Input::new("login_name"),
            password: Input::new("login_password"),
            show_password: false,
            state: State::None,
            service,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::LoginChanged(s) => self.login.update(s),
            Message::PasswordChanged(s) => self.password.update(s),
            Message::ShowPasswordToggled(b) => self.show_password = b,

            Message::LoginSubmitted if self.login.critical() => (),
            Message::LoginSubmitted => return Some(self.password.focus().into()),

            Message::RegisterPressed => return Some(Event::SwitchToRegister),

            Message::LoginPressed | Message::PasswordSubmitted => {
                let login_data = LoginData {
                    login: match self.login.submit() {
                        Ok(x) => x,
                        Err(t) => return Some(t.into()),
                    },
                    password: match self.password.submit() {
                        Ok(x) => x,
                        Err(t) => return Some(t.into()),
                    },
                };

                self.state = State::Requesting;
                let arc = self.service.clone();

                return Some(
                    Task::perform(
                        async move {
                            let Some(service) = arc.try_lock() else {
                                return Err(Error::Other(
                                    "other authentication request is being performed".into(),
                                ));
                            };
                            service.login(login_data).await
                        },
                        |r| Message::RequestResult(Arc::new(r)),
                    )
                    .into(),
                );
            }
            Message::RequestResult(r) => match &*r {
                Ok(a) => {
                    self.state = State::Success;
                    return Some(Event::Authenticated(a.clone()));
                }

                Err(e) => {
                    self.state = State::None;
                    match e {
                        Error::LoginNotFound => self.login.set_warning(e),
                        Error::IncorrectPassword => self.password.set_warning(e),
                        Error::InvalidPassword(_) => self.password.set_error(e),

                        _ => self.state = State::Error(e.to_string()),
                    }
                }
            },
        }

        None
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

    pub fn title(&self) -> String {
        let errors = [
            self.login.error(),
            self.password.error(),
            self.login.warning(),
            self.password.warning(),
        ];
        let error = errors.into_iter().flatten().next();

        match &self.state {
            State::None => error.map_or_else(|| "Login".into(), Into::into),
            State::Success => "Success".into(),
            State::Requesting => "Requesting...".into(),
            State::Error(e) => e.into(),
        }
    }
}
