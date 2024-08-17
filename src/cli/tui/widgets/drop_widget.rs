use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::traits::{DisplayName, HandleInput, IsActive, KeyBindings};
use crate::cli::tui::widgets::utils::{auto_hide_cursor, RoundedBlockExt};
use crate::cli::tui::widgets::utils;
use crate::network::modules::stats::drop_stats::DropStats;

pub struct DropWidget<'a> {
    title: String,
    pub probability_text_area: TextArea<'a>,
    is_active: bool,
    interacting: bool,
    drop_rate: f64,
    dropped_packets: usize,
    total_packets: usize,
}

impl DropWidget<'_> {
    pub fn new() -> Self {
        DropWidget {
            title: "Drop".to_string(),
            probability_text_area: TextArea::default(),
            is_active: false,
            interacting: false,
            drop_rate: 0.0,
            dropped_packets: 0,
            total_packets: 0
        }
    }

    pub fn update_data(&mut self, drop_stats: &DropStats) {
        self.drop_rate = drop_stats.recent_drop_rate();
        self.dropped_packets = drop_stats.total_dropped;
        self.total_packets = drop_stats.total_packets;
    }
}

impl HandleInput for DropWidget<'_> {
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
            if self.probability_text_area.input(key) {
                let _valid = utils::validate_probability(&mut self.probability_text_area);
            }
            return true;
        }
        return false;
    }
}

impl DisplayName for DropWidget<'_> {
    fn name(&self) -> &str {
        &self.title
    }
}

impl KeyBindings for DropWidget<'_> {
    fn key_bindings(&self) -> String {
        "Exit: Esc".to_string()
    }
}

impl IsActive for DropWidget<'_> {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, state: bool) {
        self.is_active = state;
    }
}

impl Widget for &mut DropWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let [drop_probability_area, info_area] = Layout::horizontal([
            Constraint::Max(10),
            Constraint::Min(25),
        ]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

        auto_hide_cursor(&mut self.probability_text_area, self.interacting);
        self.probability_text_area.set_placeholder_text("0.1");
        self.probability_text_area.set_cursor_line_style(Style::default());
        if self.probability_text_area.block() == None { self.probability_text_area.set_block(Block::roundedt("Probability")); }
        self.probability_text_area.render(drop_probability_area, buf);

        let [drop_rate_info, drop_count_info, _excess_info] = Layout::horizontal([
            Constraint::Max(15),
            Constraint::Max(20),
            Constraint::Fill(1)
        ]).areas(info_area);
        Paragraph::new(format!("{:.2}%", self.drop_rate * 100.0)).block(Block::bordered().title("Drop rate")).render(drop_rate_info, buf);
        Paragraph::new(format!("{}/{}", self.dropped_packets, self.total_packets)).right_aligned().block(Block::bordered().title("Drop count")).render(drop_count_info, buf);
    }
}