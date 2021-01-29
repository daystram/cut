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

        Ok(Cut {
            name: res
                .get("name")
                .ok_or(HandlerErrorKind::RedisError)?
                .to_string(),
            hash: res
                .get("hash")
                .ok_or(HandlerErrorKind::RedisError)?
                .to_string(),
            owner: res
                .get("owner")
                .ok_or(HandlerErrorKind::RedisError)?
                .to_string(),
            variant: res
                .get("variant")
                .ok_or(HandlerErrorKind::RedisError)?
                .to_string(),
            metadata: res
                .get("metadata")
                .ok_or(HandlerErrorKind::RedisError)?
                .to_string(),
            data: res
                .get("data")
                .ok_or(HandlerErrorKind::RedisError)?
                .to_string(),
            expiry: res
                .get("expiry")
                .ok_or(HandlerErrorKind::RedisError)?
                .parse()?,
            created_at: res
                .get("created_at")
                .ok_or(HandlerErrorKind::RedisError)?
                .parse()?,
            views: res
                .get("views")
                .ok_or(HandlerErrorKind::RedisError)?
                .parse()?,
        })
    }

    pub fn to_array(&self) -> [(&str, String); 9] {
        return [
            ("name", self.name.clone()),
            ("hash", self.hash.clone()),
            ("owner", self.owner.clone()),
            ("variant", self.variant.clone()),
            ("metadata", self.metadata.clone()),
            ("data", self.data.clone()),
            ("expiry", self.expiry.to_string()),
            ("created_at", self.created_at.to_string()),
            ("views", self.views.to_string()),
        ];
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub hash: String,
}
