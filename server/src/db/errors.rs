use crate::response::api_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("{0}")]
    #[allow(dead_code)]
    SomethingWentWrong(String),
    #[error("Internal error happened, please try again!")]
    CannotGetPooledConnection(String),
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let status_code = match self {
            DbError::SomethingWentWrong(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::CannotGetPooledConnection(_) => StatusCode::CONFLICT,
        };

        ApiErrorResponse::create_response(status_code, Some(self.to_string()))
    }
}
