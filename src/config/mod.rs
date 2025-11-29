use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub rest_url: String,
    pub log_dir: String,
    pub log_name: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            rest_url: env::var("REST_URL").expect("REST_URL must be set"),
            log_dir: env::var("LOG_DIR").expect("LOG_DIR must be set"),
            log_name: env::var("LOG_NAME").expect("LOG_NAME must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default-secret-key-change-in-production".to_string()),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .expect("JWT_EXPIRATION_HOURS must be a valid number"),
        }
    }
}
