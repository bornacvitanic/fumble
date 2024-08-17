use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Widget};
use tui_textarea::TextArea;
use crate::cli::tui::widgets::utils::{auto_hide_cursor, RoundedBlockExt};
use crate::network::utils::filter::validate_filter;

pub struct FilterWidget<'a> {
    pub textarea: TextArea<'a>,
    pub inputting: bool,
    changed_input: bool,
    valid: bool,
}

impl FilterWidget<'_> {
    pub fn new() -> Self {
        FilterWidget {
            textarea: TextArea::default(),
            inputting: false,
            changed_input: false,
            valid: true
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
            self.changed_input = self.textarea.input(key);
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
        if self.changed_input {
            self.valid = validate_filter_contents(&mut self.textarea);
        }
        if self.valid { self.textarea.set_block(text_area_block); }
        self.textarea.render(area, buf);
    }
}

fn validate_filter_contents(textarea: &mut TextArea) -> bool {
    let res = validate_filter(&textarea.lines()[0]);
    match res {
        Err(err) => {
            textarea.set_style(Style::default().fg(Color::LightRed));
            textarea.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Color::LightRed)
                    .title(format!("ERROR: {}", err)),
            );
            false
        }
        Ok(_) => {
            textarea.set_style(Style::default());
            true
        }
    }
}