#[derive(Debug, Copy, Clone)]
pub enum HandlerErrorKind {
    UnauthorizedError,
    CutNotFoundError,
    FormParseError,
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
            HandlerErrorKind::FormParseError => HandlerError {
                kind: HandlerErrorKind::FormParseError,
                message: "failed parsing cut from form-data".into(),
            },
            _ => HandlerError {
                kind: HandlerErrorKind::GeneralError,
                message: format!("an error has occurred. {:?}", error),
            },
        }
    }
}

impl From<std::num::ParseIntError> for HandlerError {
    fn from(error: std::num::ParseIntError) -> Self {
        HandlerError {
            kind: HandlerErrorKind::GeneralError,
            message: error.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for HandlerError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        HandlerError {
            kind: HandlerErrorKind::RedisError,
            message: error.to_string(),
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
