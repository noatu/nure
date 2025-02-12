mod authentication;
mod input;
mod search;
//mod statistics;
mod widget;

use std::sync::Arc;

use crate::authentication::Authentication;
use crate::search::Search;
//use crate::statistics::Statistics;

use data::{MySqlPool, MySqlSearchAdapter, MySqlUserAdapter, SqlxPool};
use iced::{
    Element, Subscription, Task, Theme,
    futures::lock::Mutex,
    widget::{center, row},
    window,
};
use service::{
    Authenticated, AuthenticationAdapter, AuthenticationService, SearchAdapter, SearchService,
};

struct Repository {
    scale_factor: f64,
    main_id: window::Id,
    screen: Screen,

    authenticated: Option<Authenticated>,

    search: Search<SearchService<SearchAdapter<MySqlPool, SqlxPool, MySqlSearchAdapter>>>,
    authentication: Authentication<
        AuthenticationService<AuthenticationAdapter<MySqlPool, SqlxPool, MySqlUserAdapter>>,
    >,
}

enum Screen {
    Search,
    // Statistics,
    Authentication,
}

#[derive(Debug)]
enum Message {
    ScaleUp,
    ScaleDown,
    WindowOpened(window::Id),
    WindowClosed(window::Id),

    Search(search::Message),
    Authentecation(authentication::Message),
}

impl Repository {
    fn new() -> (Self, Task<Message>) {
        let (main_id, open_task) = window::open(window::Settings::default());
        // let (main_window, main_window_task) = MainWindow::new();

        let pool = MySqlPool::new(
            SqlxPool::connect_lazy(
                &std::env::var("DATABASE_URL")
                    .expect("environment variable `DATABASE_URL` should be set"),
            )
            .unwrap(),
        );

        let auth_service = Arc::new(Mutex::new(AuthenticationService::new(
            AuthenticationAdapter::new(pool.clone()),
        )));

        let search_service = Arc::new(Mutex::new(SearchService::new(SearchAdapter::new(
            pool.clone(),
        ))));

        (
            Self {
                main_id,
                scale_factor: 1.4,
                screen: Screen::Search,


                authenticated: None,

                search: Search::new(search_service),
                authentication: Authentication::new(auth_service),
            },
            Task::batch([
                open_task.map(Message::WindowOpened),
                // main_window_task.map(Message::MainWindow),
            ]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ScaleUp => self.scale_factor = (self.scale_factor + 0.2).min(5.0),
            Message::ScaleDown => self.scale_factor = (self.scale_factor - 0.2).max(0.2),
            Message::WindowOpened(id) => {
                log!("Window opened: {id}");
                return iced::widget::focus_next();
            }
            Message::WindowClosed(id) => {
                log!("Window closed: {id}");
                if id == self.main_id {
                    return iced::exit();
                }
            }
            Message::Authentecation(message) => {
                if let Some(event) = self.authentication.update(message) {
                    match event {
                        authentication::Event::Task(task) => {
                            return task.map(Message::Authentecation);
                        }
                        authentication::Event::Authenticated(authenticated) => {
                            log!("authenticated as {:#?}", authenticated);
                            self.authenticated = Some(authenticated);
                            self.screen = Screen::Search;
                        }
                    }
                }
            }
            Message::Search(m) => {
                if let Some(event) = self.search.update(m) {
                    match event {
                        search::Event::Task(task) => {
                            return task.map(Message::Search);
                        }
                        search::Event::OpenPackage(id) => {
                            log!("opening package {id}")
                        }
                        search::Event::OpenBase(id) => {
                            log!("opening package base {id}")
                        }
                        search::Event::OpenURL(url) => {
                            log!("opening url {url}")
                        }
                    }
                }
            }
        }

        Task::none()
    }

    fn view(&self, id: window::Id) -> Element<Message> {
        if self.main_id == id {
            match self.screen {
                Screen::Search => self.search.view().map(Message::Search),
                Screen::Authentication => self.authentication.view().map(Message::Authentecation),
            }
        } else {
            center(row!["This window is unknown.", "It may be closed."]).into()
        }
    }

    fn title(&self, _: window::Id) -> String {
        // "Repository".into()
        self.authentication.title()
    }

    fn subscription(&self) -> Subscription<Message> {
        use iced::keyboard::{self, Key, Modifiers};

        let hotkeys = keyboard::on_key_press(|key, modifiers| match (modifiers, key) {
            (Modifiers::CTRL, Key::Character(c)) if c == "-" => Some(Message::ScaleDown),
            (Modifiers::CTRL, Key::Character(c)) if c == "=" || c == "+" => Some(Message::ScaleUp),
            _ => None,
        });

        Subscription::batch([hotkeys, window::close_events().map(Message::WindowClosed)])
    }

    const fn scale_factor(&self, _: window::Id) -> f64 {
        self.scale_factor
    }

    const fn theme(_: &Self, _: window::Id) -> Theme {
        Theme::TokyoNight
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*)
    };
}

fn main() -> iced::Result {
    iced::daemon(Repository::title, Repository::update, Repository::view)
        .subscription(Repository::subscription)
        .scale_factor(Repository::scale_factor)
        .theme(Repository::theme)
        .run_with(Repository::new)
}
