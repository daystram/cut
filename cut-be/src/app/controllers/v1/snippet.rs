use crate::app::{datatransfers::cut::Cut, handlers, Module};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

#[post("")]
pub async fn post_snippet_create(
    m: web::Data<Module>,
    cut: web::Json<Cut>,
    _req: HttpRequest,
) -> impl Responder {
    match handlers::snippet::insert(m, cut.0) {
        Ok(res) => HttpResponse::Ok().body(format!("{}", res)),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}
