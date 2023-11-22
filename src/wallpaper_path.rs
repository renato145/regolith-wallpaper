use crate::{Message, StatusBar};
use iced::widget::{
    button, column, container, horizontal_space, row, text, text_input, vertical_space,
};
use iced::{theme, Color, Length};
use iced::{Command, Element};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum WallpaperPathMessage {
    InputEdit(String),
    Ok,
    Cancel,
}

/// Paths from which to load wallpaper images
pub struct WallpaperPath {
    input: String,
    path: Option<PathBuf>,
    status_bar: StatusBar,
}

impl Default for WallpaperPath {
    fn default() -> Self {
        Self::new()
    }
}

impl WallpaperPath {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            path: None,
            status_bar: StatusBar::None,
        }
    }

    pub fn update(&mut self, message: WallpaperPathMessage) -> Command<Message> {
        match message {
            WallpaperPathMessage::InputEdit(input) => {
                self.input = input;
                Command::none()
            }
            WallpaperPathMessage::Ok => {
                let input = self.input.replace(
                    '~',
                    &std::env::var("HOME").expect("$HOME env var not found."),
                );
                let path = PathBuf::from(&input);
                if path.exists() {
                    self.status_bar = StatusBar::Ok(format!("Path setted to {:?}", path));
                    self.input = path.to_str().unwrap().to_string();
                    self.path = Some(path);
                } else {
                    self.status_bar = StatusBar::Error(format!("Invalid path: {:?}", path));
                }
                Command::none()
            }
            WallpaperPathMessage::Cancel => {
                if let Some(path) = &self.path {
                    self.input = path.to_str().unwrap().to_string();
                }
                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<WallpaperPathMessage> {
        let label = text("Wallpapers folder path:").size(16);

        let input = text_input("Enter folder path...", &self.input)
            .on_input(WallpaperPathMessage::InputEdit);

        let button_ok = button(container(text("Ok").size(16)).width(100).center_x())
            .padding(10)
            .style(theme::Button::Positive)
            .on_press(WallpaperPathMessage::Ok);
        let button_cancel = button(container(text("Cancel").size(16)).width(100).center_x())
            .padding(10)
            .style(theme::Button::Destructive)
            .on_press_maybe(self.path.as_ref().map(|_| WallpaperPathMessage::Cancel));
        let buttons = row!(horizontal_space(Length::Fill), button_ok, button_cancel).spacing(10);

        let status_bar = self.status_bar.view();

        container(
            column!(label, input, buttons, vertical_space(20), status_bar)
                .spacing(10)
                .max_width(800),
        )
        .style(|_: &_| container::Appearance {
            border_width: 1.0,
            border_color: Color::WHITE,
            ..Default::default()
        })
        .padding(30)
        .into()
    }
}
