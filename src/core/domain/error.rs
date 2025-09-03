use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Authentication error: {message}")]
    Authentication { message: String },

    #[error("Authorization error: {message}")]
    Authorization { message: String },

    #[error("Not found: {resource}")]
    NotFound { resource: String },

    #[error("Conflict: {message}")]
    Conflict { message: String },

    #[error("Internal server error")]
    Internal,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub meta: MetaInfo,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MetaInfo {
    pub version: String,
    pub app: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let meta = MetaInfo {
            version: "1.0.0".to_string(),
            app: "afaf-rest-rust".to_string(),
        };

        let (error_type, message, mut status) = match self {
            AppError::Database(e) => {
                log::error!("Database error: {:?}", e);
                match e {
                    sqlx::Error::RowNotFound => {
                        ("not_found", "Resource not found", HttpResponse::NotFound())
                    }
                    sqlx::Error::Database(db_err) => {
                        if db_err.constraint().is_some() {
                            (
                                "conflict",
                                "Resource already exists",
                                HttpResponse::Conflict(),
                            )
                        } else {
                            (
                                "database_error",
                                "Database operation failed",
                                HttpResponse::InternalServerError(),
                            )
                        }
                    }
                    _ => (
                        "database_error",
                        "Database operation failed",
                        HttpResponse::InternalServerError(),
                    ),
                }
            }
            AppError::Validation { message } => (
                "validation_error",
                message.as_str(),
                HttpResponse::BadRequest(),
            ),
            AppError::Authentication { message } => (
                "authentication_error",
                message.as_str(),
                HttpResponse::Unauthorized(),
            ),
            AppError::Authorization { message } => (
                "authorization_error",
                message.as_str(),
                HttpResponse::Forbidden(),
            ),
            AppError::NotFound { resource: _ } => {
                ("not_found", "Resource not found", HttpResponse::NotFound())
            }
            AppError::Conflict { message } => {
                ("conflict", message.as_str(), HttpResponse::Conflict())
            }
            AppError::Internal => (
                "internal_error",
                "Internal server error",
                HttpResponse::InternalServerError(),
            ),
        };

        status.json(ErrorResponse {
            error: error_type.to_string(),
            message: message.to_string(),
            meta,
        })
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let message = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let field_errors: Vec<String> = errors
                    .iter()
                    .map(|error| {
                        error
                            .message
                            .as_ref()
                            .map(|msg| msg.to_string())
                            .unwrap_or_else(|| format!("Invalid {}", field))
                    })
                    .collect();
                format!("{}: {}", field, field_errors.join(", "))
            })
            .collect::<Vec<String>>()
            .join("; ");

        AppError::Validation { message }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(_: bcrypt::BcryptError) -> Self {
        AppError::Internal
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        AppError::Authentication {
            message: "Invalid token".to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
