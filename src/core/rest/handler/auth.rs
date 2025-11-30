use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use validator::Validate;

use crate::core::{
    domain::{
        auth::service::AuthService,
        error::{AppError, Result},
        users::model::{CreateUserRequest, LoginRequest, UserRole},
    },
    rest::handler::response::build_success_response,
};

/// Register a new user
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Invalid request payload"),
        (status = 409, description = "Email already exists"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/auth/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateUserRequest>,
) -> Result<impl Responder> {
    let payload = payload.into_inner();

    // Create auth service with JWT secret from environment
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()); // TODO: Move to config
    let auth_service = AuthService::new(&jwt_secret);

    // Register user
    let response = auth_service.register_user(&pool, payload).await?;

    Ok(HttpResponse::Created().json(build_success_response(
        response,
        "User registered successfully",
    )))
}

/// User login
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 400, description = "Invalid request payload"),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/auth/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    payload: web::Json<LoginRequest>,
) -> Result<impl Responder> {
    let payload = payload.into_inner();

    // Create auth service with JWT secret from environment
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()); // TODO: Move to config
    let auth_service = AuthService::new(&jwt_secret);

    // Login user
    let response = auth_service.login_user(&pool, payload).await?;

    Ok(HttpResponse::Ok().json(build_success_response(response, "Login successful")))
}

/// Get current user profile (protected route)
#[utoipa::path(
    get,
    path = "/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = PublicUser),
        (status = 401, description = "Authentication required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/auth/me")]
pub async fn me(pool: web::Data<PgPool>, req: actix_web::HttpRequest) -> Result<impl Responder> {
    // Extract Bearer token from Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Authentication {
            message: "Authorization header required".to_string(),
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Authentication {
            message: "Invalid authorization format".to_string(),
        });
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix

    // Create auth service with JWT secret from environment
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()); // TODO: Move to config
    let auth_service = AuthService::new(&jwt_secret);

    // Verify token and get user ID
    let claims = auth_service.verify_token(token)?;
    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| AppError::Authentication {
        message: "Invalid token".to_string(),
    })?;

    // Get user from database
    let user = auth_service.get_user_by_id(&pool, user_id).await?;

    Ok(HttpResponse::Ok().json(build_success_response(
        user,
        "User profile retrieved successfully",
    )))
}

/// Password change request
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct ChangePasswordRequest {
    /// Current password
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,
    /// New password
    #[validate(length(
        min = 8,
        max = 128,
        message = "New password must be between 8 and 128 characters"
    ))]
    pub new_password: String,
}

/// Change user password (protected route)
#[utoipa::path(
    post,
    path = "/auth/change-password",
    tag = "auth",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Invalid request payload"),
        (status = 401, description = "Authentication required or current password incorrect"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/auth/change-password")]
pub async fn change_password(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    payload: web::Json<ChangePasswordRequest>,
) -> Result<impl Responder> {
    let payload = payload.into_inner();
    payload.validate()?;

    // Extract Bearer token from Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Authentication {
            message: "Authorization header required".to_string(),
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Authentication {
            message: "Invalid authorization format".to_string(),
        });
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix

    // Create auth service with JWT secret from environment
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()); // TODO: Move to config
    let auth_service = AuthService::new(&jwt_secret);

    // Verify token and get user ID
    let claims = auth_service.verify_token(token)?;
    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| AppError::Authentication {
        message: "Invalid token".to_string(),
    })?;

    // Get current user
    let user = sqlx::query_as::<_, crate::core::domain::users::model::User>(
        "SELECT * FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(&**pool)
    .await?
    .ok_or_else(|| AppError::NotFound {
        resource: "User".to_string(),
    })?;

    // Verify current password
    if !auth_service.verify_password(&payload.current_password, &user.password_hash)? {
        return Err(AppError::Authentication {
            message: "Current password is incorrect".to_string(),
        });
    }

    // Hash new password
    let new_password_hash = auth_service.hash_password(&payload.new_password)?;

    // Update password in database
    sqlx::query("UPDATE users SET password_hash = $1, updated_at = $2 WHERE id = $3")
        .bind(&new_password_hash)
        .bind(chrono::Utc::now())
        .bind(user_id)
        .execute(&**pool)
        .await?;

    #[derive(Serialize)]
    struct EmptyResponse {}

    Ok(HttpResponse::Ok().json(build_success_response(
        EmptyResponse {},
        "Password changed successfully",
    )))
}

/// Admin endpoint to create user with specific role
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateUserWithRoleRequest {
    /// Full name of the user
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    #[schema(example = "John Doe")]
    pub name: String,
    /// Email address of the user
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    /// Password for the user
    #[validate(length(
        min = 8,
        max = 128,
        message = "Password must be between 8 and 128 characters"
    ))]
    #[schema(example = "password123")]
    pub password: String,
    /// Role for the user (user, admin, moderator)
    #[schema(example = "user")]
    pub role: String,
}

#[utoipa::path(
    post,
    path = "/auth/admin/create-user",
    tag = "auth",
    request_body = CreateUserWithRoleRequest,
    responses(
        (status = 201, description = "User created successfully", body = PublicUser),
        (status = 400, description = "Invalid request payload"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Admin access required"),
        (status = 409, description = "Email already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/auth/admin/create-user")]
pub async fn admin_create_user(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
    payload: web::Json<CreateUserWithRoleRequest>,
) -> Result<impl Responder> {
    let payload = payload.into_inner();
    payload.validate()?;

    // Extract Bearer token from Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Authentication {
            message: "Authorization header required".to_string(),
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Authentication {
            message: "Invalid authorization format".to_string(),
        });
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix

    // Create auth service with JWT secret from environment
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()); // TODO: Move to config
    let auth_service = AuthService::new(&jwt_secret);

    // Verify token and check admin role
    let claims = auth_service.verify_token(token)?;
    if claims.role != UserRole::Admin.to_string() {
        return Err(AppError::Authorization {
            message: "Admin access required".to_string(),
        });
    }

    // Parse and validate role
    let role = payload
        .role
        .parse::<UserRole>()
        .map_err(|e| AppError::Validation { message: e })?;

    // Create user request
    let create_request = CreateUserRequest {
        name: payload.name,
        email: payload.email,
        password: payload.password,
        role: Some(role.to_string()),
    };

    // Register user with specified role
    let response = auth_service.register_user(&pool, create_request).await?;

    Ok(HttpResponse::Created().json(build_success_response(
        response.user,
        "User created successfully",
    )))
}
