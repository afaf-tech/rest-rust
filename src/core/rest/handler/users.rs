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

#[get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let repo = UserRepository { pool: &pool };
    match repo.find_all().await {
        Ok(users) => {
            // Return a structured success response
            HttpResponse::Ok().json(build_success_response(users, "Users retrieved successfully."))
        }
        Err(sqlx::Error::RowNotFound) => {
            // Handle case where no users are found
            HttpResponse::NotFound()
                .json(build_error_response("not_found", "No users found."))
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


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserPayload {
    name: String,
    email: String,
}

#[post("/users")]
async fn create_user(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateUserPayload>,
) -> impl Responder {
    let CreateUserPayload { name, email } = payload.into_inner();

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
            if db_err
                .constraint()
                .map_or(false, |c| c == "users_email_key")
            {
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
