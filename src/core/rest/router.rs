use crate::core::rest::handler::{
    auth::{admin_create_user, change_password, login, me, register},
    users::{create_user, get_users},
};
use crate::core::rest::openapi::ApiDoc;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // Public routes
        .service(super::handler::home::home)
        // User management routes
        .service(get_users)
        .service(create_user)
        // Authentication routes
        .service(register)
        .service(login)
        .service(me)
        .service(change_password)
        .service(admin_create_user)
        // Swagger UI
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        );
}
