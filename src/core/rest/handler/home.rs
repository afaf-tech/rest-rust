use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().json(String::from(r#"{}"#))
}
