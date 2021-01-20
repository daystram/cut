use actix_web::{get, HttpResponse, Responder};

#[get("")]
pub async fn get_ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}
