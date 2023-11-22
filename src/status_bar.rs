use iced::{
    widget::{container, text},
    Background, Color, Element, Length,
};

pub enum StatusBar {
    None,
    Ok(String),
    Error(String),
}

impl StatusBar {
    pub fn view<'a, T: 'a>(&'a self) -> Element<T> {
        let text = match self {
            StatusBar::None => text(""),
            StatusBar::Ok(s) => text(s),
            StatusBar::Error(s) => text(s).style(Color::from_rgb(0.9, 0.2, 0.2)),
        };
        container(text)
            .padding([5, 10])
            .width(Length::Fill)
            .style(|_: &_| container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.16, 0.21, 0.2))),
                ..Default::default()
            })
            .into()
    }
}
