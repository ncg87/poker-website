use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

/// Error types for the server
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Database error: {0}")]
    Database(#[from] tokio_postgres::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error")]
    Internal(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Auth(_) => HttpResponse::Unauthorized().json(self.to_string()),
            AppError::Database(_) => HttpResponse::InternalServerError().json("Database error occurred"),
            AppError::Validation(_) => HttpResponse::BadRequest().json(self.to_string()),
            AppError::NotFound(_) => HttpResponse::NotFound().json(self.to_string()),
            AppError::Internal(_) => HttpResponse::InternalServerError().json("Internal server error"),
        }
    }
}