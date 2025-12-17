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
        let filtered_people = self.filtered_people();

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
        let mut update_button = button("Update");
        if self.selected_index.is_some() {
            update_button = update_button.on_press(Message::UpdatePressed);
        }

        let mut delete_button = button("Delete");
        if self.selected_index.is_some() {
            delete_button = delete_button.on_press(Message::DeletePressed);
        }

        let bottom_buttons = container(
            row![
                button("Create").on_press(Message::CreatePressed),
                update_button,
                delete_button,
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

    pub fn filtered_people(&self) -> Vec<(usize, &Person)> {
        self.people
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
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_test::{Error, simulator};

    #[test]
    fn ui_create_user() -> Result<(), Error> {
        let mut crud = App {
            first_name_input: "Test".to_string(),
            last_name_input: "User".to_string(),
            ..App::default()
        };

        {
            let mut ui = simulator(crud.view());
            let _ = ui.click("Create")?;
        } // ui is dropped here

        crud.update(Message::CreatePressed);

        assert!(
            crud.people
                .iter()
                .any(|p| p.first_name == "Test" && p.last_name == "User")
        );

        Ok(())
    }

    fn create_test_app() -> App {
        App::default()
    }

    #[test]
    fn test_default_state() {
        let app = create_test_app();
        assert_eq!(app.filter_input, "");
        assert_eq!(app.first_name_input, "");
        assert_eq!(app.last_name_input, "");
        assert_eq!(app.people.len(), 5);
        assert_eq!(app.selected_index, None);
    }

    #[test]
    fn test_default_people() {
        let app = create_test_app();
        assert_eq!(app.people[0].first_name, "John");
        assert_eq!(app.people[0].last_name, "Doe");
        assert_eq!(app.people[1].first_name, "Jane");
        assert_eq!(app.people[1].last_name, "Smith");
        assert_eq!(app.people[2].first_name, "Bob");
        assert_eq!(app.people[2].last_name, "Johnson");
        assert_eq!(app.people[3].first_name, "Alice");
        assert_eq!(app.people[3].last_name, "Williams");
        assert_eq!(app.people[4].first_name, "Charlie");
        assert_eq!(app.people[4].last_name, "Brown");
    }

    #[test]
    fn test_filter_input_changed() {
        let mut app = create_test_app();
        app.update(Message::FilterInputChanged("test".to_string()));
        assert_eq!(app.filter_input, "test");
    }

    #[test]
    fn test_first_name_input_changed() {
        let mut app = create_test_app();
        app.update(Message::FirstNameInputChanged("Alice".to_string()));
        assert_eq!(app.first_name_input, "Alice");
    }

    #[test]
    fn test_last_name_input_changed() {
        let mut app = create_test_app();
        app.update(Message::LastNameInputChanged("Johnson".to_string()));
        assert_eq!(app.last_name_input, "Johnson");
    }

    #[test]
    fn test_item_selected() {
        let mut app = create_test_app();
        app.update(Message::ItemSelected(1));
        assert_eq!(app.selected_index, Some(1));
        assert_eq!(app.first_name_input, "Jane");
        assert_eq!(app.last_name_input, "Smith");
    }

    #[test]
    fn test_item_selected_different_index() {
        let mut app = create_test_app();
        app.update(Message::ItemSelected(3));
        assert_eq!(app.selected_index, Some(3));
        assert_eq!(app.first_name_input, "Alice");
        assert_eq!(app.last_name_input, "Williams");
    }

    #[test]
    fn test_create_pressed() {
        let mut app = create_test_app();
        app.first_name_input = "Tom".to_string();
        app.last_name_input = "Hanks".to_string();

        app.update(Message::CreatePressed);

        assert_eq!(app.people.len(), 6);
        assert_eq!(app.people[5].first_name, "Tom");
        assert_eq!(app.people[5].last_name, "Hanks");
        assert_eq!(app.first_name_input, "");
        assert_eq!(app.last_name_input, "");
    }

    #[test]
    fn test_create_pressed_clears_inputs() {
        let mut app = create_test_app();
        app.first_name_input = "Test".to_string();
        app.last_name_input = "User".to_string();

        app.update(Message::CreatePressed);

        assert_eq!(app.first_name_input, "");
        assert_eq!(app.last_name_input, "");
    }

    #[test]
    fn test_create_pressed_with_selection() {
        let mut app = create_test_app();
        app.selected_index = Some(0);
        app.first_name_input = "New".to_string();
        app.last_name_input = "Person".to_string();

        app.update(Message::CreatePressed);

        assert_eq!(app.people.len(), 6);
        assert_eq!(app.selected_index, Some(0)); // Selection should remain
    }

    #[test]
    fn test_update_pressed_with_selection() {
        let mut app = create_test_app();
        app.update(Message::ItemSelected(0));
        app.first_name_input = "Updated".to_string();
        app.last_name_input = "Name".to_string();

        app.update(Message::UpdatePressed);

        assert_eq!(app.people[0].first_name, "Updated");
        assert_eq!(app.people[0].last_name, "Name");
        assert_eq!(app.selected_index, Some(0));
        assert_eq!(app.people.len(), 5); // Should not add, only update
    }

    #[test]
    fn test_update_pressed_without_selection() {
        let mut app = create_test_app();
        app.first_name_input = "Test".to_string();
        app.last_name_input = "User".to_string();

        let original_people = app.people.clone();
        app.update(Message::UpdatePressed);

        // Nothing should change
        assert_eq!(app.people.len(), original_people.len());
        for (i, person) in app.people.iter().enumerate() {
            assert_eq!(person.first_name, original_people[i].first_name);
            assert_eq!(person.last_name, original_people[i].last_name);
        }
    }

    #[test]
    fn test_delete_pressed_with_selection() {
        let mut app = create_test_app();
        app.update(Message::ItemSelected(1));

        app.update(Message::DeletePressed);

        assert_eq!(app.people.len(), 4);
        assert_eq!(app.selected_index, None);
        assert_eq!(app.first_name_input, "");
        assert_eq!(app.last_name_input, "");
        // Verify the correct person was deleted (Jane Smith)
        assert_eq!(app.people[0].first_name, "John");
        assert_eq!(app.people[1].first_name, "Bob");
    }

    #[test]
    fn test_delete_pressed_without_selection() {
        let mut app = create_test_app();
        let original_len = app.people.len();

        app.update(Message::DeletePressed);

        assert_eq!(app.people.len(), original_len);
        assert_eq!(app.selected_index, None);
    }

    #[test]
    fn test_delete_first_item() {
        let mut app = create_test_app();
        app.update(Message::ItemSelected(0));

        app.update(Message::DeletePressed);

        assert_eq!(app.people.len(), 4);
        assert_eq!(app.people[0].first_name, "Jane");
    }

    #[test]
    fn test_delete_last_item() {
        let mut app = create_test_app();
        let last_index = app.people.len() - 1;
        app.update(Message::ItemSelected(last_index));

        app.update(Message::DeletePressed);

        assert_eq!(app.people.len(), 4);
        assert_eq!(app.people.last().unwrap().first_name, "Alice");
    }

    #[test]
    fn test_filtered_people_no_filter() {
        let app = create_test_app();
        let filtered = app.filtered_people();
        assert_eq!(filtered.len(), 5);
    }

    #[test]
    fn test_filtered_people_with_filter() {
        let mut app = create_test_app();
        app.filter_input = "S".to_string();
        let filtered = app.filtered_people();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].1.last_name, "Smith");
    }

    #[test]
    fn test_filtered_people_case_insensitive() {
        let mut app = create_test_app();
        app.filter_input = "s".to_string();
        let filtered = app.filtered_people();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].1.last_name, "Smith");
    }

    #[test]
    fn test_filtered_people_multiple_matches() {
        let mut app = create_test_app();
        app.filter_input = "B".to_string();
        let filtered = app.filtered_people();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].1.last_name, "Brown");
    }

    #[test]
    fn test_filtered_people_no_matches() {
        let mut app = create_test_app();
        app.filter_input = "Z".to_string();
        let filtered = app.filtered_people();
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filtered_people_partial_match() {
        let mut app = create_test_app();
        app.filter_input = "Jo".to_string();
        let filtered = app.filtered_people();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].1.last_name, "Johnson");
    }

    #[test]
    fn test_filtered_people_preserves_indices() {
        let mut app = create_test_app();
        app.filter_input = "W".to_string();
        let filtered = app.filtered_people();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].0, 3); // Williams is at index 3
    }

    #[test]
    fn test_person_clone() {
        let person = Person {
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
        };
        let cloned = person.clone();
        assert_eq!(cloned.first_name, person.first_name);
        assert_eq!(cloned.last_name, person.last_name);
    }

    #[test]
    fn test_message_clone() {
        let msg = Message::FilterInputChanged("test".to_string());
        let _cloned = msg.clone();
        // Just verify it compiles and doesn't panic
    }

    #[test]
    fn test_multiple_operations_sequence() {
        let mut app = create_test_app();

        // Create a new person
        app.first_name_input = "New".to_string();
        app.last_name_input = "Person".to_string();
        app.update(Message::CreatePressed);
        assert_eq!(app.people.len(), 6);

        // Select and update
        app.update(Message::ItemSelected(5));
        app.first_name_input = "Updated".to_string();
        app.last_name_input = "User".to_string();
        app.update(Message::UpdatePressed);
        assert_eq!(app.people[5].first_name, "Updated");

        // Delete the updated person
        app.update(Message::DeletePressed);
        assert_eq!(app.people.len(), 5);
        assert_eq!(app.selected_index, None);
    }

    #[test]
    fn test_update_after_delete() {
        let mut app = create_test_app();

        // Delete first item
        app.update(Message::ItemSelected(0));
        app.update(Message::DeletePressed);

        // Try to select old index
        app.update(Message::ItemSelected(0));
        assert_eq!(app.first_name_input, "Jane"); // Now points to what was index 1
    }

    #[test]
    fn test_create_empty_name() {
        let mut app = create_test_app();
        app.first_name_input = "".to_string();
        app.last_name_input = "".to_string();

        app.update(Message::CreatePressed);

        assert_eq!(app.people.len(), 6);
        assert_eq!(app.people[5].first_name, "");
        assert_eq!(app.people[5].last_name, "");
    }

    #[test]
    fn test_filter_then_select() {
        let mut app = create_test_app();
        app.filter_input = "S".to_string();

        // Select from filtered list (index in original list)
        app.update(Message::ItemSelected(1));

        assert_eq!(app.selected_index, Some(1));
        assert_eq!(app.first_name_input, "Jane");
        assert_eq!(app.last_name_input, "Smith");
    }
}
