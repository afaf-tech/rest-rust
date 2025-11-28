use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

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
}
