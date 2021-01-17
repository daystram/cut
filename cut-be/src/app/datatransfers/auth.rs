use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub active: bool,
    #[serde(default)] pub sub: String,
    #[serde(default)] pub client_id: String,
}
