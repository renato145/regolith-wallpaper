use iced::{Application, Settings};
use regolith_wallpaper::{get_configuration, RegolithWallpaperApp};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> iced::Result {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "iced=warn,regolith_wallpaper=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut configuration = get_configuration().expect("Failed to get configuration.");
    if let Some(max_images) = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
    {
        configuration.max_images = Some(max_images);
    }
    tracing::info!("Loaded configuration: {:#?}", configuration);

    RegolithWallpaperApp::run(Settings {
        flags: configuration,
        ..Default::default()
    })?;
    Ok(())
}
