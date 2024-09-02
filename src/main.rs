mod calculation;
mod calculation_handlers;
mod config;
mod middleware;
mod routes;

use crate::config::env_config::EnvConfig;
use axum::extract::State;
use axum::routing::get;
use axum::Router;

#[derive(Clone)]
struct AppState {
    pub env_config: EnvConfig,
}

#[tokio::main]
async fn main() {
    let env_config = config::env_config::load();

    let port = &env_config.port.clone();
    let state = AppState { env_config };

    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health))
        .merge(routes::root::create())
        .with_state(state.clone());

    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Server is listening on address {}", &address);

    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn health(_: State<AppState>) -> &'static str {
    "OK"
}
