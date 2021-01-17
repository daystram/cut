use crate::app::{
    datatransfers::{
        auth::TokenInfo,
        cut::{CreateResponse, Cut},
    },
    handlers, Module,
};
use crate::core::error::HandlerErrorKind;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

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

    match handlers::snippet::insert(m, user.sub, cut.0) {
        Ok(hash) => HttpResponse::Ok().json(CreateResponse { hash: hash }),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}
