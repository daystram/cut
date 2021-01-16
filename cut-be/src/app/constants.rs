#[macro_export]
macro_rules! RD_SNIPPET {
    ($id: expr) => {
        format!("snippet::{}", $id)
    };
}

#[macro_export]
macro_rules! RD_URL {
    ($id: expr) => {
        format!("url::{}", $id)
    };
}
