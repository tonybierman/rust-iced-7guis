use iced::alignment::Vertical;
use iced::widget::{button, column, container, pick_list, row, text, text_input};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flight {
    OneWay,
    Return,
}

impl std::fmt::Display for Flight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flight::OneWay => write!(f, "One-way flight"),
            Flight::Return => write!(f, "Return flight"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    FlightSelected(Flight),
    DepartureChanged(String),
    ReturnDateChanged(String),
}

struct App {
    flight_type: Flight,
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
            flight_type: Flight::OneWay,
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
                self.flight_type = f;
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
        let return_enabled = self.flight_type == Flight::Return 
            && !self.departure_input.is_empty()
            && self.departure.is_some() 
            && self.departure_error.is_none();

        let return_input = if return_enabled {
            text_input("Return", &self.return_date_input)
                .on_input(Message::ReturnDateChanged)
                .width(160)
        } else {
            text_input("Return", &self.return_date_input)
                .width(160)
        };

        container(column![
            pick_list(
                &[Flight::OneWay, Flight::Return][..],
                Some(self.flight_type),
                Message::FlightSelected
            )
            .width(160),
            text_input("Departure", &self.departure_input)
                .on_input(Message::DepartureChanged)
                .width(160),
            text(
                self.departure_error
                    .as_ref()
                    .map_or("", |e| e.as_str())
            ),
            return_input,
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