use iced::alignment::Vertical;
use iced::widget::{column, container, row, text, text_input};
use iced::{Element, Settings};

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .settings(Settings {
            antialiasing: true,
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

#[derive(Debug, Clone, Default)]
pub struct App {
    celsius: Option<f32>,
    fahrenheit: Option<f32>,
    celsius_input: String,
    fahrenheit_input: String,
}

impl App {
    fn new() -> (Self, iced::Task<Message>) {
        (
            App {
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
                    println!("Celsius: {}", c);
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let row = row![
            text("Celsius:"),
            text_input("0", &self.celsius_input)
                .id("celsius-input")
                .on_input(Message::CelsiusChanged)
                .width(80),
            text("Fahrenheit:"),
            text_input("32", &self.fahrenheit_input)
                .id("farenheit-input")
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

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;
    use iced::Settings;
    use iced_test::selector::id;
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
    fn ui_calc_farenheit_to_celsius() -> Result<(), Error> {
        let mut app = App {
            celsius: None,
            fahrenheit: None,
            celsius_input: String::new(),
            fahrenheit_input: String::new(),
        };

        let mut ui = simulator(&app);

        let _input = ui.click(id("farenheit-input"))?;
        let _ = ui.typewrite("32");

        for message in ui.into_messages() {
            app.update(message);
        }

        assert_eq!(app.fahrenheit, Some(32.0));
        assert_eq!(app.celsius, Some(0.0));

        Ok(())
    }

    #[test]
    fn ui_calc_celsius_to_farenheit() -> Result<(), Error> {
        let (mut app, _command) = App::new();

        let mut ui = simulator(&app);

        let _input = ui.click(id("celsius-input"))?;
        let _ = ui.typewrite("30");

        for message in ui.into_messages() {
            app.update(message);
        }

        assert!(app.fahrenheit == Some(86.0));

        Ok(())
    }

    #[test]
    #[allow(unused_variables)]
    fn test_calculator_new() {
        let (calculator, task) = App::new();
        assert_eq!(calculator.celsius, Some(0.0));
        assert_eq!(calculator.fahrenheit, Some(32.0));
        // Verify task is none (though we can't directly test Task internals easily)
    }

    #[test]
    fn test_from_celsius() {
        let mut calculator = App {
            celsius_input: "20".to_string(),
            ..App::default()
        };
        calculator.update(Message::CelsiusChanged("20".to_string()));
        assert_eq!(calculator.fahrenheit, Some(68.0));
    }

    #[test]
    fn test_from_farenheit() {
        let mut calculator = App {
            fahrenheit_input: "86".to_string(),
            ..App::default()
        };
        calculator.update(Message::FahrenheitChanged("86".to_string()));
        assert_eq!(calculator.celsius, Some(30.000002));
    }
}
