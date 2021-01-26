use crate::app::{constants::HASH_LENGTH, datatransfers::cut::Cut, Module};
use crate::core::error::{HandlerError, HandlerErrorKind};
use crate::utils::hash;
use actix_web::web;
use r2d2_redis::redis::Commands;
use std::collections::HashMap;

pub fn get_one(m: web::Data<Module>, hash: String) -> Result<Cut, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    let key = format!("cut::{}", hash);
    let mut cut: Cut = match rd.hgetall::<String, HashMap<String, String>>(key.clone()) {
        Ok(res) => {
            if res.is_empty() {
                return Err(HandlerErrorKind::CutNotFoundError.into());
            }
            Cut::from_hashmap(res)?
        }
        Err(e) => return Err(e.into()),
    };
    cut.views += 1;
    if cut.expiry < 0 {
        let _ = rd.del::<String, i32>(key.clone());
    } else {
        let _ = rd.hset::<String, &str, u64, i32>(key.clone(), "views", cut.views);
    }
    Ok(cut)
}

pub fn insert(m: web::Data<Module>, cut: Cut) -> Result<String, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
pub fn insert(m: web::Data<Module>, mut cut: Cut) -> Result<String, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    cut.hash = hash::generate(HASH_LENGTH).into();
    let key = format!("cut::{}", cut.hash.clone());
    match rd.hset_multiple::<String, &str, String, String>(key.clone(), &cut.to_array()) {
        Ok(_) => {
            if cut.expiry >= 0 {
                if let Err(e) = rd.expire::<String, i32>(key.clone(), cut.expiry as usize) {
                    let _ = rd.del::<String, i32>(key.clone());
                    return Err(e.into())
                }
            };
        }
        Err(e) => return Err(e.into()),
    };
    let timestamp = if cut.expiry >= 0 {
        (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64) + cut.expiry
    } else {
        cut.expiry
    };
    match rd.zadd::<String, i64, String, i32>(format!("cut_list::{}", cut.owner.clone()), key.clone(), timestamp) {
        Ok(_) => {
            Ok(cut.hash)
        },
        Err(e) => {
            let _ = rd.del::<String, i32>(key.clone());
            Err(e.into())
        }
    }
}
