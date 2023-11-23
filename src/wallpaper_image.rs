use crate::{Error, Result};
use iced::{
    widget::{
        container,
        image::{self, Handle},
        Image,
    },
    Background, Color, ContentFit, Element,
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct WallpaperImage {
    image: Handle,
}

impl WallpaperImage {
    #[tracing::instrument]
    pub async fn from_path(path: PathBuf) -> Result<Self> {
        tracing::info!("Loading image...");
        let bytes = tokio::fs::read(&path).await.map_err(|e| {
            tracing::error!(error.cause_chain=?e, error.message=%e, "Failed to read file.");
            Error::FailedToRead(path)
        })?;
        let image = image::Handle::from_memory(bytes);
        tracing::info!("Image loaded.");
        Ok(Self { image })
    }

    pub fn view<'a, T: 'a>(&'a self) -> Element<T> {
        container(Image::new(self.image.clone()).content_fit(ContentFit::Contain))
            .style(|_: &_| container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.7, 0.2))),
                ..Default::default()
            })
            .width(360)
            .height(200)
            .center_x()
            .center_y()
            .into()
    }
}
