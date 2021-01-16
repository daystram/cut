use actix_web::{HttpResponse, Responder};

pub async fn get_ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}
