use std::fmt;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Internal(String),
    NotFound(String),
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Internal(msg) => write!(f, "internal error: {msg}"),
            AppError::NotFound(msg) => write!(f, "not found: {msg}"),
            AppError::BadRequest(msg) => write!(f, "bad request: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };
        (status, Json(ErrorBody { error: message })).into_response()
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}
