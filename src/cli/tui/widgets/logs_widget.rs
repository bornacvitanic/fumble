use log::{Level, LevelFilter};
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Style};
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::widgets::{Block, Borders, List, ListItem, Widget};
use crate::cli::tui::custom_logger::{LOG_BUFFER, LogEntry, set_logger_level_filter};

pub struct LogsWidget {
    pub(crate) open: bool,
    pub(crate) focused: bool,
}

impl LogsWidget {
    pub fn new() -> Self {
        LogsWidget {
            open: false,
            focused: false,
        }
    }
    pub fn input(&mut self, key: KeyEvent) {
        if !self.focused {
            if KeyCode::Char('l') == key.code {
                self.open = true;
                self.focused = true;
            }
        } else {
            if KeyCode::Char('l') == key.code {
                self.open = false;
                self.focused = false;
                return;
            }
            if let KeyCode::Esc = key.code {
                self.focused = false;
                return;
            }
            Self::change_log_level(key);
        }
    }

    fn change_log_level(key: KeyEvent) {
        match key.code {
            KeyCode::Char('t') => {
                set_logger_level_filter(LevelFilter::Trace);
            }
            KeyCode::Char('d') => {
                set_logger_level_filter(LevelFilter::Debug);
            }
            KeyCode::Char('i') => {
                set_logger_level_filter(LevelFilter::Info);
            }
            KeyCode::Char('w') => {
                set_logger_level_filter(LevelFilter::Warn);
            }
            KeyCode::Char('e') => {
                set_logger_level_filter(LevelFilter::Error);
            }
            _ => {}
        }
    }
}

impl Widget for &mut LogsWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        if self.open {
            let log_display_height = area.height.saturating_sub(2) as usize;
            let logs = LOG_BUFFER.get_logs();
            let start = logs.len().saturating_sub(log_display_height);
            let recent_logs = &logs[start..];
            let items: Vec<ListItem> = format_logs_for_tui(recent_logs);
            let mut logging_area_block = Block::bordered().title("[L]-Logs");
            if self.focused { logging_area_block = logging_area_block.fg(Color::Yellow); }
            let list = List::new(items).add_modifier(Modifier::ITALIC).block(logging_area_block);
            list.render(area, buf);
        } else {
            Block::bordered().borders(Borders::TOP).title("[L]-Logs").render(area, buf)
        }
    }
}

fn format_logs_for_tui(logs: &[LogEntry]) -> Vec<ListItem> {
    logs.iter()
        .map(|log| {
            let color = match log.level {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Info => Color::Green,
                Level::Debug => Color::Blue,
                Level::Trace => Color::Cyan,
            };

            let level_span = Span::styled(format!("{}: ", log.level), Style::default().fg(color));
            let message_span = Span::raw(&log.message);

            ListItem::new(Line::default().spans(vec![level_span, message_span]))
        })
        .collect()
}