use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};
use dotenvy::dotenv;
use std::env;
use tracing::warn;

const ENV_API_KEY: &str = "API_KEY";
const HEADER_API_KEY_PROPERTY: &str = "x-api-key";

// add ret to instrument to get logs when called
// TODO: check documentation later
#[tracing::instrument()]
pub async fn api_key_middleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let key = env::var(ENV_API_KEY).expect("API KEY is not provided!");

    // Check if the header "x-auth-token" exists
    if let Some(auth_token) = req.headers().get(HEADER_API_KEY_PROPERTY) {
        if let Ok(unwrapped_token) = auth_token.to_str() {
            if unwrapped_token == key {
                return Ok(next.run(req).await);
            }
        } else {
            warn!("Failed to convert auth token to str in a request!");
        }
    }

    // If the token is missing or incorrect, return a 403 Forbidden status
    Err((StatusCode::FORBIDDEN, "Forbidden".to_string()))
}
