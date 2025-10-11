use crate::config::LogConfig;
use crate::context::{ContextStack, LogContext};
use crate::formatter::Formatter;
use crate::level::LogLevel;
use crate::output::{Output, OutputType};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogFlowError {
    #[error("Output error: {0}")]
    Output(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Context error: {0}")]
    Context(String),
}

type Result<T> = std::result::Result<T, LogFlowError>;

pub struct LogFlow {
    config: LogConfig,
    formatter: Formatter,
    output: Arc<Mutex<Output>>,
    context_stack: ContextStack,
}

impl LogFlow {
    pub fn new() -> LogFlowBuilder {
        LogFlowBuilder::new()
    }

    pub fn with_config(config: LogConfig) -> Result<Self> {
        let formatter = Formatter::new(config.clone());
        let output = Output::new(config.output.clone())?;

        Ok(Self {
            formatter,
            output: Arc::new(Mutex::new(output)),
            config,
            context_stack: ContextStack::new(),
        })
    }

    pub fn log(&self, level: LogLevel, message: &str) -> Result<()> {
        self.log_with_context(level, message, None)
    }

    pub fn log_with_context(
        &self,
        level: LogLevel,
        message: &str,
        extra_context: Option<LogContext>,
    ) -> Result<()> {
        let target = std::module_path!().to_string();

        if !self.config.should_log(level, &target) {
            return Ok(());
        }

        let context = if let Some(ctx) = extra_context {
            ctx
        } else if let Some(current_ctx) = self.context_stack.current() {
            current_ctx.child(target)
        } else {
            LogContext::new(target)
        };

        let formatted = self.formatter.format(level, message, &context);

        if let Ok(mut output) = self.output.lock() {
            output.write_line(&formatted)?;
        }

        Ok(())
    }

    pub fn log_with_subtitle(&self, level: LogLevel, subtitle: &str, message: &str) -> Result<()> {
        let target = std::module_path!().to_string();

        if !self.config.should_log(level, &target) {
            return Ok(());
        }

        let context = if let Some(current_ctx) = self.context_stack.current() {
            current_ctx.child(target).with_subtitle(subtitle)
        } else {
            LogContext::new(target).with_subtitle(subtitle)
        };

        let formatted = self.formatter.format(level, message, &context);

        if let Ok(mut output) = self.output.lock() {
            output.write_line(&formatted)?;
        }

        Ok(())
    }

    pub fn trace(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Trace, message)
    }

    pub fn debug(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Debug, message)
    }

    pub fn info(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Info, message)
    }

    pub fn warn(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Warn, message)
    }

    pub fn error(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Error, message)
    }

    pub fn fatal(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Fatal, message)
    }

    pub fn trace_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.log_with_subtitle(LogLevel::Trace, subtitle, message)
    }

    pub fn debug_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.log_with_subtitle(LogLevel::Debug, subtitle, message)
    }

    pub fn info_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.log_with_subtitle(LogLevel::Info, subtitle, message)
    }

    pub fn warn_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.log_with_subtitle(LogLevel::Warn, subtitle, message)
    }

    pub fn error_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.log_with_subtitle(LogLevel::Error, subtitle, message)
    }

    pub fn fatal_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.log_with_subtitle(LogLevel::Fatal, subtitle, message)
    }

    pub fn begin_scope(&self, name: &str) -> LogScope {
        let target = format!("{}::{}", std::module_path!(), name);
        let context = if let Some(current) = self.context_stack.current() {
            current.child(target)
        } else {
            LogContext::new(target)
        };

        self.context_stack.push(context.clone());

        LogScope {
            logger: self,
            context,
            name: name.to_string(),
        }
    }

    pub fn end_scope(&self) {
        self.context_stack.pop();
    }

    pub fn with_field<T>(&self, key: &str, value: T) -> FieldLogger
    where
        T: serde::Serialize,
    {
        let mut context = self
            .context_stack
            .current()
            .unwrap_or_else(|| LogContext::new(std::module_path!().to_string()));

        context = context.with_field(key, value);

        FieldLogger {
            logger: self,
            context,
        }
    }

    pub fn current_depth(&self) -> usize {
        self.context_stack.depth()
    }

    pub fn flush(&self) -> Result<()> {
        if let Ok(mut output) = self.output.lock() {
            output.flush()?;
        }
        Ok(())
    }
}

impl Default for LogFlow {
    fn default() -> Self {
        LogFlow::with_config(LogConfig::default()).unwrap()
    }
}

pub struct LogFlowBuilder {
    config: LogConfig,
}

impl LogFlowBuilder {
    pub fn new() -> Self {
        Self {
            config: LogConfig::default(),
        }
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.config = self.config.with_level(level);
        self
    }

