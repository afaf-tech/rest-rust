use crate::core::rest::middleware::http_logger::HttpLogger;
use crate::core::rest::router;
use crate::{config::Config, pkg::logger::setup_logger};
use actix_web::{App, HttpServer};
use sqlx::PgPool;

#[actix_web::main]
pub async fn run_rest() -> std::io::Result<()> {
    let config = Config::from_env();

    // setup postgres
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Database connection failed");

    setup_logger(&config);

    log::info!("Starting server at http://{}", config.rest_url);

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone())) // Share the database pool
            .wrap(HttpLogger) // Use the custom logging middleware
            .configure(router::config) // Configure routes
    })
    .workers(2) // Set the number of workers
    .bind(&config.rest_url)?
    .run()
    .await
}
