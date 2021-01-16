use actix_web::{web, HttpResponse, HttpRequest, Responder};
use crate::app::Module;

pub async fn get_snippet_list(m: web::Data<Module>, _req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body(&m.config.client_secret)
}

pub async fn post_snippet_create(_: web::Data<Module>, _req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}
