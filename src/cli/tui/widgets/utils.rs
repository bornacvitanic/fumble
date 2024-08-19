use std::fmt::Display;
use std::str::FromStr;
use tui_textarea::TextArea;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, BorderType};

pub fn validate_probability(textarea: &mut TextArea) -> bool {
    let res = textarea.lines()[0].parse::<f64>();
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
        Ok(num) => {
            if num < 0f64 || num > 1.0 {
                textarea.set_style(Style::default().fg(Color::LightRed));
                textarea.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Color::LightRed)
                        .title(format!("ERROR: {}", "Not a valid probability (0-1)")),
                );
                false
            } else {
                textarea.set_style(Style::default());
                textarea.remove_block();
                true
            }
        }
    }
}

pub(crate) fn validate_usize(textarea: &mut TextArea) -> bool {
    let res = textarea.lines()[0].parse::<usize>();
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
            textarea.remove_block();
            true
        }
    }
}

pub(crate) fn display_validity<T, E>(textarea: &mut TextArea, res: &Result<T, E>) -> bool
where
    E: Display,
{
    match res {
        Err(err) => {
            textarea.set_style(Style::default().fg(Color::LightRed));
            textarea.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightRed))
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

pub fn auto_hide_cursor(text_area: &mut TextArea, active: bool) {
    text_area.set_cursor_style(if active {Style::default().add_modifier(Modifier::REVERSED)} else {Style::default().bg(Color::Black)});
}

pub trait RoundedBlockExt<'a> {
    fn rounded() -> Block<'a>;
    fn roundedt(title: &'a str) -> Block<'a>;
    fn invisible() -> Block<'a>;
}

impl<'a> RoundedBlockExt<'a> for Block<'a> {
    /// Creates a new block with all rounded borders
    fn rounded() -> Block<'a> {
        Block::bordered().border_type(BorderType::Rounded)
    }

    /// Creates a new block with all rounded borders and the specified title
    fn roundedt(title: &'a str) -> Block<'a> {
        Block::rounded().title(title)
    }

    /// Creates a new block with invisible borders
    fn invisible() -> Block<'a> {
        Block::bordered().border_style(Style::new().fg(Color::Black))
    }
}

pub trait TextAreaExt {
    fn set_text(&mut self, text: &str);
}

impl<'a> TextAreaExt for TextArea<'a> {
    fn set_text(&mut self, text: &str) {
        self.set_yank_text(text);
        self.paste();
    }
}

pub trait ParseFromTextArea: Sized {
    fn from_text_area(widget: &TextArea) -> Option<Self> {
        Self::parse_from_text_area(widget).ok()
    }

    fn parse_from_text_area(widget: &TextArea) -> Result<Self, String>;
}

impl<T> ParseFromTextArea for T
where
    <T as FromStr>::Err: Display, T: FromStr
{
    fn parse_from_text_area(widget: &TextArea) -> Result<Self, String> {
        widget
            .lines()
            .first()
            .ok_or_else(|| "No input found".to_string())
            .and_then(|line| line.parse::<T>().map_err(|e| format!("Failed to parse input: {}", e)))
    }
}