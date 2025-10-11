use logflow::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Timestamp Format Examples ===\n");

    // Default: Time only
    println!("--- Default (Time Only) ---");
    let time_only_logger = LogFlow::new().pretty().with_timestamps(true).build()?;

    time_only_logger.info("Time only format")?;
    time_only_logger.warn("Warning with time only")?;
    time_only_logger.error("Error with time only")?;

    println!("\n--- Full Date and Time ---");
    let full_datetime_logger = LogFlow::new()
        .pretty()
        .with_timestamps(true)
        .with_date(true)
        .build()?;

    full_datetime_logger.info("Full date and time format")?;
    full_datetime_logger.warn("Warning with full datetime")?;
    full_datetime_logger.error("Error with full datetime")?;

    println!("\n--- No Timestamps ---");
    let no_timestamp_logger = LogFlow::new().pretty().with_timestamps(false).build()?;

    no_timestamp_logger.info("No timestamp format")?;
    no_timestamp_logger.warn("Warning without timestamp")?;
    no_timestamp_logger.error("Error without timestamp")?;

    println!("\n--- Compact Mode with Date ---");
    let compact_with_date = LogFlow::new()
        .compact()
        .with_timestamps(true)
        .with_date(true)
        .build()?;

    compact_with_date.info("Compact with full date")?;
    compact_with_date.warn("Compact warning with date")?;
    compact_with_date.error("Compact error with date")?;

    println!("\n--- JSON with Timestamps ---");
    let json_logger = LogFlow::new().json().build()?;

    json_logger.info("JSON always includes full ISO timestamp")?;

    println!("\n--- Nested Logging with Full Dates ---");
    let nested_logger = LogFlow::new().dev().with_date(true).build()?;

    nested_logger.info("Starting process with full dates")?;
    {
        let scope = nested_logger.begin_scope("database");
        scope.info("Database operation with full timestamp")?;
        {
            let inner_scope = scope.begin_scope("query");
            inner_scope.debug("Query execution with nested date logging")?;
        }
    }
    nested_logger.info("Process completed")?;

    println!("\n=== Timestamp Format Examples Complete ===");

    Ok(())
}
