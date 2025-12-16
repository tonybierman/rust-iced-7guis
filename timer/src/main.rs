use iced::alignment::Vertical;
use iced::time::{self, milliseconds};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Settings, Subscription} ;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
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
    Increment,
}

#[derive(Default)]
struct App {
    value: f32,
}

impl App {
    fn new() -> (Self, iced::Task<Message>) {
        (App { value: 0.0 }, iced::Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 0.05;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {

        container(
            column![
                text(format!("{:.2}", self.value))
                    .size(24)
            ]
            .padding(20),
        )
        .center_x(iced::Length::Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(milliseconds(50)).map(|_| Message::Increment)
    }

}