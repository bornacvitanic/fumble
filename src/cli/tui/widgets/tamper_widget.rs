use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::prelude::{Modifier, Span};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Paragraph, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::traits::{DisplayName, HandleInput, IsActive, KeyBindings};
use crate::cli::tui::widgets::utils::{auto_hide_cursor, RoundedBlockExt};
use crate::cli::tui::widgets::utils;

pub struct TamperWidget<'a> {
    title: String,
    pub probability_text_area: TextArea<'a>,
    pub tamper_amount: TextArea<'a>,
    pub recalculate_checksums: bool,
    is_active: bool,
    interacting: bool,
    selected: usize,
}

impl TamperWidget<'_> {
    pub fn new() -> Self {
        TamperWidget {
            title: "Tamper".to_string(),
            probability_text_area: TextArea::default(),
            tamper_amount: TextArea::default(),
            recalculate_checksums: true,
            is_active: false,
            interacting: false,
            selected: 0,
        }
    }
}

impl HandleInput for TamperWidget<'_> {
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
                    if self.tamper_amount.input(key) {
                        let _valid = utils::validate_probability(&mut self.tamper_amount);
                    }
                }
                2 => {
                    if key.code == KeyCode::Char(' ') && key.kind == KeyEventKind::Press {
                        self.recalculate_checksums = !self.recalculate_checksums;
                    }
                }
                _ => {}
            }

            return true;
        }
        return false;
    }
}

impl DisplayName for TamperWidget<'_> {
    fn name(&self) -> &str {
        &self.title
    }
}

impl KeyBindings for TamperWidget<'_> {
    fn key_bindings(&self) -> String {
        "Exit: Esc | Navigation: Left and Right".to_string()
    }
}

impl IsActive for TamperWidget<'_> {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, state: bool) {
        self.is_active = state;
    }
}

impl Widget for &mut TamperWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let [probability_area, duration_area, checksum_area, drop_info_area] = Layout::horizontal([
            Constraint::Max(10),
            Constraint::Max(10),
            Constraint::Max(25),
            Constraint::Min(25),
        ]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

        auto_hide_cursor(&mut self.probability_text_area, self.interacting && self.selected == 0);
        self.probability_text_area.set_placeholder_text("0.1");
        self.probability_text_area.set_cursor_line_style(Style::default());
        if self.probability_text_area.block() == None { self.probability_text_area.set_block(Block::roundedt("Probability")); }
        self.probability_text_area.render(probability_area, buf);

        auto_hide_cursor(&mut self.tamper_amount, self.interacting && self.selected == 1);
        self.tamper_amount.set_placeholder_text("0.1");
        self.tamper_amount.set_cursor_line_style(Style::default());
        if self.tamper_amount.block() == None { self.tamper_amount.set_block(Block::roundedt("Amount")); }
        self.tamper_amount.render(duration_area, buf);

        let mut checksum_span = Span::from(self.recalculate_checksums.to_string());
        if self.selected == 2 && self.interacting {
            checksum_span = checksum_span.add_modifier(Modifier::RAPID_BLINK);
        }
        let checksum_paragraph = Paragraph::new(checksum_span).block(Block::roundedt("Recalculate Checksums"));
        checksum_paragraph.render(checksum_area, buf);

        Paragraph::new("Tampering with XX% of packets, YY% of their data").block(Block::invisible()).render(drop_info_area, buf);
    }
}