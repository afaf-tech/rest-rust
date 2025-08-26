use serde::Serialize;
use utoipa::ToSchema;

// Constants for meta information
const APP_NAME: &str = "afaf-rest-rust";
const VERSION: &str = "1.0.0";

/// Error response structure
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Metadata about the API
    pub meta: Meta,
    /// Error code identifier
    #[schema(example = "not_found")]
    pub error: String,
    /// Detailed error message
    #[schema(example = "The requested resource was not found")]
    pub message: String,
}

/// API metadata structure
#[derive(Debug, Serialize, ToSchema)]
pub struct Meta {
    /// API version
    #[schema(example = "1.0.0")]
    pub version: String,
    /// Application name
    #[schema(example = "afaf-rest-rust")]
    pub app: String,
}

// Helper function to build the error response
pub fn build_error_response(error_code: &str, message: &str) -> ErrorResponse {
    ErrorResponse {
        meta: Meta {
            version: VERSION.to_string(),
            app: APP_NAME.to_string(),
        },
        error: error_code.to_string(),
        message: message.to_string(),
    }
}

/// Success response structure
#[derive(Debug, Serialize, ToSchema)]
pub struct Response<T> {
    /// Metadata about the API
    pub meta: Meta,
    /// Response payload
    pub data: T,
    /// Success message
    #[schema(example = "Operation completed successfully")]
    pub message: String,
}

// Helper function to build the success response
pub fn build_success_response<T: Serialize>(data: T, message: &str) -> Response<T> {
    Response {
        meta: Meta {
            version: VERSION.to_string(),
            app: APP_NAME.to_string(),
        },
        data,
        message: message.to_string(),
    }
}
