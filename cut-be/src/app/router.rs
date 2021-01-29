use crate::app::{constants, controllers::v1};
use actix_form_data::{Field, Form};
use actix_web::web;

pub fn init(app: &mut web::ServiceConfig) {
    let form = Form::new()
        .field("name", Field::text())
        .field("expiry", Field::int())
        .field("metadata", Field::text())
        .field("data", Field::text())
        .field("file", Field::bytes())
        .max_field_size(constants::MAX_SIZE_FILE); // max_field_size (instead of file) because Field::bytes() is used
    app.service(web::scope("/raw").service(v1::cut::get_cut_raw))
        .service(
            web::scope("/api/v1")
                .service(web::scope("/ping").service(v1::ping::get_ping))
                .service(
                    web::scope("/cut/file")
                        .wrap(form.clone())
                        .service(v1::cut::post_file_upload),
                )
                .service(
                    web::scope("/cut")
                        .service(v1::cut::get_cut_list)
                        .service(v1::cut::get_cut)
                        .service(v1::cut::post_snippet_create)
                        .service(v1::cut::delete_cut),
                ),
        );
}
