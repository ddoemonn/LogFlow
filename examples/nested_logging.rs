use logflow::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a development logger with all features enabled
    let logger = LogFlow::new().dev().with_level(LogLevel::Debug).build()?;

    logger.info("Starting application with nested logging example")?;

    // Create nested scopes to demonstrate hierarchical logging
    {
        let database_scope = logger.begin_scope("database");
        database_scope.info("Connecting to database")?;

        {
            let query_scope = database_scope.begin_scope("query");
            query_scope.debug("Preparing SQL query")?;
            query_scope.debug("Query: SELECT * FROM users WHERE active = true")?;
            query_scope.info("Query executed successfully")?;

            {
                let result_scope = query_scope.begin_scope("result_processing");
                result_scope.debug("Processing 150 records")?;
                result_scope.debug("Applying business logic filters")?;
                result_scope.info("Results processed and cached")?;
            } // result_processing scope ends

            query_scope.info("Query operation completed")?;
        } // query scope ends

        database_scope.info("Database connection closed")?;
    } // database scope ends

    logger.info("Application startup completed")?;

    // Demonstrate nested HTTP request handling
    simulate_http_request(&logger)?;

    Ok(())
}

fn simulate_http_request(logger: &LogFlow) -> Result<(), Box<dyn std::error::Error>> {
    let http_scope = logger.begin_scope("http_handler");
    http_scope.info("Received HTTP request: GET /api/users")?;

    {
        let auth_scope = http_scope.begin_scope("authentication");
        auth_scope.debug("Validating JWT token")?;
        auth_scope.info("User authenticated: user_id=12345")?;
    }

    {
        let business_scope = http_scope.begin_scope("business_logic");
        business_scope.debug("Checking user permissions")?;
        business_scope.debug("Loading user preferences")?;
        business_scope.info("Business logic completed")?;
    }

    {
        let response_scope = http_scope.begin_scope("response");
        response_scope.debug("Serializing response data")?;
        response_scope.debug("Setting response headers")?;
        response_scope.info("Response sent: 200 OK")?;
    }

    http_scope.info("HTTP request completed successfully")?;

    Ok(())
}
