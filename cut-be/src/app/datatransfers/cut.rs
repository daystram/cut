use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cut {
    pub name: String,
    pub variant: String,
    pub metadata: String,
    pub data: String,
    pub created_at: i64,
}
