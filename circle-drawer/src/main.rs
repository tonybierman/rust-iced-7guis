use iced::alignment::Horizontal;
use iced::mouse;
use iced::widget::{Canvas, Stack, button, canvas, column, container, row, slider, text};
use iced::{Border, Color, Element, Event, Length, Point, Rectangle, Settings, Theme};

const INITIAL_DIAMETER: f32 = 40.0;
const MIN_DIAMETER: f32 = 10.0;
const MAX_DIAMETER: f32 = 100.0;
const DIALOG_WIDTH: f32 = 400.0;
const DIALOG_HEIGHT: f32 = 250.0;
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const CONTROLS_HEIGHT: f32 = 50.0; // Approximate height of controls area

pub fn main() -> iced::Result {
    iced::application(CircleDrawer::new, CircleDrawer::update, CircleDrawer::view)
        .settings(Settings {
            antialiasing: true,
            ..Default::default()
        })
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT))
        .run()
}

#[derive(Debug, Clone, Copy)]
struct Circle {
    center: Point,
    diameter: f32,
}

impl Circle {
    fn new(center: Point) -> Self {
        Self {
            center,
            diameter: INITIAL_DIAMETER,
        }
    }

    fn contains(&self, point: Point) -> bool {
        let dx = self.center.x - point.x;
        let dy = self.center.y - point.y;
        let distance_squared = dx * dx + dy * dy;
        let radius = self.diameter / 2.0;
        distance_squared <= radius * radius
    }
}

#[derive(Debug, Clone)]
enum Change {
    AddCircle(Circle),
    AdjustDiameter {
        index: usize,
        old_diameter: f32,
        new_diameter: f32,
    },
}

struct CircleDrawer {
    circles: Vec<Circle>,
    selected_circle: Option<usize>,
    history: Vec<Change>,
    history_index: usize,
    dialog_open: bool,
    temp_diameter: f32,
    adjusting_index: Option<usize>,
    original_diameter: Option<f32>,
}

#[derive(Debug, Clone)]
enum Message {
    CanvasEvent(canvas::Event),
    Undo,
    Redo,
    OpenDialog,
    CloseDialog,
    DiameterChanged(f32),
}

impl CircleDrawer {
    fn new() -> (Self, iced::Task<Message>) {
        (
            Self {
                circles: Vec::new(),
                selected_circle: None,
                history: Vec::new(),
                history_index: 0,
                dialog_open: false,
                temp_diameter: INITIAL_DIAMETER,
                adjusting_index: None,
                original_diameter: None,
            },
            iced::Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CanvasEvent(event) => {
                if self.dialog_open {
                    return;
                }

                if let canvas::Event::Mouse(mouse_event) = event {
                    match mouse_event {
                        mouse::Event::ButtonPressed(button) => {
                            if let mouse::Button::Left = button {
                                if let Some(cursor_position) = mouse_event::get_cursor_position() {
                                    let circle = Circle::new(cursor_position);
                                    self.circles.push(circle);

                                    self.history.truncate(self.history_index);
                                    self.history.push(Change::AddCircle(circle));
                                    self.history_index += 1;

                                    self.selected_circle = Some(self.circles.len() - 1);
                                }
                            } else if let mouse::Button::Right = button
                                && self.selected_circle.is_some()
                            {
                                self.update(Message::OpenDialog);
                            }
                        }
                        mouse::Event::CursorMoved { position } => {
                            let mut nearest = None;
                            let mut nearest_distance = f32::MAX;

                            for (i, circle) in self.circles.iter().enumerate() {
                                if circle.contains(position) {
                                    let dx = circle.center.x - position.x;
                                    let dy = circle.center.y - position.y;
                                    let distance = (dx * dx + dy * dy).sqrt();
                                    if distance < nearest_distance {
                                        nearest = Some(i);
                                        nearest_distance = distance;
                                    }
                                }
                            }

                            self.selected_circle = nearest;
                        }
                        _ => {}
                    }
                }
            }
            Message::Undo => {
                if self.history_index > 0 {
                    self.history_index -= 1;
                    let change = &self.history[self.history_index];

                    match change {
                        Change::AddCircle(_) => {
                            self.circles.pop();
                            self.selected_circle = None;
                        }
                        Change::AdjustDiameter {
                            index,
                            old_diameter,
                            ..
                        } => {
                            if let Some(circle) = self.circles.get_mut(*index) {
                                circle.diameter = *old_diameter;
                            }
                        }
                    }
                }
            }
            Message::Redo => {
                if self.history_index < self.history.len() {
                    let change = self.history[self.history_index].clone();
                    self.history_index += 1;

                    match change {
                        Change::AddCircle(circle) => {
                            self.circles.push(circle);
                            self.selected_circle = Some(self.circles.len() - 1);
                        }
                        Change::AdjustDiameter {
                            index,
                            new_diameter,
                            ..
                        } => {
                            if let Some(circle) = self.circles.get_mut(index) {
                                circle.diameter = new_diameter;
                            }
                        }
                    }
                }
            }
            Message::OpenDialog => {
                if let Some(index) = self.selected_circle {
                    self.dialog_open = true;
                    self.adjusting_index = Some(index);
                    self.temp_diameter = self.circles[index].diameter;
                    self.original_diameter = Some(self.circles[index].diameter);
                }
            }
            Message::CloseDialog => {
                self.dialog_open = false;

                if let (Some(index), Some(old_diameter)) =
                    (self.adjusting_index, self.original_diameter)
                {
                    let new_diameter = self.circles[index].diameter;

                    if (old_diameter - new_diameter).abs() > 0.001 {
                        self.history.truncate(self.history_index);
                        self.history.push(Change::AdjustDiameter {
                            index,
                            old_diameter,
                            new_diameter,
                        });
                        self.history_index += 1;
                    }
                }

                self.adjusting_index = None;
                self.original_diameter = None;
            }
            Message::DiameterChanged(diameter) => {
                self.temp_diameter = diameter;
                if let Some(index) = self.adjusting_index
                    && let Some(circle) = self.circles.get_mut(index)
                {
                    circle.diameter = diameter;
                }
            }
        }
    }

