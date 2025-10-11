use logflow::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a beautiful logger for the showcase
    let logger = LogFlow::new()
        .pretty()
        .with_level(LogLevel::Debug)
        .with_date(true)
        .build()?;

    // Demonstrate subtitle functionality
    logger.info_with_subtitle("STARTUP", "Starting LogFlow Web Application")?;
    logger.info_with_subtitle("CONFIG", "Loading configuration from config.yaml")?;
    logger.info_with_subtitle("DATABASE", "Connecting to database at localhost:5432")?;

    {
        let db_scope = logger.begin_scope("database");
        db_scope.debug_with_subtitle("MIGRATION", "Running database migrations")?;
        db_scope.info_with_subtitle("CONNECTION", "Database connection established")?;
        db_scope.debug_with_subtitle("POOL", "Connection pool size: 10")?;
    }

    logger.info_with_subtitle("SERVER", "Starting HTTP server on port 8080")?;

    // Simulate some API requests
    {
        let server_scope = logger.begin_scope("http_server");
        server_scope.info_with_subtitle("LISTENING", "Server listening on http://localhost:8080")?;

        // Simulate incoming requests
        simulate_api_request(&server_scope, "POST", "/api/users", 201, 45)?;
        simulate_api_request(&server_scope, "GET", "/api/users/123", 200, 12)?;
        simulate_api_request(&server_scope, "PUT", "/api/users/123", 200, 89)?;
        simulate_api_request(&server_scope, "DELETE", "/api/users/456", 404, 8)?;
        simulate_api_request(&server_scope, "GET", "/api/orders", 200, 156)?;
    }

    // Simulate some background tasks
    {
        let background_scope = logger.begin_scope("background_tasks");
        background_scope.info_with_subtitle("JOBS", "Starting background job processor")?;

        {
            let email_scope = background_scope.begin_scope("email_service");
            email_scope.info_with_subtitle("EMAIL", "Processing email queue")?;
            email_scope
                .with_field("emails_sent", 42)
                .with_field("failed", 1)
                .with_subtitle("BATCH")
                .info("Email batch processed")?;
        }

        {
            let cleanup_scope = background_scope.begin_scope("cleanup");
            cleanup_scope.debug_with_subtitle("CLEANUP", "Cleaning up temporary files")?;
            cleanup_scope
                .with_field("files_deleted", 156)
                .with_field("space_freed_mb", 23.4)
                .with_subtitle("COMPLETE")
                .info("Cleanup completed")?;
        }
    }

    // Simulate some errors and warnings
    logger.warn_with_subtitle("MEMORY", "High memory usage detected: 85% of available RAM")?;

    {
        let cache_scope = logger.begin_scope("cache");
        cache_scope
            .with_field("cache_hit_rate", 0.94)
            .with_field("total_requests", 15847)
            .with_subtitle("METRICS")
            .info("Cache performance metrics")?;

        cache_scope.error_with_subtitle("REDIS", "Redis connection lost, falling back to in-memory cache")?;
        cache_scope.info_with_subtitle("RETRY", "Attempting to reconnect to Redis...")?;
        cache_scope.info_with_subtitle("SUCCESS", "Redis connection restored")?;
    }

    // Application metrics
    logger
        .with_field("uptime_seconds", 3600)
        .with_field("total_requests", 1247)
        .with_field("active_connections", 23)
        .with_field("memory_usage_mb", 156.7)
        .with_subtitle("HEALTH")
        .info("Application health metrics")?;

    logger.info_with_subtitle("STATUS", "Application running smoothly - all systems operational")?;
    logger.debug_with_subtitle("BACKUP", "Next scheduled backup in 2 hours")?;

    Ok(())
}

fn simulate_api_request(
    scope: &logflow::LogScope,
    method: &str,
    path: &str,
    status: u16,
    duration_ms: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let request_scope = scope.begin_scope("request");

    request_scope
        .with_field("method", method)
        .with_field("path", path)
        .with_field("user_agent", "LogFlow-Client/1.0")
        .with_subtitle("REQUEST")
        .debug("Incoming HTTP request")?;

    // Simulate request processing
    if status >= 400 {
        request_scope
            .with_field("status", status)
            .with_field("duration_ms", duration_ms)
            .with_subtitle("FAILED")
            .error("Request failed")?;
    } else {
        request_scope
            .with_field("status", status)
            .with_field("duration_ms", duration_ms)
            .with_subtitle("SUCCESS")
            .info("Request completed")?;
    }

    Ok(())
}