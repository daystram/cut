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
    #[serde(skip_deserializing)]
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub hash: String,
}
