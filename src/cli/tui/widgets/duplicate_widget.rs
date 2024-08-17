use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::traits::{DisplayName, HandleInput, IsActive, KeyBindings};
use crate::cli::tui::widgets::utils::{auto_hide_cursor, RoundedBlockExt};
use crate::cli::tui::widgets::utils;

pub struct DuplicateWidget<'a> {
    title: String,
    pub probability_text_area: TextArea<'a>,
    pub duplicate_count: TextArea<'a>,
    is_active: bool,
    interacting: bool,
    selected: usize
}

impl DuplicateWidget<'_> {
    pub fn new() -> Self {
        DuplicateWidget {
            title: "Duplicate".to_string(),
            probability_text_area: TextArea::default(),
            duplicate_count: TextArea::default(),
            is_active: false,
            interacting: false,
            selected: 0
        }
    }
}

impl HandleInput for DuplicateWidget<'_> {
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
            if key.code == KeyCode::Right {
                if self.selected < 1 {
                    self.selected += 1;
                }
            }
            if key.code == KeyCode::Left {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            match self.selected {
                0 => {
                    if self.probability_text_area.input(key) {
                        let _valid = utils::validate_probability(&mut self.probability_text_area);
                    }
                }
                1 => {
                    if self.duplicate_count.input(key) {
                        let _valid = utils::validate_usize(&mut self.duplicate_count);
                    }
                }
                _ => {}
            }

            return true;
        }
        return false;
    }
}

impl DisplayName for DuplicateWidget<'_> {
    fn name(&self) -> &str {
        &self.title
    }
}

impl KeyBindings for DuplicateWidget<'_> {
    fn key_bindings(&self) -> String {
        "Exit: Esc | Navigation: Left and Right".to_string()
    }
}

impl IsActive for DuplicateWidget<'_> {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, state: bool) {
        self.is_active = state;
    }
}

impl Widget for &mut DuplicateWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let [probability_area, duration_area, drop_info_area] = Layout::horizontal([
            Constraint::Max(10),
            Constraint::Max(10),
            Constraint::Min(25),
        ]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

        auto_hide_cursor(&mut self.probability_text_area, self.interacting && self.selected == 0);
        self.probability_text_area.set_placeholder_text("0.1");
        self.probability_text_area.set_cursor_line_style(Style::default());
        if self.probability_text_area.block() == None { self.probability_text_area.set_block(Block::roundedt("Probability")); }
        self.probability_text_area.render(probability_area, buf);

        auto_hide_cursor(&mut self.duplicate_count, self.interacting && self.selected == 1);
        self.duplicate_count.set_placeholder_text("1");
        self.duplicate_count.set_cursor_line_style(Style::default());
        if self.duplicate_count.block() == None { self.duplicate_count.set_block(Block::roundedt("Count")); }
        self.duplicate_count.render(duration_area, buf);

        Paragraph::new("Duplicating XX% of packets Y times").block(Block::invisible()).render(drop_info_area, buf);
    }
}