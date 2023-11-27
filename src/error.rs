use std::path::PathBuf;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to run `regolith-look refresh`.")]
    FailedToRunRefresh,
    #[error("Regolith config file not found (~/.config/regolith3/Xresources).")]
    NoRegConfigFile,
    #[error("Failed to read regolith config file (~/.config/regolith3/Xresources).")]
    FailedReadRegConfigFile,
    #[error("No wallpaper setting (`regolith.wallpaper.file`) on config file (~/.config/regolith3/Xresources).")]
    NoWallpaperOnRegConfigFile,
    #[error("Failed to read file: {0}")]
    FailedToRead(PathBuf),
    #[error("Failed to write file: {0}")]
    FailedToWriteFile(PathBuf),
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
    #[error("No home directory found.")]
    NoHomeDir,
    #[error("{0}")]
    UnexpectedError(String),
}