    fn calculate_dialog_position(&self, circle_center: Point, circle_diameter: f32) -> (f32, f32) {
        let canvas_y_offset = CONTROLS_HEIGHT + 10.0; // Include top padding
        let circle_y_in_window = circle_center.y + canvas_y_offset;

        // Try to place dialog close to the right of the circle (10px gap)
        let preferred_x = circle_center.x + circle_diameter / 2.0 + 10.0;
        let preferred_y = circle_y_in_window - DIALOG_HEIGHT / 2.0;

        // Check if dialog fits to the right
        let x = if preferred_x + DIALOG_WIDTH <= WINDOW_WIDTH - 10.0 {
            preferred_x
        } else {
            // Try to the left of the circle (10px gap)
            let left_x = circle_center.x - circle_diameter / 2.0 - DIALOG_WIDTH - 10.0;
            if left_x >= 10.0 {
                left_x
            } else {
                // Place above or below the circle if sides don't work
                (circle_center.x - DIALOG_WIDTH / 2.0)
                    .clamp(10.0, WINDOW_WIDTH - DIALOG_WIDTH - 10.0)
            }
        };

        // Ensure dialog stays within vertical bounds
        let y = if preferred_y < canvas_y_offset {
            canvas_y_offset
        } else if preferred_y + DIALOG_HEIGHT > WINDOW_HEIGHT - 10.0 {
            WINDOW_HEIGHT - DIALOG_HEIGHT - 10.0
        } else {
            preferred_y
        };

        // Clamp to ensure we're within bounds
        let x = x.clamp(10.0, WINDOW_WIDTH - DIALOG_WIDTH - 10.0);
        let y = y.clamp(canvas_y_offset, WINDOW_HEIGHT - DIALOG_HEIGHT - 10.0);

        (x, y)
    }

    fn view(&self) -> Element<'_, Message> {
        let undo_button = button("Undo").on_press_maybe(if self.history_index > 0 {
            Some(Message::Undo)
        } else {
            None
        });

        let redo_button =
            button("Redo").on_press_maybe(if self.history_index < self.history.len() {
                Some(Message::Redo)
            } else {
                None
            });

        let controls = container(row![undo_button, redo_button].spacing(10))
            .width(Length::Fill)
            .align_x(Horizontal::Center);

        let canvas_widget = Canvas::new(CircleCanvas {
            circles: &self.circles,
            selected_circle: self.selected_circle,
        })
        .width(Length::Fill)
        .height(Length::Fill);

        let content = column![controls, canvas_widget].spacing(10).padding(10);

