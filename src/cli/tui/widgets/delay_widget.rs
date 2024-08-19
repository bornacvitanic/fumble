use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::traits::{DisplayName, HandleInput, IsActive, KeyBindings};
use crate::cli::tui::widgets::utils::style_textarea_based_on_validation;
use crate::cli::tui::widgets::utils::block_ext::RoundedBlockExt;
use crate::cli::tui::widgets::utils::textarea_ext::{TextAreaExt};
use crate::cli::tui::widgets::utils::textarea_parsing::ParseFromTextArea;
use crate::network::modules::stats::delay_stats::DelayStats;

pub struct DelayWidget<'a> {
    title: String,
    delay_duration: TextArea<'a>,
    is_active: bool,
    interacting: bool,
    pub delay: Result<u64, String>,
    delayed_packet_count: usize,
}

impl DelayWidget<'_> {
    pub fn new() -> Self {
        DelayWidget {
            title: "Delay".to_string(),
            delay_duration: TextArea::default(),
            is_active: false,
            interacting: false,
            delay: Ok(0),
            delayed_packet_count: 0,
        }
    }

    pub fn set_delay(&mut self, duration_ms: u64) {
        self.delay_duration.set_text(&duration_ms.to_string());
        self.delay = Ok(duration_ms);
    }

    pub fn update_data(&mut self, stats: &DelayStats) {
        self.delayed_packet_count = stats.delayed_package_count;
    }
}

impl HandleInput for DelayWidget<'_> {
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
                self.delay = u64::parse_from_text_area(&self.delay_duration);
            }
            return true;
        }
        return false;
    }
}

impl DisplayName for DelayWidget<'_> {
    fn name(&self) -> &str {
        &self.title
    }
}

impl KeyBindings for DelayWidget<'_> {
    fn key_bindings(&self) -> String {
        "Exit: Esc".to_string()
    }
}

impl IsActive for DelayWidget<'_> {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, state: bool) {
        self.is_active = state;
    }
}

impl Widget for &mut DelayWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let [delay_duration_area, info_area] = Layout::horizontal([
            Constraint::Max(10),
            Constraint::Min(25),
        ]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

        self.delay_duration.set_cursor_visibility(self.interacting);
        self.delay_duration.set_placeholder_text("500");
        self.delay_duration.set_cursor_line_style(Style::default());
        self.delay_duration.set_block(Block::roundedt("Duration"));
        if !self.delay_duration.lines()[0].is_empty() { style_textarea_based_on_validation(&mut self.delay_duration, &self.delay); }
        self.delay_duration.render(delay_duration_area, buf);

        let [delay_count_info, _excess_info] = Layout::horizontal([
            Constraint::Max(30),
            Constraint::Fill(1)
        ]).areas(info_area);
        Paragraph::new(format!("{} packets", self.delayed_packet_count)).block(Block::bordered().title("Delayed packets")).render(delay_count_info, buf);
    }
}