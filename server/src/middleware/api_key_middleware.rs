use crate::SharedState;
use axum::extract::State;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};
use tracing::warn;

const HEADER_API_KEY_PROPERTY: &str = "x-api-key";

// add ret to instrument to get logs when called
// TODO: check documentation later
// #[tracing::instrument()] // no need here, but must stay because of my comments above
pub async fn api_key_middleware(
    State(state): State<SharedState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let api_key = &state.env_config.api_key.clone();

    // Check if the header "x-auth-token" exists
    if let Some(auth_token) = req.headers().get(HEADER_API_KEY_PROPERTY) {
        if let Ok(unwrapped_token) = auth_token.to_str() {
            if unwrapped_token == api_key {
                return Ok(next.run(req).await);
            }
        } else {
            warn!("Failed to convert auth token to str in a request!");
        }
    }

    // If the token is missing or incorrect, return a 403 Forbidden status
    Err((StatusCode::FORBIDDEN, "Forbidden".to_string()))
}
