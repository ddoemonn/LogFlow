#[cfg(feature = "async")]
use crate::config::LogConfig;
#[cfg(feature = "async")]
use crate::context::LogContext;
#[cfg(feature = "async")]
use crate::formatter::Formatter;
#[cfg(feature = "async")]
use crate::level::LogLevel;
#[cfg(feature = "async")]
use crate::output::{Output, OutputType};
#[cfg(feature = "async")]
use crate::LogFlowError;
#[cfg(feature = "async")]
use std::sync::Arc;
#[cfg(feature = "async")]
use tokio::sync::{Mutex, RwLock};
#[cfg(feature = "async")]
use tokio::time::{Duration, Instant};

#[cfg(feature = "async")]
type Result<T> = std::result::Result<T, LogFlowError>;

#[cfg(feature = "async")]
pub struct AsyncLogFlow {
    config: LogConfig,
    formatter: Formatter,
    output: Arc<Mutex<Output>>,
    context_stack: Arc<RwLock<Vec<LogContext>>>,
    buffer: Arc<Mutex<Vec<String>>>,
    buffer_size: usize,
    flush_interval: Duration,
    last_flush: Arc<Mutex<Instant>>,
}

#[cfg(feature = "async")]
impl AsyncLogFlow {
    pub fn new() -> AsyncLogFlowBuilder {
        AsyncLogFlowBuilder::new()
    }

    pub async fn with_config(config: LogConfig) -> Result<Self> {
        let formatter = Formatter::new(config.clone());
        let output = Output::new(config.output.clone())?;

        Ok(Self {
            formatter,
            output: Arc::new(Mutex::new(output)),
            config,
            context_stack: Arc::new(RwLock::new(Vec::new())),
            buffer: Arc::new(Mutex::new(Vec::new())),
            buffer_size: 100,
            flush_interval: Duration::from_millis(100),
            last_flush: Arc::new(Mutex::new(Instant::now())),
        })
    }

    pub async fn log(&self, level: LogLevel, message: &str) -> Result<()> {
        self.log_with_context(level, message, None).await
    }

    pub async fn log_with_context(
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
        } else {
            let stack = self.context_stack.read().await;
            if let Some(current_ctx) = stack.last() {
                current_ctx.child(target)
            } else {
                LogContext::new(target)
            }
        };

        let formatted = self.formatter.format(level, message, &context);

        self.buffer_log(formatted).await?;
        self.try_flush().await?;

        Ok(())
    }

    async fn buffer_log(&self, formatted: String) -> Result<()> {
        let mut buffer = self.buffer.lock().await;
        buffer.push(formatted);

        if buffer.len() >= self.buffer_size {
            drop(buffer);
            self.flush().await?;
        }

        Ok(())
    }

    async fn try_flush(&self) -> Result<()> {
        let last_flush = self.last_flush.lock().await;
        if last_flush.elapsed() >= self.flush_interval {
            drop(last_flush);
            self.flush().await?;
        }
        Ok(())
    }

    pub async fn flush(&self) -> Result<()> {
        let mut buffer = self.buffer.lock().await;
        if buffer.is_empty() {
            return Ok(());
        }

        let messages = buffer.drain(..).collect::<Vec<_>>();
        drop(buffer);

        let mut output = self.output.lock().await;
        for message in messages {
            output.write_line(&message)?;
        }
        output.flush()?;

        let mut last_flush = self.last_flush.lock().await;
        *last_flush = Instant::now();

        Ok(())
    }

    pub async fn trace(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Trace, message).await
    }

    pub async fn debug(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Debug, message).await
    }

    pub async fn info(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Info, message).await
    }

    pub async fn warn(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Warn, message).await
    }

    pub async fn error(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Error, message).await
    }

    pub async fn fatal(&self, message: &str) -> Result<()> {
        self.log(LogLevel::Fatal, message).await
    }

    pub async fn begin_scope(&self, name: &str) -> AsyncLogScope {
        let target = format!("{}::{}", std::module_path!(), name);
        let context = {
            let stack = self.context_stack.read().await;
            if let Some(current) = stack.last() {
                current.child(target)
            } else {
                LogContext::new(target)
            }
        };

        {
            let mut stack = self.context_stack.write().await;
            stack.push(context.clone());
        }

        AsyncLogScope {
            logger: self,
            context,
            name: name.to_string(),
        }
    }

    pub async fn end_scope(&self) {
        let mut stack = self.context_stack.write().await;
        stack.pop();
    }

    pub async fn with_field<T>(&self, key: &str, value: T) -> AsyncFieldLogger
    where
        T: serde::Serialize,
    {
        let context = {
            let stack = self.context_stack.read().await;
            let context = stack
                .last()
                .cloned()
                .unwrap_or_else(|| LogContext::new(std::module_path!().to_string()));
            context.with_field(key, value)
        };

        AsyncFieldLogger {
            logger: self,
            context,
        }
    }

    pub async fn current_depth(&self) -> usize {
        let stack = self.context_stack.read().await;
        stack.len()
    }

    pub fn start_background_flush(&self) -> tokio::task::JoinHandle<()> {
        let buffer = Arc::clone(&self.buffer);
        let output = Arc::clone(&self.output);
        let last_flush = Arc::clone(&self.last_flush);
        let flush_interval = self.flush_interval;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(flush_interval);
            loop {
                interval.tick().await;

                let should_flush = {
                    let last_flush = last_flush.lock().await;
                    last_flush.elapsed() >= flush_interval
                };

                if should_flush {
                    let mut buffer = buffer.lock().await;
                    if !buffer.is_empty() {
                        let messages = buffer.drain(..).collect::<Vec<_>>();
                        drop(buffer);

                        let mut output = output.lock().await;
                        for message in messages {
                            let _ = output.write_line(&message);
                        }
                        let _ = output.flush();

                        let mut last_flush = last_flush.lock().await;
                        *last_flush = Instant::now();
                    }
                }
            }
        })
    }
}

