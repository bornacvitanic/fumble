use std::fmt::Display;
use std::str::FromStr;
use tui_textarea::TextArea;

pub trait ParseFromTextArea: Sized {
    fn from_text_area(widget: &TextArea) -> Option<Self> {
        Self::parse_from_text_area(widget).ok()
    }

    fn parse_from_text_area(widget: &TextArea) -> Result<Self, String>;
}

impl<T> ParseFromTextArea for T
where
    <T as FromStr>::Err: Display,
    T: FromStr,
{
    fn parse_from_text_area(widget: &TextArea) -> Result<Self, String> {
        widget
            .lines()
            .first()
            .ok_or_else(|| "No input found".to_string())
            .and_then(|line| {
                line.parse::<T>()
                    .map_err(|e| format!("Failed to parse input: {}", e))
            })
    }
}
