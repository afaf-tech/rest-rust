use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// User role enumeration
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    /// Regular user with basic permissions
    #[default]
    User,
    /// Administrator with full permissions
    Admin,
    /// Moderator with elevated permissions
    Moderator,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::Moderator => write!(f, "moderator"),
        }
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "user" => Ok(UserRole::User),
            "admin" => Ok(UserRole::Admin),
            "moderator" => Ok(UserRole::Moderator),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

/// Represents a user in the system
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    /// Unique identifier for the user
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    /// Full name of the user
    #[schema(example = "John Doe")]
    pub name: String,
    /// Email address of the user
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    /// Password hash (not exposed in API responses)
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// User role for authorization
    #[schema(example = "user")]
    pub role: String, // We store as string in DB, convert to enum as needed
    /// Account creation timestamp
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Get the user's role as an enum
    pub fn get_role(&self) -> Result<UserRole, String> {
        self.role.parse()
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: &UserRole) -> bool {
        match self.get_role() {
            Ok(user_role) => user_role == *role,
            Err(_) => false,
        }
    }

    /// Check if user is an admin
    pub fn is_admin(&self) -> bool {
        self.has_role(&UserRole::Admin)
    }

    /// Check if user is a moderator or admin
    pub fn is_moderator_or_admin(&self) -> bool {
        self.has_role(&UserRole::Moderator) || self.has_role(&UserRole::Admin)
    }
}

/// User data for public API responses (excludes sensitive information)
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PublicUser {
    /// Unique identifier for the user
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    /// Full name of the user
    #[schema(example = "John Doe")]
    pub name: String,
    /// Email address of the user
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    /// User role for authorization
    #[schema(example = "user")]
    pub role: UserRole,
    /// Account creation timestamp
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    #[schema(example = "2023-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        let role = user.get_role().unwrap_or_default();
        PublicUser {
            id: user.id,
            name: user.name,
            email: user.email,
            role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// Request payload for creating a new user
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUserRequest {
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

    /// Password for the user account
    #[validate(length(
        min = 8,
        max = 128,
        message = "Password must be between 8 and 128 characters"
    ))]
    #[schema(example = "securepassword123")]
    pub password: String,

    /// User role (optional, defaults to 'user')
    #[schema(example = "user")]
    pub role: Option<String>,
}

/// Request payload for user login
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    /// Email address of the user
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "john.doe@example.com")]
    pub email: String,

    /// Password for the user account
    #[validate(length(min = 1, message = "Password is required"))]
    #[schema(example = "securepassword123")]
    pub password: String,
}

/// Response payload for successful authentication
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    /// JWT token for authentication
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...")]
    pub token: String,
    /// User information
    pub user: PublicUser,
}
