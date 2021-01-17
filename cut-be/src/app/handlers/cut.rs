use crate::app::{datatransfers::cut::Cut, Module};
use crate::core::error::{HandlerError, HandlerErrorKind};
use actix_web::web;
use r2d2_redis::redis::Commands;
use std::collections::HashMap;

pub fn get_one(m: web::Data<Module>, id: String) -> Result<Cut, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    match rd.hgetall::<String, HashMap<String, String>>(id) {
        Ok(res) => {
            if res.is_empty() {
                return Err(HandlerErrorKind::CutNotFoundError.into());
            }
            Ok(Cut {
                name: res
                    .get("name")
                    .ok_or(HandlerErrorKind::CutNotFoundError)?
                    .to_string(),
                owner: res
                    .get("owner")
                    .ok_or(HandlerErrorKind::CutNotFoundError)?
                    .to_string(),
                variant: res
                    .get("variant")
                    .ok_or(HandlerErrorKind::CutNotFoundError)?
                    .to_string(),
                metadata: res
                    .get("metadata")
                    .ok_or(HandlerErrorKind::CutNotFoundError)?
                    .to_string(),
                data: res
                    .get("data")
                    .ok_or(HandlerErrorKind::CutNotFoundError)?
                    .to_string(),
                created_at: res
                    .get("created_at")
                    .ok_or(HandlerErrorKind::CutNotFoundError)?
                    .parse()?,
            })
        }
        Err(e) => Err(e.into()),
    }
}
