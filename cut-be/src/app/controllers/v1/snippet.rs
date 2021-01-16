use crate::app::handlers;
use crate::app::Module;
use crate::core::error::HandlerErrorKind;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn get_snippet_list(m: web::Data<Module>, _req: HttpRequest) -> impl Responder {
    match handlers::snippet::get(m) {
        Ok(Some(res)) => HttpResponse::Ok().body(format!("{}", res)),
        Ok(None) => HttpResponse::Ok().body("Not found"),
        Err(e) => match e.kind {
            HandlerErrorKind::GeneralError => {
                HttpResponse::InternalServerError().body(format!("{:?}", e))
            }
            _ => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        },
    }
}

pub async fn post_snippet_create(_: web::Data<Module>, _req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}
