use crate::app::{constants::VARIANT_SNIPPET, datatransfers::cut::Cut, Module};
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
                name: res.get("name").unwrap().to_string(),
                variant: res.get("variant").unwrap().to_string(),
                metadata: res.get("metadata").unwrap().to_string(),
                data: res.get("data").unwrap().to_string(),
                created_at: res.get("created_at").unwrap().parse().unwrap(),
            })
        }
        Err(e) => Err(e.into()),
    }
}

pub fn insert(m: web::Data<Module>, cut: Cut) -> Result<String, HandlerError> {
    let rd = &mut m.rd_pool.get()?;
    let hash: String = "ID".into(); // TODO: generate random hash
    match rd.hset_multiple::<String, &str, String, String>(
        hash.clone(),
        &[
            ("name", cut.name),
            ("variant", VARIANT_SNIPPET.into()),
            ("metadata", cut.metadata),
            ("data", cut.data),
            ("created_at", cut.created_at.to_string()), // TODO: user current time
        ],
    ) {
        Ok(_) => Ok(hash),
        Err(e) => Err(e.into()),
    }
}
