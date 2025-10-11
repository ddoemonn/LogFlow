#[cfg(feature = "async")]
use logflow::prelude::*;
#[cfg(feature = "async")]
use tokio::time::{sleep, Duration};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an async logger with buffering
    let logger = AsyncLogFlow::new()
        .pretty()
        .with_level(LogLevel::Debug)
        .with_buffer_size(50)
        .with_flush_interval(Duration::from_millis(200))
        .build()
        .await?;

    // Start background flushing
    let _flush_task = logger.start_background_flush();

    logger.info("Starting async logging demonstration").await?;

    // Simulate concurrent operations
    let tasks = vec![
        simulate_async_operation(&logger, "user_service", 1),
        simulate_async_operation(&logger, "order_service", 2),
        simulate_async_operation(&logger, "notification_service", 3),
    ];

    // Wait for all tasks to complete
    futures::future::try_join_all(tasks).await?;

    // Demonstrate async scopes
    {
        let batch_scope = logger.begin_scope("batch_processing").await;
        batch_scope.info("Starting batch job processing").await?;

        for i in 1..=5 {
            let item_scope = batch_scope.begin_scope(&format!("item_{}", i)).await;
            item_scope
                .with_field("item_id", i)
                .with_field("status", "processing")
                .debug("Processing batch item")
                .await?;

            // Simulate processing time
            sleep(Duration::from_millis(100)).await;

            item_scope
                .with_field("item_id", i)
                .with_field("status", "completed")
                .with_field("processing_time_ms", 100)
                .info("Batch item completed")
                .await?;
        }

        batch_scope.info("Batch job completed successfully").await?;
    }

    // Final flush to ensure all logs are written
    logger.flush().await?;
    logger.info("Async logging demonstration completed").await?;

    // Give time for background flush
    sleep(Duration::from_millis(300)).await;

    Ok(())
}

#[cfg(feature = "async")]
async fn simulate_async_operation(
    logger: &AsyncLogFlow,
    service_name: &str,
    operation_id: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let service_scope = logger.begin_scope(service_name).await;

    service_scope
        .with_field("operation_id", operation_id)
        .with_field("service", service_name)
        .info("Service operation started")
        .await?;

    // Simulate some async work
    for step in 1..=3 {
        service_scope
            .with_field("operation_id", operation_id)
            .with_field("step", step)
            .debug(&format!("Executing step {}", step))
            .await?;

        // Simulate async I/O
        sleep(Duration::from_millis(50)).await;
    }

    // Simulate potential error in one service
    if operation_id == 2 && service_name == "order_service" {
        service_scope
            .with_field("operation_id", operation_id)
            .with_field("error_type", "validation_error")
            .warn("Order validation failed, retrying...")
            .await?;

        sleep(Duration::from_millis(100)).await;

        service_scope
            .with_field("operation_id", operation_id)
            .with_field("retry_attempt", 1)
            .info("Retry successful")
            .await?;
    }

    service_scope
        .with_field("operation_id", operation_id)
        .with_field("service", service_name)
        .with_field("duration_ms", 200 + operation_id * 50)
        .info("Service operation completed")
        .await?;

    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("This example requires the 'async' feature to be enabled.");
    println!("Run with: cargo run --example async_logging --features async");
}
