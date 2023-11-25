use std::path::PathBuf;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Faild to write regolith config file (~/.config/regolith3/Xresources).")]
    FailedWriteRegConfigFile,
    #[error("Regolith config file not found (~/.config/regolith3/Xresources).")]
    NoRegConfigFile,
    #[error("Failed to read regolith config file (~/.config/regolith3/Xresources).")]
    FailedReadRegConfigFile,
    #[error("No wallpaper setting (`regolith.wallpaper.file`) on config file (~/.config/regolith3/Xresources).")]
    NoWallpaperOnRegConfigFile,
    #[error("Failed to read file: {0}")]
    FailedToRead(PathBuf),
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
    #[error("{0}")]
    UnexpectedError(String),
}
