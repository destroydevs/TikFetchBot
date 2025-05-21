use colored::{ColoredString, Colorize};
use lazy_static::lazy_static;
use std::fmt::Display;
use chrono::Local;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Success,
    Debug,
    Executing,
}

pub struct Logger {
    pub show_timestamps: bool,
    pub show_levels: bool,
    pub max_level: LogLevel,
}

lazy_static! {
    pub static ref LOGGER: Logger = Logger::default();
}

impl Logger {
    pub fn new(show_timestamps: bool, show_levels: bool, max_level: LogLevel) -> Self {
        Logger {
            show_timestamps,
            show_levels,
            max_level,
        }
    }

    pub fn log<T: Display>(&self, level: LogLevel, message: T) {
        if self.should_log(level) {
            let mut output = String::new();

            if self.show_timestamps {
                output.push_str(&self.get_timestamp());
                output.push(' ');
            }

            if self.show_levels {
                output.push_str(&self.get_level_prefix(level).to_string());
                output.push(' ');
            }

            output.push_str(&message.to_string());

            println!("{}", output);
        }
    }

    fn should_log(&self, level: LogLevel) -> bool {
        match (self.max_level, level) {
            (LogLevel::Error, _) => matches!(level, LogLevel::Error),
            (LogLevel::Warn, _) => matches!(level, LogLevel::Error | LogLevel::Warn),
            (LogLevel::Info, _) => !matches!(level, LogLevel::Debug | LogLevel::Success | LogLevel::Executing),
            (LogLevel::Success, _) => !matches!(level, LogLevel::Debug),
            _ => true,
        }
    }

    fn get_timestamp(&self) -> ColoredString {
        let now = Local::now();
        format!("[{}]", now.format("%Y-%m-%d %H:%M:%S")).bright_black()
    }

    fn get_level_prefix(&self, level: LogLevel) -> ColoredString {
        match level {
            LogLevel::Error => "[ERROR]".bright_red().bold(),
            LogLevel::Warn => "[WARN]".bright_yellow(),
            LogLevel::Info => "[INFO]".bright_white(),
            LogLevel::Success => "[SUCCESS]".bright_green().bold(),
            LogLevel::Debug => "[DEBUG]".bright_magenta(),
            LogLevel::Executing => "[EXECUTING]".bright_blue(),
        }
    }

    pub fn error<T: Display>(&self, message: T) {
        self.log(LogLevel::Error, message);
    }

    pub fn warn<T: Display>(&self, message: T) {
        self.log(LogLevel::Warn, message);
    }

    pub fn info<T: Display>(&self, message: T) {
        self.log(LogLevel::Info, message);
    }

    pub fn success<T: Display>(&self, message: T) {
        self.log(LogLevel::Success, message);
    }

    pub fn debug<T: Display>(&self, message: T) {
        self.log(LogLevel::Debug, message);
    }

    pub fn executing<T: Display>(&self, message: T) {
        self.log(LogLevel::Executing, message);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            show_timestamps: true,
            show_levels: true,
            max_level: LogLevel::Debug,
        }
    }
}