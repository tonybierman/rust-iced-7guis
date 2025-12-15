use iced::alignment::Vertical;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Settings};
use chrono::{Local, NaiveDate};

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .settings(Settings {
            antialiasing: true,
            ..Default::default()
        })
        .window_size((200, 240))
        .centered()
        .resizable(false)
        .run()
}

#[derive(Debug, Clone)]
enum Flight {
    OneWay,
    Return,
}

#[derive(Debug, Clone)]
enum Message {
    FlightSelected(Flight),
    DepartureChanged(String),
    ReturnDateChanged(String),
}

struct App {
    departure: Option<NaiveDate>,
    departure_input: String,
    departure_error: Option<String>,
    return_date: Option<NaiveDate>,
    return_date_input: String,
    return_date_error: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            departure: Some(Local::now().date_naive()),
            departure_input: "".to_string(),
            departure_error: None,
            return_date: Some(Local::now().date_naive()),
            return_date_input: "".to_string(),
            return_date_error: None,
        }
    }
}

impl App {
    fn new() -> (Self, iced::Task<Message>) {
        (App::default(), iced::Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::FlightSelected(f) => {
                match f {
                    Flight::OneWay => {
                    },
                    Flight::Return => {
                    },
                }
            },
            Message::DepartureChanged(v) => {
                self.departure_input = v.clone();
                match App::validate_date(&v) {
                    Ok(date) => {
                        self.departure = Some(date);
                        self.departure_error = App::validate_at_least(date, Local::now().date_naive()).err();
                    }
                    Err(parse_error) => {
                        self.departure = None;
                        self.departure_error = Some(parse_error);
                    }
                }
            },
            Message::ReturnDateChanged(v) => {
                self.return_date_input = v.clone();
                match App::validate_date(&v) {
                    Ok(date) => {
                        self.return_date = Some(date);
                        self.return_date_error = match self.departure {
                            Some(departure_date) => App::validate_at_least(date, departure_date).err(),
                            None => Some("Please select a departure date first".to_string()),
                        };
                    }
                    Err(parse_error) => {
                        self.return_date = None;
                        self.return_date_error = Some(parse_error);
                    }
                }
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {

        container(column![
            text_input("Departure", &self.departure_input)
                .on_input(Message::DepartureChanged)
                .width(160),
            text(
                self.departure_error
                    .as_ref()
                    .map_or("", |e| e.as_str())
            ),
            text_input("Return", &self.return_date_input)
                .on_input(Message::ReturnDateChanged)
                .width(160),
            text(
                self.return_date_error
                    .as_ref()
                    .map_or("", |e| e.as_str())
            )
            ].padding(20))
            .center_x(iced::Length::Fill)
            .into()
    }

    fn validate_date(input: &str) -> Result<NaiveDate, String> {
        NaiveDate::parse_from_str(input, "%Y-%m-%d")
            .map_err(|_| "Use YYYY-MM-DD".to_string())
    }

    fn validate_at_least(date: NaiveDate, compare_date: NaiveDate) -> Result<(), String> {
        if date >= compare_date {
            Ok(())
        } else {
            Err(format!("Must be at least {}", compare_date))
        }
    }
}
