use iced::widget::{Column, Row, button, container, scrollable, text, text_input};
use iced::{Element, Length, Task};
use std::collections::{HashMap, HashSet};

fn main() -> iced::Result {
    iced::application(Cells::new, Cells::update, Cells::view)
        .window_size((800.0, 600.0))
        .run()
}

const ROWS: usize = 100;
const COLS: usize = 26;

#[derive(Debug, Clone)]
enum Message {
    CellClicked(usize, usize),
    FormulaChanged(String),
    FinishEditing,
}

#[derive(Debug, Clone, PartialEq)]
enum CellValue {
    Number(f64),
    Text(String),
    Error(String),
}

struct Cells {
    // Cell data: formula input by user
    formulas: HashMap<(usize, usize), String>,
    // Cell data: evaluated value
    values: HashMap<(usize, usize), CellValue>,
    // Dependency tracking: which cells does each cell depend on
    dependencies: HashMap<(usize, usize), HashSet<(usize, usize)>>,
    // Reverse dependencies: which cells depend on this cell
    dependents: HashMap<(usize, usize), HashSet<(usize, usize)>>,
    // Currently editing cell
    editing_cell: Option<(usize, usize)>,
    // Current formula being edited
    editing_formula: String,
}

impl Cells {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                formulas: HashMap::new(),
                values: HashMap::new(),
                dependencies: HashMap::new(),
                dependents: HashMap::new(),
                editing_cell: None,
                editing_formula: String::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CellClicked(row, col) => {
                // If clicking a different cell while editing, finish current edit
                if let Some((editing_row, editing_col)) = self.editing_cell
                    && (editing_row, editing_col) != (row, col) {
                    self.update_cell(editing_row, editing_col, self.editing_formula.clone());
                    self.editing_cell = None;
                    self.editing_formula.clear();
                }

                // Single-click to edit - start editing the cell immediately
                if self.editing_cell != Some((row, col)) {
                    let formula = self.formulas.get(&(row, col)).cloned().unwrap_or_default();
                    self.editing_cell = Some((row, col));
                    self.editing_formula = formula;
                }
            }
            Message::FormulaChanged(new_formula) => {
                self.editing_formula = new_formula;
            }
            Message::FinishEditing => {
                if let Some((row, col)) = self.editing_cell {
                    self.update_cell(row, col, self.editing_formula.clone());
                    self.editing_cell = None;
                    self.editing_formula.clear();
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        // Create complete grid (headers + data, all together)
        let grid = self.create_complete_grid();

        let scrollable_grid = scrollable(grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .direction(scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::default(),
                horizontal: scrollable::Scrollbar::default(),
            });

        container(scrollable_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    fn create_complete_grid(&self) -> Element<'_, Message> {
        let mut grid = Column::new();

        // Add column headers as first row
        let mut header_row = Row::new();

        // Corner cell
        header_row = header_row.push(button(text("")).width(60).height(30).padding(5).style(
            |theme: &iced::Theme, _status| {
                let palette = theme.palette();
                button::Style {
                    background: Some(iced::Background::Color(palette.background)),
                    border: iced::Border {
                        color: palette.text.scale_alpha(0.3),
                        width: 0.5,
                        radius: 0.0.into(),
                    },
                    text_color: palette.text,
                    ..Default::default()
                }
            },
        ));

        // Column headers (A, B, C...)
        for col in 0..COLS {
            header_row = header_row.push(
                button(text(col_to_letter(col)).size(14))
                    .width(80)
                    .height(30)
                    .padding(5)
                    .style(|theme: &iced::Theme, _status| {
                        let palette = theme.palette();
                        button::Style {
                            background: Some(iced::Background::Color(palette.background)),
                            border: iced::Border {
                                color: palette.text.scale_alpha(0.3),
                                width: 0.5,
                                radius: 0.0.into(),
                            },
                            text_color: palette.text,
                            ..Default::default()
                        }
                    }),
            );
        }
        grid = grid.push(header_row);

        // Data rows
        for row in 0..ROWS {
            let mut data_row = Row::new();

            // Row header
            data_row = data_row.push(
                button(text(format!("{}", row)).size(14))
                    .width(60)
                    .height(30)
                    .padding(5)
                    .style(|theme: &iced::Theme, _status| {
                        let palette = theme.palette();
                        button::Style {
                            background: Some(iced::Background::Color(palette.background)),
                            border: iced::Border {
                                color: palette.text.scale_alpha(0.3),
                                width: 0.5,
                                radius: 0.0.into(),
                            },
                            text_color: palette.text,
                            ..Default::default()
                        }
                    }),
            );

            // Data cells
            for col in 0..COLS {
                let is_editing = self.editing_cell == Some((row, col));

                // Show editing formula if this cell is being edited, otherwise show the value
                let cell_content = if is_editing {
                    self.editing_formula.clone()
                } else {
                    self.get_cell_display(row, col)
                };

                // Determine alignment: numbers right, text left
                let is_number = self.is_cell_number(row, col);

                // Create cell widget with consistent structure
                let cell_widget: Element<'_, Message> = if is_editing {
                    // Editing cell - show text input with primary border
                    container(
                        text_input("", &cell_content)
                            .on_input(Message::FormulaChanged)
                            .on_submit(Message::FinishEditing)
                            .size(14)
                            .padding([5, 5])
                            .style(|theme: &iced::Theme, _status| {
                                let palette = theme.palette();
                                text_input::Style {
                                    background: iced::Background::Color(palette.background),
                                    border: iced::Border {
                                        color: iced::Color::TRANSPARENT,
                                        width: 0.0,
                                        radius: 0.0.into(),
                                    },
                                    icon: palette.text,
                                    placeholder: palette.text,
                                    value: palette.text,
                                    selection: palette.primary,
                                }
                            }),
                    )
                    .width(80)
                    .height(30)
                    .style(|theme: &iced::Theme| {
                        let palette = theme.palette();
                        container::Style {
                            border: iced::Border {
                                color: palette.primary,
                                width: 1.5,
                                radius: 0.0.into(),
                            },
                            background: Some(iced::Background::Color(palette.background)),
                            ..Default::default()
                        }
                    })
                    .into()
                } else {
                    // Normal cell - show as button with aligned text
                    let text_widget = if is_number {
                        // Numbers: right-aligned
                        container(text(cell_content).size(14))
                            .width(Length::Fill)
                            .align_right(70)
                    } else {
                        // Text: left-aligned
                        container(text(cell_content).size(14))
                            .width(Length::Fill)
                            .align_left(5)
                    };

                    button(text_widget)
                        .on_press(Message::CellClicked(row, col))
                        .width(80)
                        .height(30)
                        .padding(5)
                        .style(|theme: &iced::Theme, _status| {
                            let palette = theme.palette();
                            button::Style {
                                background: Some(iced::Background::Color(palette.background)),
                                border: iced::Border {
                                    color: palette.text.scale_alpha(0.3),
                                    width: 0.5,
                                    radius: 0.0.into(),
                                },
                                text_color: palette.text,
                                ..Default::default()
                            }
                        })
                        .into()
                };

                data_row = data_row.push(cell_widget);
            }
            grid = grid.push(data_row);
        }

        grid.into()
    }

    fn get_cell_display(&self, row: usize, col: usize) -> String {
        match self.values.get(&(row, col)) {
            Some(CellValue::Number(value)) => format!("{:.2}", value),
            Some(CellValue::Text(text)) => text.clone(),
            Some(CellValue::Error(err)) => format!("#{}", err),
            None => String::new(),
        }
    }

    fn is_cell_number(&self, row: usize, col: usize) -> bool {
        matches!(self.values.get(&(row, col)), Some(CellValue::Number(_)))
    }

    fn update_cell(&mut self, row: usize, col: usize, formula: String) {
        let formula = formula.trim().to_string();

        // Remove old dependencies
        if let Some(old_deps) = self.dependencies.remove(&(row, col)) {
            for dep in old_deps {
                if let Some(rev_deps) = self.dependents.get_mut(&dep) {
                    rev_deps.remove(&(row, col));
                }
            }
        }

        // Update or remove the formula
        if formula.is_empty() {
            self.formulas.remove(&(row, col));
            self.values.remove(&(row, col));
        } else {
            self.formulas.insert((row, col), formula.clone());

            // Parse and evaluate the formula
            let deps = parse_dependencies(&formula);
            if !deps.is_empty() {
                self.dependencies.insert((row, col), deps.clone());
                for dep in deps {
                    self.dependents
                        .entry(dep)
                        .or_default()
                        .insert((row, col));
                }
            }

            let value = self.evaluate_formula(&formula);
            self.values.insert((row, col), value);
        }

        // Propagate changes to dependent cells
        self.propagate_changes(row, col);
    }

    fn evaluate_formula(&self, formula: &str) -> CellValue {
        let formula = formula.trim();

        // Empty formula
        if formula.is_empty() {
            return CellValue::Text(String::new());
        }

        // If it starts with '=', it's a formula
        if let Some(stripped) = formula.strip_prefix('=') {
            match self.evaluate_expression(stripped) {
                Ok(num) => CellValue::Number(num),
                Err(err) => CellValue::Error(err),
            }
        } else {
            // Try to parse as a number
            match formula.parse::<f64>() {
                Ok(num) => CellValue::Number(num),
                Err(_) => CellValue::Text(formula.to_string()), // It's text/label
            }
        }
    }

    fn evaluate_expression(&self, expr: &str) -> Result<f64, String> {
        let expr = expr.trim();

        // Try to parse as a simple number first
        if let Ok(num) = expr.parse::<f64>() {
            return Ok(num);
        }

        // Check if it's a cell reference (e.g., "A0", "Z99")
        if let Some(cell_ref) = parse_cell_reference(expr) {
            return match self.values.get(&cell_ref) {
                Some(CellValue::Number(n)) => Ok(*n),
                Some(CellValue::Text(_)) => Err("TEXT".to_string()),
                Some(CellValue::Error(e)) => Err(e.clone()),
                None => Ok(0.0),
            };
        }

        // Handle arithmetic expressions
        // Support: +, -, *, /
        for (i, ch) in expr.char_indices() {
            if ch == '+' || ch == '-' || ch == '*' || ch == '/' {
                let left = self.evaluate_expression(&expr[..i])?;
                let right = self.evaluate_expression(&expr[i + 1..])?;

                return match ch {
                    '+' => Ok(left + right),
                    '-' => Ok(left - right),
                    '*' => Ok(left * right),
                    '/' => {
                        if right == 0.0 {
                            Err("DIV0".to_string())
                        } else {
                            Ok(left / right)
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }

        Err("ERR".to_string())
    }

    fn propagate_changes(&mut self, row: usize, col: usize) {
        let mut to_update: Vec<(usize, usize)> = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        // Collect all dependent cells
        if let Some(deps) = self.dependents.get(&(row, col)) {
            to_update.extend(deps.iter().copied());
        }

        while let Some(cell) = to_update.pop() {
            if visited.contains(&cell) {
                continue;
            }
            visited.insert(cell);

            // Re-evaluate this cell
            if let Some(formula) = self.formulas.get(&cell).cloned() {
                let value = self.evaluate_formula(&formula);
                self.values.insert(cell, value);

                // Add its dependents to the queue
                if let Some(deps) = self.dependents.get(&cell) {
                    to_update.extend(deps.iter().copied());
                }
            }
        }
    }
}

fn col_to_letter(col: usize) -> String {
    ((b'A' + col as u8) as char).to_string()
}

fn letter_to_col(letter: char) -> Option<usize> {
    let letter = letter.to_ascii_uppercase();
    if letter.is_ascii_uppercase() {
        Some((letter as u8 - b'A') as usize)
    } else {
        None
    }
}

fn parse_cell_reference(s: &str) -> Option<(usize, usize)> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let mut chars = s.chars();
    let first_char = chars.next()?;
    let col = letter_to_col(first_char)?;

    let row_str: String = chars.collect();
    let row = row_str.parse::<usize>().ok()?;

    if row < ROWS && col < COLS {
        Some((row, col))
    } else {
        None
    }
}

fn parse_dependencies(formula: &str) -> HashSet<(usize, usize)> {
    let mut deps = HashSet::new();
    let formula = formula.trim();

    if let Some(expr) = formula.strip_prefix('=') {
        // Simple parser: look for patterns like A0, B5, Z99
        let mut i = 0;
        let chars: Vec<char> = expr.chars().collect();
        while i < chars.len() {
            if chars[i].is_ascii_uppercase() {
                let col_char = chars[i];
                i += 1;

                // Collect digits for row number
                let mut row_str = String::new();
                while i < chars.len() && chars[i].is_ascii_digit() {
                    row_str.push(chars[i]);
                    i += 1;
                }

                if !row_str.is_empty()
                    && let Some(cell_ref) =
                        parse_cell_reference(&format!("{}{}", col_char, row_str))
                {
                    deps.insert(cell_ref);
                }
            } else {
                i += 1;
            }
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_col_to_letter() {
        assert_eq!(col_to_letter(0), "A");
        assert_eq!(col_to_letter(1), "B");
        assert_eq!(col_to_letter(25), "Z");
    }

    #[test]
    fn test_letter_to_col() {
        assert_eq!(letter_to_col('A'), Some(0));
        assert_eq!(letter_to_col('a'), Some(0));
        assert_eq!(letter_to_col('B'), Some(1));
        assert_eq!(letter_to_col('Z'), Some(25));
        assert_eq!(letter_to_col('0'), None);
        assert_eq!(letter_to_col('!'), None);
    }

    #[test]
    fn test_parse_cell_reference() {
        assert_eq!(parse_cell_reference("A0"), Some((0, 0)));
        assert_eq!(parse_cell_reference("B5"), Some((5, 1)));
        assert_eq!(parse_cell_reference("Z99"), Some((99, 25)));
        assert_eq!(parse_cell_reference("A100"), None); // Row out of bounds
        assert_eq!(parse_cell_reference("AA0"), None); // Invalid format
        assert_eq!(parse_cell_reference(""), None);
        assert_eq!(parse_cell_reference("A"), None);
        assert_eq!(parse_cell_reference("0"), None);
    }

    #[test]
    fn test_parse_dependencies_simple() {
        let deps = parse_dependencies("=A0");
        assert_eq!(deps.len(), 1);
        assert!(deps.contains(&(0, 0)));
    }

    #[test]
    fn test_parse_dependencies_multiple() {
        let deps = parse_dependencies("=A0+B5");
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&(0, 0)));
        assert!(deps.contains(&(5, 1)));
    }

    #[test]
    fn test_parse_dependencies_complex() {
        let deps = parse_dependencies("=A1+B2*C3-D4/E5");
        assert_eq!(deps.len(), 5);
        assert!(deps.contains(&(1, 0)));
        assert!(deps.contains(&(2, 1)));
        assert!(deps.contains(&(3, 2)));
        assert!(deps.contains(&(4, 3)));
        assert!(deps.contains(&(5, 4)));
    }

    #[test]
    fn test_parse_dependencies_no_formula() {
        let deps = parse_dependencies("123");
        assert_eq!(deps.len(), 0);
    }

    #[test]
    fn test_parse_dependencies_empty() {
        let deps = parse_dependencies("");
        assert_eq!(deps.len(), 0);
    }

    #[test]
    fn test_evaluate_formula_number() {
        let cells = Cells::new().0;
        let result = cells.evaluate_formula("123");
        assert!(matches!(result, CellValue::Number(n) if (n - 123.0).abs() < 0.001));
    }

    #[test]
    fn test_evaluate_formula_decimal() {
        let cells = Cells::new().0;
        let result = cells.evaluate_formula("45.67");
        assert!(matches!(result, CellValue::Number(n) if (n - 45.67).abs() < 0.001));
    }

    #[test]
    fn test_evaluate_formula_text() {
        let cells = Cells::new().0;
        let result = cells.evaluate_formula("Hello");
        assert!(matches!(result, CellValue::Text(s) if s == "Hello"));
    }

    #[test]
    fn test_evaluate_formula_simple_addition() {
        let cells = Cells::new().0;
        let result = cells.evaluate_formula("=5+3");
        assert!(matches!(result, CellValue::Number(n) if (n - 8.0).abs() < 0.001));
    }

    #[test]
    fn test_evaluate_formula_subtraction() {
        let cells = Cells::new().0;
        let result = cells.evaluate_formula("=10-4");
        assert!(matches!(result, CellValue::Number(n) if (n - 6.0).abs() < 0.001));
    }

    #[test]
    fn test_evaluate_formula_multiplication() {
        let cells = Cells::new().0;
        let result = cells.evaluate_formula("=6*7");
        assert!(matches!(result, CellValue::Number(n) if (n - 42.0).abs() < 0.001));
    }

    #[test]
    fn test_evaluate_formula_division() {
        let cells = Cells::new().0;
        let result = cells.evaluate_formula("=20/4");
        assert!(matches!(result, CellValue::Number(n) if (n - 5.0).abs() < 0.001));
    }

    #[test]
    fn test_evaluate_formula_division_by_zero() {
        let cells = Cells::new().0;
        let result = cells.evaluate_formula("=10/0");
        assert!(matches!(result, CellValue::Error(e) if e == "DIV0"));
    }

    #[test]
    fn test_evaluate_formula_invalid() {
        let cells = Cells::new().0;
        let result = cells.evaluate_formula("=ABC");
        assert!(matches!(result, CellValue::Error(_)));
    }

    #[test]
    fn test_evaluate_formula_cell_reference() {
        let mut cells = Cells::new().0;
        // Set A0 to 42
        cells.values.insert((0, 0), CellValue::Number(42.0));

        let result = cells.evaluate_formula("=A0");
        assert!(matches!(result, CellValue::Number(n) if (n - 42.0).abs() < 0.001));
    }

    #[test]
    fn test_evaluate_formula_cell_reference_addition() {
        let mut cells = Cells::new().0;
        cells.values.insert((0, 0), CellValue::Number(10.0));
        cells.values.insert((1, 1), CellValue::Number(20.0));

        let result = cells.evaluate_formula("=A0+B1");
        assert!(matches!(result, CellValue::Number(n) if (n - 30.0).abs() < 0.001));
    }

    #[test]
    fn test_evaluate_formula_text_in_formula() {
        let mut cells = Cells::new().0;
        cells
            .values
            .insert((0, 0), CellValue::Text("Hello".to_string()));

        let result = cells.evaluate_formula("=A0+5");
        assert!(matches!(result, CellValue::Error(e) if e == "TEXT"));
    }

    #[test]
    fn test_update_cell_number() {
        let mut cells = Cells::new().0;
        cells.update_cell(0, 0, "42".to_string());

        assert_eq!(cells.formulas.get(&(0, 0)), Some(&"42".to_string()));
        assert!(
            matches!(cells.values.get(&(0, 0)), Some(CellValue::Number(n)) if (n - 42.0).abs() < 0.001)
        );
    }

    #[test]
    fn test_update_cell_text() {
        let mut cells = Cells::new().0;
        cells.update_cell(0, 0, "Hello".to_string());

        assert_eq!(cells.formulas.get(&(0, 0)), Some(&"Hello".to_string()));
        assert!(matches!(cells.values.get(&(0, 0)), Some(CellValue::Text(s)) if s == "Hello"));
    }

    #[test]
    fn test_update_cell_formula() {
        let mut cells = Cells::new().0;
        cells.update_cell(0, 0, "=5+3".to_string());

        assert_eq!(cells.formulas.get(&(0, 0)), Some(&"=5+3".to_string()));
        assert!(
            matches!(cells.values.get(&(0, 0)), Some(CellValue::Number(n)) if (n - 8.0).abs() < 0.001)
        );
    }

    #[test]
    fn test_update_cell_clear() {
        let mut cells = Cells::new().0;
        cells.update_cell(0, 0, "42".to_string());
        cells.update_cell(0, 0, "".to_string());

        assert_eq!(cells.formulas.get(&(0, 0)), None);
        assert_eq!(cells.values.get(&(0, 0)), None);
    }

    #[test]
    fn test_dependency_tracking() {
        let mut cells = Cells::new().0;
        cells.update_cell(0, 0, "10".to_string());
        cells.update_cell(1, 1, "=A0+5".to_string());

        // B1 should depend on A0
        assert!(cells.dependencies.get(&(1, 1)).unwrap().contains(&(0, 0)));
        // A0 should have B1 as a dependent
        assert!(cells.dependents.get(&(0, 0)).unwrap().contains(&(1, 1)));
    }

    #[test]
    fn test_change_propagation() {
        let mut cells = Cells::new().0;
        cells.update_cell(0, 0, "10".to_string());
        cells.update_cell(1, 1, "=A0*2".to_string());

        // B1 should be 20
        assert!(
            matches!(cells.values.get(&(1, 1)), Some(CellValue::Number(n)) if (n - 20.0).abs() < 0.001)
        );

        // Update A0
        cells.update_cell(0, 0, "15".to_string());

        // B1 should now be 30
        assert!(
            matches!(cells.values.get(&(1, 1)), Some(CellValue::Number(n)) if (n - 30.0).abs() < 0.001)
        );
    }

    #[test]
    fn test_transitive_dependencies() {
        let mut cells = Cells::new().0;
        cells.update_cell(0, 0, "5".to_string());
        cells.update_cell(1, 1, "=A0*2".to_string());
        cells.update_cell(2, 2, "=B1+10".to_string());

        // C2 should be 20 (5*2+10)
        assert!(
            matches!(cells.values.get(&(2, 2)), Some(CellValue::Number(n)) if (n - 20.0).abs() < 0.001)
        );

        // Update A0
        cells.update_cell(0, 0, "10".to_string());

        // B1 should be 20, C2 should be 30
        assert!(
            matches!(cells.values.get(&(1, 1)), Some(CellValue::Number(n)) if (n - 20.0).abs() < 0.001)
        );
        assert!(
            matches!(cells.values.get(&(2, 2)), Some(CellValue::Number(n)) if (n - 30.0).abs() < 0.001)
        );
    }

    #[test]
    fn test_get_cell_display_number() {
        let mut cells = Cells::new().0;
        cells.values.insert((0, 0), CellValue::Number(42.5));

        assert_eq!(cells.get_cell_display(0, 0), "42.50");
    }

    #[test]
    fn test_get_cell_display_integer() {
        let mut cells = Cells::new().0;
        cells.values.insert((0, 0), CellValue::Number(42.0));

        assert_eq!(cells.get_cell_display(0, 0), "42.00");
    }

    #[test]
    fn test_get_cell_display_text() {
        let mut cells = Cells::new().0;
        cells
            .values
            .insert((0, 0), CellValue::Text("Hello".to_string()));

        assert_eq!(cells.get_cell_display(0, 0), "Hello");
    }

    #[test]
    fn test_get_cell_display_error() {
        let mut cells = Cells::new().0;
        cells
            .values
            .insert((0, 0), CellValue::Error("DIV0".to_string()));

        assert_eq!(cells.get_cell_display(0, 0), "#DIV0");
    }

    #[test]
    fn test_get_cell_display_empty() {
        let cells = Cells::new().0;
        assert_eq!(cells.get_cell_display(0, 0), "");
    }

    #[test]
    fn test_is_cell_number() {
        let mut cells = Cells::new().0;
        cells.values.insert((0, 0), CellValue::Number(42.0));
        cells
            .values
            .insert((1, 1), CellValue::Text("Hello".to_string()));
        cells
            .values
            .insert((2, 2), CellValue::Error("ERR".to_string()));

        assert!(cells.is_cell_number(0, 0));
        assert!(!cells.is_cell_number(1, 1));
        assert!(!cells.is_cell_number(2, 2));
        assert!(!cells.is_cell_number(3, 3)); // Empty cell
    }

    #[test]
    fn test_dependency_removal_on_update() {
        let mut cells = Cells::new().0;
        cells.update_cell(0, 0, "10".to_string());
        cells.update_cell(1, 1, "=A0+5".to_string());

        // B1 depends on A0
        assert!(cells.dependencies.get(&(1, 1)).unwrap().contains(&(0, 0)));

        // Change B1 to not depend on A0
        cells.update_cell(1, 1, "20".to_string());

        // B1 should no longer have dependencies
        assert_eq!(cells.dependencies.get(&(1, 1)), None);
        // A0 should no longer have B1 as dependent
        assert!(
            !cells.dependents.contains_key(&(0, 0))
                || !cells.dependents.get(&(0, 0)).unwrap().contains(&(1, 1))
        );
    }

    #[test]
    fn test_message_debug() {
        let msg = Message::CellClicked(5, 10);
        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("CellClicked"));
    }

    #[test]
    fn test_cell_value_debug() {
        let val = CellValue::Number(42.0);
        let debug_str = format!("{:?}", val);
        assert!(debug_str.contains("Number"));
    }
}
