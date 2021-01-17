#[derive(Debug, Copy, Clone)]
pub enum HandlerErrorKind {
    UnauthorizedError,
    CutNotFoundError,
    RedisError,
    GeneralError,
}

#[derive(Debug)]
pub struct HandlerError {
    pub kind: HandlerErrorKind,
    pub message: String,
}

impl From<HandlerErrorKind> for HandlerError {
    fn from(error: HandlerErrorKind) -> Self {
        match error {
            HandlerErrorKind::UnauthorizedError => HandlerError {
                kind: HandlerErrorKind::UnauthorizedError,
                message: "request unauthorized".into(),
            },
            HandlerErrorKind::CutNotFoundError => HandlerError {
                kind: HandlerErrorKind::CutNotFoundError,
                message: "cut not found".into(),
            },
            _ => HandlerError {
                kind: HandlerErrorKind::GeneralError,
                message: format!("an error has occurred. {:?}", error),
            },
        }
    }
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
