use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, BorderType};

pub trait RoundedBlockExt<'a> {
    fn rounded() -> Block<'a>;
    fn roundedt(title: &'a str) -> Block<'a>;
    fn invisible() -> Block<'a>;
    fn highlight_if(self, active: bool) -> Self;
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

    fn highlight_if(self, condition: bool) -> Self {
        if condition {
            self.border_style(Style::default().fg(Color::Yellow))
        } else {
            self
        }
    }
}
