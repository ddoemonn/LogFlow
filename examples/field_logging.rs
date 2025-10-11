use logflow::prelude::*;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a pretty logger for demonstrations
    let logger = LogFlow::new()
        .pretty()
        .with_level(LogLevel::Debug)
        .build()?;

    logger.info("Demonstrating field logging capabilities")?;

    // Logging with simple fields
    logger
        .with_field("user_id", 12345)
        .with_field("action", "login")
        .info("User logged in")?;

    // Logging with multiple fields
    logger
        .with_field("request_id", "req_abc123")
        .with_field("method", "POST")
        .with_field("path", "/api/orders")
        .with_field("status", 201)
        .with_field("duration_ms", 145)
        .info("API request completed")?;

    // Logging with complex field values
    let user_data = json!({
        "id": 67890,
        "email": "alice@example.com",
        "role": "admin",
        "last_login": "2024-10-11T10:30:00Z",
        "permissions": ["read", "write", "admin"]
    });

    logger
        .with_field("user", user_data)
        .with_field("operation", "user_update")
        .info("User profile updated")?;

    // Error logging with context
    logger
        .with_field("error_code", "DB_CONNECTION_FAILED")
        .with_field("retry_count", 3)
        .with_field("last_attempt", "2024-10-11T10:35:00Z")
        .error("Database connection failed after retries")?;

    // Business metrics logging
    logger
        .with_field("metric", "order_processing")
        .with_field("orders_processed", 1247)
        .with_field("average_time_ms", 234)
        .with_field("success_rate", 0.987)
        .info("Order processing metrics")?;

    // Nested scope with fields
    {
        let payment_scope = logger.begin_scope("payment_processing");

        payment_scope
            .with_field("payment_id", "pay_xyz789")
            .with_field("amount", 29.99)
            .with_field("currency", "USD")
            .info("Payment processing started")?;

        // Simulate payment steps with different field contexts
        {
            let validation_scope = payment_scope.begin_scope("validation");
            validation_scope
                .with_field("card_type", "visa")
                .with_field("card_last4", "1234")
                .debug("Validating payment method")?;

            validation_scope
                .with_field("fraud_score", 0.12)
                .with_field("risk_level", "low")
                .info("Fraud check passed")?;
        }

        {
            let gateway_scope = payment_scope.begin_scope("gateway");
            gateway_scope
                .with_field("gateway", "stripe")
                .with_field("transaction_id", "txn_456def")
                .info("Payment submitted to gateway")?;

            gateway_scope
                .with_field("gateway_response", "approved")
                .with_field("auth_code", "123456")
                .info("Payment approved by gateway")?;
        }

        payment_scope
            .with_field("final_status", "completed")
            .with_field("processing_time_ms", 1850)
            .info("Payment processing completed")?;
    }

    logger.info("Field logging demonstration completed")?;

    Ok(())
}
