use crate::app::datatransfers::cut_file::CutFile;
use crate::app::{
    constants::{HASH_LENGTH, VARIANT_FILE},
    datatransfers::cut::Cut,
    Module,
};
use crate::core::error::{HandlerError, HandlerErrorKind};
use crate::utils::hash;
use actix_web::web;
use r2d2_redis::redis::Commands;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Cut::file field not included
pub fn get_one(m: web::Data<Module>, hash: String) -> Result<Cut, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    let key = format!("cut::{}", hash);
    let mut cut: Cut = match rd.hgetall::<String, HashMap<String, Vec<u8>>>(key.clone()) {
        Ok(res) => {
            if res.is_empty() {
                return Err(HandlerErrorKind::CutNotFoundError.into());
            }
            Cut::from_hashmap(res)?
        }
        Err(e) => return Err(e.into()),
    };
    cut.views += 1;
    if cut.expiry < 0 && cut.variant != VARIANT_FILE {
        let _ = rd.del::<String, i32>(key.clone());
    } else {
        let _ = rd.hset::<String, &str, u64, i32>(key.clone(), "views", cut.views);
    }
    Ok(cut)
}

pub fn get_file(m: web::Data<Module>, hash: String) -> Result<CutFile, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    let key_file = format!("cut-file::{}", hash.clone());
    let cut: Cut = match get_one(m, hash.clone()) {
        Ok(cut) => cut,
        Err(e) => return Err(e.into()),
    };
    let mut file: CutFile = match rd.hgetall::<String, HashMap<String, Vec<u8>>>(key_file.clone()) {
        Ok(res) => {
            if res.is_empty() {
                return Err(HandlerErrorKind::CutNotFoundError.into());
            }
            CutFile::from_hashmap(res)?
        }
        Err(e) => return Err(e.into()),
    };
    file.downloads += 1;
    if cut.expiry < 0 {
        let _ = rd.del::<String, i32>(format!("cut::{}", hash));
        let _ = rd.del::<String, i32>(key_file.clone());
    } else {
        let _ = rd.hset::<String, &str, u64, i32>(key_file.clone(), "downloads", file.downloads);
    }
    Ok(file)
}

pub fn get_list(m: web::Data<Module>, owner: String) -> Result<Vec<Cut>, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    let key = format!("cut_list::{}", owner.clone());
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    rd.zrembyscore::<String, i64, i64, i32>(key.clone(), 0, timestamp)?;
    let cut_keys = rd.zrangebyscore::<String, i64, String, Vec<String>>(key.clone(), -1, "+inf".into())?;
    let mut cuts = Vec::new();
    for cut_key in &cut_keys {
        if let Ok(res) = rd.hgetall::<String, HashMap<String, String>>(cut_key.clone()) {
                if !res.is_empty() {
                    cuts.push(Cut::from_hashmap(res)?)
                }
            }
    };
    Ok(cuts)
}

pub fn insert(m: web::Data<Module>, mut cut: Cut) -> Result<String, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    cut.hash = hash::generate(HASH_LENGTH).into();
    let key = format!("cut::{}", cut.hash.clone());
    let key_file = format!("cut-file::{}", cut.hash.clone());
    match rd.hset_multiple::<String, &str, Vec<u8>, String>(key.clone(), &cut.to_array()) {
        Ok(_) => {
            if cut.expiry >= 0 {
                if let Err(e) = rd.expire::<String, i32>(key.clone(), cut.expiry as usize) {
                    let _ = rd.del::<String, i32>(key.clone());
                    return Err(e.into());
                }
            };
        }
        Err(e) => return Err(e.into()),
    };
    if cut.variant == VARIANT_FILE {
        let file: CutFile = CutFile::from_cut(cut.clone());
        if let Err(e) =
            rd.hset_multiple::<String, &str, Vec<u8>, String>(key_file.clone(), &file.to_array())
        {
            return Err(e.into());
        };
    }
    let timestamp = if cut.expiry >= 0 {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64)
            + cut.expiry
    } else {
        cut.expiry
    };
    match rd.zadd::<String, i64, String, i32>(
        format!("cut_list::{}", cut.owner.clone()),
        key.clone(),
        timestamp,
    ) {
        Ok(_) => Ok(cut.hash),
        Err(e) => {
            let _ = rd.del::<String, i32>(key.clone());
            if cut.variant == VARIANT_FILE {
                let _ = rd.del::<String, i32>(key_file.clone());
            };
            Err(e.into())
        }
    }
}
