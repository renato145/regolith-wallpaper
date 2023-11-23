use crate::{Error, Message, Result};
use iced::{
    widget::{
        container,
        image::{self, Handle},
        mouse_area, Image,
    },
    Color, ContentFit, Element,
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct WallpaperImage {
    pub id: usize,
    pub path: PathBuf,
    pub image: Handle,
    pub selected: bool,
}

impl WallpaperImage {
    #[tracing::instrument]
    pub async fn from_path(id: usize, path: PathBuf) -> Result<Self> {
        tracing::info!("Loading image...");
        let bytes = tokio::fs::read(&path).await.map_err(|e| {
            tracing::error!(error.cause_chain=?e, error.message=%e, "Failed to read file.");
            Error::FailedToRead(path.clone())
        })?;
        let image = image::Handle::from_memory(bytes);
        tracing::info!("Image loaded.");
        Ok(Self {
            id,
            path,
            image,
            selected: false,
        })
    }

    pub fn select(&mut self) -> PathBuf {
        self.path.clone()
    }

    pub fn view(&self) -> Element<Message> {
        let border_color = if self.selected {
            Color::from_rgb(0.1, 0.75, 0.3)
        } else {
            Color::TRANSPARENT
        };
        mouse_area(
            container(Image::new(self.image.clone()).content_fit(ContentFit::Contain))
                .width(360)
                .height(200)
                .center_x()
                .center_y()
                .padding(10)
                .style(move |_: &_| container::Appearance {
                    border_width: 2.0,
                    border_color,
                    ..Default::default()
                }),
        )
        .on_press(Message::SelectImage(self.id))
        .into()
    }
}
