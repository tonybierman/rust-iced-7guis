use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length, Settings};

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .settings(Settings {
            antialiasing: true,
            ..Default::default()
        })
        .window_size((500, 400))
        .centered()
        .resizable(false)
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    FilterInputChanged(String),
    FirstNameInputChanged(String),
    LastNameInputChanged(String),
    CreatePressed,
    UpdatePressed,
    DeletePressed,
    ItemSelected(usize),
}

#[derive(Debug, Clone)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

pub struct App {
    filter_input: String,
    first_name_input: String,
    last_name_input: String,
    people: Vec<Person>,
    selected_index: Option<usize>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            filter_input: String::new(),
            first_name_input: String::new(),
            last_name_input: String::new(),
            people: vec![
                Person {
                    first_name: "John".to_string(),
                    last_name: "Doe".to_string(),
                },
                Person {
                    first_name: "Jane".to_string(),
                    last_name: "Smith".to_string(),
                },
                Person {
                    first_name: "Bob".to_string(),
                    last_name: "Johnson".to_string(),
                },
                Person {
                    first_name: "Alice".to_string(),
                    last_name: "Williams".to_string(),
                },
                Person {
                    first_name: "Charlie".to_string(),
                    last_name: "Brown".to_string(),
                },
            ],
            selected_index: None,
        }
    }
}

impl App {
    pub fn new() -> (Self, iced::Task<Message>) {
        (App::default(), iced::Task::none())
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FilterInputChanged(v) => {
                self.filter_input = v;
            }
            Message::FirstNameInputChanged(v) => {
                self.first_name_input = v;
            }
            Message::LastNameInputChanged(v) => {
                self.last_name_input = v;
            }
            Message::ItemSelected(index) => {
                self.selected_index = Some(index);
                // Load the selected person's data into the form
                if let Some(person) = self.people.get(index) {
                    self.first_name_input = person.first_name.clone();
                    self.last_name_input = person.last_name.clone();
                }
            }
            Message::CreatePressed => {
                println!("Create pressed!");
                let new_person = Person {
                    first_name: self.first_name_input.clone(),
                    last_name: self.last_name_input.clone(),
                };
                self.people.push(new_person);
                self.first_name_input.clear();
                self.last_name_input.clear();
            }
            Message::UpdatePressed => {
                println!("Update pressed!");
                if let Some(index) = self.selected_index
                    && let Some(person) = self.people.get_mut(index)
                {
                    person.first_name = self.first_name_input.clone();
                    person.last_name = self.last_name_input.clone();
                }
            }
            Message::DeletePressed => {
                println!("Delete pressed!");
                if let Some(index) = self.selected_index {
                    self.people.remove(index);
                    self.selected_index = None;
                    self.first_name_input.clear();
                    self.last_name_input.clear();
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // Filter the people list
        let filtered_people: Vec<(usize, &Person)> = self
            .people
            .iter()
            .enumerate()
            .filter(|(_, person)| {
                if self.filter_input.is_empty() {
                    true
                } else {
                    person
                        .last_name
                        .to_lowercase()
                        .starts_with(&self.filter_input.to_lowercase())
                }
            })
            .collect();

        // Create list items
        let list_items =
            filtered_people
                .iter()
                .fold(column![].spacing(5), |col, (index, person)| {
                    let is_selected = self.selected_index == Some(*index);
                    let item_text = format!("{}, {}", person.last_name, person.first_name);

                    let item_button = button(text(item_text).size(14))
                        .on_press(Message::ItemSelected(*index))
                        .width(Length::Fill)
                        .style(if is_selected {
                            button::primary
                        } else {
                            button::secondary
                        });

                    col.push(item_button)
                });

        // Left column content with scrollable list
        let left_column = column![
            row![
                text("Filter prefix:").size(12),
                text_input("Filter...", &self.filter_input)
                    .size(12)
                    .on_input(Message::FilterInputChanged),
            ]
            .spacing(10)
            .align_y(Vertical::Center),
            scrollable(list_items).height(Length::Fill),
        ]
        .spacing(10)
        .padding(20)
        .width(Length::Fill);

        // Right column content
        let right_column = column![
            row![
                text("Name:").size(12),
                text_input("First...", &self.first_name_input)
                    .size(12)
                    .on_input(Message::FirstNameInputChanged),
            ]
            .spacing(10)
            .align_y(Vertical::Center),
            row![
                text("Surname:").size(12),
                text_input("Last...", &self.last_name_input)
                    .size(12)
                    .on_input(Message::LastNameInputChanged),
            ]
            .spacing(10)
            .align_y(Vertical::Center),
        ]
        .spacing(20)
        .padding(20)
        .width(Length::Fill);

        // Create the two-column row layout
        let columns = row![left_column, right_column]
            .spacing(10)
            .align_y(Alignment::Start);

        // Bottom button row
        let bottom_buttons = container(
            row![
                button("Create").on_press(Message::CreatePressed),
                button("Update").on_press(Message::UpdatePressed),
                button("Delete").on_press(Message::DeletePressed),
            ]
            .spacing(10),
        )
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .padding(10);

        // Combine columns and buttons in a vertical layout
        let content = column![columns, bottom_buttons,].spacing(10);

        // Wrap in a container
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}
