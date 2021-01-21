use crate::app::{constants::HASH_LENGTH, datatransfers::cut::Cut, Module};
use crate::core::error::{HandlerError, HandlerErrorKind};
use crate::utils::hash;
use actix_web::web;
use r2d2_redis::redis::Commands;
use std::collections::HashMap;

pub fn get_one(m: web::Data<Module>, hash: String) -> Result<Cut, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    let cut: Cut = match rd.hgetall::<String, HashMap<String, String>>(format!("cut::{}", hash)) {
        Ok(res) => {
            if res.is_empty() {
                return Err(HandlerErrorKind::CutNotFoundError.into());
            }
            Cut::from_hashmap(res)?
        }
        Err(e) => return Err(e.into()),
    };
    match rd.hset::<String, &str, u64, i32>(format!("cut::{}", hash), "views", cut.views + 1) {
        Ok(_) => Ok(cut),
        Err(e) => Err(e.into()),
    }
}

pub fn insert(m: web::Data<Module>, cut: Cut) -> Result<String, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    let hash: String = hash::generate(HASH_LENGTH).into();
    match rd.hset_multiple::<String, &str, String, String>(
        format!("cut::{}", hash.clone()),
        &cut.to_array(),
    ) {
        Ok(_) => Ok(hash),
        Err(e) => Err(e.into()),
    }
}
