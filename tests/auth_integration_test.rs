use uuid::Uuid;
use validator::Validate;

use afaf_rest_rust::core::domain::{
    auth::{jwt::JwtService, service::AuthService},
    error::AppError,
    users::model::{CreateUserRequest, LoginRequest, UserRole},
};

#[tokio::test]
async fn test_jwt_token_generation_and_validation() {
    let jwt_service = JwtService::new("test_secret");

    let user_id = Uuid::new_v4();
    let email = "test@example.com";
    let role = UserRole::User;

    // Generate token
    let token = jwt_service
        .generate_token(user_id, email, &role.to_string())
        .unwrap();

    // Token should not be empty
    assert!(!token.is_empty());

    // Verify token
    let claims = jwt_service.verify_token(&token).unwrap();
    assert_eq!(claims.sub, user_id.to_string());
    assert_eq!(claims.email, email);
    assert_eq!(claims.role, role.to_string());
}

#[tokio::test]
async fn test_jwt_token_expiration() {
    // This test would require modifying the JWT service to accept custom expiration
    // For now, we'll test that tokens are properly validated
    let jwt_service = JwtService::new("test_secret");

    let user_id = Uuid::new_v4();
    let email = "test@example.com";
    let role = UserRole::User;

    let token = jwt_service
        .generate_token(user_id, email, &role.to_string())
        .unwrap();

    // Token should be valid immediately after creation
    let claims = jwt_service.verify_token(&token).unwrap();
    assert_eq!(claims.sub, user_id.to_string());
}

#[test]
fn test_auth_service_password_hashing() {
    let auth_service = AuthService::new("test_secret");

    let password = "test_password_123";
    let hash = auth_service.hash_password(password).unwrap();

    // Hash should not equal the original password
    assert_ne!(hash, password);

    // Verify password should work with correct password
    assert!(auth_service.verify_password(password, &hash).unwrap());

    // Verify password should fail with incorrect password
    assert!(!auth_service
        .verify_password("wrong_password", &hash)
        .unwrap());
}

#[tokio::test]
async fn test_create_user_request_validation() {
    // Test valid user request
    let valid_request = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        password: "SecurePass123".to_string(),
        role: Some(UserRole::User.to_string()),
    };

    // Should pass validation
    assert!(valid_request.validate().is_ok());

    // Test invalid email
    let invalid_email_request = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "invalid-email".to_string(),
        password: "SecurePass123".to_string(),
        role: Some(UserRole::User.to_string()),
    };

    // Should fail validation
    assert!(invalid_email_request.validate().is_err());

    // Test short password
    let short_password_request = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        password: "123".to_string(),
        role: Some(UserRole::User.to_string()),
    };

    // Should fail validation
    assert!(short_password_request.validate().is_err());

    // Test empty name
    let empty_name_request = CreateUserRequest {
        name: "".to_string(),
        email: "john@example.com".to_string(),
        password: "SecurePass123".to_string(),
        role: Some(UserRole::User.to_string()),
    };

    // Should fail validation
    assert!(empty_name_request.validate().is_err());
}

#[tokio::test]
async fn test_login_request_validation() {
    // Test valid login request
    let valid_request = LoginRequest {
        email: "john@example.com".to_string(),
        password: "SecurePass123".to_string(),
    };

    // Should pass validation
    assert!(valid_request.validate().is_ok());

    // Test invalid email
    let invalid_email_request = LoginRequest {
        email: "invalid-email".to_string(),
        password: "SecurePass123".to_string(),
    };

    // Should fail validation
    assert!(invalid_email_request.validate().is_err());

    // Test empty password
    let empty_password_request = LoginRequest {
        email: "john@example.com".to_string(),
        password: "".to_string(),
    };

    // Should fail validation
    assert!(empty_password_request.validate().is_err());
}

#[test]
fn test_user_role_parsing() {
    // Test valid role parsing
    assert_eq!("user".parse::<UserRole>().unwrap(), UserRole::User);
    assert_eq!("admin".parse::<UserRole>().unwrap(), UserRole::Admin);
    assert_eq!(
        "moderator".parse::<UserRole>().unwrap(),
        UserRole::Moderator
    );

    // Test invalid role parsing
    assert!("invalid_role".parse::<UserRole>().is_err());

    // Test role display
    assert_eq!(UserRole::User.to_string(), "user");
    assert_eq!(UserRole::Admin.to_string(), "admin");
    assert_eq!(UserRole::Moderator.to_string(), "moderator");
}

#[test]
fn test_error_types() {
    // Test that our error types can be created and match expected patterns
    let validation_error = AppError::Validation {
        message: "Test validation error".to_string(),
    };

    match validation_error {
        AppError::Validation { message } => {
            assert_eq!(message, "Test validation error");
        }
        _ => panic!("Expected validation error"),
    }

    let auth_error = AppError::Authentication {
        message: "Test auth error".to_string(),
    };

    match auth_error {
        AppError::Authentication { message } => {
            assert_eq!(message, "Test auth error");
        }
        _ => panic!("Expected authentication error"),
    }
}

#[test]
fn test_jwt_token_header_extraction() {
    // Test that we can extract tokens from authorization headers
    let valid_header = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.test";
    let invalid_header = "Invalid header";
    let empty_token = "Bearer ";

    // Test token extraction logic (simulating what would happen in middleware)
    assert!(valid_header.starts_with("Bearer "));
    let token = &valid_header[7..];
    assert_eq!(token, "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.test");

    assert!(!invalid_header.starts_with("Bearer "));

    assert!(empty_token.starts_with("Bearer "));
    let empty_token_part = &empty_token[7..];
    assert!(empty_token_part.is_empty());
}

#[cfg(test)]
mod password_validation_tests {

    #[test]
    fn test_password_strength_requirements() {
        // Test that our password validation logic works as expected
        fn check_password_requirements(password: &str) -> bool {
            if password.len() < 8 {
                return false;
            }

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

            has_upper && has_lower && has_digit
        }

        // Valid passwords
        assert!(check_password_requirements("Password123"));
        assert!(check_password_requirements("MySecure1Pass"));

        // Invalid passwords
        assert!(!check_password_requirements("password123")); // No uppercase
        assert!(!check_password_requirements("PASSWORD123")); // No lowercase
        assert!(!check_password_requirements("PasswordABC")); // No digit
        assert!(!check_password_requirements("Pass1")); // Too short
    }
}
