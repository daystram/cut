use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cut {
    pub name: String,
    #[serde(skip)]
    pub owner: String,
    #[serde(skip_deserializing)]
    pub variant: String,
    pub metadata: String,
    pub data: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub hash: String,
}
