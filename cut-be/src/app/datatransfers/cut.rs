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
    pub fn to_array(&self) -> [(&str, String); 7] {
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
