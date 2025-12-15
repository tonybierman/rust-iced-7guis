use chrono::{Local, NaiveDate};
use iced::widget::{button, column, container, pick_list, text, text_input};
use iced::{Element, Settings};

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
    BookFlight,
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
            departure_input: Local::now().date_naive().format("%Y-%m-%d").to_string(),
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
            }
            Message::DepartureChanged(v) => {
                self.departure_input = v.clone();
                match App::validate_date(&v) {
                    Ok(date) => {
                        self.departure = Some(date);
                        self.departure_error =
                            App::validate_at_least(date, Local::now().date_naive()).err();
                    }
                    Err(parse_error) => {
                        self.departure = None;
                        self.departure_error = Some(parse_error);
                    }
                }
            }
            Message::ReturnDateChanged(v) => {
                self.return_date_input = v.clone();
                match App::validate_date(&v) {
                    Ok(date) => {
                        self.return_date = Some(date);
                        self.return_date_error = match self.departure {
                            Some(departure_date) => {
                                App::validate_at_least(date, departure_date).err()
                            }
                            None => Some("Please select a departure date first".to_string()),
                        };
                    }
                    Err(parse_error) => {
                        self.return_date = None;
                        self.return_date_error = Some(parse_error);
                    }
                }
            }
            Message::BookFlight => {
                // Clear the form
                *self = App::default();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let return_enabled = self.flight_type == Flight::Return
            && !self.departure_input.is_empty()
            && self.departure.is_some()
            && self.departure_error.is_none();

        // Check if form is valid
        let is_form_valid = match self.flight_type {
            Flight::OneWay => {
                !self.departure_input.is_empty()
                    && self.departure.is_some()
                    && self.departure_error.is_none()
            }
            Flight::Return => {
                !self.departure_input.is_empty()
                    && self.departure.is_some()
                    && self.departure_error.is_none()
                    && !self.return_date_input.is_empty()
                    && self.return_date.is_some()
                    && self.return_date_error.is_none()
            }
        };

        let return_input = if return_enabled {
            text_input("Return", &self.return_date_input)
                .on_input(Message::ReturnDateChanged)
                .width(160)
        } else {
            text_input("Return", &self.return_date_input).width(160)
        };

        let book_button = if is_form_valid {
            button("Book it!").on_press(Message::BookFlight).width(160)
        } else {
            button("Book it!").width(160)
        };

        container(
            column![
                container(
                    pick_list(
                        &[Flight::OneWay, Flight::Return][..],
                        Some(self.flight_type),
                        Message::FlightSelected
                    )
                    .width(160)
                )
                .padding(iced::padding::bottom(20)),
                text_input("Departure", &self.departure_input)
                    .on_input(Message::DepartureChanged)
                    .width(160),
                text(self.departure_error.as_ref().map_or("", |e| e.as_str())),
                return_input,
                text(self.return_date_error.as_ref().map_or("", |e| e.as_str())),
                book_button
            ]
            .padding(20),
        )
        .center_x(iced::Length::Fill)
        .into()
    }

    fn validate_date(input: &str) -> Result<NaiveDate, String> {
        NaiveDate::parse_from_str(input, "%Y-%m-%d").map_err(|_| "Use YYYY-MM-DD".to_string())
    }

    fn validate_at_least(date: NaiveDate, compare_date: NaiveDate) -> Result<(), String> {
        if date >= compare_date {
            Ok(())
        } else {
            Err(format!("Must be at least {}", compare_date))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_flight_display() {
        assert_eq!(format!("{}", Flight::OneWay), "One-way flight");
        assert_eq!(format!("{}", Flight::Return), "Return flight");
    }

    #[test]
    fn test_flight_equality() {
        assert_eq!(Flight::OneWay, Flight::OneWay);
        assert_eq!(Flight::Return, Flight::Return);
        assert_ne!(Flight::OneWay, Flight::Return);
    }

    #[test]
    fn test_app_default() {
        let app = App::default();
        assert_eq!(app.flight_type, Flight::OneWay);
        assert!(app.departure.is_some());
        assert!(!app.departure_input.is_empty());
        assert_eq!(app.departure_error, None);
        assert!(app.return_date.is_some());
        assert_eq!(app.return_date_input, "");
        assert_eq!(app.return_date_error, None);
    }

    #[test]
    fn test_app_new() {
        let (app, task) = App::new();
        assert_eq!(app.flight_type, Flight::OneWay);
        assert!(app.departure.is_some());
        // Task should be none
        let _ = task;
    }

    #[test]
    fn test_validate_date_valid() {
        let result = App::validate_date("2025-12-25");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()
        );
    }

    #[test]
    fn test_validate_date_invalid_format() {
        let result = App::validate_date("12/25/2025");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Use YYYY-MM-DD");
    }

    #[test]
    fn test_validate_date_invalid_date() {
        let result = App::validate_date("2025-13-45");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Use YYYY-MM-DD");
    }

    #[test]
    fn test_validate_date_empty() {
        let result = App::validate_date("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Use YYYY-MM-DD");
    }

    #[test]
    fn test_validate_at_least_valid_equal() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
        let compare = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
        let result = App::validate_at_least(date, compare);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_at_least_valid_after() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 26).unwrap();
        let compare = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
        let result = App::validate_at_least(date, compare);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_at_least_invalid() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 24).unwrap();
        let compare = NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
        let result = App::validate_at_least(date, compare);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Must be at least 2025-12-25");
    }

    #[test]
    fn test_update_flight_selected() {
        let mut app = App::default();
        assert_eq!(app.flight_type, Flight::OneWay);

        app.update(Message::FlightSelected(Flight::Return));
        assert_eq!(app.flight_type, Flight::Return);

        app.update(Message::FlightSelected(Flight::OneWay));
        assert_eq!(app.flight_type, Flight::OneWay);
    }

    #[test]
    fn test_update_departure_changed_valid() {
        let mut app = App::default();
        app.update(Message::DepartureChanged("2025-12-25".to_string()));

        assert_eq!(app.departure_input, "2025-12-25");
        assert!(app.departure.is_some());
        assert_eq!(
            app.departure.unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()
        );
        assert_eq!(app.departure_error, None);
    }

    #[test]
    fn test_update_departure_changed_invalid_format() {
        let mut app = App::default();
        app.update(Message::DepartureChanged("invalid".to_string()));

        assert_eq!(app.departure_input, "invalid");
        assert!(app.departure.is_none());
        assert_eq!(app.departure_error, Some("Use YYYY-MM-DD".to_string()));
    }

    #[test]
    fn test_update_departure_changed_past_date() {
        let mut app = App::default();
        app.update(Message::DepartureChanged("2020-01-01".to_string()));

        assert_eq!(app.departure_input, "2020-01-01");
        assert!(app.departure.is_some());
        assert!(app.departure_error.is_some());
        assert!(
            app.departure_error
                .as_ref()
                .unwrap()
                .starts_with("Must be at least")
        );
    }

    #[test]
    fn test_update_return_date_changed_valid() {
        let mut app = App::default();
        app.update(Message::DepartureChanged("2025-12-20".to_string()));
        app.update(Message::ReturnDateChanged("2025-12-25".to_string()));

        assert_eq!(app.return_date_input, "2025-12-25");
        assert!(app.return_date.is_some());
        assert_eq!(
            app.return_date.unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()
        );
        assert_eq!(app.return_date_error, None);
    }

    #[test]
    fn test_update_return_date_changed_before_departure() {
        let mut app = App::default();
        app.update(Message::DepartureChanged("2025-12-25".to_string()));
        app.update(Message::ReturnDateChanged("2025-12-20".to_string()));

        assert_eq!(app.return_date_input, "2025-12-20");
        assert!(app.return_date.is_some());
        assert!(app.return_date_error.is_some());
        assert_eq!(
            app.return_date_error.as_ref().unwrap(),
            "Must be at least 2025-12-25"
        );
    }

    #[test]
    fn test_update_return_date_changed_no_departure() {
        let mut app = App {
            departure: None,
            ..Default::default()
        };
        app.update(Message::ReturnDateChanged("2025-12-25".to_string()));

        assert_eq!(app.return_date_input, "2025-12-25");
        assert!(app.return_date.is_some());
        assert_eq!(
            app.return_date_error,
            Some("Please select a departure date first".to_string())
        );
    }

    #[test]
    fn test_update_return_date_changed_invalid_format() {
        let mut app = App::default();
        app.update(Message::DepartureChanged("2025-12-25".to_string()));
        app.update(Message::ReturnDateChanged("invalid".to_string()));

        assert_eq!(app.return_date_input, "invalid");
        assert!(app.return_date.is_none());
        assert_eq!(app.return_date_error, Some("Use YYYY-MM-DD".to_string()));
    }

    #[test]
    fn test_update_book_flight_resets_form() {
        let mut app = App {
            flight_type: Flight::Return,
            ..Default::default()
        };
        app.update(Message::DepartureChanged("2025-12-25".to_string()));
        app.update(Message::ReturnDateChanged("2025-12-30".to_string()));

        // Book flight should reset to default
        app.update(Message::BookFlight);

        assert_eq!(app.flight_type, Flight::OneWay);
        assert_eq!(app.return_date_input, "");
    }

    #[test]
    fn test_message_clone() {
        let msg = Message::FlightSelected(Flight::OneWay);
        let cloned = msg.clone();
        // Just ensure we can clone messages
        let _ = cloned;
    }

    #[test]
    fn test_flight_debug() {
        // Ensure Debug trait works
        let flight = Flight::OneWay;
        let debug_str = format!("{:?}", flight);
        assert!(debug_str.contains("OneWay"));
    }
}
