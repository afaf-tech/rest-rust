use crate::core::{
    domain::users::repository::UserRepository,
    rest::handler::{
        response::{build_error_response, build_success_response},
        validator::is_valid_email,
    },
};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

/// Get all users in the system
#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    responses(
        (status = 200, description = "List of all users retrieved successfully", body = Vec<User>),
        (status = 404, description = "No users found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/users")]
pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let repo = UserRepository { pool: &pool };
    match repo.find_all().await {
        Ok(users) => {
            // Return a structured success response
            HttpResponse::Ok().json(build_success_response(
                users,
                "Users retrieved successfully.",
            ))
        }
        Err(sqlx::Error::RowNotFound) => {
            // Handle case where no users are found
            HttpResponse::NotFound().json(build_error_response("not_found", "No users found."))
        }
        Err(e) => {
            // Log the error for debugging
            log::error!("Failed to fetch users: {:?}", e);

            // Return a structured error response
            HttpResponse::InternalServerError().json(build_error_response(
                "internal_server_error",
                "Failed to retrieve users.",
            ))
        }
    }
}

/// Request payload for creating a new user
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserPayload {
    /// Full name of the user
    #[schema(example = "John Doe")]
    pub name: Option<String>,
    /// Email address of the user
    #[schema(example = "john.doe@example.com")]
    pub email: Option<String>,
}

/// Create a new user in the system
#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUserPayload,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Invalid request payload", body = ErrorResponse),
        (status = 409, description = "Email already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/users")]
pub async fn create_user(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateUserPayload>,
) -> impl Responder {
    let payload = payload.into_inner();

    // Deserialization errors
    let name = match payload.name {
        Some(name) => name,
        None => {
            return HttpResponse::BadRequest()
                .json(build_error_response("bad_request", "Name is required."))
        }
    };
    let email = match payload.email {
        Some(email) => email,
        None => {
            return HttpResponse::BadRequest()
                .json(build_error_response("bad_request", "Email is required."))
        }
    };

    // Validate input
    if name.trim().is_empty() {
        return HttpResponse::BadRequest()
            .json(build_error_response("bad_request", "Name cannot be empty."));
    }
    if !is_valid_email(&email) {
        return HttpResponse::BadRequest()
            .json(build_error_response("bad_request", "Invalid email format."));
    }

    let repo = UserRepository { pool: &pool };
    match repo.create_user(&name, &email).await {
        Ok(user) => {
            HttpResponse::Created().json(build_success_response(user, "User created successfully."))
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.constraint() == Some("users_email_key") {
                HttpResponse::Conflict().json(build_error_response("conflict", "Email is in use."))
            } else {
                HttpResponse::InternalServerError().json(build_error_response(
                    "internal_server_error",
                    "Database error.",
                ))
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(build_error_response(
            "internal_server_error",
            "Unexpected error.",
        )),
    }
}
