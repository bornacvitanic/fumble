pub mod block_ext;
pub mod textarea_ext;
pub mod textarea_parsing;

use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block};
use std::fmt::Display;
use tui_textarea::TextArea;
use crate::cli::tui::widgets::utils::block_ext::RoundedBlockExt;

pub(crate) fn style_textarea_based_on_validation<T, E>(
    textarea: &mut TextArea,
    res: &Result<T, E>,
) -> bool
where
    E: Display,
{
    match res {
        Err(err) => {
            textarea.set_style(Style::default().fg(Color::LightRed));
            let block = match textarea.block() {
                None => { Block::rounded() }
                Some(block) => {  block.clone() }
            };
            textarea.set_block(block.border_style(Style::default().fg(Color::LightRed))
                .title_bottom(format!("ERROR: {}", err)));

            false
        }
        Ok(_) => {
            textarea.set_style(Style::default());
            true
        }
    }
}