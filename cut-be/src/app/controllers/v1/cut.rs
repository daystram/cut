use crate::app::{
    constants,
    datatransfers::{
        auth::TokenInfo,
        cut::{CreateResponse, Cut},
    },
    handlers, Module,
};
use crate::core::error::HandlerErrorKind;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

#[get("/{id}")]
pub async fn get_cut_raw(m: web::Data<Module>, req: HttpRequest) -> impl Responder {
    let id: String = req.match_info().query("id").parse().unwrap();
    match handlers::cut::get_one(m, id) {
        Ok(cut) => match cut.variant.as_str() {
            constants::VARIANT_SNIPPET => HttpResponse::Ok().body(cut.data),
            constants::VARIANT_URL => HttpResponse::TemporaryRedirect()
                .header("Location", cut.data)
                .finish(),
            _ => HttpResponse::NotFound().finish(),
        },
        Err(e) => match e.kind {
            HandlerErrorKind::CutNotFoundError => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        },
    }
}

#[get("/{id}")]
pub async fn get_cut(m: web::Data<Module>, req: HttpRequest) -> impl Responder {
    let id: String = req.match_info().query("id").parse().unwrap();
    match handlers::cut::get_one(m, id) {
        Ok(cut) => HttpResponse::Ok().json(cut),
        Err(e) => match e.kind {
            HandlerErrorKind::CutNotFoundError => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        },
    }
}

#[post("")]
pub async fn post_snippet_create(
    m: web::Data<Module>,
    cut: web::Json<Cut>,
    req: HttpRequest,
) -> impl Responder {
    let user: TokenInfo = match handlers::auth::authorize(&m, &req).await {
        Ok(resp) => resp,
        Err(e) => match e.kind {
            HandlerErrorKind::UnauthorizedError => return HttpResponse::Unauthorized().finish(),
            _ => return HttpResponse::InternalServerError().finish(),
        },
    };
    match cut.variant.as_str() {
        constants::VARIANT_SNIPPET => (),
        constants::VARIANT_URL => (),
        _ => return HttpResponse::BadRequest().finish(),
    };
    match handlers::cut::insert(m, user.sub, cut.0) {
        Ok(hash) => HttpResponse::Ok().json(CreateResponse { hash: hash }),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}
