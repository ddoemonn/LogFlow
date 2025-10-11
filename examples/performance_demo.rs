use logflow::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== LogFlow Performance Demonstration ===\n");

    // Test 2: Nested scope performance
    println!("\n--- Nested Scope Performance Test ---");
    test_nested_scope_performance()?;

    // Test 3: Field logging performance
    println!("\n--- Field Logging Performance Test ---");
    test_field_logging_performance()?;

    // Test 4: Different formatter performance
    println!("\n--- Formatter Performance Comparison ---");
    test_formatter_performance()?;

    // Test 5: Buffer vs direct output performance
    println!("\n--- Output Method Performance Comparison ---");
    test_output_performance()?;

    // Test 1: High-volume logging
    println!("--- High-Volume Logging Test ---");
    test_high_volume_logging()?;

    println!("\n=== Performance Tests Complete ===");

    Ok(())
}

fn test_high_volume_logging() -> Result<(), Box<dyn std::error::Error>> {
    let logger = LogFlow::new()
        .with_level(LogLevel::Info)
        .compact() // Use compact format for speed
        .build()?;

    let num_messages = 100;
    let start = Instant::now();

    for i in 0..num_messages {
        logger.info(&format!("High volume message #{}", i))?;
    }

    let duration = start.elapsed();
    let messages_per_second = num_messages as f64 / duration.as_secs_f64();

    println!("Logged {} messages in {:?}", num_messages, duration);
    println!("Performance: {:.0} messages/second", messages_per_second);

    Ok(())
}

fn test_nested_scope_performance() -> Result<(), Box<dyn std::error::Error>> {
    let logger = LogFlow::new().with_level(LogLevel::Info).pretty().build()?;

    let num_operations = 1_000;
    let start = Instant::now();

    for i in 0..num_operations {
        let scope1 = logger.begin_scope("level1");
        scope1.info(&format!("Level 1 message {}", i))?;

        {
            let scope2 = scope1.begin_scope("level2");
            scope2.info(&format!("Level 2 message {}", i))?;

            {
                let scope3 = scope2.begin_scope("level3");
                scope3.info(&format!("Level 3 message {}", i))?;
            } // scope3 ends
        } // scope2 ends
    } // scope1 ends

    let duration = start.elapsed();
    let operations_per_second = num_operations as f64 / duration.as_secs_f64();

    println!(
        "Completed {} nested operations in {:?}",
        num_operations, duration
    );
    println!(
        "Performance: {:.0} operations/second",
        operations_per_second
    );

    Ok(())
}

fn test_field_logging_performance() -> Result<(), Box<dyn std::error::Error>> {
    let logger = LogFlow::new()
        .with_level(LogLevel::Info)
        .json() // JSON format for structured data
        .build()?;

    let num_messages = 5_000;
    let start = Instant::now();

    for i in 0..num_messages {
        logger
            .with_field("iteration", i)
            .with_field("timestamp", chrono::Utc::now().to_rfc3339())
            .with_field("random_value", i * 42 % 1000)
            .with_field("category", if i % 3 == 0 { "critical" } else { "normal" })
            .info("Structured log message")?;
    }

    let duration = start.elapsed();
    let messages_per_second = num_messages as f64 / duration.as_secs_f64();

    println!(
        "Logged {} structured messages in {:?}",
        num_messages, duration
    );
    println!("Performance: {:.0} messages/second", messages_per_second);

    Ok(())
}

fn test_formatter_performance() -> Result<(), Box<dyn std::error::Error>> {
    let num_messages = 5_000;

    // Test Pretty formatter
    {
        let logger = LogFlow::new().pretty().build()?;
        let start = Instant::now();

        for i in 0..num_messages {
            logger.info(&format!("Pretty format message {}", i))?;
        }

        let duration = start.elapsed();
        println!(
            "Pretty formatter: {:?} ({:.0} msg/s)",
            duration,
            num_messages as f64 / duration.as_secs_f64()
        );
    }

    // Test Compact formatter
    {
        let logger = LogFlow::new().compact().build()?;
        let start = Instant::now();

        for i in 0..num_messages {
            logger.info(&format!("Compact format message {}", i))?;
        }

        let duration = start.elapsed();
        println!(
            "Compact formatter: {:?} ({:.0} msg/s)",
            duration,
            num_messages as f64 / duration.as_secs_f64()
        );
    }

    // Test JSON formatter
    {
        let logger = LogFlow::new().json().build()?;
        let start = Instant::now();

        for i in 0..num_messages {
            logger.info(&format!("JSON format message {}", i))?;
        }

        let duration = start.elapsed();
        println!(
            "JSON formatter: {:?} ({:.0} msg/s)",
            duration,
            num_messages as f64 / duration.as_secs_f64()
        );
    }

    Ok(())
}

fn test_output_performance() -> Result<(), Box<dyn std::error::Error>> {
    let num_messages = 3_000;

    // Test buffer output
    {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let logger = LogFlow::new()
            .with_output(OutputType::Buffer(buffer.clone()))
            .compact()
            .build()?;

        let start = Instant::now();

        for i in 0..num_messages {
            logger.info(&format!("Buffer message {}", i))?;
        }

        let duration = start.elapsed();
        println!(
            "Buffer output: {:?} ({:.0} msg/s)",
            duration,
            num_messages as f64 / duration.as_secs_f64()
        );

        // Check buffer size
        if let Ok(buffer_data) = buffer.lock() {
            println!("  Buffer size: {} bytes", buffer_data.len());
        };
    }

    // Test stdout output (default)
    {
        let logger = LogFlow::new()
            .with_output(OutputType::Stdout)
            .compact()
            .build()?;

        let start = Instant::now();

        for i in 0..num_messages {
            logger.info(&format!("Stdout message {}", i))?;
        }

        let duration = start.elapsed();
        println!(
            "Stdout output: {:?} ({:.0} msg/s)",
            duration,
            num_messages as f64 / duration.as_secs_f64()
        );
    }

    Ok(())
}
