pub mod login;
pub mod register;

use crate::input::Validation;

use iced::Task;

pub struct Authentication {
    screen: Screen,
    login: login::Login,
    register: register::Register,
}
pub enum Screen {
    Login,
    Register,
}

#[derive(Debug)]
pub enum Message {
    Login(login::Message),
    Register(register::Message),
}

pub enum Request {
    Task(Task<Message>),
    SimpleLoginValidation(login::Field),
    SimpleRegisterValidation(register::Field),
    Login {
        login: String,
        password: String,
    },
    Register {
        name: String,
        email: String,
        password: String,
    },
}

pub enum RequestResult {
    Error(String),
    LoginValidation(login::Field, Validation),
    RegisterValidation(register::Field, Validation),
}

impl Default for Authentication {
    fn default() -> Self {
        Self::new()
    }
}

impl Authentication {
    pub const fn new() -> Self {
        Self {
            screen: Screen::Login,
            login: Login::new(),
            register: Register::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Request> {
        Some(match message {
            Message::Login(message) => match self.login.update(message)? {
                login::Request::SwitchToRegister => {
                    self.screen = Screen::Register;
                    return None;
                }
                login::Request::SimpleValidation(x) => Request::SimpleLoginValidation(x),
                login::Request::Task(task) => Request::Task(task.map(Message::Login)),
                login::Request::Login { login, password } => Request::Login { login, password },
            },
            Message::Register(message) => match self.register.update(message)? {
                register::Request::SwitchToLogin => {
                    self.screen = Screen::Login;
                    return None;
                }
                register::Request::SimpleValidation(x) => Request::SimpleRegisterValidation(x),
                register::Request::Task(task) => Request::Task(task.map(Message::Register)),
                register::Request::Register {
                    name,
                    email,
                    password,
                } => Request::Register {
                    name,
                    email,
                    password,
                },
            },
        })
    }

    pub fn view(&self) -> iced::Element<Message> {
        match self.screen {
            Screen::Login => self.login.view().map(Message::Login),
            Screen::Register => self.register.view().map(Message::Register),
        }
    }

    pub fn title(&self) -> std::borrow::Cow<str> {
        match self.screen {
            Screen::Login => self.login.title(),
            Screen::Register => self.register.title(),
        }
    }
}
