use log::{debug, error, info, Level, LevelFilter, trace, warn};
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span, Style};
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::widgets::{Block, Borders, List, ListItem, Widget};
use crate::cli::tui::custom_logger::{LOG_BUFFER, LogEntry, set_logger_level_filter};
use crate::cli::tui::traits::KeyBindings;
use crate::cli::tui::widgets::utils::block_ext::RoundedBlockExt;

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
                trace!("Logging level set to trace.")
            }
            KeyCode::Char('d') => {
                set_logger_level_filter(LevelFilter::Debug);
                debug!("Logging level set to debug.")
            }
            KeyCode::Char('i') => {
                set_logger_level_filter(LevelFilter::Info);
                info!("Logging level set to info.")
            }
            KeyCode::Char('w') => {
                set_logger_level_filter(LevelFilter::Warn);
                warn!("Logging level set to warning.")
            }
            KeyCode::Char('e') => {
                set_logger_level_filter(LevelFilter::Error);
                error!("Logging level set to error.")
            }
            _ => {}
        }
    }
}

impl KeyBindings for LogsWidget {
    fn key_bindings(&self) -> String {
        "Exit: Esc | Trace: t | Debug: d | Info: i | Warn: w | Error: e".to_string()
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
            logging_area_block = logging_area_block.highlight_if(self.focused);
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

            let timestamp_span = if matches!(log::max_level(), LevelFilter::Debug | LevelFilter::Trace) {
                Span::styled(format!("[{} ", log.timestamp), Style::default().fg(Color::DarkGray))
            } else {
                Span::raw(String::new())
            };

            let level_span = Span::styled(format!("{}", log.level), Style::default().fg(color));

            let module_path_span = if matches!(log::max_level(), LevelFilter::Debug | LevelFilter::Trace) {
                log.module_path.as_ref().map_or(Span::raw(" "), |module| {
                    Span::styled(format!(" {}] ", module), Style::default().fg(Color::DarkGray))
                })
            } else {
                Span::raw(String::new())
            };

            let message_span = Span::raw(&log.message);

            let separator_span = Span::raw(" ");

            ListItem::new(Line::default().spans(vec![
                timestamp_span,
                level_span,
                module_path_span,
                separator_span,
                message_span,
            ]))
        })
        .collect()
}