use iced::font::Weight;
use iced::widget::{column, container, image, text, text_input, vertical_space, Column};
use iced::{executor, theme, Font};
use iced::{Application, Command, Element, Settings, Theme};
use std::fs;
use std::path::PathBuf;
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
            Message::WallpaperPathMessage(msg) => self.wallpaper_paths.update(msg),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("Regolith wallpaper picker").size(20).font(Font {
            weight: Weight::Bold,
            ..Default::default()
        });

        container(column!(title, vertical_space(10), self.wallpaper_paths.view()).spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

#[derive(Debug, Clone)]
enum WallpaperPathMessage {
    WallpaperInputEdit(String),
}
/// Paths from which to load wallpaper images
struct WallpaperPath {
    input: String,
    path: Option<PathBuf>,
}

impl WallpaperPath {
    fn new() -> Self {
        Self {
            input: String::new(),
            path: None,
        }
    }

    fn update(&mut self, message: WallpaperPathMessage) -> Command<Message> {
        match message {
            WallpaperPathMessage::WallpaperInputEdit(input) => {
                self.input = input;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.path {
            Some(_) => todo!(),
            None => {
                let label = text("Wallpapers folder path:").size(16);
                let input = text_input("Enter folder path...", &self.input).on_input(|x| {
                    Message::WallpaperPathMessage(WallpaperPathMessage::WallpaperInputEdit(x))
                });
                column!(label, input).into()
            }
        }
    }
}
