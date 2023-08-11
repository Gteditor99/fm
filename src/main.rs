use std::fs;
use std::io::{self, stdout, Write};
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::widgets::Paragraph;
use ratatui::text::Text;
use ratatui::Terminal;
use ratatui::text::Span;
use serde_json::error;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Clear the screen
    crossterm::execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;

    // Get the files in the current directory
    let mut files: Vec<PathBuf> = fs::read_dir(".")?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    // Initialize the terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut selected_files = vec![false; files.len()];
    let mut selection_index = 0;
    let mut exit = false;
    let mut error_message = String::new();
    loop {
        // Draw the UI
        terminal.draw(|frame| {
            // Create a list widget for the files
            let items: Vec<ListItem> = files.iter().enumerate().map(|(i, file)| {
                let style = if selected_files[i] {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                };
                ListItem::new(file.display().to_string()).style(style)
            }).collect();

            let files_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Files"))
                .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
                .highlight_symbol("> ");

            let error_message_span = Span::styled(
                &error_message,
                Style::default().fg(Color::Red),
            );

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(frame.size());

            frame.render_widget(files_list, chunks[0]);

            let error_message_paragraph = Paragraph::new(error_message_span);
            frame.render_widget(error_message_paragraph, chunks[1]);
            
        })?;

        // Handle user input
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                match event.code {
                    // Quit the program or selection mode
                    KeyCode::Char('q') => {
                        exit = true;
                    }

                    // Quit the program
                    KeyCode::Char('e') => {
                        exit = true;
                    }

                    // Move the selection up
                    KeyCode::Char('k') => {
                        if selection_index > 0 {
                            selected_files[selection_index] = false;
                            selection_index -= 1;
                            selected_files[selection_index] = true;
                        }
                    }

                    // Move the selection down
                    KeyCode::Char('j') => {
                        if selection_index < files.len() - 1 {
                            selected_files[selection_index] = false;
                            selection_index += 1;
                            selected_files[selection_index] = true;
                        }
                    }

                    // Move into a folder
                    KeyCode::Char('l') => {
                        let selected_file = &files[selection_index];
                        if selected_file.is_dir() {
                            files = fs::read_dir(selected_file)?
                                .filter_map(Result::ok)
                                .map(|entry| entry.path())
                                .collect();
                            selection_index = 0;
                        } else {
                            let error_message = format!("{} is not a directory", selected_file.display());
                        }
                    }

                    // Move out of a folder
                    KeyCode::Char('h') => {
                        if let Some(parent) = files[0].parent() {
                            files = fs::read_dir(parent)?
                                .filter_map(Result::ok)
                                .map(|entry| entry.path())
                                .collect();
                            selection_index = 0;
                        }
                    }
                    // Other keys
                    _ => {}
                }
            }
        }

        // Exit the program
        if exit {
            break;
        }
    }

    Ok(())
}