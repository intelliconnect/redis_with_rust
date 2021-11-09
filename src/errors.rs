use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub enum Myerror {
    RedisError(String),
}

impl ResponseError for Myerror {
    fn error_response(&self) -> HttpResponse {
        match self {
            Myerror::RedisError(s) => HttpResponse::InternalServerError().body(s.to_owned()),
        }
    }
}

impl std::convert::From<redis::RedisError> for Myerror {
    fn from(err: redis::RedisError) -> Self {
        Self::RedisError(err.to_string())
    }
}

impl std::fmt::Display for Myerror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
