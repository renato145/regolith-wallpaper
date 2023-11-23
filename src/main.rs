use iced::{Application, Settings};
use regolith_wallpaper::RegolithWallpaperApp;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> iced::Result {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "iced=warn,regolith_wallpaper=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    RegolithWallpaperApp::run(Settings::default())?;
    Ok(())
}
