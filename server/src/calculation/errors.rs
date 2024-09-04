use crate::response::api_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CalculationError {
    #[error("Could not store calculation.")]
    SaveFailed(String),
}

impl IntoResponse for CalculationError {
    fn into_response(self) -> Response {
        let status_code = match self {
            CalculationError::SaveFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ApiErrorResponse::create_response(status_code, Some(self.to_string()))
    }
}
