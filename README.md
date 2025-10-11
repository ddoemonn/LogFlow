# LogFlow

**A beautiful, customizable, and performant logging library for Rust with perfect terminal UI.**

[![Image](https://i.hizliresim.com/5tp9kpx.png)](https://hizliresim.com/5tp9kpx)

LogFlow is designed to address the common pain points in Rust logging by providing:

- **Beautiful terminal output** with rich colors and formatting
- **Zero-config defaults** with extensive customization options
- **High performance** with minimal overhead
- **Native async support** with proper context propagation
- **Hierarchical/nested logging** with visual indentation
- **Thread-safe by design**
- **Multiple output formats** (JSON, pretty, compact, custom)
- **Real-time filtering** and log level management

## Quick Start

Add LogFlow to your `Cargo.toml`:

```toml
[dependencies]
logflow = "0.1.0"

# For async support
logflow = { version = "0.1.0", features = ["async"] }
```

### Basic Usage

```rust
use logflow::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a logger with beautiful defaults
    let logger = LogFlow::new().pretty().build()?;

    logger.info("Application started")?;
    logger.warn("This is a warning")?;
    logger.error("Something went wrong")?;

    Ok(())
}
```

### Nested Logging

```rust
use logflow::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = LogFlow::new().dev().build()?;

    logger.info("Starting request processing")?;

    {
        let db_scope = logger.begin_scope("database");
        db_scope.info("Connecting to database")?;

        {
            let query_scope = db_scope.begin_scope("query");
            query_scope.debug("Executing SELECT query")?;
            query_scope.info("Query completed successfully")?;
        } // query scope ends automatically

        db_scope.info("Database operation completed")?;
    } // database scope ends automatically

    logger.info("Request processing finished")?;
    Ok(())
}
```

### Structured Logging with Fields

```rust
use logflow::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = LogFlow::new().json().build()?;

    // Log with structured fields
    logger
        .with_field("user_id", 12345)
        .with_field("action", "login")
        .with_field("ip_address", "192.168.1.100")
        .info("User authentication successful")?;

    Ok(())
}
```

### Async Logging

```rust
#[cfg(feature = "async")]
use logflow::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = AsyncLogFlow::new()
        .pretty()
        .with_buffer_size(100)
        .build()
        .await?;

    // Start background flushing
    let _flush_task = logger.start_background_flush();

    logger.info("Async logging started").await?;

    // Your async application logic here...

    logger.flush().await?; // Ensure all logs are written
    Ok(())
}
```

## Features

### Beautiful Terminal Output

LogFlow provides multiple formatting options:

- **Pretty Mode**: Beautiful colors and clear hierarchy
- **Compact Mode**: Minimal output for production environments
- **JSON Mode**: Structured output for log aggregation
- **Development Mode**: All debugging information included

### Hierarchical Logging

Create nested log scopes that automatically indent and track context:

```rust
let scope1 = logger.begin_scope("outer");
scope1.info("Outer scope message")?;

{
    let scope2 = scope1.begin_scope("inner");
    scope2.info("Inner scope message")?; // Automatically indented
} // Inner scope ends

scope1.info("Back to outer scope")?;
```

### Structured Fields

Add structured data to your logs:

```rust
logger
    .with_field("request_id", "req_123")
    .with_field("duration_ms", 145)
    .with_field("status", "success")
    .info("API request completed")?;
```

### Subtitle Support

LogFlow supports subtitles for better log categorization and visual organization:

```rust
use logflow::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = LogFlow::new().pretty().build()?;

    // Log messages with subtitles
    logger.info_with_subtitle("STARTUP", "Application is starting")?;
    logger.warn_with_subtitle("CONFIG", "Configuration file not found, using defaults")?;
    logger.error_with_subtitle("DATABASE", "Connection failed")?;

    // Subtitles can also be used with scoped logging
    let scope = logger.begin_scope("server");
    scope.info_with_subtitle("LISTENING", "Server started on port 8080")?;

    Ok(())
}
```

Subtitles appear as colored, bold labels that help categorize and organize your logs visually.

### High Performance

- Efficient formatting with minimal allocations
- Optional buffering for high-throughput scenarios
- Background flushing for async applications
- Zero-cost abstractions where possible

### Extensive Customization

```rust
let logger = LogFlow::new()
    .with_level(LogLevel::Debug)
    .with_colors(true)
    .with_timestamps(true)
    .with_module(true)
    .with_file_line(true)
    .with_bold_subtitles(true)
    .build()?;
```

## Configuration Options

### Log Levels

- `Trace` - Very detailed diagnostic information
- `Debug` - Diagnostic information for debugging
- `Info` - General information about program execution
- `Warn` - Warning messages for potentially harmful situations
- `Error` - Error messages for error conditions
- `Fatal` - Critical errors that may cause termination

### Output Formats

- **Pretty**: Colorful format perfect for development
- **Compact**: Minimal format ideal for production
- **JSON**: Structured format for log aggregation systems
- **Custom**: Implement your own formatting logic

### Output Destinations

- **Stdout**: Standard output (default)
- **Stderr**: Standard error
- **File**: Write to a specific file
- **Buffer**: Write to an in-memory buffer
- **Custom**: Implement your own output writer

## Examples

The `examples/` directory contains comprehensive demonstrations:

- [`basic_usage.rs`](examples/basic_usage.rs) - Getting started with LogFlow
- [`nested_logging.rs`](examples/nested_logging.rs) - Hierarchical logging patterns
- [`field_logging.rs`](examples/field_logging.rs) - Structured logging with fields
- [`async_logging.rs`](examples/async_logging.rs) - Async logging patterns
- [`custom_formatting.rs`](examples/custom_formatting.rs) - Customization options
- [`performance_demo.rs`](examples/performance_demo.rs) - Performance benchmarks

Run examples with:

```bash
cargo run --example basic_usage
cargo run --example async_logging --features async
```

---

*LogFlow - Where logs flow beautifully*
