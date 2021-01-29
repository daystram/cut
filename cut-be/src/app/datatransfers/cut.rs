use crate::{
    app::constants,
    core::error::{HandlerError, HandlerErrorKind},
};
use actix_form_data::Value;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cut {
    pub name: String,
    #[serde(skip_deserializing)]
    pub hash: String,
    #[serde(skip)]
    pub owner: String,
    pub variant: String,
    pub metadata: String,
    pub data: String,
    pub expiry: i64,
    #[serde(default = "current_time")]
    pub created_at: u64,
    #[serde(default = "Default::default")]
    pub views: u64,
    #[serde(skip)]
    pub file: Vec<u8>,
}

fn current_time() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    }
}

impl Cut {
    pub fn from_form(form: Value) -> Result<Self, HandlerError> {
        let mut cut = Cut {
            name: Default::default(),
            hash: Default::default(),
            owner: Default::default(),
            variant: constants::VARIANT_FILE.into(),
            metadata: Default::default(),
            data: Default::default(),
            expiry: Default::default(),
            created_at: current_time(),
            views: Default::default(),
            file: Default::default(),
        };
        match form {
            Value::Map(mut hashmap) => {
                match hashmap.remove("name") {
                    Some(value) => match value {
                        Value::Text(name) => cut.name = name,
                        _ => return Err(HandlerErrorKind::FormParseError.into()),
                    },
                    None => return Err(HandlerErrorKind::FormParseError.into()),
                };
                match hashmap.remove("expiry") {
                    Some(value) => match value {
                        Value::Int(expiry) => cut.expiry = expiry,
                        _ => return Err(HandlerErrorKind::FormParseError.into()),
                    },
                    None => return Err(HandlerErrorKind::FormParseError.into()),
                };
                match hashmap.remove("metadata") {
                    Some(value) => match value {
                        Value::Text(metadata) => cut.metadata = metadata,
                        _ => return Err(HandlerErrorKind::FormParseError.into()),
                    },
                    None => return Err(HandlerErrorKind::FormParseError.into()),
                };
                match hashmap.remove("data") {
                    Some(value) => match value {
                        Value::Text(data) => cut.data = data,
                        _ => return Err(HandlerErrorKind::FormParseError.into()),
                    },
                    None => return Err(HandlerErrorKind::FormParseError.into()),
                };
                match hashmap.remove("file") {
                    Some(value) => match value {
                        Value::Bytes(file) => cut.file = file.to_vec(),
                        _ => return Err(HandlerErrorKind::FormParseError.into()),
                    },
                    None => return Err(HandlerErrorKind::FormParseError.into()),
                };
            }
            _ => return Err(HandlerErrorKind::FormParseError.into()),
        };
        Ok(cut)
    }

    pub fn from_hashmap(
        res: std::collections::HashMap<std::string::String, Vec<u8>>,
    ) -> Result<Self, HandlerError> {
        Ok(Cut {
            name: String::from_utf8(
                res.get("name")
                .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?,
            hash: String::from_utf8(
                res.get("hash")
                .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?,
            owner: String::from_utf8(
                res.get("owner")
                .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?,
            variant: String::from_utf8(
                res.get("variant")
                .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?,
            metadata: String::from_utf8(
                res.get("metadata")
                .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?,
            data: String::from_utf8(
                res.get("data")
                .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?,
            expiry: String::from_utf8(
                res.get("expiry")
                .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?
                .parse()?,
            created_at: String::from_utf8(
                res.get("created_at")
                .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?
                .parse()?,
            views: String::from_utf8(
                res.get("views")
                .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?
                .parse()?,
            file: Default::default(),
        })
    }

    pub fn to_array(&self) -> [(&str, Vec<u8>); 9] {
        return [
            ("name", self.name.as_bytes().to_vec()),
            ("hash", self.hash.as_bytes().to_vec()),
            ("owner", self.owner.as_bytes().to_vec()),
            ("variant", self.variant.as_bytes().to_vec()),
            ("metadata", self.metadata.as_bytes().to_vec()),
            ("data", self.data.as_bytes().to_vec()),
            ("expiry", self.expiry.to_string().as_bytes().to_vec()),
            (
                "created_at",
                self.created_at.to_string().as_bytes().to_vec(),
            ),
            ("views", self.views.to_string().as_bytes().to_vec()),
        ];
    }
}
