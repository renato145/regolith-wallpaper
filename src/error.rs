#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    UnexpectedError(#[from] anyhow::Error),
}
