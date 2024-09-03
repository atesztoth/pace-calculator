use crate::response::api_response::ApiErrorResponse;
use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                ApiErrorResponse::new(StatusCode::BAD_REQUEST, Some(message))
            }

            ServerError::JsonRejection(_) => {
                ApiErrorResponse::new(StatusCode::BAD_REQUEST, Some(self.to_string()))
            }

            ServerError::AxumFormRejection(_) => {
                ApiErrorResponse::new(StatusCode::BAD_REQUEST, Some(self.to_string()))
            }
        }
        .into_response()
    }
}
