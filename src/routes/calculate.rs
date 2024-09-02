use crate::calculation::route_handlers::run_calculation;
use crate::SharedState;
use axum::routing::post;
use axum::Router;

pub fn create() -> Router<SharedState> {
    Router::new().route("/calculate", post(run_calculation))
}