#[cfg(feature = "async")]
impl Default for AsyncLogFlow {
    fn default() -> Self {
        futures::executor::block_on(AsyncLogFlow::with_config(LogConfig::default())).unwrap()
    }
}

#[cfg(feature = "async")]
pub struct AsyncLogFlowBuilder {
    config: LogConfig,
    buffer_size: usize,
    flush_interval: Duration,
}

#[cfg(feature = "async")]
impl AsyncLogFlowBuilder {
    pub fn new() -> Self {
        Self {
            config: LogConfig::default(),
            buffer_size: 100,
            flush_interval: Duration::from_millis(100),
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

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn with_flush_interval(mut self, interval: Duration) -> Self {
        self.flush_interval = interval;
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

    pub async fn build(self) -> Result<AsyncLogFlow> {
        let mut logger = AsyncLogFlow::with_config(self.config).await?;
        logger.buffer_size = self.buffer_size;
        logger.flush_interval = self.flush_interval;
        Ok(logger)
    }
}

#[cfg(feature = "async")]
impl Default for AsyncLogFlowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "async")]
pub struct AsyncLogScope<'a> {
    logger: &'a AsyncLogFlow,
    context: LogContext,
    name: String,
}

#[cfg(feature = "async")]
impl<'a> AsyncLogScope<'a> {
    pub async fn trace(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Trace, message, Some(self.context.clone()))
            .await
    }

    pub async fn debug(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Debug, message, Some(self.context.clone()))
            .await
    }

    pub async fn info(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Info, message, Some(self.context.clone()))
            .await
    }

    pub async fn warn(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Warn, message, Some(self.context.clone()))
            .await
    }

    pub async fn error(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Error, message, Some(self.context.clone()))
            .await
    }

    pub async fn fatal(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Fatal, message, Some(self.context.clone()))
            .await
    }

    pub async fn begin_scope(&self, name: &str) -> AsyncLogScope {
        self.logger.begin_scope(name).await
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn with_field<T>(&self, key: &str, value: T) -> AsyncFieldLogger
    where
        T: serde::Serialize,
    {
        let context = self.context.clone().with_field(key, value);
        AsyncFieldLogger {
            logger: self.logger,
            context,
        }
    }
}

#[cfg(feature = "async")]
impl<'a> Drop for AsyncLogScope<'a> {
    fn drop(&mut self) {
        // Note: We can't use async operations in Drop, so we rely on manual scope management
        // or use the sync version when precise scope tracking is needed
    }
}

#[cfg(feature = "async")]
pub struct AsyncFieldLogger<'a> {
    logger: &'a AsyncLogFlow,
    context: LogContext,
}

#[cfg(feature = "async")]
impl<'a> AsyncFieldLogger<'a> {
    pub fn with_field<T>(mut self, key: &str, value: T) -> Self
    where
        T: serde::Serialize,
    {
        self.context = self.context.with_field(key, value);
        self
    }

    pub async fn trace(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Trace, message, Some(self.context.clone()))
            .await
    }

    pub async fn debug(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Debug, message, Some(self.context.clone()))
            .await
    }

    pub async fn info(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Info, message, Some(self.context.clone()))
            .await
    }

    pub async fn warn(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Warn, message, Some(self.context.clone()))
            .await
    }

    pub async fn error(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Error, message, Some(self.context.clone()))
            .await
    }

    pub async fn fatal(&self, message: &str) -> Result<()> {
        self.logger
            .log_with_context(LogLevel::Fatal, message, Some(self.context.clone()))
            .await
    }
}
