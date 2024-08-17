use log::{Record, Level, Metadata, LevelFilter};
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct LogEntry {
    pub level: Level,
    pub message: String,
}

impl LogEntry {
    fn new(level: Level, message: String) -> Self {
        Self { level, message }
    }
}

pub struct LogBuffer {
    buffer: Mutex<Vec<LogEntry>>
}

impl LogBuffer {
    fn new() -> LogBuffer {
        LogBuffer {
            buffer: Mutex::new(Vec::new()),
        }
    }

    fn log(&self, record: &Record) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(LogEntry::new(record.level(), record.args().to_string()))
    }

    pub(crate) fn get_logs(&self) -> Vec<LogEntry> {
        let buffer = self.buffer.lock().unwrap();
        buffer.clone()
    }
}

lazy_static! {
    pub static ref LOG_BUFFER: LogBuffer = LogBuffer::new();
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            LOG_BUFFER.log(record);
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init_logger() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Failed to initialize logger");
}

pub fn set_logger_level_filter(level_filter: LevelFilter) {
    log::set_max_level(level_filter);
}