use iced::{Application, Settings};
use regolith_wallpaper::RegolithWallpaperApp;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    RegolithWallpaperApp::run(Settings::default())?;
    Ok(())
}
