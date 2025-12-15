use iced::alignment::Vertical;
use iced::widget::{button, column, container, row, text};
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
    departure_input: Option<String>,
    departure_error: Option<String>,
    return_date: Option<NaiveDate>,
    return_date_input: Option<String>,
    return_date_error: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            departure: Some(Local::now().date_naive()),
            departure_input: None,
            departure_error: None,
            return_date: Some(Local::now().date_naive()),
            return_date_input: None,
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
            },
            Message::ReturnDateChanged(v) => {
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {

        container(column![
            text("Flight Booking Application")
                .size(24)
            ].padding(20))
            .center_x(iced::Length::Fill)
            .into()
    }

    fn validate_date(input: &str) -> Result<NaiveDate, String> {
        NaiveDate::parse_from_str(input, "%Y-%m-%d")
            .map_err(|_| "Invalid date format. Use YYYY-MM-DD".to_string())
    }

    fn validate_today_or_future_date(date: NaiveDate) -> Result<(), String> {
        let today = Local::now().date_naive();
        
        if date >= today {
            Ok(())
        } else {
            Err("Date must be in the future".to_string())
        }
    }
}
