use actix_web::{dev::Payload, Error, FromRequest, HttpMessage, HttpRequest};
use futures::future::{ready, Ready};
use uuid::Uuid;

use crate::core::domain::{auth::jwt::JwtService, error::AppError, users::model::UserRole};

/// Authentication data extracted from JWT token
#[derive(Debug, Clone)]
pub struct AuthData {
    pub user_id: Uuid,
    pub email: String,
    pub role: UserRole,
}

/// Extractor for authentication data from request
impl FromRequest for AuthData {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<AuthData>() {
            Some(auth_data) => ready(Ok(auth_data.clone())),
            None => {
                let error = AppError::Authentication {
                    message: "Authentication data not found in request".to_string(),
                };
                ready(Err(error.into()))
            }
        }
    }
}

/// Authentication functions for extracting and validating JWT tokens
pub struct AuthExtractor;

impl AuthExtractor {
    /// Extract and validate JWT token from request, return AuthData
    pub fn extract_auth_data(
        req: &HttpRequest,
        jwt_service: &JwtService,
    ) -> Result<AuthData, AppError> {
        // Extract Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::Authentication {
                message: "Missing Authorization header".to_string(),
            })?;

        // Extract token from header (remove "Bearer " prefix)
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Authentication {
                message: "Invalid authorization format".to_string(),
            });
        }
        let token = &auth_header[7..];

        // Validate token
        let claims = jwt_service.verify_token(token)?;

        // Parse user role from claims
        let user_role = claims
            .role
            .parse::<UserRole>()
            .map_err(|_| AppError::Authentication {
                message: "Invalid role in token".to_string(),
            })?;

        // Parse user ID
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Authentication {
            message: "Invalid user ID in token".to_string(),
        })?;

        Ok(AuthData {
            user_id,
            email: claims.email,
            role: user_role,
        })
    }

    /// Check if user has required role
    pub fn check_role(auth_data: &AuthData, required_role: &UserRole) -> Result<(), AppError> {
        let has_permission = match required_role {
            UserRole::User => true, // All authenticated users have user-level access
            UserRole::Moderator => matches!(auth_data.role, UserRole::Moderator | UserRole::Admin),
            UserRole::Admin => matches!(auth_data.role, UserRole::Admin),
        };

        if !has_permission {
            return Err(AppError::Authorization {
                message: format!("Insufficient permissions. Required: {}", required_role),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_checking_logic() {
        let auth_data = AuthData {
            user_id: uuid::Uuid::new_v4(),
            email: "test@example.com".to_string(),
            role: UserRole::Admin,
        };

        // Admin should have access to all roles
        assert!(AuthExtractor::check_role(&auth_data, &UserRole::User).is_ok());
        assert!(AuthExtractor::check_role(&auth_data, &UserRole::Moderator).is_ok());
        assert!(AuthExtractor::check_role(&auth_data, &UserRole::Admin).is_ok());

        let user_auth_data = AuthData {
            user_id: uuid::Uuid::new_v4(),
            email: "user@example.com".to_string(),
            role: UserRole::User,
        };

        // Regular user should only have user-level access
        assert!(AuthExtractor::check_role(&user_auth_data, &UserRole::User).is_ok());
        assert!(AuthExtractor::check_role(&user_auth_data, &UserRole::Moderator).is_err());
        assert!(AuthExtractor::check_role(&user_auth_data, &UserRole::Admin).is_err());
    }
}