        if self.dialog_open {
            // Calculate dialog position to avoid covering the circle and going off screen
            let (dialog_x, dialog_y) = if let Some(index) = self.selected_circle {
                let circle = &self.circles[index];
                self.calculate_dialog_position(circle.center, circle.diameter)
            } else {
                // Fallback to center if no circle selected
                (
                    (WINDOW_WIDTH - DIALOG_WIDTH) / 2.0,
                    (WINDOW_HEIGHT - DIALOG_HEIGHT) / 2.0,
                )
            };

            let dialog_content = column![
                text("Adjust diameter:")
                    .size(16)
                    .width(Length::Fill)
                    .color(Color::BLACK)
                    .align_x(Horizontal::Center),
                slider(
                    MIN_DIAMETER..=MAX_DIAMETER,
                    self.temp_diameter,
                    Message::DiameterChanged
                )
                .width(Length::Fill),
                text(format!("Diameter: {:.1}", self.temp_diameter))
                    .size(14)
                    .width(Length::Fill)
                    .color(Color::BLACK)
                    .align_x(Horizontal::Center),
                container(button("Close").on_press(Message::CloseDialog).padding(10))
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
            ]
            .spacing(20)
            .padding(30)
            .width(DIALOG_WIDTH)
            .height(DIALOG_HEIGHT);

            let dialog_container = container(dialog_content)
                .style(|_theme| container::Style {
                    background: Some(Color::WHITE.into()),
                    border: Border {
                        color: Color::BLACK,
                        width: 2.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                })
                .width(DIALOG_WIDTH)
                .height(DIALOG_HEIGHT);

            // Position the dialog using padding on a transparent container
            let positioned_dialog = container(dialog_container)
                .padding(iced::Padding {
                    top: dialog_y,
                    right: 0.0,
                    bottom: 0.0,
                    left: dialog_x,
                })
                .width(Length::Fill)
                .height(Length::Fill);

            // Use Stack to overlay the dialog without affecting layout
            Stack::new().push(content).push(positioned_dialog).into()
        } else {
            content.into()
        }
    }
}

struct CircleCanvas<'a> {
    circles: &'a [Circle],
    selected_circle: Option<usize>,
}

impl canvas::Program<Message> for CircleCanvas<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        for (i, circle) in self.circles.iter().enumerate() {
            let is_selected = self.selected_circle == Some(i);

            let stroke_color = Color::BLACK;
            let fill_color = if is_selected {
                Color::from_rgb(0.8, 0.8, 0.8)
            } else {
                Color::TRANSPARENT
            };

            let circle_path = canvas::Path::circle(circle.center, circle.diameter / 2.0);

            frame.fill(&circle_path, fill_color);
            frame.stroke(
                &circle_path,
                canvas::Stroke::default()
                    .with_color(stroke_color)
                    .with_width(2.0),
            );
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        if let Some(position) = cursor.position_in(bounds)
            && let Event::Mouse(mouse_event) = event
        {
            let adjusted_event = match mouse_event {
                mouse::Event::ButtonPressed(button) => {
                    canvas::Event::Mouse(mouse::Event::ButtonPressed(*button))
                }
                mouse::Event::CursorMoved { .. } => {
                    canvas::Event::Mouse(mouse::Event::CursorMoved { position })
                }
                _ => return None,
            };

            mouse_event::set_cursor_position(position);

            return Some(canvas::Action::publish(Message::CanvasEvent(
                adjusted_event,
            )));
        }

        None
    }
}

mod mouse_event {
    use iced::Point;

    thread_local! {
        static CURSOR_POSITION: std::cell::Cell<Option<Point>> = const { std::cell::Cell::new(None) };
    }

    pub fn set_cursor_position(position: Point) {
        CURSOR_POSITION.with(|pos| pos.set(Some(position)));
    }

