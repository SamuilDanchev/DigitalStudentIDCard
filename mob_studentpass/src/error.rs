#![allow(unused)]
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("wrong credentials")]
    CredentialsError,
    #[error("Internal Error")]
    DBError,
    #[error("invalid token")]
    TokenError,
    #[error("no token found")]
    TokenNotFoundError,
    #[error("unknown user")]
    UnknownUserError,
    #[error("wrong query parameter")]
    QueryParameterError,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl warp::reject::Reject for ApiError {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    println!("{err:?}");
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<ApiError>() {
        match e {
            ApiError::CredentialsError => {
                code = StatusCode::UNAUTHORIZED;
                message = "wrong credentials"
            }
            ApiError::DBError => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "DBError"
            }
            ApiError::TokenError => {
                code = StatusCode::UNAUTHORIZED;
                message = "token error"
            }
            ApiError::TokenNotFoundError => {
                code = StatusCode::UNAUTHORIZED;
                message = "invalid token"
            }
            ApiError::UnknownUserError => {
                code = StatusCode::FORBIDDEN;
                message = "user not found"
            }
            ApiError::QueryParameterError => {
                code = StatusCode::NOT_FOUND;
                message = "wrong query parameter used"
            }
            _ => {
            eprintln!("unhandled application error: {:?}", err);
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "Internal Server Error";
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