    pub fn with_colors(mut self, enabled: bool) -> Self {
        self.config = self.config.with_colors(enabled);
        self
    }

    pub fn with_timestamps(mut self, enabled: bool) -> Self {
        self.config = self.config.with_timestamps(enabled);
        self
    }

    pub fn with_date(mut self, enabled: bool) -> Self {
        self.config = self.config.with_date(enabled);
        self
    }

    pub fn with_output(mut self, output: OutputType) -> Self {
        self.config = self.config.with_output(output);
        self
    }

    pub fn with_target(mut self, enabled: bool) -> Self {
        self.config = self.config.with_target(enabled);
        self
    }

    pub fn with_module(mut self, enabled: bool) -> Self {
        self.config = self.config.with_module(enabled);
        self
    }

    pub fn with_file_line(mut self, enabled: bool) -> Self {
        self.config = self.config.with_file_line(enabled);
        self
    }

    pub fn with_bold_subtitles(mut self, enabled: bool) -> Self {
        self.config = self.config.with_bold_subtitles(enabled);
        self
    }

    pub fn pretty(mut self) -> Self {
        self.config = LogConfig::pretty();
        self
    }

    pub fn compact(mut self) -> Self {
        self.config = LogConfig::compact();
        self
    }

    pub fn json(mut self) -> Self {
        self.config = LogConfig::json();
        self
    }

    pub fn dev(mut self) -> Self {
        self.config = LogConfig::dev();
        self
    }

    pub fn build(self) -> Result<LogFlow> {
        LogFlow::with_config(self.config)
    }
}

impl Default for LogFlowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LogScope<'a> {
    logger: &'a LogFlow,
    context: LogContext,
    name: String,
}

impl<'a> LogScope<'a> {
    pub fn trace(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Trace, message, Some(self.context.clone()))
    }

    pub fn debug(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Debug, message, Some(self.context.clone()))
    }

    pub fn info(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Info, message, Some(self.context.clone()))
    }

    pub fn warn(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Warn, message, Some(self.context.clone()))
    }

    pub fn error(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Error, message, Some(self.context.clone()))
    }

    pub fn fatal(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Fatal, message, Some(self.context.clone()))
    }

    pub fn trace_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.logger.log_with_context(
            LogLevel::Trace,
            message,
            Some(self.context.clone().with_subtitle(subtitle)),
        )
    }

    pub fn debug_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.logger.log_with_context(
            LogLevel::Debug,
            message,
            Some(self.context.clone().with_subtitle(subtitle)),
        )
    }

    pub fn info_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.logger.log_with_context(
            LogLevel::Info,
            message,
            Some(self.context.clone().with_subtitle(subtitle)),
        )
    }

    pub fn warn_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.logger.log_with_context(
            LogLevel::Warn,
            message,
            Some(self.context.clone().with_subtitle(subtitle)),
        )
    }

    pub fn error_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.logger.log_with_context(
            LogLevel::Error,
            message,
            Some(self.context.clone().with_subtitle(subtitle)),
        )
    }

    pub fn fatal_with_subtitle(&self, subtitle: &str, message: &str) -> Result<()> {
        self.logger.log_with_context(
            LogLevel::Fatal,
            message,
            Some(self.context.clone().with_subtitle(subtitle)),
        )
    }

    pub fn begin_scope(&self, name: &str) -> LogScope {
        self.logger.begin_scope(name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn with_field<T>(&self, key: &str, value: T) -> FieldLogger
    where
        T: serde::Serialize,
    {
        let context = self.context.clone().with_field(key, value);
        FieldLogger {
            logger: self.logger,
            context,
        }
    }
}

impl<'a> Drop for LogScope<'a> {
    fn drop(&mut self) {
        self.logger.end_scope();
    }
}

pub struct FieldLogger<'a> {
    logger: &'a LogFlow,
    context: LogContext,
}

impl<'a> FieldLogger<'a> {
    pub fn with_field<T>(mut self, key: &str, value: T) -> Self
    where
        T: serde::Serialize,
    {
        self.context = self.context.with_field(key, value);
        self
    }

    pub fn with_subtitle(mut self, subtitle: &str) -> Self {
        self.context = self.context.with_subtitle(subtitle);
        self
    }

    pub fn trace(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Trace, message, Some(self.context.clone()))
    }

    pub fn debug(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Debug, message, Some(self.context.clone()))
    }

    pub fn info(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Info, message, Some(self.context.clone()))
    }

    pub fn warn(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Warn, message, Some(self.context.clone()))
    }

    pub fn error(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Error, message, Some(self.context.clone()))
    }

    pub fn fatal(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Fatal, message, Some(self.context.clone()))
    }
}
