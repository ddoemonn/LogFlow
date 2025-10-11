use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRC",
            LogLevel::Debug => "DBG",
            LogLevel::Info => "INF",
            LogLevel::Warn => "WRN",
            LogLevel::Error => "ERR",
            LogLevel::Fatal => "FTL",
        }
    }

    pub fn from_str(s: &str) -> Option<LogLevel> {
        match s.to_uppercase().as_str() {
            "TRACE" | "TRC" => Some(LogLevel::Trace),
            "DEBUG" | "DBG" => Some(LogLevel::Debug),
            "INFO" | "INF" => Some(LogLevel::Info),
            "WARN" | "WRN" | "WARNING" => Some(LogLevel::Warn),
            "ERROR" | "ERR" => Some(LogLevel::Error),
            "FATAL" | "FTL" => Some(LogLevel::Fatal),
            _ => None,
        }
    }

    pub fn all() -> &'static [LogLevel] {
        &[
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
            LogLevel::Fatal,
        ]
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}
