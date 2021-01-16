use crate::app::{handlers, Module};
use crate::core::error::HandlerErrorKind;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

#[get("/{id}")]
pub async fn get_cut(m: web::Data<Module>, req: HttpRequest) -> impl Responder {
    let id: String = req.match_info().query("id").parse().unwrap();
    match handlers::snippet::get_one(m, id) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => match e.kind {
            HandlerErrorKind::CutNotFoundError => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        },
    }
}
