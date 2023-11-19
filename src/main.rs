use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};
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
        "Hello, world!".into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
