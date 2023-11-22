use std::path::PathBuf;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
    #[error("{0}")]
    UnexpectedError(String),
}
