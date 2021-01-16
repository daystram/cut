use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CreateRequest {
    name: String,
}
