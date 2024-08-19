use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::widgets::utils::{auto_hide_cursor, display_validity, RoundedBlockExt, TextAreaExt};
use crate::network::utils::filter::{FilterError, validate_filter};

pub struct FilterWidget<'a> {
    textarea: TextArea<'a>,
    pub inputting: bool,
    pub filter: Result<String, FilterError>,
}

impl FilterWidget<'_> {
    pub fn new() -> Self {
        FilterWidget {
            textarea: TextArea::default(),
            inputting: false,
            filter: Err(FilterError::InvalidSyntax("No filter provided".to_string())),
        }
    }

    pub fn set_filter(&mut self, filter: &str) {
        self.filter = validate_filter(filter);
        if self.filter.is_ok() {
            self.textarea.set_text(filter)
        }
    }

    pub fn input(&mut self, key: KeyEvent) {
        if !self.inputting {
            if key.code == KeyCode::Char('f'){
                self.inputting = true;
            }
        } else {
            if key.code == KeyCode::Esc {
                self.inputting = false;

                return;
            }
            if self.textarea.input(key) {
                self.filter = validate_filter(&self.textarea.lines()[0]);
            }
        }
    }
}

impl Widget for &mut FilterWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let mut text_area_block = Block::roundedt("[F]-Filter");
        if self.inputting {
            text_area_block = text_area_block.fg(Color::Yellow);
        }
        auto_hide_cursor(&mut self.textarea, self.inputting);
        self.textarea.set_cursor_line_style(Style::default());
        display_validity(&mut self.textarea, &self.filter);
        if self.filter.is_ok() { self.textarea.set_block(text_area_block); }
        self.textarea.render(area, buf);
    }
}