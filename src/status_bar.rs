use iced::{widget::text, Color, Element};

pub enum StatusBar {
    None,
    Ok(String),
    Error(String),
}

impl StatusBar {
    pub fn view<T>(&self) -> Element<T> {
        match self {
            StatusBar::None => text(""),
            StatusBar::Ok(s) => text(s),
            StatusBar::Error(s) => text(s).style(Color::from_rgb(0.9, 0.2, 0.2)),
        }
        .into()
    }
}
