use jsonwebtoken::errors::Error;
use thiserror::Error;
use std::convert::Infallible;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Token signature invalid")]
    InvalidSignature,
    #[error("token signature invalid")]
    InvalidToken,
    #[error("invalid username or password")]
    InvalidCredentials,
    #[error("expired signature")]
    ExpiredSignature,
    #[error("an error occurred while creating the duration")]
    TimeConvertError,
    #[error("this error is unknown")]
    UnknownError,
}


impl From<sqlx::Error> for LoginError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => LoginError::InvalidCredentials,
            _ => LoginError::UnknownError,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for LoginError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        match e.into_kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => LoginError::InvalidToken,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => LoginError::InvalidSignature,
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => LoginError::ExpiredSignature,
            _ => LoginError::UnknownError,
        }
    }
}
