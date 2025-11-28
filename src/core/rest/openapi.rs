use crate::core::{
    domain::{
        error::ErrorResponse,
        users::model::{AuthResponse, CreateUserRequest, LoginRequest, PublicUser, User, UserRole},
    },
    rest::handler::{
        auth::{ChangePasswordRequest, CreateUserWithRoleRequest},
        response::{Meta, Response},
        users::CreateUserPayload,
    },
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // User management endpoints
        crate::core::rest::handler::users::get_users,
        crate::core::rest::handler::users::create_user,

        // Authentication endpoints
        crate::core::rest::handler::auth::register,
        crate::core::rest::handler::auth::login,
        crate::core::rest::handler::auth::me,
        crate::core::rest::handler::auth::change_password,
        crate::core::rest::handler::auth::admin_create_user,
    ),
    components(
        schemas(
            // User models
            User,
            PublicUser,
            UserRole,

            // Request/Response types
            CreateUserRequest,
            CreateUserPayload,
            LoginRequest,
            AuthResponse,
            ChangePasswordRequest,
            CreateUserWithRoleRequest,

            // Error handling
            ErrorResponse,

            // Response wrappers
            Response<User>,
            Response<Vec<User>>,
            Response<PublicUser>,
            Response<AuthResponse>,
            Meta,
        )
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "auth", description = "Authentication and authorization endpoints")
    ),
    modifiers(&SecurityAddon),
    info(
        title = "AFAF REST API",
        version = "1.0.0",
        description = "A Rust-based RESTful service with authentication and authorization",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
pub struct ApiDoc;

use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::Modify;

/// Security configuration for JWT Bearer tokens
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}
