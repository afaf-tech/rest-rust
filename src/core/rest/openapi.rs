use crate::core::{
    domain::users::model::User,
    rest::handler::{
        response::{ErrorResponse, Meta, Response},
        users::CreateUserPayload,
    },
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::core::rest::handler::users::get_users,
        crate::core::rest::handler::users::create_user,
    ),
    components(
        schemas(
            User,
            CreateUserPayload,
            ErrorResponse,
            Response<User>,
            Response<Vec<User>>,
            Meta,
        )
    ),
    tags(
        (name = "users", description = "User management endpoints")
    ),
    info(
        title = "AFAF REST API",
        version = "1.0.0",
        description = "A Rust-based RESTful service",
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
