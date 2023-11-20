use iced::executor;
use iced::widget::{column, container, image, text};
use iced::{Application, Command, Element, Settings, Theme};
use std::fs;
use tracing_subscriber::fmt;

fn main() -> iced::Result {
    fmt::init();
    tracing::info!("Started");
    RegolithWallpaperApp::run(Settings::default())?;
    tracing::info!("Finished");
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
                "/home/renato/Dropbox/variety-favorites/OHR.MaasaiGiraffe_EN-US4914727610_1920x1080.jpg",
            )
            .expect("Failed to read image.");
            let image_data = image::Handle::from_memory(bytes);
            tracing::info!("About to get handle");
            image::viewer(image_data)
        };
        tracing::info!("Finished to get handle");
        container(column!(body, sample_pic)).padding(10).into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
