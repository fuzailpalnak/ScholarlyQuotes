use actix_web::{HttpResponse, Responder};

#[actix_web::get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Server is running")
}
