use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::widgets::utils::style_textarea_based_on_validation;
use crate::cli::tui::widgets::utils::block_ext::RoundedBlockExt;
use crate::cli::tui::widgets::utils::textarea_ext::{TextAreaExt};
use crate::network::utils::filter::{FilterError, validate_filter};

pub struct FilterWidget<'a> {
    textarea: TextArea<'a>,
    pub inputting: bool,
    pub filter: Result<String, FilterError>,
    validation_filter: Result<String, FilterError>,
}

impl FilterWidget<'_> {
    pub fn new() -> Self {
        FilterWidget {
            textarea: TextArea::default(),
            inputting: false,
            filter: Err(FilterError::InvalidSyntax("No filter provided".to_string())),
            validation_filter: Err(FilterError::InvalidSyntax("No filter provided".to_string())),
        }
    }

    pub fn set_filter(&mut self, filter: &str) {
        self.filter = validate_filter(filter);
        self.validation_filter = self.filter.clone();
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
            if let KeyCode::Esc = key.code {
                self.inputting = false;
                if let Ok(filter) = &self.filter {
                    self.set_filter(&filter.to_string());
                }

                return;
            }
            if let KeyCode::Enter = key.code {
                if let Ok(filter) = &self.validation_filter {
                    self.set_filter(&filter.to_string());
                }
                if self.validation_filter.is_ok() {
                    self.inputting = false;
                }

                return;
            }
            if self.textarea.input(key) {
                self.validation_filter = validate_filter(&self.textarea.lines()[0]);
            }
        }
    }
}

impl Widget for &mut FilterWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        self.textarea.set_cursor_visibility(self.inputting);
        self.textarea.set_cursor_line_style(Style::default());
        let mut text_area_block = Block::roundedt("[F]-Filter");
        if self.inputting {
            text_area_block = text_area_block.fg(Color::Yellow);
        }
        self.textarea.set_block(text_area_block);
        style_textarea_based_on_validation(&mut self.textarea, &self.validation_filter);
        self.textarea.render(area, buf);
    }
}