use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::core::domain::error::{AppError, Result};

/// User creation payload with validation
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserPayload {
    /// Full name of the user (must be between 2 and 100 characters)
    #[validate(length(
        min = 2,
        max = 100,
        message = "Name must be between 2 and 100 characters"
    ))]
    #[schema(example = "John Doe")]
    pub name: String,

    /// Email address of the user (must be a valid email format)
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "john.doe@example.com")]
    pub email: String,

    /// Password (must be at least 8 characters long)
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[schema(example = "password123")]
    pub password: String,
}

/// User login payload with validation
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginPayload {
    /// Email address of the user
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "john.doe@example.com")]
    pub email: String,

    /// User password
    #[validate(length(min = 1, message = "Password is required"))]
    #[schema(example = "password123")]
    pub password: String,
}

/// User update payload with validation (all fields optional)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserPayload {
    /// Full name of the user (optional, must be between 2 and 100 characters if provided)
    #[validate(length(
        min = 2,
        max = 100,
        message = "Name must be between 2 and 100 characters"
    ))]
    #[schema(example = "John Doe")]
    pub name: Option<String>,

    /// Email address of the user (optional, must be valid email format if provided)
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "john.doe@example.com")]
    pub email: Option<String>,
}

/// Password change payload with validation
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordPayload {
    /// Current password
    #[validate(length(min = 1, message = "Current password is required"))]
    #[schema(example = "old_password")]
    pub current_password: String,

    /// New password (must be at least 8 characters long)
    #[validate(length(min = 8, message = "New password must be at least 8 characters long"))]
    #[schema(example = "new_password123")]
    pub new_password: String,
}

/// Trait for validating request payloads
pub trait ValidatedPayload: Validate {
    /// Validate the payload and return an AppResult
    fn validate_payload(&self) -> Result<()> {
        self.validate().map_err(AppError::from)
    }
}

// Implement ValidatedPayload for all our payload types
impl ValidatedPayload for CreateUserPayload {}
impl ValidatedPayload for LoginPayload {}
impl ValidatedPayload for UpdateUserPayload {}
impl ValidatedPayload for ChangePasswordPayload {}

/// Custom validator for password strength
pub fn validate_password_strength(password: &str) -> Result<()> {
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in password.chars() {
        if c.is_uppercase() {
            has_upper = true;
        } else if c.is_lowercase() {
            has_lower = true;
        } else if c.is_ascii_digit() {
            has_digit = true;
        }
    }

    if password.len() < 8 {
        return Err(AppError::Validation {
            message: "Password must be at least 8 characters long".to_string(),
        });
    }

    if !has_upper || !has_lower || !has_digit {
        return Err(AppError::Validation {
            message: "Password must contain at least one uppercase letter, one lowercase letter, and one digit".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_payload_validation() {
        // Valid payload
        let valid_payload = CreateUserPayload {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert!(valid_payload.validate_payload().is_ok());

        // Invalid email
        let invalid_email = CreateUserPayload {
            name: "John Doe".to_string(),
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
        };
        assert!(invalid_email.validate_payload().is_err());

        // Short name
        let short_name = CreateUserPayload {
            name: "J".to_string(),
            email: "john@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert!(short_name.validate_payload().is_err());

        // Short password
        let short_password = CreateUserPayload {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            password: "pass".to_string(),
        };
        assert!(short_password.validate_payload().is_err());
    }

    #[test]
    fn test_password_strength_validation() {
        // Too short
        assert!(validate_password_strength("Pass1!").is_err());

        // No uppercase
        assert!(validate_password_strength("password123!").is_err());

        // No lowercase
        assert!(validate_password_strength("PASSWORD123!").is_err());

        // No digit
        assert!(validate_password_strength("Password!").is_err());

        // Valid strong password
        assert!(validate_password_strength("Password123!").is_ok());
        assert!(validate_password_strength("StrongPass123").is_ok());
    }

    #[test]
    fn test_login_payload_validation() {
        // Valid payload
        let valid_payload = LoginPayload {
            email: "john@example.com".to_string(),
            password: "password".to_string(),
        };
        assert!(valid_payload.validate_payload().is_ok());

        // Invalid email
        let invalid_email = LoginPayload {
            email: "not-an-email".to_string(),
            password: "password".to_string(),
        };
        assert!(invalid_email.validate_payload().is_err());

        // Empty password
        let empty_password = LoginPayload {
            email: "john@example.com".to_string(),
            password: "".to_string(),
        };
        assert!(empty_password.validate_payload().is_err());
    }
}
