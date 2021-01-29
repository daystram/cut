use crate::app::datatransfers::cut::Cut;
use crate::core::error::{HandlerError, HandlerErrorKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CutFile {
    pub name: String,
    pub mime: String,
    pub size: usize,
    pub downloads: u64,
    pub file: Vec<u8>,
}

impl CutFile {
    pub fn from_cut(cut: Cut) -> Self {
        CutFile {
            name: cut.name,
            mime: cut.data,
            size: cut.file.len(),
            downloads: 0,
            file: cut.file,
        }
    }

    pub fn from_hashmap(
        res: std::collections::HashMap<std::string::String, Vec<u8>>,
    ) -> Result<Self, HandlerError> {
        Ok(CutFile {
            name: String::from_utf8(
                res.get("name")
                    .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?,
            mime: String::from_utf8(
                res.get("mime")
                    .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?,
            size: String::from_utf8(
                res.get("size")
                    .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?
            .parse()?,
            downloads: String::from_utf8(
                res.get("downloads")
                    .ok_or(HandlerErrorKind::RedisError)?
                    .to_vec(),
            )?
            .parse()?,
            file: res
                .get("file")
                .ok_or(HandlerErrorKind::RedisError)?
                .to_vec(),
        })
    }

    pub fn to_array(&self) -> [(&str, Vec<u8>); 5] {
        return [
            ("name", self.name.as_bytes().to_vec()),
            ("mime", self.mime.as_bytes().to_vec()),
            ("size", self.size.to_string().as_bytes().to_vec()),
            ("downloads", self.downloads.to_string().as_bytes().to_vec()),
            ("file", self.file.clone()),
        ];
    }
}
