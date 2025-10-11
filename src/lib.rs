//! # Logflow
//!
//! A beautiful, customizable, and performant logging library for Rust with perfect terminal UI.
//!
//! ## Features
//!
//! - 🎨 Beautiful terminal output with rich colors and formatting
//! - 🚀 Zero-config defaults with extensive customization options
//! - ⚡ High performance with minimal overhead
//! - 🔄 Native async support with proper context propagation
//! - 🌳 Hierarchical/nested logging with visual indentation
//! - 🔒 Thread-safe by design
//! - 📊 Multiple output formats (JSON, pretty, compact, custom)
//! - 🎯 Real-time filtering and log level management
//!
//! ## Quick Start
//!
//! ```rust
//! use logflow::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let logger = LogFlow::new().with_colors(true).build()?;
//!
//!     logger.info("Application started")?;
//!     logger.warn("This is a warning")?;
//!     logger.error("Something went wrong")?;
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod context;
pub mod formatter;
pub mod level;
pub mod logger;
pub mod macros;
pub mod output;

#[cfg(feature = "async")]
pub mod async_logger;

pub use config::*;
pub use context::*;
pub use formatter::*;
pub use level::*;
pub use logger::*;

pub use macros::*;

#[cfg(feature = "async")]
pub use async_logger::*;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::output::OutputType;
    pub use crate::{LogConfig, LogContext, LogFlow, LogLevel};

    #[cfg(feature = "async")]
    pub use crate::AsyncLogFlow;
}
