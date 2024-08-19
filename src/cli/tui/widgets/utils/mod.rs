pub mod textarea_ext;
pub mod textarea_parsing;
pub mod block_ext;

use std::fmt::Display;
use tui_textarea::TextArea;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders};

pub(crate) fn style_textarea_based_on_validation<T, E>(textarea: &mut TextArea, res: &Result<T, E>) -> bool
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