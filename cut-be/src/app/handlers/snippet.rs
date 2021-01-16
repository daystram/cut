use crate::core::error::{HandlerError, HandlerErrorKind};
use crate::app::Module;
use actix_web::web;
use r2d2_redis::redis::{Commands, ErrorKind};

pub fn get(m: web::Data<Module>) -> Result<Option<String>, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    match rd.get::<&str, String>("asd") {
        Ok(res) => Ok(Some(res)),
        Err(e) => match e.kind() {
            ErrorKind::ClientError => Err(HandlerError{kind: HandlerErrorKind::GeneralError, message: format!("{:?}", e)}),
            ErrorKind::AuthenticationFailed => Ok(Some("auth".into())),
            _ => Err(e.into()),
        },
    }
}
