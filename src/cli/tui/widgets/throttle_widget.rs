use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::traits::{DisplayName, HandleInput, IsActive, KeyBindings};
use crate::cli::tui::widgets::utils::{auto_hide_cursor, RoundedBlockExt};
use crate::cli::tui::widgets::utils;
use crate::network::modules::stats::throttle_stats::ThrottleStats;

pub struct ThrottleWidget<'a> {
    title: String,
    pub probability_text_area: TextArea<'a>,
    pub throttle_duration: TextArea<'a>,
    pub drop: bool,
    is_active: bool,
    interacting: bool,
    selected: usize,
    is_throttling: bool,
    dropped_count: usize,
}

impl ThrottleWidget<'_> {
    pub fn new() -> Self {
        ThrottleWidget {
            title: "Throttle".to_string(),
            probability_text_area: TextArea::default(),
            throttle_duration: TextArea::default(),
            drop: false,
            is_active: false,
            interacting: false,
            selected: 0,
            is_throttling: false,
            dropped_count: 0,
        }
    }

    pub fn update_data(&mut self, stats: &ThrottleStats) {
        self.is_throttling = stats.is_throttling;
        self.dropped_count = stats.dropped_count;
    }
}

impl HandleInput for ThrottleWidget<'_> {
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
                if self.selected < 2 {
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
                    if self.throttle_duration.input(key) {
                        let _valid = utils::validate_usize(&mut self.throttle_duration);
                    }
                }
                2 => {
                    if key.code == KeyCode::Char(' ') && key.kind == KeyEventKind::Press {
                        self.drop = !self.drop;
                    }
                }
                _ => {}
            }

            return true;
        }
        return false;
    }
}

impl DisplayName for ThrottleWidget<'_> {
    fn name(&self) -> &str {
        &self.title
    }
}

impl KeyBindings for ThrottleWidget<'_> {
    fn key_bindings(&self) -> String {
        "Exit: Esc | Navigation: Left and Right".to_string()
    }
}

impl IsActive for ThrottleWidget<'_> {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, state: bool) {
        self.is_active = state;
    }
}

impl Widget for &mut ThrottleWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let [probability_area, duration_area, drop_area, info_area] = Layout::horizontal([
            Constraint::Max(12),
            Constraint::Max(10),
            Constraint::Max(8),
            Constraint::Min(25),
        ]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

        auto_hide_cursor(&mut self.probability_text_area, self.interacting && self.selected == 0);
        self.probability_text_area.set_placeholder_text("0.1");
        self.probability_text_area.set_cursor_line_style(Style::default());
        if self.probability_text_area.block() == None { self.probability_text_area.set_block(Block::roundedt("Probability")); }
        self.probability_text_area.render(probability_area, buf);

        auto_hide_cursor(&mut self.throttle_duration, self.interacting && self.selected == 1);
        self.throttle_duration.set_placeholder_text("30");
        self.throttle_duration.set_cursor_line_style(Style::default());
        if self.throttle_duration.block() == None { self.throttle_duration.set_block(Block::roundedt("Duration")); }
        self.throttle_duration.render(duration_area, buf);

        let mut drop_span = Span::from(self.drop.to_string());
        if self.selected == 2 && self.interacting {
            drop_span = drop_span.add_modifier(Modifier::RAPID_BLINK);
        }
        let drop_paragraph = Paragraph::new(drop_span).block(Block::roundedt("Drop"));
        drop_paragraph.render(drop_area, buf);

        let [is_throttling_info, drop_count, _excess_info] = Layout::horizontal([
            Constraint::Max(10),
            Constraint::Max(10),
            Constraint::Fill(1)
        ]).areas(info_area);
        Paragraph::new(format!("{}", self.is_throttling)).block(Block::bordered().title("Throttling")).render(is_throttling_info, buf);
        Paragraph::new(format!("{}", self.dropped_count)).block(Block::bordered().title("Dropped")).render(drop_count, buf);
    }
}