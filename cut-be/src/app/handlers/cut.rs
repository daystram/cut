use crate::app::{constants::HASH_LENGTH, datatransfers::cut::Cut, Module};
use crate::core::error::{HandlerError, HandlerErrorKind};
use crate::utils::hash;
use actix_web::web;
use r2d2_redis::redis::Commands;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn get_one(m: web::Data<Module>, id: String) -> Result<Cut, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    match rd.hgetall::<String, HashMap<String, String>>(format!("cut::{}", id)) {
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

pub fn insert(
    m: web::Data<Module>,
    user_subject: String,
    cut: Cut,
) -> Result<String, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    let hash: String = hash::generate(HASH_LENGTH).into();
    let created_at = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => return Err(HandlerErrorKind::GeneralError.into()),
    };
    match rd.hset_multiple::<String, &str, String, String>(
        format!("cut::{}", hash.clone()),
        &[
            ("name", cut.name),
            ("owner", user_subject),
            ("variant", cut.variant),
            ("metadata", cut.metadata),
            ("data", cut.data),
            ("created_at", created_at.to_string()),
        ],
    ) {
        Ok(_) => Ok(hash),
        Err(e) => Err(e.into()),
    }
}
