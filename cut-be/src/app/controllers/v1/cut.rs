use crate::app::{
    constants,
    datatransfers::{auth::TokenInfo, cut::Cut, response::CreateResponse},
    handlers, Module,
};
use crate::core::error::HandlerErrorKind;
use actix_form_data::Value;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

#[get("/{id}")]
pub async fn get_cut_raw(m: web::Data<Module>, req: HttpRequest) -> impl Responder {
    let id: String = req.match_info().query("id").parse().unwrap();
    match handlers::cut::get_one(m.clone(), id.clone()) {
        Ok(cut) => match cut.variant.as_str() {
            constants::VARIANT_SNIPPET => HttpResponse::Ok()
                .header("Content-Type", "text/plain")
                .body(cut.data),
            constants::VARIANT_URL => HttpResponse::TemporaryRedirect()
                .header("Location", cut.data)
                .finish(),
            constants::VARIANT_FILE => {
                let file = match handlers::cut::get_file(m, id) {
                    Ok(file) => file,
                    Err(e) => return HttpResponse::InternalServerError().body(format!("{:?}", e)),
                };
                HttpResponse::Ok()
                    .header(
                        "Content-Disposition",
                        format!("attachment; filename=\"{}\"", file.name),
                    )
                    .header("Content-Type", file.mime)
                    .header("Content-Length", file.size)
                    .body(file.file)
            }
            _ => HttpResponse::NotFound().finish(),
        },
        Err(e) => match e.kind {
            HandlerErrorKind::CutNotFoundError => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().body(format!("{:?}", e)),
        },
    }
}

#[get("/list")]
pub async fn get_cut_list(m: web::Data<Module>, req: HttpRequest) -> impl Responder {
    let user: TokenInfo = match handlers::auth::authorize(&m, &req).await {
        Ok(res) => res,
        Err(e) => match e.kind {
            HandlerErrorKind::UnauthorizedError => return HttpResponse::Unauthorized().finish(),
            _ => return HttpResponse::InternalServerError().finish(),
        },
    };
    match handlers::cut::get_list(m, user.sub) {
        Ok(list) => HttpResponse::Ok().json(list),
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
    mut cut: web::Json<Cut>,
    req: HttpRequest,
) -> impl Responder {
    let user: TokenInfo = match handlers::auth::authorize(&m, &req).await {
        Ok(res) => res,
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
    if cut.data.trim().chars().count() == 0 {
        return HttpResponse::BadRequest().finish();
    };
    cut.0.owner = user.sub;
    match handlers::cut::insert(m, cut.0) {
        Ok(hash) => HttpResponse::Ok().json(CreateResponse { hash: hash }),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}

#[post("")]
pub async fn post_file_upload(
    m: web::Data<Module>,
    form: Value,
    req: HttpRequest,
) -> impl Responder {
    let user: TokenInfo = match handlers::auth::authorize(&m, &req).await {
        Ok(res) => res,
        Err(e) => match e.kind {
            HandlerErrorKind::UnauthorizedError => return HttpResponse::Unauthorized().finish(),
            _ => return HttpResponse::InternalServerError().finish(),
        },
    };
    let mut cut: Cut = match Cut::from_form(form) {
        Ok(cut) => cut,
        Err(e) => return HttpResponse::InternalServerError().body(format!("{:?}", e)),
    };
    cut.owner = user.sub;
    match handlers::cut::insert(m, cut) {
        Ok(hash) => HttpResponse::Ok().json(CreateResponse { hash: hash }),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}
