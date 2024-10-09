// src/lib/errors/error.rs

// dependencies
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("File read error: {0}")]
    FileRead(std::io::Error),
    #[error("Directory read error: {0}")]
    DirectoryRead(std::io::Error),
    #[error("Deserialization error: {0}")]
    DeserializeFrontMatter(serde_json::error::Error),
    #[error("Database error: {0}")]
    Database(sqlx::Error),
    #[error("Lock is poisoned")]
    LockPoisoned,
    #[error("Regex error: {0}")]
    Regex(regex::Error),
}

// implement the From trait for sqlx::Error, for use in the AppError enum
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

// implement the IntoResponse trait for the AppError type, for use in returning an error from a handler
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error");
        (status, msg).into_response()
    }
}
