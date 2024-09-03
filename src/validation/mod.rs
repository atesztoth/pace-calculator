use axum::extract::FromRequest;
use axum::response::IntoResponse;
use serde::de::DeserializeOwned;
use validator::Validate;

pub(crate) mod server_error;
pub(crate) mod valid_json_request;
