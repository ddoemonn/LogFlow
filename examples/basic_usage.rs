use logflow::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a basic logger with default settings
    let logger = LogFlow::new().build()?;

    // Basic logging at different levels
    logger.trace("This is a trace message")?;
    logger.debug("This is a debug message")?;
    logger.info("Application started successfully")?;
    logger.warn("This is a warning message")?;
    logger.error("This is an error message")?;
    logger.fatal("This is a fatal error message")?;

    println!("\n--- Pretty Logger ---");

    // Create a pretty logger with colors
    let pretty_logger = LogFlow::new()
        .pretty()
        .with_level(LogLevel::Debug)
        .build()?;

    pretty_logger.trace("Trace with pretty formatting")?;
    pretty_logger.debug("Debug with colors")?;
    pretty_logger.info("Info message looks great!")?;
    pretty_logger.warn("Warning with visual indicators")?;
    pretty_logger.error("Error with clear highlighting")?;
    pretty_logger.fatal("Fatal error stands out!")?;

    println!("\n--- Compact Logger ---");

    // Compact logger for minimal output
    let compact_logger = LogFlow::new().compact().build()?;

    compact_logger.info("Compact info message")?;
    compact_logger.warn("Compact warning")?;
    compact_logger.error("Compact error")?;

    println!("\n--- JSON Logger ---");

    // JSON logger for structured output
    let json_logger = LogFlow::new().json().build()?;

    json_logger.info("JSON formatted message")?;
    json_logger.error("JSON error message")?;

    Ok(())
}
