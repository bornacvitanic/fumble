use std::cmp::PartialEq;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::prelude::{Color, Line, Style, Stylize};
use crate::cli::tui::state::TuiState;
use crate::cli::tui::traits::{DisplayName, IsActive, KeyBindings};
use crate::cli::tui::widgets::utils::RoundedBlockExt;

pub fn ui(frame: &mut Frame, state: &mut TuiState) {
    update_focus(state);
    let (filter_area, middle_area, key_bind_area) = setup_layout(frame);
    let (main_area, log_area) = arrange_middle_area(state, middle_area);

    frame.render_widget(&mut state.filter_widget, filter_area);
    render_sections(frame, state, main_area);
    frame.render_widget(&mut state.logs_widget, log_area);
    render_keybindings(frame, state, key_bind_area);
}

#[derive(PartialEq)]
pub enum LayoutSection {
    Filter,
    Main,
    Logging
}

fn update_focus(state: &mut TuiState) {
    if state.filter_widget.inputting {
        state.focused = LayoutSection::Filter;
    } else if state.logs_widget.focused {
        state.focused = LayoutSection::Logging;
    } else {
        state.focused = LayoutSection::Main;
    }
}

fn setup_layout(frame: &mut Frame) -> (Rect, Rect, Rect) {
    let [filter_area, middle_area, key_bind_area] = Layout::vertical([
        Constraint::Max(3),
        Constraint::Min(0),
        Constraint::Length(1)
    ]).areas(frame.area());
    (filter_area, middle_area, key_bind_area)
}

fn arrange_middle_area(state: &mut TuiState, middle_area: Rect) -> (Rect, Rect) {
    let [main_area, log_area] = if middle_area.height + 60 >= middle_area.width || !state.logs_widget.open {
        Layout::vertical([
            Constraint::Max(500),
            Constraint::Max(if state.logs_widget.open { 10 } else { 1 })
        ]).areas(middle_area)
    } else {
        Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(if state.logs_widget.open { 1 } else { 0 })
        ]).areas(middle_area)
    };
    (main_area, log_area)
}

fn render_sections(frame: &mut Frame, state: &mut TuiState, main_area: Rect) {
    let total_sections = state.sections.len();
    let default_height = 5;
    let available_rect = main_area.inner(Margin { horizontal: 1, vertical: 1 });
    let available_height = available_rect.height as usize;

    // Calculate how many sections can be displayed given the available height.
    let max_visible_sections = available_height / default_height;
    let half_visible = max_visible_sections / 2;

    // Ensure at least one section is visible
    let max_visible_sections = max_visible_sections.max(1);

    let (start_index, end_index) = if max_visible_sections >= total_sections {
        // If we have enough space to show all sections, just display all.
        (0, total_sections - 1)
    } else {
        // Center the selected section, adjusting for edges
        let start = state.selected.saturating_sub(half_visible);
        let end = (start + max_visible_sections - 1).min(total_sections - 1);

        // Adjust start if we're at the end of the list
        let start = if end == total_sections - 1 {
            end.saturating_sub(max_visible_sections - 1)
        } else {
            start
        };
        (start, end)
    };

    // Apply the constraints
    let constraints: Vec<Constraint> = (0..total_sections)
        .map(|i| if i >= start_index && i <= end_index {
            Constraint::Length(default_height as u16)
        } else {
            Constraint::Length(0)
        })
        .collect();

    let section_areas: [Rect; 7] = Layout::vertical(constraints).areas(available_rect);

    let mut main_block = Block::roundedt("Main").title_bottom(Line::from("This is the main area").right_aligned());
    if state.focused == LayoutSection::Main { main_block = main_block.border_style(Style::default().fg(Color::Yellow)) }
    frame.render_widget(main_block, main_area);

    for (i, option) in state.sections.iter_mut().enumerate() {
        let mut area_block = Block::rounded().title(format!("[{}]-{}", i + 1, option.name()));
        if !option.is_active() {
            area_block = area_block.fg(Color::DarkGray);
        }
        if state.selected == i {
            area_block = area_block.fg(Color::Green);
        }
        if state.interacting == Some(i) {
            area_block = area_block.fg(Color::Yellow);
        }
        frame.render_widget(area_block, section_areas[i]);
        frame.render_widget(option, section_areas[i]);
    }


    if total_sections > max_visible_sections {
        // Calculate scrollbar state
        let scroll_position = state.selected;
        let mut scrollbar_state = ScrollbarState::new(total_sections)
            .viewport_content_length(max_visible_sections)
            .position(scroll_position);

        // Render the scrollbar on the right side
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .thumb_symbol("█")
            .thumb_style(Style::default().fg(Color::DarkGray));

        frame.render_stateful_widget(scrollbar, available_rect, &mut scrollbar_state);
    }
}

fn render_keybindings(frame: &mut Frame, state: &mut TuiState, key_bind_area: Rect) {
    let mut keybinds = "Quit: q | Toggle: Space | Navigation: Up and Down".to_string();
    if let Some(index) = state.interacting {
        keybinds = (&mut state.sections[index]).key_bindings();
    }
    frame.render_widget(Paragraph::new(keybinds).style(Color::Blue), key_bind_area)
}