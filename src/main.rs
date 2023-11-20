use iced::executor;
use iced::widget::{column, container, image, text};
use iced::{Application, Command, Element, Settings, Theme};
use std::fs;
use std::path::PathBuf;
use tracing_subscriber::fmt;

fn main() -> iced::Result {
    fmt::init();
    RegolithWallpaperApp::run(Settings::default())?;
    Ok(())
}

struct RegolithWallpaperApp;

impl Application for RegolithWallpaperApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (RegolithWallpaperApp, Command<Self::Message>) {
        (RegolithWallpaperApp, Command::none())
    }

    fn title(&self) -> String {
        String::from("regolith-wallpaper")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let body = text("Hello there (:");
        let sample_pic = {
            let bytes = fs::read(
                "/home/renato/Dropbox/variety-favorites/OHR.MaasaiGiraffe_EN-US4914727610_1920x1080.jpg"
            )
            .expect("Failed to read image.");
            let image_data = image::Handle::from_memory(bytes);
            tracing::warn!("{:?}", image_data.data());
            image::viewer(image_data)
        };
        container(column!(body, sample_pic)).padding(10).into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

/// Paths from which to load wallpaper images
struct WallpaperPaths {
    path: Option<PathBuf>,
}

impl WallpaperPaths {
    fn load() -> Self {
        Self { path: None }
    }
}
