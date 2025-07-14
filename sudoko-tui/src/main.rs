//! Terminal User Interface for the Sudoku solver using ratatui

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
    Frame, Terminal,
};
use std::io;
use sudoko::{Difficulty, Sudoku, SudokuSolver};

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppMode {
    Normal,
    Help,
}

struct SudokuApp {
    puzzle: Sudoku,
    cursor: (usize, usize),
    message: String,
    is_solved: bool,
    mode: AppMode,
    should_quit: bool,
    // hint_tracker: Vec<(usize, usize, u8)>, // Track given hints to avoid repeating
}

impl SudokuApp {
    fn new() -> Self {
        let puzzle_str =
            "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
        let puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();

        Self {
            puzzle,
            cursor: (0, 0),
            message: "Welcome to Sudoku! Use arrow keys to move, 1-9 to fill cells, ? for help"
                .to_string(),
            is_solved: false,
            mode: AppMode::Normal,
            should_quit: false,
            // hint_tracker: Vec::new(),
        }
    }

    fn handle_key(&mut self, key: KeyCode) {
        match self.mode {
            AppMode::Help => {
                if matches!(key, KeyCode::Char('?') | KeyCode::Esc | KeyCode::Enter) {
                    self.mode = AppMode::Normal;
                }
            }
            AppMode::Normal => match key {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.should_quit = true;
                }
                KeyCode::Char('?') => {
                    self.mode = AppMode::Help;
                }
                KeyCode::Up => self.move_cursor(-1, 0),
                KeyCode::Down => self.move_cursor(1, 0),
                KeyCode::Left => self.move_cursor(0, -1),
                KeyCode::Right => self.move_cursor(0, 1),
                KeyCode::Char('s') | KeyCode::Char('S') => self.solve_puzzle(),
                KeyCode::Char('r') | KeyCode::Char('R') => self.reset_puzzle(),
                KeyCode::Char('h') | KeyCode::Char('H') => self.get_hint(),
                KeyCode::Char('e') | KeyCode::Char('E') => {
                    self.generate_new_puzzle(Difficulty::Easy)
                }
                KeyCode::Char('m') | KeyCode::Char('M') => {
                    self.generate_new_puzzle(Difficulty::Medium)
                }
                KeyCode::Char('d') | KeyCode::Char('D') => {
                    self.generate_new_puzzle(Difficulty::Hard)
                }
                KeyCode::Char('x') | KeyCode::Char('X') => {
                    self.generate_new_puzzle(Difficulty::Expert)
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    if let Some(digit) = c.to_digit(10) {
                        let value = digit as u8;
                        if value <= 9 {
                            self.set_value(value);
                        } else {
                            self.message = "Invalid digit: only 1-9 are allowed".to_string();
                        }
                    } else {
                        self.message = "Invalid input: not a valid digit".to_string();
                    }
                }
                KeyCode::Delete | KeyCode::Backspace | KeyCode::Char('0') => {
                    self.set_value(0);
                }
                _ => {}
            },
        }
    }

    fn move_cursor(&mut self, dr: i32, dc: i32) {
        let new_row = (self.cursor.0 as i32 + dr)
            .max(0)
            .min(self.puzzle.size as i32 - 1) as usize;
        let new_col = (self.cursor.1 as i32 + dc)
            .max(0)
            .min(self.puzzle.size as i32 - 1) as usize;
        self.cursor = (new_row, new_col);
    }

    fn set_value(&mut self, value: u8) {
        let (row, col) = self.cursor;

        // Check if the cell is a given (preset) cell
        if let Some(cell) = self.puzzle.get(row, col) {
            if cell.is_given() {
                self.message = "Cannot modify given cells!".to_string();
                return;
            }
        }

        match self.puzzle.set(row, col, value) {
            Ok(_) => {
                if value == 0 {
                    self.message = format!("Cleared cell ({}, {})", row + 1, col + 1);
                } else {
                    self.message = format!("Set {} at ({}, {})", value, row + 1, col + 1);
                }

                // Check if puzzle is complete
                if self.puzzle.is_complete() && self.puzzle.is_valid() {
                    self.is_solved = true;
                    self.message = "🎉 Congratulations! Puzzle solved!".to_string();
                }
            }
            Err(e) => {
                self.message = format!("Error: {}", e);
            }
        }
    }

    fn solve_puzzle(&mut self) {
        let mut solver = SudokuSolver::new();
        match solver.solve(self.puzzle.clone()) {
            Ok(solution) => {
                self.puzzle = solution;
                self.is_solved = true;
                self.message = "Puzzle solved automatically!".to_string();
            }
            Err(_) => {
                self.message = "No solution found for this puzzle.".to_string();
            }
        }
    }

    fn reset_puzzle(&mut self) {
        let puzzle_str =
            "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
        self.puzzle = Sudoku::from_string(puzzle_str, 9).unwrap();
        self.cursor = (0, 0);
        self.is_solved = false;
        self.message = "Puzzle reset to original state.".to_string();
    }

    fn generate_new_puzzle(&mut self, difficulty: Difficulty) {
        let mut solver = SudokuSolver::new();
        match solver.generate_puzzle(9, difficulty) {
            Ok(new_puzzle) => {
                self.puzzle = new_puzzle;
                self.cursor = (0, 0);
                self.is_solved = false;
                self.message = format!("Generated new {:?} puzzle!", difficulty);
            }
            Err(_) => {
                self.message = "Failed to generate new puzzle.".to_string();
            }
        }
    }

    fn get_hint(&mut self) {
        let mut solver = SudokuSolver::new();
        let mut temp_puzzle = self.puzzle.clone();
        match solver.get_hint(&mut temp_puzzle) {
            Some((row, col, value)) => {
                self.cursor = (row, col);
                self.message = format!("Hint: Try {} at ({}, {})", value, row + 1, col + 1);
            }
            None => {
                self.message = "No obvious hints available.".to_string();
            }
        }
    }
}

