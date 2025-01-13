use serde::Serialize;

// Constants for meta information
const APP_NAME: &str = "afaf-rest-rust";
const VERSION: &str = "1.0.0";

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    meta: Meta,
    error: String,  // Error code
    message: String, // Detailed error message
}

#[derive(Debug, Serialize)]
pub struct Meta {
    version: String,
    app: String,
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

#[derive(Debug, Serialize)]
pub struct Response<T> {
    meta: Meta,
    data: T,
    message: String,
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
