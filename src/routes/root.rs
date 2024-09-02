use crate::{
    middleware::api_key_middleware::api_key_middleware, routes::calculate, AppState, SharedState,
};
use axum::{middleware as axum_middleware, Router};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

// Had to pass the state here, I cannot yet get the state in a middleware.
// I wanted to ge the state pass by the with_state, but I think there is literally
// no way to do that, even creating a custom Layer would require me to pass in the state by hand.
// This was getting way too far. See this and the links arising from here:
// https://github.com/tokio-rs/axum/discussions/1912
// Probably I should use the ServiceBuilder from Tower maybe with the filter. Will look at it later.
// (Still, that would not exactly be what I wanted to achieve. I explicitly wanted to access the state
// in a middleware created from a function.)
pub fn create(s: SharedState) -> Router<SharedState> {
    let app_router = Router::<SharedState>::new()
        .merge(calculate::create())
        .layer(axum_middleware::from_fn_with_state(
            s.clone(),
            api_key_middleware,
        ));

    let main_router = Router::<SharedState>::new().nest("/", app_router).layer(
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
