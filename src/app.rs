use crate::{WallpaperPath, WallpaperPathMessage};
use iced::font::Weight;
use iced::widget::{self, column, container, text, vertical_space};
use iced::{event, executor, keyboard, Event, Font, Subscription};
use iced::{Application, Command, Element, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    WallpaperPathMessage(WallpaperPathMessage),
}

pub struct RegolithWallpaperApp {
    wallpaper_path: WallpaperPath,
}

impl Application for RegolithWallpaperApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (RegolithWallpaperApp, Command<Self::Message>) {
        let wallpaper_path = WallpaperPath::new();
        let cmd = if wallpaper_path.path.is_some() {
            Command::none()
        } else {
            wallpaper_path.focus_input()
        };
        (RegolithWallpaperApp { wallpaper_path }, cmd)
    }

    fn title(&self) -> String {
        String::from("regolith-wallpaper")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen().map(Message::Event)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::WallpaperPathMessage(msg) => self.wallpaper_path.update(msg),
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Tab,
                    modifiers,
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                _ => Command::none(),
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("Regolith wallpaper picker").size(20).font(Font {
            weight: Weight::Bold,
            ..Default::default()
        });
        let wallpaper_paths = self
            .wallpaper_path
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
