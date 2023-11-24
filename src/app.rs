use crate::{Error, Result, StatusBar, WallpaperImage, WallpaperPath, WallpaperPathMessage};
use futures::StreamExt;
use iced::font::Weight;
use iced::widget::{button, column, container, scrollable, text, vertical_space};
use iced::{executor, Font, Length};
use iced::{Application, Command, Element, Theme};
use iced_aw::Grid;
use image::ImageFormat;
use std::path::PathBuf;
use tokio::fs::read_dir;
use tokio_stream::wrappers::ReadDirStream;

#[derive(Debug, Clone)]
pub enum Message {
    WallpaperPathMessage(WallpaperPathMessage),
    WallpaperPathToogle {
        show: bool,
        msg: Option<Result<String>>,
    },
    WallpaperPathSetted,
    LoadedPaths(Result<Vec<PathBuf>>),
    LoadedImage(Result<WallpaperImage>),
    SelectImage(usize),
    UpdateStatusBar(Result<String>),
}

pub struct RegolithWallpaperApp {
    wallpaper_path: WallpaperPath,
    wallpaper_path_show: bool,
    images: Vec<WallpaperImage>,
    status_bar: StatusBar,
    limit: Option<usize>,
}

impl Application for RegolithWallpaperApp {
    type Executor = executor::Default;
    type Flags = Option<usize>;
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (RegolithWallpaperApp, Command<Self::Message>) {
        let wallpaper_path = WallpaperPath::new();
        let (wallpaper_path_show, cmd) = if wallpaper_path.path.is_some() {
            (false, Command::none())
        } else {
            (true, wallpaper_path.focus_input())
        };
        (
            RegolithWallpaperApp {
                wallpaper_path,
                wallpaper_path_show,
                images: Vec::new(),
                status_bar: StatusBar::None,
                limit: flags,
            },
            cmd,
        )
    }

    fn title(&self) -> String {
        String::from("regolith-wallpaper")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::WallpaperPathMessage(msg) => match self.wallpaper_path.update(msg) {
                Some(msg) => self.update(msg),
                None => Command::none(),
            },
            Message::WallpaperPathToogle { show, msg } => {
                self.wallpaper_path_show = show;
                if let Some(msg) = msg {
                    self.update(Message::UpdateStatusBar(msg))
                } else {
                    Command::none()
                }
            }
            Message::WallpaperPathSetted => {
                if let Some(path) = self.wallpaper_path.path.clone() {
                    let msg = Message::WallpaperPathToogle {
                        show: false,
                        msg: Some(Ok(format!("Path setted to {:?}", path))),
                    };
                    self.images.clear();
                    let cmd = self.update(msg);
                    Command::batch(vec![
                        cmd,
                        Command::perform(load_image_files(path), Message::LoadedPaths),
                    ])
                } else {
                    Command::none()
                }
            }
            Message::LoadedPaths(Ok(paths)) => {
                let commands = paths
                    .into_iter()
                    .filter(|path| {
                        if path.is_dir() {
                            return false;
                        }
                        match path.extension() {
                            Some(ext) => ImageFormat::from_extension(ext).is_some(),
                            None => false,
                        }
                    })
                    .enumerate()
                    .take(self.limit.unwrap_or(usize::MAX))
                    .map(|(i, path)| {
                        Command::perform(WallpaperImage::from_path(i, path), Message::LoadedImage)
                    })
                    .collect::<Vec<_>>();
                Command::batch(commands)
            }
            Message::LoadedPaths(Err(e)) => {
                self.status_bar = StatusBar::Error(e.to_string());
                Command::none()
            }
            Message::LoadedImage(Ok(image)) => {
                self.images.push(image);
                Command::none()
            }
            Message::LoadedImage(Err(e)) => {
                self.status_bar = StatusBar::Error(e.to_string());
                Command::none()
            }
            Message::SelectImage(id) => {
                self.images.iter_mut().for_each(|image| {
                    if image.id == id {
                        image.selected = !image.selected;
                    } else {
                        image.selected = false;
                    }
                });
                Command::none()
            }
            Message::UpdateStatusBar(result) => {
                match result {
                    Ok(success) => self.status_bar = StatusBar::Ok(success),
                    Err(e) => self.status_bar = StatusBar::Error(e.to_string()),
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("Regolith wallpaper picker").size(20).font(Font {
            weight: Weight::Bold,
            ..Default::default()
        });

        let mut content = column!(title, vertical_space(10)).spacing(25).padding(20);

        if self.wallpaper_path_show {
            let wallpaper_path = self
                .wallpaper_path
                .view()
                .map(Message::WallpaperPathMessage);
            content = content.push(wallpaper_path);
        } else {
            let edit_path_btn = button(container(text("Edit path").size(16)).width(100).center_x())
                .padding([5, 10])
                .on_press(Message::WallpaperPathToogle {
                    show: true,
                    msg: None,
                });
            content = content.push(edit_path_btn);
        }

        if !self.images.is_empty() {
            let images = Grid::with_children(
                self.images
                    .iter()
                    .map(|image| image.view())
                    .collect::<Vec<_>>(),
            )
            .strategy(iced_aw::Strategy::ColumnWidth(384.0));
            content = content
                .push(scrollable(container(images).width(Length::Fill).center_x()))
                .height(Length::FillPortion(9));
        }

        column!(
            content,
            vertical_space(Length::Fill),
            self.status_bar.view()
        )
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

#[tracing::instrument]
async fn load_image_files(path: PathBuf) -> Result<Vec<PathBuf>> {
    tracing::info!("Loading files...");
    let image_files = ReadDirStream::new(
        read_dir(path)
            .await
            .map_err(|e| Error::UnexpectedError(format!("Failed to read files {}", e)))?,
    )
    .filter_map(|res| async {
        match res {
            Ok(x) => Some(x.path()),
            Err(_) => None,
        }
    })
    .collect::<Vec<_>>()
    .await;
    tracing::info!("Files loaded...");
    Ok(image_files)
}
