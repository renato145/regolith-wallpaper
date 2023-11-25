use crate::{
    expand_home_dir, Error, Result, StatusBar, WallpaperImage, WallpaperPath, WallpaperPathMessage,
};
use futures::StreamExt;
use iced::font::Weight;
use iced::widget::{button, column, container, scrollable, text, vertical_space};
use iced::{executor, Font, Length};
use iced::{Application, Command, Element, Theme};
use iced_aw::Grid;
use image::ImageFormat;
use std::path::PathBuf;
use tokio::fs::{read_dir, read_to_string, write};
use tokio_stream::wrappers::ReadDirStream;

#[derive(Debug, Clone)]
pub enum Message {
    CurrentWallpaperPath(Result<PathBuf>),
    CurrentWallpaperImage(Result<WallpaperImage>),
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
    current_wallpaper: Option<WallpaperImage>,
    current_wallpaper_error: Option<String>,
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
        let (wallpaper_path_show, focus_cmd) = if wallpaper_path.path.is_some() {
            (false, Command::none())
        } else {
            (true, wallpaper_path.focus_input())
        };
        let load_regolith_config_cmd =
            Command::perform(load_regolith_config(), Message::CurrentWallpaperPath);
        (
            RegolithWallpaperApp {
                current_wallpaper: None,
                current_wallpaper_error: None,
                wallpaper_path,
                wallpaper_path_show,
                images: Vec::new(),
                status_bar: StatusBar::None,
                limit: flags,
            },
            Command::batch(vec![focus_cmd, load_regolith_config_cmd]),
        )
    }

    fn title(&self) -> String {
        String::from("regolith-wallpaper")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::CurrentWallpaperPath(Ok(path)) => Command::perform(
                WallpaperImage::from_path(0, path),
                Message::CurrentWallpaperImage,
            ),
            Message::CurrentWallpaperPath(Err(e)) => {
                tracing::error!(error.cause_chain=?e, error.message=%e, "Failed to get wallpaper path from current regolith configuration.");
                self.current_wallpaper_error = Some(e.to_string());
                Command::none()
            }
            Message::CurrentWallpaperImage(Ok(image)) => {
                self.current_wallpaper = Some(image);
                Command::none()
            }
            Message::CurrentWallpaperImage(Err(e)) => {
                tracing::error!(error.cause_chain=?e, error.message=%e, "Failed to load image from wallpaper path from current regolith configuration.");
                self.current_wallpaper_error = Some(e.to_string());
                Command::none()
            }
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
                if let Some(image) = self.images.iter().find(|image| image.id == id) {
                    Command::perform(
                        set_wallpaper_on_config(image.path.clone()),
                        Message::CurrentWallpaperPath,
                    )
                } else {
                    Command::none()
                }
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

        let mut content = column!(title).spacing(25).padding(20);

        if self.wallpaper_path_show {
            let wallpaper_path = self
                .wallpaper_path
                .view()
                .map(Message::WallpaperPathMessage);
            content = content.push(wallpaper_path);
        } else {
            let edit_path_btn = button(
                container(text("Edit wallpapers path").size(14))
                    .width(200)
                    .center_x(),
            )
            .padding([2, 4])
            .on_press(Message::WallpaperPathToogle {
                show: true,
                msg: None,
            });
            content = content.push(edit_path_btn);

            if let Some(image) = &self.current_wallpaper {
                content = content.push(column!(text("Current wallpaper"), image.view()).spacing(4));
            }
            if let Some(e) = &self.current_wallpaper_error {
                content = content.push(text(e));
            }
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
            Ok(x) => {
                let path = x.path();
                if path.is_dir() {
                    None
                } else {
                    path.extension()
                        .and_then(ImageFormat::from_extension)
                        .map(|_| path)
                }
            }
            Err(_) => None,
        }
    })
    .collect::<Vec<_>>()
    .await;
    tracing::info!("{} files loaded.", image_files.len());
    Ok(image_files)
}

async fn read_regolith_config() -> Result<String> {
    let path = expand_home_dir("~/.config/regolith3/Xresources");
    if !path.exists() {
        return Err(Error::NoRegConfigFile);
    }
    read_to_string(path).await.map_err(|e| {
        tracing::error!(error.cause_chain=?e, error.message=%e, "Failed to read file.");
        Error::FailedReadRegConfigFile
    })
}

async fn load_regolith_config() -> Result<PathBuf> {
    read_regolith_config()
        .await?
        .lines()
        .find(|line| line.starts_with("regolith.wallpaper.file:"))
        .ok_or(Error::NoWallpaperOnRegConfigFile)?
        .split(':')
        .nth(1)
        .ok_or(Error::NoWallpaperOnRegConfigFile)
        .map(|path| expand_home_dir(path.trim()))
}

/// Sets the path on the current regolith config file, if success returns the
/// setted image path
async fn set_wallpaper_on_config(path: PathBuf) -> Result<PathBuf> {
    let content = read_regolith_config().await?;
    let mut lines = content.lines();
    let mut new_content = String::new();
    for line in &mut lines {
        if line.starts_with("regolith.wallpaper.file:") {
            let path_str = path.to_str().unwrap().to_string();
            new_content.push_str(&format!("regolith.wallpaper.file: {}\n", path_str));
            break;
        } else {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }
    new_content.push_str(&lines.collect::<Vec<_>>().join("\n"));
    let config_path = expand_home_dir("~/.config/regolith3/Xresources");
    write(config_path, new_content).await.map_err(|e| {
        tracing::error!(error.cause_chain=?e, error.message=%e, "Failed to write file.");
        Error::FailedWriteRegConfigFile
    })?;
    let exit_status = tokio::process::Command::new("/usr/bin/regolith-look")
        .arg("refresh")
        .status()
        .await
        .map_err(|e| {
            tracing::error!(error.cause_chain=?e, error.message=%e);
            Error::FailedToRunRefresh
        })?;
    if exit_status.success() {
        Ok(path)
    } else {
        tracing::error!(
            "Failed to run command, exited with code: {:?}",
            exit_status.code()
        );
        Err(Error::FailedToRunRefresh)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn load_regolith_config_works() {
        let res = load_regolith_config().await;
        println!("{:?}", res);
    }
}
