mod calculation;
mod config;
mod middleware;
mod response;
mod routes;
mod validation;

use crate::calculation::service::CalculatorService;
use crate::config::env_config::EnvConfig;
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use std::sync::{Arc, RwLock};

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone)]
struct AppState {
    pub env_config: EnvConfig,
    pub calculator: CalculatorService,
}

#[tokio::main]
async fn main() {
    let env_config = config::env_config::load();

    let port = env_config.port.clone();
    let state = AppState {
        env_config,
        calculator: CalculatorService::new(),
    };
    let shared_state: SharedState = Arc::new(RwLock::new(state));

    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health))
        .merge(routes::root::create(Arc::clone(&shared_state)))
        .with_state(Arc::clone(&shared_state));

    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Server is listening on address {}", &address);

    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn health(_: State<SharedState>) -> &'static str {
    "OK"
}
