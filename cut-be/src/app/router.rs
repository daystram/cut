use crate::app::controllers::v1;
use actix_web::web;

pub fn init(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/api/v1")
            .service(web::scope("/ping").service(v1::ping::get_ping))
            .service(web::scope("/cut").service(v1::cut::get_cut))
            .service(web::scope("/snippet").service(v1::snippet::post_snippet_create)),
    );
}
