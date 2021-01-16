use actix_web::{web, HttpResponse, HttpRequest, Responder};
use crate::app::Module;

pub async fn get_ping(_: web::Data<Module>, _: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}
