use super::Result;
use crate::Error;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Configuration {
    pub wallpapers_path: Option<PathBuf>,
    pub max_images: Option<usize>,
}

impl Configuration {
    fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = read_to_string(path).map_err(|e| Error::UnexpectedError(e.to_string()))?;
        serde_yaml::from_str(&content).map_err(|e| Error::UnexpectedError(e.to_string()))
    }
}

pub fn get_configuration_path() -> Result<PathBuf> {
    let path = BaseDirs::new()
        .ok_or(Error::NoHomeDir)?
        .config_dir()
        .join("regolith-wallpaper/config.yaml");
    Ok(path)
}

pub fn get_configuration() -> Result<Configuration> {
    let path = get_configuration_path()?;

    if path.exists() {
        return Configuration::from_file(&path);
    }

    tracing::info!("Config not found, creating a default one...");
    create_dir_all(path.parent().unwrap()).map_err(|e| {
        tracing::error!(error.cause_chain=?e, error.message=%e, ?path, "Failed to create folder.");
        Error::UnexpectedError(e.to_string())
    })?;
    let config = Configuration::default();
    let content = serde_yaml::to_string(&config).map_err(|e| {
        tracing::error!(error.cause_chain=?e, error.message=%e, "Failed to serialize content.");
        Error::UnexpectedError(e.to_string())
    })?;
    write(&path, content).map_err(|e| {
        tracing::error!(error.cause_chain=?e, error.message=%e, ?path, "Failed to path file.");
        Error::FailedToWriteFile(path.clone())
    })?;
    tracing::info!("Wrote config file to: {:?}", path);
    Ok(config)
}
