use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiErrorResponse {
    message: Option<String>,
    status: StatusCode,
}

impl ApiErrorResponse {
    pub fn create_response(status: StatusCode, message: Option<String>) -> Response {
        ApiErrorResponse { message, status }.into_response()
    }
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl Serialize for ApiErrorResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ApiErrorResponse", 2)?;

        state.serialize_field("message", &self.message)?;
        state.serialize_field("code", &self.status.as_u16())?;

        state.end()
    }
}
