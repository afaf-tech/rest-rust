use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub rest_url: String,
    pub log_dir: String,
    pub log_name: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            rest_url: env::var("REST_URL").expect("REST_URL must be set"),
            log_dir: env::var("LOG_DIR").expect("LOG_DIR must be set"),
            log_name: env::var("LOG_NAME").expect("LOG_NAME must be set"),
        }
    }
}
