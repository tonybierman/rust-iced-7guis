use iced::alignment::Vertical;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Settings};

pub fn main() -> iced::Result {
    iced::application(Counter::new, Counter::update, Counter::view)
        .settings(Settings {
            ..Default::default()
        })
        .window_size((200, 80))
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: i64,
}

impl Counter {
    fn new() -> (Self, iced::Task<Message>) {
        (Counter { value: 0 }, iced::Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Note: return type changed to Element
        let row = row![
            text(self.value).size(24),
            button("Increment").on_press(Message::Increment),
        ]
        .spacing(10)
        .align_y(Vertical::Center);

        container(column![row].padding(20))
            .center_x(iced::Length::Fill)
            .into()
    }
}
