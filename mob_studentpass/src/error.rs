#![allow(unused)]

/*use std::convert::Infallible;
use log::info;

use warp::reply::WithStatus;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Debug)]
pub enum ApiError {
    CredentialsError,
    DBError,
    DateParseError,
    JsonParseError,
    SessionExpired,
    TokenError,
    TokenNotFoundError,
}

impl warp::reject::Reject for ApiError {}

pub async fn handle_rejection(e: Rejection) -> Result<impl Reply, Infallible> {
    info!("{e:?}");
    match e.find() {
        Some(ApiError::DBError) => Ok(error_response(
            "Internal Error",
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
        Some(ApiError::DateParseError) => Ok(error_response(
            "Wrong date format: expected yyyy-mm-dd",
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
        Some(ApiError::JsonParseError) => Ok(error_response(
            "Json Parse Error",
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
        Some(ApiError::SessionExpired) => {
            Ok(error_response("Session expired", StatusCode::UNAUTHORIZED))
        }
        Some(ApiError::CredentialsError) => Ok(error_response(
            "Invalid Username or Password",
            StatusCode::FORBIDDEN,
        )),
        Some(ApiError::TokenError) => Ok(error_response("Invalid Token", StatusCode::FORBIDDEN)),
        Some(ApiError::TokenNotFoundError) => {
            Ok(error_response("Token not present", StatusCode::FORBIDDEN))
        }
        _ => Ok(error_response(
            "Unknown Error",
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

fn error_response(msg: &str, status_code: StatusCode) -> WithStatus<String> {
    warp::reply::with_status(format!("{}", msg), status_code)
}

impl From<chrono::ParseError> for ApiError {
    fn from(_: chrono::ParseError) -> ApiError {
        ApiError::DateParseError
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(_: sqlx::Error) -> ApiError {
        ApiError::DBError
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(_: serde_json::Error) -> ApiError {
        ApiError::JsonParseError
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(_: jsonwebtoken::errors::Error) -> ApiError {
        ApiError::TokenError
    }
}*/

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
