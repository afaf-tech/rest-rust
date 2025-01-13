use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(super::handler::users::get_users);
    cfg.service(super::handler::users::create_user);
    cfg.service(super::handler::users::create_user);
    cfg.service(super::handler::home::home);
}
