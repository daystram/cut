use crate::app::Module;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn get_ping(_: web::Data<Module>, _: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}
