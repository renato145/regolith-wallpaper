use iced::{
    widget::{
        container,
        image::{self, Handle},
        Image,
    },
    Background, Color, ContentFit, Element,
};
use std::path::PathBuf;

pub struct WallpaperImage {
    image: Handle,
}

impl WallpaperImage {
    #[tracing::instrument]
    pub fn from_path(path: PathBuf) -> Self {
        tracing::info!("Loading image...");
        let image = image::Handle::from_path(path);
        tracing::info!("Image loaded.");
        Self { image }
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
