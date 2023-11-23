use crate::{Result, StatusBar, WallpaperPath, WallpaperPathMessage};
use iced::font::Weight;
use iced::widget::{self, button, column, container, text, vertical_space};
use iced::{event, executor, keyboard, Event, Font, Length, Subscription};
use iced::{Application, Command, Element, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    WallpaperPathMessage(WallpaperPathMessage),
    WallpaperPathToogle {
        show: bool,
        msg: Option<Result<String>>,
    },
    UpdateStatusBar(Result<String>),
}

pub struct RegolithWallpaperApp {
    wallpaper_path: WallpaperPath,
    wallpaper_path_show: bool,
    status_bar: StatusBar,
}

impl Application for RegolithWallpaperApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (RegolithWallpaperApp, Command<Self::Message>) {
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
                status_bar: StatusBar::None,
            },
            cmd,
        )
    }

    fn title(&self) -> String {
        String::from("regolith-wallpaper")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen().map(Message::Event)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
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

        let mut content = column!(title, vertical_space(10)).spacing(10).padding(20);

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
