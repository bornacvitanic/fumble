use tui_textarea::TextArea;
use ratatui::prelude::{Color, Modifier, Style};

pub trait TextAreaExt {
    fn set_text(&mut self, text: &str);
    fn set_cursor_visibility(&mut self, active:bool);
}

impl<'a> TextAreaExt for TextArea<'a> {
    fn set_text(&mut self, text: &str) {
        self.set_yank_text(text);
        self.select_all();
        self.paste();
    }

    fn set_cursor_visibility(&mut self, active: bool) {
        self.set_cursor_style(if active {Style::default().add_modifier(Modifier::REVERSED)} else {Style::default().bg(Color::Black)});
    }
}