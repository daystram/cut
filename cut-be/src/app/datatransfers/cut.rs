use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cut {
    pub name: String,
    #[serde(skip)]
    pub owner: String,
    pub variant: String,
    pub metadata: String,
    pub data: String,
    #[serde(default = "current_time")]
    pub created_at: u64,
    #[serde(default = "Default::default")]
    pub views: u64,
}

fn current_time() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    }
}

impl Cut {
    pub fn from_hashmap(res: HashMap<String, String>) -> Result<Self, HandlerError> {
        Ok(Cut {
            name: res
                .get("name")
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

    pub fn to_array(&self) -> [(&str, String); 8] {
        return [
            ("name", self.name.clone()),
            ("owner", self.owner.clone()),
            ("variant", self.variant.clone()),
            ("metadata", self.metadata.clone()),
            ("data", self.data.clone()),
            ("created_at", self.created_at.to_string()),
            ("views", self.views.to_string()),
        ];
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub hash: String,
}
