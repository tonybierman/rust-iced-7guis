use iced::alignment::Horizontal;
use iced::time::{self, milliseconds};
use iced::widget::{ProgressBar, Slider, button, column, container, row, text};
use iced::{Element, Settings, Subscription} ;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .settings(Settings {
            antialiasing: true,
            ..Default::default()
        })
        .window_size((360, 360))
        .centered()
        .resizable(false)
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    DurationChanged(f32),
    Reset,
}

#[derive(Default)]
struct App {
    elapsed: f32,
    duration: f32,
    max_duration: f32,
}

impl App {
    fn new() -> (Self, iced::Task<Message>) {
        (App { elapsed: 0.0, duration: 10.0, max_duration: 100.0 }, iced::Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                if self.elapsed < self.duration {
                    self.elapsed += 0.05;
                }
            },
            Message::DurationChanged(new_duration) => {
                self.duration = new_duration;
            },
            Message::Reset => {
                self.elapsed = 0.0;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {

        container(
            column![
                row![
                    text("Elapsed Time:")
                        .size(12)
                        .align_x(Horizontal::Left),
                    ProgressBar::new(0.0..=self.duration, self.elapsed)
                ],
                row![
                    text(format!("{:.2}", self.elapsed))
                        .size(24)
                        .align_x(Horizontal::Center)
                ],
                row![
                    text("Duration:")
                        .size(12)
                        .align_x(Horizontal::Left),
                    Slider::new(0.0..=self.max_duration, self.duration, Message::DurationChanged)
                ],
                row![
                    button("Reset")
                        .on_press(Message::Reset)
                ],
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