    pub fn get_cursor_position() -> Option<Point> {
        CURSOR_POSITION.with(|pos| pos.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_new() {
        let center = Point::new(100.0, 150.0);
        let circle = Circle::new(center);
        assert_eq!(circle.center.x, 100.0);
        assert_eq!(circle.center.y, 150.0);
        assert_eq!(circle.diameter, INITIAL_DIAMETER);
    }

    #[test]
    fn test_circle_contains_center() {
        let circle = Circle::new(Point::new(100.0, 100.0));
        assert!(circle.contains(Point::new(100.0, 100.0)));
    }

    #[test]
    fn test_circle_contains_edge() {
        let circle = Circle::new(Point::new(100.0, 100.0));
        // Point at the edge (radius = 20.0)
        assert!(circle.contains(Point::new(120.0, 100.0)));
        assert!(circle.contains(Point::new(80.0, 100.0)));
        assert!(circle.contains(Point::new(100.0, 120.0)));
        assert!(circle.contains(Point::new(100.0, 80.0)));
    }

    #[test]
    fn test_circle_contains_inside() {
        let circle = Circle::new(Point::new(100.0, 100.0));
        assert!(circle.contains(Point::new(105.0, 105.0)));
        assert!(circle.contains(Point::new(95.0, 95.0)));
    }

    #[test]
    fn test_circle_contains_outside() {
        let circle = Circle::new(Point::new(100.0, 100.0));
        assert!(!circle.contains(Point::new(130.0, 100.0)));
        assert!(!circle.contains(Point::new(70.0, 100.0)));
        assert!(!circle.contains(Point::new(100.0, 130.0)));
        assert!(!circle.contains(Point::new(100.0, 70.0)));
    }

    #[test]
    fn test_circle_drawer_new() {
        let (drawer, _task) = CircleDrawer::new();
        assert_eq!(drawer.circles.len(), 0);
        assert_eq!(drawer.selected_circle, None);
        assert_eq!(drawer.history.len(), 0);
        assert_eq!(drawer.history_index, 0);
        assert!(!drawer.dialog_open);
        assert_eq!(drawer.temp_diameter, INITIAL_DIAMETER);
        assert_eq!(drawer.adjusting_index, None);
        assert_eq!(drawer.original_diameter, None);
    }

    #[test]
    fn test_add_circle_left_click() {
        let (mut drawer, _) = CircleDrawer::new();
        mouse_event::set_cursor_position(Point::new(50.0, 60.0));

        let event = canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));
        drawer.update(Message::CanvasEvent(event));

        assert_eq!(drawer.circles.len(), 1);
        assert_eq!(drawer.circles[0].center.x, 50.0);
        assert_eq!(drawer.circles[0].center.y, 60.0);
        assert_eq!(drawer.circles[0].diameter, INITIAL_DIAMETER);
        assert_eq!(drawer.selected_circle, Some(0));
        assert_eq!(drawer.history.len(), 1);
        assert_eq!(drawer.history_index, 1);
    }

