use crate::core::{
    domain::{auth::jwt::JwtService, error::AppError},
    rest::middleware::auth::{AuthData, AuthExtractor},
};
use actix_web::{HttpMessage, HttpRequest};

/// Guard function that validates JWT token and extracts auth data
pub async fn auth_guard(req: &HttpRequest, jwt_service: &JwtService) -> Result<AuthData, AppError> {
    AuthExtractor::extract_auth_data(req, jwt_service)
}

/// Macro to create protected route handlers
#[macro_export]
macro_rules! protected_route {
    ($handler:ident, $jwt_service:expr) => {
        |req: actix_web::HttpRequest, payload: actix_web::web::Payload| async move {
            match auth_guard(&req, $jwt_service).await {
                Ok(auth_data) => {
                    // Add auth data to request extensions
                    req.extensions_mut().insert(auth_data);
                    $handler(req, payload).await
                }
                Err(error) => Ok(actix_web::HttpResponse::Unauthorized().json(error.to_string())),
            }
        }
    };
}

/// Helper function to manually inject auth data for testing/integration
pub fn inject_auth_data(req: &HttpRequest, auth_data: AuthData) {
    req.extensions_mut().insert(auth_data);
}
