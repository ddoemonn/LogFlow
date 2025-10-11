use logflow::prelude::*;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Custom Formatting Examples ===\n");

    // Example 1: Custom formatter function
    println!("--- Custom Formatter ---");
    let _custom_logger = LogFlow::new()
        .with_colors(true)
        .with_timestamps(true)
        .build()?;

    // Note: For this example, we'll use the built-in formatters
    // but show how different configurations create different outputs

    // Development mode with all details
    println!("\n--- Development Mode (All Details) ---");
    let dev_logger = LogFlow::new().dev().build()?;

    dev_logger.info("Development log with full context")?;
    dev_logger.warn("Warning with module and file info")?;
    dev_logger.error("Error with complete debugging info")?;

    // Pretty mode with colors
    println!("\n--- Pretty Mode with Colors ---");
    let pretty_logger = LogFlow::new()
        .pretty()
        .with_level(LogLevel::Debug)
        .build()?;

    pretty_logger.trace("Trace message")?;
    pretty_logger.debug("Debug message")?;
    pretty_logger.info("Info message")?;
    pretty_logger.warn("Warning message")?;
    pretty_logger.error("Error message")?;
    pretty_logger.fatal("Fatal message")?;

    // Compact mode for production
    println!("\n--- Compact Mode ---");
    let compact_logger = LogFlow::new()
        .compact()
        .with_level(LogLevel::Info)
        .build()?;

    compact_logger.info("Compact info message")?;
    compact_logger.warn("Compact warning")?;
    compact_logger.error("Compact error")?;

    // JSON mode for log aggregation
    println!("\n--- JSON Mode ---");
    let json_logger = LogFlow::new().json().with_level(LogLevel::Debug).build()?;

    json_logger.info("JSON info message")?;
    json_logger
        .with_field("user_id", 12345)
        .with_field("action", "login")
        .info("JSON message with fields")?;

    // Custom width limiting
    println!("\n--- Width Limited Logger ---");
    let width_limited = LogFlow::new().pretty().with_colors(true).build()?;

    // This would be implemented with custom config in a real scenario
    width_limited.info("This is a very long message that would normally span multiple lines but we want to demonstrate how width limiting would work in practice")?;

    // Buffer output example
    println!("\n--- Buffer Output Example ---");
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let buffer_logger = LogFlow::new()
        .with_output(OutputType::Buffer(buffer.clone()))
        .pretty()
        .build()?;

    buffer_logger.info("Message written to buffer")?;
    buffer_logger.warn("Warning written to buffer")?;
    buffer_logger.error("Error written to buffer")?;

    // Read from buffer and display
    if let Ok(buffer_data) = buffer.lock() {
        let output = String::from_utf8_lossy(&buffer_data);
        println!("Buffer contents:\n{}", output);
    }

    // Filtered logging example
    println!("\n--- Filtered Logging ---");
    let filtered_logger = LogFlow::new()
        .with_level(LogLevel::Warn) // Only show warnings and above
        .pretty()
        .build()?;

    filtered_logger.debug("This debug message won't appear")?;
    filtered_logger.info("This info message won't appear")?;
    filtered_logger.warn("This warning will appear")?;
    filtered_logger.error("This error will appear")?;

    // No colors for terminals that don't support them
    println!("\n--- No Colors Mode ---");
    let no_color_logger = LogFlow::new()
        .with_colors(false)
        .with_timestamps(true)
        .build()?;

    no_color_logger.info("Info without colors")?;
    no_color_logger.warn("Warning without colors")?;
    no_color_logger.error("Error without colors")?;

    println!("\n=== Custom Formatting Examples Complete ===");

    Ok(())
}