    #[test]
    fn test_add_multiple_circles() {
        let (mut drawer, _) = CircleDrawer::new();

        mouse_event::set_cursor_position(Point::new(50.0, 60.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        mouse_event::set_cursor_position(Point::new(150.0, 160.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        assert_eq!(drawer.circles.len(), 2);
        assert_eq!(drawer.history.len(), 2);
        assert_eq!(drawer.history_index, 2);
        assert_eq!(drawer.selected_circle, Some(1));
    }

    #[test]
    fn test_circle_selection_on_hover() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        // Move cursor over the circle
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::CursorMoved {
                position: Point::new(105.0, 105.0),
            },
        )));

        assert_eq!(drawer.selected_circle, Some(0));

        // Move cursor away from the circle
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::CursorMoved {
                position: Point::new(200.0, 200.0),
            },
        )));

        assert_eq!(drawer.selected_circle, None);
    }

    #[test]
    fn test_undo_circle_creation() {
        let (mut drawer, _) = CircleDrawer::new();

        mouse_event::set_cursor_position(Point::new(50.0, 60.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        assert_eq!(drawer.circles.len(), 1);

        drawer.update(Message::Undo);

        assert_eq!(drawer.circles.len(), 0);
        assert_eq!(drawer.selected_circle, None);
        assert_eq!(drawer.history_index, 0);
    }

    #[test]
    fn test_undo_when_no_history() {
        let (mut drawer, _) = CircleDrawer::new();
        drawer.update(Message::Undo);

        assert_eq!(drawer.circles.len(), 0);
        assert_eq!(drawer.history_index, 0);
    }

    #[test]
    fn test_redo_circle_creation() {
        let (mut drawer, _) = CircleDrawer::new();

        mouse_event::set_cursor_position(Point::new(50.0, 60.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        drawer.update(Message::Undo);
        assert_eq!(drawer.circles.len(), 0);

        drawer.update(Message::Redo);
        assert_eq!(drawer.circles.len(), 1);
        assert_eq!(drawer.circles[0].center.x, 50.0);
        assert_eq!(drawer.selected_circle, Some(0));
    }

    #[test]
    fn test_redo_when_at_end() {
        let (mut drawer, _) = CircleDrawer::new();

        mouse_event::set_cursor_position(Point::new(50.0, 60.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        drawer.update(Message::Redo);

        // Should not crash or change anything
        assert_eq!(drawer.circles.len(), 1);
        assert_eq!(drawer.history_index, 1);
    }

    #[test]
    fn test_history_truncation_after_undo() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add first circle
        mouse_event::set_cursor_position(Point::new(50.0, 60.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        // Add second circle
        mouse_event::set_cursor_position(Point::new(150.0, 160.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        assert_eq!(drawer.history.len(), 2);

        // Undo once
        drawer.update(Message::Undo);
        assert_eq!(drawer.history_index, 1);

        // Add a new circle - should truncate forward history
        mouse_event::set_cursor_position(Point::new(200.0, 200.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        assert_eq!(drawer.history.len(), 2); // Old second entry replaced
        assert_eq!(drawer.history_index, 2);
        assert_eq!(drawer.circles.len(), 2);
    }

    #[test]
    fn test_open_dialog() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        drawer.update(Message::OpenDialog);

        assert!(drawer.dialog_open);
        assert_eq!(drawer.adjusting_index, Some(0));
        assert_eq!(drawer.temp_diameter, INITIAL_DIAMETER);
        assert_eq!(drawer.original_diameter, Some(INITIAL_DIAMETER));
    }

    #[test]
    fn test_open_dialog_no_selection() {
        let (mut drawer, _) = CircleDrawer::new();
        drawer.selected_circle = None;

        drawer.update(Message::OpenDialog);

        assert!(!drawer.dialog_open);
        assert_eq!(drawer.adjusting_index, None);
    }

    #[test]
    fn test_diameter_change() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        drawer.update(Message::OpenDialog);
        drawer.update(Message::DiameterChanged(60.0));

        assert_eq!(drawer.temp_diameter, 60.0);
        assert_eq!(drawer.circles[0].diameter, 60.0);
    }

    #[test]
    fn test_diameter_change_no_adjusting_index() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        // Don't open dialog, just try to change diameter
        drawer.update(Message::DiameterChanged(60.0));

        assert_eq!(drawer.temp_diameter, 60.0);
        assert_eq!(drawer.circles[0].diameter, INITIAL_DIAMETER); // Unchanged
    }

    #[test]
    fn test_close_dialog_records_history() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        assert_eq!(drawer.history.len(), 1);

        drawer.update(Message::OpenDialog);
        drawer.update(Message::DiameterChanged(60.0));
        drawer.update(Message::CloseDialog);

        assert!(!drawer.dialog_open);
        assert_eq!(drawer.history.len(), 2); // Circle creation + diameter change
        assert_eq!(drawer.history_index, 2);
        assert_eq!(drawer.adjusting_index, None);
        assert_eq!(drawer.original_diameter, None);
    }

    #[test]
    fn test_close_dialog_no_change_no_history() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        drawer.update(Message::OpenDialog);
        // Don't change diameter
        drawer.update(Message::CloseDialog);

        assert_eq!(drawer.history.len(), 1); // Only circle creation
    }

    #[test]
    fn test_undo_diameter_change() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        drawer.update(Message::OpenDialog);
        drawer.update(Message::DiameterChanged(60.0));
        drawer.update(Message::CloseDialog);

        assert_eq!(drawer.circles[0].diameter, 60.0);

        drawer.update(Message::Undo);

        assert_eq!(drawer.circles[0].diameter, INITIAL_DIAMETER);
    }

    #[test]
    fn test_redo_diameter_change() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        drawer.update(Message::OpenDialog);
        drawer.update(Message::DiameterChanged(60.0));
        drawer.update(Message::CloseDialog);

        drawer.update(Message::Undo);
        assert_eq!(drawer.circles[0].diameter, INITIAL_DIAMETER);

        drawer.update(Message::Redo);
        assert_eq!(drawer.circles[0].diameter, 60.0);
    }

    #[test]
    fn test_right_click_opens_dialog() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        // Right-click should open dialog
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Right),
        )));

        assert!(drawer.dialog_open);
    }

    #[test]
    fn test_right_click_no_selection() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        // Deselect
        drawer.selected_circle = None;

        // Right-click should not open dialog
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Right),
        )));

        assert!(!drawer.dialog_open);
    }

    #[test]
    fn test_dialog_open_blocks_canvas_events() {
        let (mut drawer, _) = CircleDrawer::new();

        drawer.dialog_open = true;

        // Try to add a circle while dialog is open
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        assert_eq!(drawer.circles.len(), 0);
    }

    #[test]
    fn test_multiple_undo_redo() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add three circles
        for i in 0..3 {
            mouse_event::set_cursor_position(Point::new(i as f32 * 50.0, 100.0));
            drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
                mouse::Event::ButtonPressed(mouse::Button::Left),
            )));
        }

        assert_eq!(drawer.circles.len(), 3);

        // Undo all
        drawer.update(Message::Undo);
        drawer.update(Message::Undo);
        drawer.update(Message::Undo);

        assert_eq!(drawer.circles.len(), 0);

        // Redo all
        drawer.update(Message::Redo);
        drawer.update(Message::Redo);
        drawer.update(Message::Redo);

        assert_eq!(drawer.circles.len(), 3);
    }

    #[test]
    fn test_select_nearest_circle() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add two circles
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        mouse_event::set_cursor_position(Point::new(200.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        // Move cursor closer to first circle
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::CursorMoved {
                position: Point::new(105.0, 100.0),
            },
        )));

        assert_eq!(drawer.selected_circle, Some(0));

        // Move cursor closer to second circle
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::CursorMoved {
                position: Point::new(195.0, 100.0),
            },
        )));

        assert_eq!(drawer.selected_circle, Some(1));
    }

    #[test]
    fn test_change_debug_impl() {
        let change1 = Change::AddCircle(Circle::new(Point::new(10.0, 20.0)));
        let debug_str = format!("{:?}", change1);
        assert!(debug_str.contains("AddCircle"));

        let change2 = Change::AdjustDiameter {
            index: 0,
            old_diameter: 40.0,
            new_diameter: 60.0,
        };
        let debug_str = format!("{:?}", change2);
        assert!(debug_str.contains("AdjustDiameter"));
    }

    #[test]
    fn test_message_debug_impl() {
        let msg = Message::Undo;
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("Undo"));
    }

    #[test]
    fn test_circle_debug_impl() {
        let circle = Circle::new(Point::new(10.0, 20.0));
        let debug_str = format!("{:?}", circle);
        assert!(debug_str.contains("Circle"));
    }

    #[test]
    fn test_mouse_event_helper() {
        let pos = Point::new(42.0, 84.0);
        mouse_event::set_cursor_position(pos);
        let retrieved = mouse_event::get_cursor_position();
        assert_eq!(retrieved, Some(pos));
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_diameter_constants() {
        assert_eq!(INITIAL_DIAMETER, 40.0);
        assert_eq!(MIN_DIAMETER, 10.0);
        assert_eq!(MAX_DIAMETER, 100.0);
        assert!(MIN_DIAMETER < INITIAL_DIAMETER);
        assert!(INITIAL_DIAMETER < MAX_DIAMETER);
    }

    #[test]
    fn test_undo_with_invalid_index() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        // Adjust diameter
        drawer.update(Message::OpenDialog);
        drawer.update(Message::DiameterChanged(60.0));
        drawer.update(Message::CloseDialog);

        // Remove the circle manually (simulating invalid state)
        drawer.circles.clear();

        // Undo should not panic
        drawer.update(Message::Undo);
    }

    #[test]
    fn test_redo_with_invalid_index() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        // Adjust diameter
        drawer.update(Message::OpenDialog);
        drawer.update(Message::DiameterChanged(60.0));
        drawer.update(Message::CloseDialog);

        // Undo
        drawer.update(Message::Undo);

        // Remove the circle manually (simulating invalid state)
        drawer.circles.clear();

        // Redo should not panic
        drawer.update(Message::Redo);
    }

    #[test]
    fn test_close_dialog_with_tiny_change() {
        let (mut drawer, _) = CircleDrawer::new();

        // Add a circle
        mouse_event::set_cursor_position(Point::new(100.0, 100.0));
        drawer.update(Message::CanvasEvent(canvas::Event::Mouse(
            mouse::Event::ButtonPressed(mouse::Button::Left),
        )));

        drawer.update(Message::OpenDialog);
        // Make a very tiny change (below threshold)
        drawer.update(Message::DiameterChanged(INITIAL_DIAMETER + 0.0001));
        drawer.update(Message::CloseDialog);

        // Should not add to history (change too small)
        assert_eq!(drawer.history.len(), 1);
    }
}
