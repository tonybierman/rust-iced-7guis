use iced::alignment::Vertical;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Settings};

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .settings(Settings {
            antialiasing: true,
            ..Default::default()
        })
        .window_size((200, 80))
        .centered()
        .resizable(false)
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

#[derive(Default)]
struct App {
}