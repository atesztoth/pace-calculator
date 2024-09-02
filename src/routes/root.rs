use crate::{middleware::api_key_middleware::api_key_middleware, routes::calculate, AppState};
use axum::{middleware as axum_middleware, Router};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

pub fn create() -> Router<AppState> {
    let app_router = Router::<AppState>::new()
        .merge(calculate::create())
        // TODO: This crashes !! Needs an Extension layer probably.
        .layer(axum_middleware::from_fn(api_key_middleware));

    let main_router = Router::<AppState>::new().nest("/", app_router).layer(
        ServiceBuilder::new().layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::DEBUG)
                        .latency_unit(LatencyUnit::Micros),
                ),
        ),
    );

    main_router
}
