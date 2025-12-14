use iced::alignment::Vertical;
use iced::widget::{column, container, row, text, text_input};
use iced::{Element, Settings, Fill};

pub fn main() -> iced::Result {
    iced::application(Calculator::new, Calculator::update, Calculator::view)
        .settings(Settings {
            ..Default::default()
        })
        .window_size((400, 120))
        .centered()
        .resizable(false)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    CelsiusChanged(String),
    FahrenheitChanged(String),
}

#[derive(Debug, Clone)]
pub struct Calculator {
    celsius: Option<f32>,
    fahrenheit: Option<f32>,
    celsius_input: String,
    fahrenheit_input: String,
}

impl Calculator {
    fn new() -> (Self, iced::Task<Message>) {
        (
            Calculator {
                celsius: Some(0.0),
                fahrenheit: Some(32.0),
                celsius_input: "0".to_string(),
                fahrenheit_input: "32".to_string(),
            },
            iced::Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CelsiusChanged(value) => {
                self.celsius_input = value.clone();
                self.celsius = value.parse().ok();
                self.fahrenheit = self.celsius.map(|c| c * (9.0 / 5.0) + 32.0);
                if let Some(f) = self.fahrenheit {
                    self.fahrenheit_input = format!("{:.2}", f);
                }
            }
            Message::FahrenheitChanged(value) => {
                self.fahrenheit_input = value.clone();
                self.fahrenheit = value.parse().ok();
                self.celsius = self.fahrenheit.map(|f| (f - 32.0) * (5.0 / 9.0));
                if let Some(c) = self.celsius {
                    self.celsius_input = format!("{:.2}", c);
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let row = row![
            text("Celsius:"),
            text_input("0", &self.celsius_input)
                .on_input(Message::CelsiusChanged)
                .width(80),
            text("Fahrenheit:"),
            text_input("32", &self.fahrenheit_input)
                .on_input(Message::FahrenheitChanged)
                .width(80),
        ]
        .spacing(10)
        .align_y(Vertical::Center);

        container(column![row].padding(20))
            .center_x(iced::Length::Fill)
            .into()
    }
}