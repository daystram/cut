use crate::app::Module;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use r2d2_redis::redis::Commands;

pub async fn get_snippet_list(m: web::Data<Module>, _req: HttpRequest) -> impl Responder {
    match &mut m.rd_pool.get() {
        Ok(rd) => match rd.get::<&str, i64>("counter") {
            Ok(result) => HttpResponse::Ok().body(format!("Counter: {}", result)),
            Err(_) => HttpResponse::NotFound().finish(),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

pub async fn post_snippet_create(_: web::Data<Module>, _req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}
