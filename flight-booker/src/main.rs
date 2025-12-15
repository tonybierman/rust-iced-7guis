use iced::alignment::Vertical;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Settings};
use chrono::{DateTime, Local};

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
    DepartureChanged,
    ReturnDateChanged,
}

#[derive(Default)]
struct App {
    departure: Option<DateTime<Local>>,
    return_date: Option<DateTime<Local>>,
}

impl App {
    fn new() -> (Self, iced::Task<Message>) {
        (App { departure: Some(Local::now()), return_date: Some(Local::now()) }, iced::Task::none())
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
            Message::DepartureChanged => {
            },
            Message::ReturnDateChanged => {
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
}
