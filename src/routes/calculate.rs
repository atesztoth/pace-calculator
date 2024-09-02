use crate::calculation::route_handlers::calculate_pace;
use crate::calculation_handlers::calculation_api_handlers::store_calculation;
use crate::SharedState;
use axum::routing::post;
use axum::Router;

pub fn create() -> Router<SharedState> {
    Router::new()
        .route("/calculate", post(store_calculation))
        .route("/calculate-test", post(calculate_pace))
}
