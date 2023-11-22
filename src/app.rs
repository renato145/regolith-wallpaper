use crate::{WallpaperPath, WallpaperPathMessage};
use iced::font::Weight;
use iced::widget::{column, container, text, vertical_space};
use iced::{executor, Font};
use iced::{Application, Command, Element, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    WallpaperPathMessage(WallpaperPathMessage),
}

pub struct RegolithWallpaperApp {
    wallpaper_paths: WallpaperPath,
}

impl Application for RegolithWallpaperApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (RegolithWallpaperApp, Command<Self::Message>) {
        (
            RegolithWallpaperApp {
                wallpaper_paths: WallpaperPath::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("regolith-wallpaper")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::WallpaperPathMessage(msg) => self.wallpaper_paths.update(msg),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("Regolith wallpaper picker").size(20).font(Font {
            weight: Weight::Bold,
            ..Default::default()
        });
        let wallpaper_paths = self
            .wallpaper_paths
            .view()
            .map(Message::WallpaperPathMessage);

        container(column!(title, vertical_space(10), wallpaper_paths).spacing(10))
            .padding(20)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
