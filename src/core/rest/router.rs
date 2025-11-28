use crate::core::rest::handler::users::{create_user, get_users};
use crate::core::rest::openapi::ApiDoc;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(create_user)
        .service(super::handler::home::home)
        // Serve Swagger UI
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        );
}
