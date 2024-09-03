use crate::validation::server_error::ServerError;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::{async_trait, Json};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidJsonRequest<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidJsonRequest<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidJsonRequest(value))
    }
}
