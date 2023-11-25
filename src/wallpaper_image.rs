use crate::{Error, Message, Result};
use iced::{
    widget::{container, image::Handle, mouse_area, Image},
    Color, Element,
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
        let now = std::time::Instant::now();
        let img = image::io::Reader::open(&path)
            .map_err(|e| Error::UnexpectedError(e.to_string()))?
            .decode()
            .map_err(|e| Error::UnexpectedError(e.to_string()))?
            .resize(360, 200, image::imageops::FilterType::Gaussian);
        let width = img.width();
        let height = img.height();
        let pixels = img.into_rgba8().into_raw();
        let image = iced::widget::image::Handle::from_pixels(width, height, pixels);
        tracing::info!(elapsed=?now.elapsed(), "Image loaded.");
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
            Color::from_rgb(0.741, 0.576, 0.976)
        } else {
            Color::TRANSPARENT
        };
        mouse_area(
            container(
                Image::new(self.image.clone()), // .content_fit(ContentFit::Contain)
            )
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
