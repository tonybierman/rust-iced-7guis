use iced::alignment::{Horizontal, Vertical};
use iced::time::{self, milliseconds};
use iced::widget::{ProgressBar, Slider, button, column, container, row, text};
use iced::{Element, Settings, Subscription};

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .settings(Settings {
            antialiasing: true,
            ..Default::default()
        })
        .window_size((300, 220))
        .centered()
        .resizable(false)
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    DurationChanged(f32),
    ResetPressed,
}

#[derive(Default)]
struct App {
    elapsed: f32,
    duration: f32,
    max_duration: f32,
}

impl App {
    fn new() -> (Self, iced::Task<Message>) {
        (
            App {
                elapsed: 0.0,
                duration: 10.0,
                max_duration: 100.0,
            },
            iced::Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                if self.elapsed < self.duration {
                    self.elapsed += 0.05;
                }
            }
            Message::DurationChanged(new_duration) => {
                self.duration = new_duration;
            }
            Message::ResetPressed => {
                self.elapsed = 0.0;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            column![
                row![
                    text("Elapsed Time:").size(12).align_x(Horizontal::Left),
                    ProgressBar::new(0.0..=self.duration, self.elapsed)
                ]
                .spacing(10)
                .align_y(Vertical::Center),
                container(text(format!("{:.2}", self.elapsed)).size(48))
                    .center_x(iced::Length::Fill),
                row![
                    text("Duration:").size(12).align_x(Horizontal::Left),
                    Slider::new(
                        0.0..=self.max_duration,
                        self.duration,
                        Message::DurationChanged
                    )
                ]
                .padding(iced::padding::bottom(20))
                .spacing(10)
                .align_y(Vertical::Center),
                container(button("Reset").on_press(Message::ResetPressed))
                    .align_y(Vertical::Center)
                    .center_x(iced::Length::Fill),
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

#[cfg(test)]
mod tests {
    use super::*;
    use iced_test::{Error, simulator};

    #[test]
    fn ui_timer_reset() -> Result<(), Error> {
        let mut timer = App {
            elapsed: 0.0,
            duration: 10.0,
            max_duration: 100.0,
        };

        {
            let mut ui = simulator(timer.view());
            let _ = ui.click("Reset")?;
        } // ui is dropped here

        timer.update(Message::ResetPressed);

        assert_eq!(timer.elapsed, 0.0);
        assert_eq!(timer.duration, 10.0); // Duration unchanged

        Ok(())
    }

    #[test]
    fn test_initial_state() {
        let (app, _) = App::new();
        assert_eq!(app.elapsed, 0.0);
        assert_eq!(app.duration, 10.0);
        assert_eq!(app.max_duration, 100.0);
    }

    #[test]
    fn test_increment_when_below_duration() {
        let mut app = App {
            elapsed: 5.0,
            duration: 10.0,
            max_duration: 100.0,
        };
        app.update(Message::Increment);
        assert_eq!(app.elapsed, 5.05);
    }

    #[test]
    fn test_increment_stops_at_duration() {
        let mut app = App {
            elapsed: 10.0,
            duration: 10.0,
            max_duration: 100.0,
        };
        app.update(Message::Increment);
        assert_eq!(app.elapsed, 10.0); // Should not increment
    }

    #[test]
    fn test_duration_changed() {
        let mut app = App {
            elapsed: 5.0,
            duration: 10.0,
            max_duration: 100.0,
        };
        app.update(Message::DurationChanged(20.0));
        assert_eq!(app.duration, 20.0);
        assert_eq!(app.elapsed, 5.0); // elapsed unchanged
    }

    #[test]
    fn test_duration_changed_restarts_timer() {
        let mut app = App {
            elapsed: 10.0,
            duration: 10.0,
            max_duration: 100.0,
        };
        // Timer is stopped (elapsed == duration)
        app.update(Message::Increment);
        assert_eq!(app.elapsed, 10.0); // Still stopped

        // Increase duration
        app.update(Message::DurationChanged(20.0));
        assert_eq!(app.duration, 20.0);

        // Timer should restart
        app.update(Message::Increment);
        assert_eq!(app.elapsed, 10.05); // Now incrementing again
    }

    #[test]
    fn test_reset() {
        let mut app = App {
            elapsed: 7.5,
            duration: 10.0,
            max_duration: 100.0,
        };
        app.update(Message::ResetPressed);
        assert_eq!(app.elapsed, 0.0);
        assert_eq!(app.duration, 10.0); // Duration unchanged
    }

    #[test]
    fn test_reset_allows_restart() {
        let mut app = App {
            elapsed: 10.0,
            duration: 10.0,
            max_duration: 100.0,
        };
        // Timer is stopped
        app.update(Message::ResetPressed);
        assert_eq!(app.elapsed, 0.0);

        // Timer should be able to increment again
        app.update(Message::Increment);
        assert_eq!(app.elapsed, 0.05);
    }
}
