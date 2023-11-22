use iced::font::Weight;
use iced::widget::{
    button, column, container, horizontal_space, image, row, text, text_input, vertical_space,
    Column,
};
use iced::{executor, theme, Background, Color, Font, Length};
use iced::{Application, Command, Element, Settings, Theme};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing_subscriber::fmt;

fn main() -> iced::Result {
    fmt::init();
    RegolithWallpaperApp::run(Settings::default())?;
    Ok(())
}

#[derive(Debug, Clone)]
enum Message {
    WallpaperPathMessage(WallpaperPathMessage),
}

struct RegolithWallpaperApp {
    wallpaper_paths: WallpaperPath,
}

impl Application for RegolithWallpaperApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (RegolithWallpaperApp, Command<Self::Message>) {
        (
            RegolithWallpaperApp {
                wallpaper_paths: WallpaperPath::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("regolith-wallpaper")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::WallpaperPathMessage(msg) => self
                .wallpaper_paths
                .update(msg)
                .map(Message::WallpaperPathMessage),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("Regolith wallpaper picker").size(20).font(Font {
            weight: Weight::Bold,
            ..Default::default()
        });
        let wallpaper_paths = self
            .wallpaper_paths
            .view()
            .map(Message::WallpaperPathMessage);

        container(column!(title, vertical_space(10), wallpaper_paths).spacing(10))
            .padding(20)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

#[derive(Debug, Clone)]
enum WallpaperPathMessage {
    InputEdit(String),
    Ok,
    Cancel,
}

/// Paths from which to load wallpaper images
struct WallpaperPath {
    input: String,
    path: Option<PathBuf>,
    status_bar: StatusBar,
}

impl WallpaperPath {
    fn new() -> Self {
        Self {
            input: String::new(),
            path: None,
            status_bar: StatusBar::None,
        }
    }

    fn update(&mut self, message: WallpaperPathMessage) -> Command<WallpaperPathMessage> {
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
                    self.status_bar = StatusBar::Ok(format!("Path setted on: {:?}", path));
                    self.path = Some(path);
                } else {
                    self.status_bar = StatusBar::Error(format!("Invalid path: {:?}", path));
                }
                Command::none()
            }
            WallpaperPathMessage::Cancel => todo!(),
        }
    }

    fn view(&self) -> Element<WallpaperPathMessage> {
        match self.path {
            Some(_) => todo!(),
            None => {
                let label = text("Wallpapers folder path:").size(16);

                let input = text_input("Enter folder path...", &self.input)
                    .on_input(WallpaperPathMessage::InputEdit);

                let button_ok = button(container(text("Ok").size(16)).width(100).center_x())
                    .padding(10)
                    .style(theme::Button::Positive)
                    .on_press(WallpaperPathMessage::Ok);
                let button_cancel =
                    button(container(text("Cancel").size(16)).width(100).center_x())
                        .padding(10)
                        .style(theme::Button::Destructive)
                        .on_press_maybe(self.path.as_ref().map(|_| WallpaperPathMessage::Cancel));
                let buttons =
                    row!(horizontal_space(Length::Fill), button_ok, button_cancel).spacing(10);

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
    }
}

enum StatusBar {
    None,
    Ok(String),
    Error(String),
}

impl StatusBar {
    fn view<T>(&self) -> Element<T> {
        match self {
            StatusBar::None => text(""),
            StatusBar::Ok(s) => text(s),
            StatusBar::Error(s) => text(s).style(Color::from_rgb(0.9, 0.2, 0.2)),
        }
        .into()
    }
}
