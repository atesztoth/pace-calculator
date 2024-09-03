use crate::calculation::errors::CalculationError;
use crate::db::errors::DbError;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

// Inspired by: https://github.com/thanipro/Axum-Rust-Rest-Api-Template/blob/f8ebaf6db8c240b307231913542d952c08d0fd33/src/error/api_error.rs
#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    CalculationError(#[from] CalculationError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::DbError(error) => error.into_response(),
            ApiError::CalculationError(error) => error.into_response(),
        }
    }
}
