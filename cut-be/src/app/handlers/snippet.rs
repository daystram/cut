use crate::app::{
    constants::{HASH_LENGTH, VARIANT_SNIPPET},
    datatransfers::cut::Cut,
    Module,
};
use crate::core::error::{HandlerError, HandlerErrorKind};
use crate::utils::hash;
use actix_web::web;
use r2d2_redis::redis::Commands;
use std::time::{SystemTime, UNIX_EPOCH};

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
        hash.clone(),
        &[
            ("name", cut.name),
            ("owner", user_subject),
            ("variant", VARIANT_SNIPPET.into()),
            ("metadata", cut.metadata),
            ("data", cut.data),
            ("created_at", created_at.to_string()),
        ],
    ) {
        Ok(_) => Ok(hash),
        Err(e) => Err(e.into()),
    }
}
