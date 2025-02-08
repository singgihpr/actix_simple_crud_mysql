use actix_web::{error::ResponseError, http::StatusCode, HttpResponse}; // Import digunakan di sini
use sqlx::Error as SqlxError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(SqlxError),
    NotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::NotFound => write!(f, "Resource not found"),
        }
    }
}

impl ResponseError for AppError { // Implementasi ResponseError
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(err) => HttpResponse::InternalServerError().body(err.to_string()),
            AppError::NotFound => HttpResponse::NotFound().body("Resource not found"),
        }
    }

    fn status_code(&self) -> StatusCode { // Implementasi status_code
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}