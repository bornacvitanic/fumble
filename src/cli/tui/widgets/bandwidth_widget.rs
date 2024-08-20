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
use crate::network::modules::stats::bandwidth_stats::BandwidthStats;

pub struct BandwidthWidget<'a> {
    title: String,
    limit_text_area: TextArea<'a>,
    is_active: bool,
    interacting: bool,
    pub limit: Result<usize, String>,
    throughput: f64,
    stored_packet_count: usize,
}

impl Default for BandwidthWidget<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl BandwidthWidget<'_> {
    pub fn new() -> Self {
        BandwidthWidget {
            title: "Bandwidth".to_string(),
            limit_text_area: TextArea::default(),
            is_active: false,
            interacting: false,
            limit: Ok(0),
            throughput: 0.0,
            stored_packet_count: 0,
        }
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.limit_text_area.set_text(&limit.to_string());
        self.limit =  Ok(limit);
    }

    pub(crate) fn update_data(&mut self, stats: &BandwidthStats) {
        self.throughput = stats.recent_throughput();
        self.stored_packet_count = stats.storage_packet_count;
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
            if let KeyCode::Enter | KeyCode::Esc = key.code {
                self.interacting = false;
                return false;
            }
            if self.limit_text_area.input(key) {
                self.limit =  usize::parse_from_text_area(&self.limit_text_area);
            }
            return true;
        }
        false
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
            Constraint::Max(15),
            Constraint::Min(25),
        ]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

        self.limit_text_area.set_cursor_visibility(self.interacting);
        self.limit_text_area.set_placeholder_text("No limit");
        self.limit_text_area.set_cursor_line_style(Style::default());
        self.limit_text_area.set_block(Block::roundedt("KBps Limit").highlight_if(self.interacting));
        if !self.limit_text_area.lines()[0].is_empty() { style_textarea_based_on_validation(&mut self.limit_text_area, &self.limit); }
        self.limit_text_area.render(delay_duration_area, buf);

        let [throughput_info, storage_packet_count_info, _excess_info] = Layout::horizontal([
            Constraint::Max(15),
            Constraint::Max(15),
            Constraint::Fill(1)
        ]).areas(info_area);
        Paragraph::new(format!("{:.2} KBps", self.throughput)).block(Block::bordered().title("Throughput")).render(throughput_info, buf);
        Paragraph::new(format!("{}", self.stored_packet_count)).block(Block::bordered().title("Stored packets")).render(storage_packet_count_info, buf);
    }
}