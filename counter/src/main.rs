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
    Increment,
}

#[derive(Default)]
struct App {
    value: i64,
}

impl App {
    fn new() -> (Self, iced::Task<Message>) {
        (App { value: 0 }, iced::Task::none())
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

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;

    use iced::Settings;
    use iced_test::{Error, Simulator};

    fn simulator(app: &App) -> Simulator<'_, Message> {
        Simulator::with_settings(
            Settings {
                ..Settings::default()
            },
            app.view(),
        )
    }

    #[test]
    fn it_counts() -> Result<(), Error> {
        let mut counter = App { value: 0 };
        let mut ui = simulator(&counter);

        let _ = ui.click("Increment")?;
        let _ = ui.click("Increment")?;

        for message in ui.into_messages() {
            counter.update(message);
        }

        assert_eq!(counter.value, 2);

        let mut ui = simulator(&counter);
        assert!(ui.find("2").is_ok(), "Counter should display 2!");

        Ok(())
    }

    #[test]
    #[allow(unused_variables)]
    fn test_counter_new() {
        let (counter, task) = App::new();
        assert_eq!(counter.value, 0);
        // Verify task is none (though we can't directly test Task internals easily)
    }

    #[test]
    fn test_counter_default() {
        let counter = App::default();
        assert_eq!(counter.value, 0);
    }

    #[test]
    fn test_single_increment() {
        let mut counter = App { value: 0 };
        counter.update(Message::Increment);
        assert_eq!(counter.value, 1);
    }

    #[test]
    fn test_multiple_increments() {
        let mut counter = App { value: 0 };
        for _ in 0..10 {
            counter.update(Message::Increment);
        }
        assert_eq!(counter.value, 10);
    }

    #[test]
    fn test_increment_from_non_zero() {
        let mut counter = App { value: 42 };
        counter.update(Message::Increment);
        assert_eq!(counter.value, 43);
    }

    #[test]
    fn test_message_debug() {
        let msg = Message::Increment;
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("Increment"));
    }
}
