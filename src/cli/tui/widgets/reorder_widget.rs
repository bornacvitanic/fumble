use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::traits::{DisplayName, HandleInput, IsActive, KeyBindings};
use crate::cli::tui::widgets::utils::{auto_hide_cursor, RoundedBlockExt};
use crate::cli::tui::widgets::utils;
use crate::network::modules::stats::reorder_stats::ReorderStats;

pub struct ReorderWidget<'a> {
    title: String,
    pub probability_text_area: TextArea<'a>,
    pub delay_duration: TextArea<'a>,
    is_active: bool,
    interacting: bool,
    selected: usize,
    reorder_rate: f64,
    delayed_packets: usize,
}

impl ReorderWidget<'_> {
    pub fn new() -> Self {
        ReorderWidget {
            title: "Reorder".to_string(),
            probability_text_area: TextArea::default(),
            delay_duration: TextArea::default(),
            is_active: false,
            interacting: false,
            selected: 0,
            reorder_rate: 0.0,
            delayed_packets: 0,
        }
    }

    pub(crate) fn update_data(&mut self, stats: &ReorderStats) {
        self.reorder_rate = stats.recent_reorder_rate();
        self.delayed_packets = stats.delayed_packets;
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
                    if self.delay_duration.input(key) {
                        let _valid = utils::validate_usize(&mut self.delay_duration);
                    }
                }
                _ => {}
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
        let [probability_area,delay_duration_area, info_area] = Layout::horizontal([
            Constraint::Max(12),
            Constraint::Max(10),
            Constraint::Min(25),
        ]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

        auto_hide_cursor(&mut self.probability_text_area, self.interacting && self.selected == 0);
        self.probability_text_area.set_placeholder_text("0.1");
        self.probability_text_area.set_cursor_line_style(Style::default());
        if self.probability_text_area.block() == None { self.probability_text_area.set_block(Block::roundedt("Probability")); }
        self.probability_text_area.render(probability_area, buf);

        auto_hide_cursor(&mut self.delay_duration, self.interacting);
        self.delay_duration.set_placeholder_text("30");
        self.delay_duration.set_cursor_line_style(Style::default());
        if self.delay_duration.block() == None { self.delay_duration.set_block(Block::roundedt("Duration")); }
        self.delay_duration.render(delay_duration_area, buf);

        let [reorder_percentage_info, delayed_count_info, _excess_info] = Layout::horizontal([
            Constraint::Max(10),
            Constraint::Max(10),
            Constraint::Fill(1)
        ]).areas(info_area);
        Paragraph::new(format!("{:.2}%", self.reorder_rate*100.0)).block(Block::bordered().title("Reorder rate")).render(reorder_percentage_info, buf);
        Paragraph::new(format!("{}", self.delayed_packets)).block(Block::bordered().title("Delayed")).render(delayed_count_info, buf);
    }
}