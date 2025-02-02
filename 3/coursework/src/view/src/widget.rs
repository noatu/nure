use iced::widget::{Scrollable, center, container, mouse_area, scrollable, text, tooltip};
use iced::{Element, color};

/// Put content into a dark container at the center of the screen
/// which can be scrolled in multiple dirrections
pub fn centerbox<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    center(scroll(
        container(content).style(container::dark).padding(20),
    ))
    .into()
}

/// Scrollable but in both vertical and horizontal directions
pub fn scroll<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    Scrollable::with_direction(content, scrollable::Direction::Both {
        vertical: scrollable::Scrollbar::default(),
        horizontal: scrollable::Scrollbar::default(),
    })
    .into()
}

/// Clickable url
pub fn url<'a, Message: Clone + 'a>(txt: &impl ToString, msg: Message) -> Element<'a, Message> {
    Element::from(mouse_area(text(txt.to_string()).color(color!(0xBB_B6_DF))).on_press(msg))
}

pub mod tip {
    pub use iced::widget::tooltip::Position;
}

/// Tooltip with some styling applied
pub fn tip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    tip: &'a str,
    position: tip::Position,
) -> Element<'a, Message> {
    tooltip(
        content,
        container(text(tip).size(14))
            .padding(5)
            .style(container::dark),
        position,
    )
    .into()
}

pub mod text_input {
    use iced::widget::text_input::{Status, Style, default};
    use iced::{Theme, color};

    pub fn success(theme: &Theme, status: Status) -> Style {
        Style {
            background: color!(0x00_33_00).into(),
            ..default(theme, status)
        }
    }
    pub fn warning(theme: &Theme, status: Status) -> Style {
        Style {
            background: color!(0x33_33_00).into(),
            ..default(theme, status)
        }
    }
    pub fn error(theme: &Theme, status: Status) -> Style {
        Style {
            background: color!(0x33_00_00).into(),
            ..default(theme, status)
        }
    }
}
