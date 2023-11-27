use crate::{expand_home_dir, Configuration, Error, Message};
use iced::widget::{button, column, container, horizontal_space, row, text, text_input};
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
    pub input: String,
    pub path: Option<PathBuf>,
    pub input_id: text_input::Id,
}

impl WallpaperPath {
    pub fn from_config(config: &Configuration) -> Self {
        let input = config
            .wallpapers_path
            .as_ref()
            .map(|path| path.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        Self {
            input,
            path: config.wallpapers_path.clone(),
            input_id: text_input::Id::unique(),
        }
    }

    pub fn focus_input(&self) -> Command<Message> {
        text_input::focus(self.input_id.clone())
    }

    pub fn update(&mut self, message: WallpaperPathMessage) -> Option<Message> {
        match message {
            WallpaperPathMessage::InputEdit(input) => {
                self.input = input;
                None
            }
            WallpaperPathMessage::Ok => {
                let path = expand_home_dir(&self.input);
                if path.exists() {
                    self.input = path.to_str().unwrap().to_string();
                    self.path = Some(path);
                    Some(Message::WallpaperPathSetted)
                } else {
                    Some(Message::UpdateStatusBar(Err(Error::InvalidPath(path))))
                }
            }
            WallpaperPathMessage::Cancel => {
                if let Some(path) = &self.path {
                    self.input = path.to_str().unwrap().to_string();
                    Some(Message::WallpaperPathToogle {
                        show: false,
                        msg: None,
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn view(&self) -> Element<WallpaperPathMessage> {
        let label = text("Wallpapers folder path:").size(16);
        let input = text_input("Enter folder path...", &self.input)
            .id(self.input_id.clone())
            .on_input(WallpaperPathMessage::InputEdit)
            .on_submit(WallpaperPathMessage::Ok);

        let button_ok = button(container(text("Ok").size(16)).width(100).center_x())
            .padding([5, 10])
            .style(theme::Button::Positive)
            .on_press(WallpaperPathMessage::Ok);
        let button_cancel = button(container(text("Cancel").size(16)).width(100).center_x())
            .padding([5, 10])
            .style(theme::Button::Destructive)
            .on_press_maybe(self.path.as_ref().map(|_| WallpaperPathMessage::Cancel));
        let buttons = row!(horizontal_space(Length::Fill), button_ok, button_cancel).spacing(10);

        container(column!(label, input, buttons).spacing(10))
            .max_width(800)
            .style(|_: &_| container::Appearance {
                border_width: 1.0,
                border_color: Color::WHITE,
                ..Default::default()
            })
            .padding(30)
            .into()
    }
}
