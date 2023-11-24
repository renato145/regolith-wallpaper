use iced::{Application, Settings};
use regolith_wallpaper::RegolithWallpaperApp;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> iced::Result {
    let limit = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "iced=warn,regolith_wallpaper=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    RegolithWallpaperApp::run(Settings {
        flags: limit,
        ..Default::default()
    })?;
    Ok(())
}
