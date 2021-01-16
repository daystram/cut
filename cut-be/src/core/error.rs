#[derive(Debug, Copy, Clone)]
pub enum HandlerErrorKind {
    SnippetNotFoundError,
    RedisError,
    GeneralError,
}

#[derive(Debug)]
pub struct HandlerError {
    pub kind: HandlerErrorKind,
    pub message: String,
}

impl From<r2d2::Error> for HandlerError {
    fn from(error: r2d2::Error) -> Self {
        HandlerError {
            kind: HandlerErrorKind::RedisError,
            message: error.to_string(),
        }
    }
}

impl From<r2d2_redis::redis::RedisError> for HandlerError {
    fn from(error: r2d2_redis::redis::RedisError) -> Self {
        HandlerError {
            kind: HandlerErrorKind::RedisError,
            message: error.to_string(),
        }
    }
}
