use clap::Parser;
use iced::{Application, Settings};
use regolith_wallpaper::{commands, get_configuration, RegolithWallpaperApp};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// regolith-wallpaper
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Max number of images to load
    #[arg(short, long)]
    max_images: Option<usize>,
    /// Pick a random wallpaper
    #[arg(short, long)]
    random_pick: bool,
}

fn main() -> iced::Result {
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "iced=warn,regolith_wallpaper=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut configuration = get_configuration().expect("Failed to get configuration.");
    if let Some(max_images) = args.max_images {
        configuration.max_images = Some(max_images);
    }
    tracing::info!("Loaded configuration: {:#?}", configuration);

    if args.random_pick {
        commands::pick_random_image(configuration).unwrap();
        return Ok(());
    }

    RegolithWallpaperApp::run(Settings {
        flags: configuration,
        ..Default::default()
    })?;
    Ok(())
}
