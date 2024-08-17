use std::io;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crate::cli::tui::state::TuiState;
use crate::cli::tui::traits::{HandleInput, IsActive};

// Main input handler function
pub fn handle_input(state: &mut TuiState) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }

            // Handle section input
            if handle_section_input(state, key) {
                return Ok(false);
            }

            // Handle widget input
            if handle_widget_input(state, key) {
                return Ok(false);
            }

            // Handle main menu input
            if handle_main_menu_input(state, key) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

// Function to handle input for sections
fn handle_section_input(state: &mut TuiState, key: KeyEvent) -> bool {
    for (i, section) in state.sections.iter_mut().enumerate() {
        if i != state.selected {
            continue;
        }
        let handled = section.handle_input(key);
        if handled {
            state.interacting = Some(i);
            return true;
        }
        state.interacting = None;
    }
    false
}

// Function to handle input for widgets (filter and logs)
fn handle_widget_input(state: &mut TuiState, key: KeyEvent) -> bool {
    if key.kind == KeyEventKind::Press {
        if state.filter_widget.inputting {
            state.filter_widget.input(key);
            return true;
        } else if state.logs_widget.focused {
            state.logs_widget.input(key);
            return true;
        }
    }
    false
}

// Function to handle main menu navigation and commands
fn handle_main_menu_input(state: &mut TuiState, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Up => {
            if state.selected > 0 {
                state.selected -= 1;
            }
        }
        KeyCode::Down => {
            if state.selected < state.sections.len() - 1 {
                state.selected += 1;
            }
        }
        KeyCode::Char(' ') => {
            let active_state = state.sections[state.selected].is_active();
            state.sections[state.selected].set_active(!active_state);
        }
        KeyCode::Char(c) if c.is_numeric() => {
            for (i, _) in state.sections.iter().enumerate() {
                if c == char::from_digit((i + 1) as u32, 10).unwrap() {
                    state.selected = i;
                    break;
                }
            }
        }
        _ => {}
    }

    // Pass the key event to widgets if it's not handled by menu navigation
    state.filter_widget.input(key);
    state.logs_widget.input(key);
    false
}