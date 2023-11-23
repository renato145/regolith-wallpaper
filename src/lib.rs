mod app;
mod error;
mod status_bar;
mod wallpaper_image;
mod wallpaper_path;

pub use app::*;
pub use error::*;
pub use status_bar::*;
pub use wallpaper_image::*;
pub use wallpaper_path::*;

pub type Result<T> = std::result::Result<T, Error>;