fn draw_sudoku_grid(f: &mut Frame, app: &SudokuApp, area: Rect) {
    // Create the table data
    let mut rows = Vec::new();

    for row in 0..app.puzzle.size {
        let mut cells = Vec::new();

        for col in 0..app.puzzle.size {
            let cell = app.puzzle.get(row, col).unwrap();
            let is_cursor = app.cursor == (row, col);

            let value_str = match cell.value() {
                Some(v) if v > 0 => format!(" {} ", v),
                _ => "   ".to_string(),
            };

            let cell_style = if is_cursor {
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else if cell.is_given() {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            cells.push(Cell::from(value_str).style(cell_style));

            // Add vertical separator for 3x3 blocks
            if (col + 1) % 3 == 0 && col < app.puzzle.size - 1 {
                cells.push(
                    Cell::from("│").style(
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                );
            }
        }

        rows.push(Row::new(cells).height(1));

        // Add horizontal separator for 3x3 blocks
        if (row + 1) % 3 == 0 && row < app.puzzle.size - 1 {
            let mut separator_cells = Vec::new();
            for col in 0..app.puzzle.size {
                separator_cells.push(
                    Cell::from("───").style(
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                );
                if (col + 1) % 3 == 0 && col < app.puzzle.size - 1 {
                    separator_cells.push(
                        Cell::from("┼").style(
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ),
                    );
                }
            }
            rows.push(Row::new(separator_cells).height(1));
        }
    }

    // Create column constraints with separators
    let mut widths = Vec::new();
    for col in 0..app.puzzle.size {
        widths.push(Constraint::Length(3));
        if (col + 1) % 3 == 0 && col < app.puzzle.size - 1 {
            widths.push(Constraint::Length(1)); // For separator
        }
    }

    let table = Table::new(rows, widths)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Sudoku Grid")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White))
        .column_spacing(0);

    f.render_widget(table, area);
}

fn draw_status_info(f: &mut Frame, app: &SudokuApp, area: Rect) {
    let status_style = if app.is_solved {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else if app.message.contains("Error") || app.message.contains("Cannot") {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::White)
    };

    let status_text = vec![
        Line::from(vec![
            Span::styled("Position: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("({}, {})", app.cursor.0 + 1, app.cursor.1 + 1),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::Gray)),
            Span::styled(&app.message, status_style),
        ]),
    ];

    let status_paragraph = Paragraph::new(status_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Status")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(status_paragraph, area);
}

fn draw_controls(f: &mut Frame, area: Rect) {
    let controls_text = vec![
        Line::from("Controls:"),
        Line::from(""),
        Line::from(vec![
            Span::styled("↑↓←→", Style::default().fg(Color::Yellow)),
            Span::raw(" Move cursor  "),
            Span::styled("1-9", Style::default().fg(Color::Yellow)),
            Span::raw(" Fill cell  "),
            Span::styled("0/Del", Style::default().fg(Color::Yellow)),
            Span::raw(" Clear"),
        ]),
        Line::from(vec![
            Span::styled("S", Style::default().fg(Color::Green)),
            Span::raw(" Solve  "),
            Span::styled("R", Style::default().fg(Color::Blue)),
            Span::raw(" Reset  "),
            Span::styled("H", Style::default().fg(Color::Magenta)),
            Span::raw(" Hint"),
        ]),
        Line::from(vec![
            Span::styled("E", Style::default().fg(Color::Green)),
            Span::raw(" Easy  "),
            Span::styled("M", Style::default().fg(Color::Yellow)),
            Span::raw(" Medium  "),
            Span::styled("D", Style::default().fg(Color::Red)),
            Span::raw(" Hard"),
        ]),
        Line::from(vec![
            Span::styled("?", Style::default().fg(Color::Cyan)),
            Span::raw(" Help  "),
            Span::styled("Q", Style::default().fg(Color::Red)),
            Span::raw(" Quit"),
        ]),
    ];

    let controls_paragraph = Paragraph::new(controls_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Controls")
            .border_style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(controls_paragraph, area);
}

fn draw_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![Span::styled(
            "🧩 SUDOKU HELP 🧩",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("OBJECTIVE:"),
        Line::from("Fill the 9×9 grid so that each row, column, and 3×3 box"),
        Line::from("contains all digits from 1 to 9."),
        Line::from(""),
        Line::from("CONTROLS:"),
        Line::from("• Arrow keys: Move cursor around the grid"),
        Line::from("• Numbers 1-9: Fill the selected cell"),
        Line::from("• 0, Delete, Backspace: Clear the selected cell"),
        Line::from("• S: Automatically solve the entire puzzle"),
        Line::from("• R: Reset puzzle to original state"),
        Line::from("• H: Get a hint for the current puzzle"),
        Line::from(""),
        Line::from("PUZZLE GENERATION:"),
        Line::from("• E: Generate new Easy puzzle"),
        Line::from("• M: Generate new Medium puzzle"),
        Line::from("• D: Generate new Hard puzzle"),
        Line::from("• X: Generate new Expert puzzle"),
        Line::from(""),
        Line::from("VISUAL CUES:"),
        Line::from(vec![
            Span::raw("• "),
            Span::styled(
                "Yellow background",
                Style::default().bg(Color::Yellow).fg(Color::Black),
            ),
            Span::raw(": Current cursor position"),
        ]),
        Line::from(vec![
            Span::raw("• "),
            Span::styled(
                "Cyan numbers",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(": Given/preset numbers (cannot be changed)"),
        ]),
        Line::from(vec![
            Span::raw("• "),
            Span::styled("White numbers", Style::default().fg(Color::White)),
            Span::raw(": Numbers you filled in"),
        ]),
        Line::from(""),
        Line::from("Press ? again, Esc, or Enter to close this help."),
    ];

    let help_paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .border_style(Style::default().fg(Color::Green)),
        )
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });

    // Clear the background and render the help
    f.render_widget(Clear, area);
    f.render_widget(help_paragraph, area);
}

fn ui(f: &mut Frame, app: &SudokuApp) {
    if app.mode == AppMode::Help {
        // Calculate a centered area for the help dialog
        let area = f.area();
        let help_area = centered_rect(80, 80, area);
        draw_help(f, help_area);
        return;
    }

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(15),    // Main content
            Constraint::Length(10), // Controls
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🧩 SUDOKU 🧩")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content area
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(30), Constraint::Length(40)])
        .split(chunks[1]);

    // Sudoku grid
    draw_sudoku_grid(f, app, main_chunks[0]);

    // Status info
    draw_status_info(f, app, main_chunks[1]);

    // Controls
    draw_controls(f, chunks[2]);
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = SudokuApp::new();

    // Main loop
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_key(key.code);
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
