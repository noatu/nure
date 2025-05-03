mod login;
mod register;

use login::Login;
use register::Register;

use service::{Authenticated, AuthenticationContract};

use iced::{Element, Task, futures::lock::Mutex};
use std::sync::Arc;

pub struct Authentication<S> {
    login: Login<S>,
    register: Register<S>,
    screen: Screen,
}

enum Screen {
    Login,
    Register,
}

#[derive(Debug)]
pub enum Message {
    Login(login::Message),
    Register(register::Message),
}

pub enum Event {
    Task(Task<Message>),
    Authenticated(Authenticated),
}
impl From<Task<Message>> for Event {
    fn from(value: Task<Message>) -> Self {
        Self::Task(value)
    }
}

impl<S: AuthenticationContract + 'static> Authentication<S> {
    pub fn new(service: Arc<Mutex<S>>) -> Self {
        Self {
            login: Login::new(service.clone()),
            register: Register::new(service),
            screen: Screen::Login,
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        Some(match message {
            Message::Login(message) => match self.login.update(message)? {
                login::Event::SwitchToRegister => {
                    self.screen = Screen::Register;
                    return None;
                }

                login::Event::Task(task) => task.map(Message::Login).into(),
                login::Event::Authenticated(x) => Event::Authenticated(x),
            },
            Message::Register(message) => match self.register.update(message)? {
                register::Event::SwitchToLogin => {
                    self.screen = Screen::Login;
                    return None;
                }
                register::Event::Task(task) => task.map(Message::Register).into(),
                register::Event::Authenticated(x) => Event::Authenticated(x),
            },
        })
    }

    pub fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Login => self.login.view().map(Message::Login),
            Screen::Register => self.register.view().map(Message::Register),
        }
    }

    pub fn title(&self) -> String {
        match self.screen {
            Screen::Login => self.login.title(),
            Screen::Register => self.register.title(),
        }
    }
}
