use crate::core::rest::router;
use crate::core::{
    domain::auth::jwt::JwtService,
    rest::middleware::{error_handler::ErrorHandler, http_logger::HttpLogger},
};
use crate::{config::Config, pkg::logger::setup_logger};
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

#[actix_web::main]
pub async fn run_rest() -> std::io::Result<()> {
    let config = Config::from_env();

    // Setup postgres
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Database connection failed");

    // Setup JWT service
    let jwt_service = JwtService::new(&config.jwt_secret);

    setup_logger(&config);

    log::info!("Starting server at http://{}", config.rest_url);
    log::info!(
        "JWT expiration set to {} hours",
        config.jwt_expiration_hours
    );
    log::info!(
        "📚 API Documentation: http://{}/swagger-ui/",
        config.rest_url
    );
    log::info!("🔗 Available endpoints:");
    log::info!("  • GET  /users - List all users");
    log::info!("  • POST /users - Create new user");
    log::info!("  • POST /auth/register - Register new user");
    log::info!("  • POST /auth/login - User login");
    log::info!("  • GET  /auth/me - Get current user");
    log::info!("  • PUT  /auth/password - Change password");
    log::info!("  • POST /auth/admin/users - Admin create user");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the database pool
            .app_data(web::Data::new(jwt_service.clone())) // Share the JWT service
            .wrap(ErrorHandler) // Error handling middleware
            .wrap(HttpLogger) // HTTP logging middleware
            .configure(router::config) // Configure routes
    })
    .workers(2) // Set the number of workers
    .bind(&config.rest_url)?
    .run()
    .await
}
