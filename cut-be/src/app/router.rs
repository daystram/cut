use crate::app::controllers::v1;
use actix_web::web;

pub fn init(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/api/v1")
            .service(web::resource("ping").route(web::get().to(v1::ping::get_ping)))
            .service(
                web::resource("snippet")
                    .route(web::get().to(v1::snippet::get_snippet_list))
                    .route(web::post().to(v1::snippet::post_snippet_create)),
            ),
    );
}
