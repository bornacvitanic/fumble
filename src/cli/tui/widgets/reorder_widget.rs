use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::traits::{DisplayName, HandleInput, IsActive, KeyBindings};
use crate::cli::tui::widgets::utils::{auto_hide_cursor, RoundedBlockExt};
use crate::cli::tui::widgets::utils;

pub struct ReorderWidget<'a> {
    title: String,
    pub delay_duration: TextArea<'a>,
    is_active: bool,
    interacting: bool,
}

impl ReorderWidget<'_> {
    pub fn new() -> Self {
        ReorderWidget {
            title: "Reorder".to_string(),
            delay_duration: TextArea::default(),
            is_active: false,
            interacting: false
        }
    }
}

impl HandleInput for ReorderWidget<'_> {
    fn handle_input(&mut self, key: KeyEvent) -> bool {
        if !self.interacting {
            if key.code == KeyCode::Enter && key.kind == KeyEventKind::Press {
                self.interacting = true;
                return true;
            }
        } else {
            if key.code == KeyCode::Esc {
                self.interacting = false;
                return false;
            }
            if self.delay_duration.input(key) {
                let _valid = utils::validate_usize(&mut self.delay_duration);
            }
            return true;
        }
        return false;
    }
}

impl DisplayName for ReorderWidget<'_> {
    fn name(&self) -> &str {
        &self.title
    }
}

impl KeyBindings for ReorderWidget<'_> {
    fn key_bindings(&self) -> String {
        "Exit: Esc".to_string()
    }
}

impl IsActive for ReorderWidget<'_> {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, state: bool) {
        self.is_active = state;
    }
}

impl Widget for &mut ReorderWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let [delay_duration_area, info_area] = Layout::horizontal([
            Constraint::Max(10),
            Constraint::Min(25),
        ]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

        auto_hide_cursor(&mut self.delay_duration, self.interacting);
        self.delay_duration.set_placeholder_text("30");
        self.delay_duration.set_cursor_line_style(Style::default());
        if self.delay_duration.block() == None { self.delay_duration.set_block(Block::roundedt("Duration")); }
        self.delay_duration.render(delay_duration_area, buf);

        Paragraph::new("Reordering packets by delaying by random value between 0 and X").block(Block::invisible()).render(info_area, buf);
    }
}