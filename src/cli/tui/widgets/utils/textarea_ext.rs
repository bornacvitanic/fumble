use ratatui::prelude::{Color, Modifier, Style};
use tui_textarea::TextArea;

pub trait TextAreaExt {
    fn set_text(&mut self, text: &str);
    fn set_cursor_visibility(&mut self, active: bool);
    fn set_dim_placeholder(&mut self, placeholder: impl Into<String>);
}

impl<'a> TextAreaExt for TextArea<'a> {
    fn set_text(&mut self, text: &str) {
        self.set_yank_text(text);
        self.select_all();
        self.paste();
    }

    fn set_cursor_visibility(&mut self, active: bool) {
        self.set_cursor_style(if active {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().bg(Color::Black)
        });
    }

    fn set_dim_placeholder(&mut self, placeholder: impl Into<String>) {
        self.set_placeholder_text(placeholder);
        self.set_placeholder_style(Style::new().add_modifier(Modifier::DIM));
    }
}