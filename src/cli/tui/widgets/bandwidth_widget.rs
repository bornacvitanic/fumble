use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{Style};
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::traits::{DisplayName, HandleInput, IsActive, KeyBindings};
use crate::cli::tui::widgets::utils;
use crate::cli::tui::widgets::utils::{auto_hide_cursor, RoundedBlockExt};

pub struct BandwidthWidget<'a> {
    title: String,
    pub limit: TextArea<'a>,
    is_active: bool,
    interacting: bool,
}

impl BandwidthWidget<'_> {
    pub fn new() -> Self {
        BandwidthWidget {
            title: "Bandwidth".to_string(),
            limit: TextArea::default(),
            is_active: false,
            interacting: false
        }
    }
}

impl HandleInput for BandwidthWidget<'_> {
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
            if self.limit.input(key) {
                let _valid = utils::validate_usize(&mut self.limit);
            }
            return true;
        }
        return false;
    }
}

impl DisplayName for BandwidthWidget<'_> {
    fn name(&self) -> &str {
        &self.title
    }
}

impl KeyBindings for BandwidthWidget<'_> {
    fn key_bindings(&self) -> String {
        "Exit: Esc".to_string()
    }
}

impl IsActive for BandwidthWidget<'_> {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, state: bool) {
        self.is_active = state;
    }
}

impl Widget for &mut BandwidthWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let [delay_duration_area, info_area] = Layout::horizontal([
            Constraint::Max(10),
            Constraint::Min(25),
        ]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

        auto_hide_cursor(&mut self.limit, self.interacting);
        self.limit.set_placeholder_text("500");
        self.limit.set_cursor_line_style(Style::default());
        if self.limit.block() == None { self.limit.set_block(Block::roundedt("KBps Limit")); }
        self.limit.render(delay_duration_area, buf);

        Paragraph::new("Limiting to XXX KBps").block(Block::invisible()).render(info_area, buf);
    }
}