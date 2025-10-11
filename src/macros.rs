#[macro_export]
macro_rules! trace {
    ($logger:expr, $($arg:tt)*) => {
        $logger.trace(&format!($($arg)*)).unwrap_or(())
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $($arg:tt)*) => {
        $logger.debug(&format!($($arg)*)).unwrap_or(())
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)*) => {
        $logger.info(&format!($($arg)*)).unwrap_or(())
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $($arg:tt)*) => {
        $logger.warn(&format!($($arg)*)).unwrap_or(())
    };
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $($arg:tt)*) => {
        $logger.error(&format!($($arg)*)).unwrap_or(())
    };
}

#[macro_export]
macro_rules! fatal {
    ($logger:expr, $($arg:tt)*) => {
        $logger.fatal(&format!($($arg)*)).unwrap_or(())
    };
}

#[macro_export]
macro_rules! log_scope {
    ($logger:expr, $name:expr, $body:block) => {{
        let _scope = $logger.begin_scope($name);
        $body
    }};
}

#[macro_export]
macro_rules! log_field {
    ($logger:expr, $key:expr, $value:expr) => {
        $logger.with_field($key, $value)
    };
}

#[macro_export]
macro_rules! logflow_trace {
    ($($arg:tt)*) => {
        if let Ok(logger) = $crate::GLOBAL_LOGGER.try_lock() {
            logger.trace(&format!($($arg)*)).unwrap_or(())
        }
    };
}

#[macro_export]
macro_rules! logflow_debug {
    ($($arg:tt)*) => {
        if let Ok(logger) = $crate::GLOBAL_LOGGER.try_lock() {
            logger.debug(&format!($($arg)*)).unwrap_or(())
        }
    };
}

#[macro_export]
macro_rules! logflow_info {
    ($($arg:tt)*) => {
        if let Ok(logger) = $crate::GLOBAL_LOGGER.try_lock() {
            logger.info(&format!($($arg)*)).unwrap_or(())
        }
    };
}

#[macro_export]
macro_rules! logflow_warn {
    ($($arg:tt)*) => {
        if let Ok(logger) = $crate::GLOBAL_LOGGER.try_lock() {
            logger.warn(&format!($($arg)*)).unwrap_or(())
        }
    };
}

#[macro_export]
macro_rules! logflow_error {
    ($($arg:tt)*) => {
        if let Ok(logger) = $crate::GLOBAL_LOGGER.try_lock() {
            logger.error(&format!($($arg)*)).unwrap_or(())
        }
    };
}

#[macro_export]
macro_rules! logflow_fatal {
    ($($arg:tt)*) => {
        if let Ok(logger) = $crate::GLOBAL_LOGGER.try_lock() {
            logger.fatal(&format!($($arg)*)).unwrap_or(())
        }
    };
}

use crate::LogFlow;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static GLOBAL_LOGGER: Lazy<Mutex<LogFlow>> = Lazy::new(|| Mutex::new(LogFlow::default()));

pub fn init_global_logger(logger: LogFlow) {
    if let Ok(mut global) = GLOBAL_LOGGER.try_lock() {
        *global = logger;
    }
}

pub fn with_global_logger<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&LogFlow) -> R,
{
    GLOBAL_LOGGER.try_lock().ok().map(|logger| f(&*logger))
}
