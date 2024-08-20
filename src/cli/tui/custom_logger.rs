use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct LogEntry {
    pub level: Level,
    pub timestamp: String,
    pub module_path: Option<String>,
    pub message: String,
}

impl LogEntry {
    fn new(level: Level, message: String, module_path: Option<&str>) -> Self {
        let timestamp = current_utc_timestamp();

        Self {
            level,
            timestamp,
            module_path: module_path.map(|s| s.to_string()),
            message,
        }
    }

    fn colored_level(&self) -> String {
        match self.level {
            Level::Error => format!("\x1b[31m{}\x1b[0m", self.level),  // Red
            Level::Warn => format!("\x1b[33m{}\x1b[0m", self.level),   // Yellow
            Level::Info => format!("\x1b[32m{}\x1b[0m", self.level),   // Green
            Level::Debug => format!("\x1b[34m{}\x1b[0m", self.level),  // Blue
            Level::Trace => format!("\x1b[35m{}\x1b[0m", self.level),  // Magenta
        }
    }

    fn formatted_log(&self) -> String {
        format!(
            "[{} {} {}] {}",
            self.timestamp,
            self.colored_level(),
            self.module_path.as_deref().unwrap_or("unknown"),
            self.message
        )
    }
}

// Function to generate a UTC timestamp in the desired format
fn current_utc_timestamp() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seconds = now.as_secs();
    let nanos = now.subsec_nanos();

    // Convert to a human-readable timestamp (ISO 8601-like)
    let datetime = seconds_to_datetime(seconds);

    format!("{}.{:02}Z", datetime, nanos / 10_000_000)
}

// Function to convert seconds since UNIX_EPOCH to a date-time string
fn seconds_to_datetime(seconds: u64) -> String {
    let days = seconds / 86400;
    let seconds_in_day = seconds % 86400;

    let hours = seconds_in_day / 3600;
    let minutes = (seconds_in_day % 3600) / 60;
    let seconds = seconds_in_day % 60;

    let year = 1970 + days / 365;
    let day_of_year = days % 365;

    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", year, day_of_year / 30 + 1, day_of_year % 30 + 1, hours, minutes, seconds)
}

pub struct LogBuffer {
    buffer: Mutex<Vec<LogEntry>>,
    console_logging: Mutex<bool>,
}

impl LogBuffer {
    fn new() -> LogBuffer {
        LogBuffer {
            buffer: Mutex::new(Vec::new()),
            console_logging: Mutex::new(false),
        }
    }

    fn log(&self, record: &Record) {
        let module_path = record.module_path();
        let mut buffer = self.buffer.lock().unwrap();
        let entry = LogEntry::new(record.level(), record.args().to_string(), module_path);

        buffer.push(entry.clone());

        if *self.console_logging.lock().unwrap() {
            eprintln!("{}", entry.formatted_log());
        }
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

pub fn init_logger() -> Result<(), SetLoggerError> {
    let env_logger = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build();

    // Set our custom logger, which also uses env_logger under the hood
    log::set_logger(&LOGGER).map(|()| {
        // Sync the max level with env_logger
        log::set_max_level(env_logger.filter());
    })
}

pub fn set_logger_level_filter(level_filter: LevelFilter) {
    log::set_max_level(level_filter);
}

pub fn set_logger_console_state(state: bool) {
    let mut console_logging = LOG_BUFFER.console_logging.lock().unwrap();
    *console_logging = state;
}