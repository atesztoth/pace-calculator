mod calculation;
mod config;
mod db;
mod error;
mod middleware;
mod models;
mod response;
mod routes;
mod schema;
mod utils;
mod validation;

use axum::extract::State;
use axum::http::{HeaderValue, Method};
use axum::routing::get;
use axum::Router;
use calculation::calculator_service::CalculatorService;
use config::env_config::EnvConfig;
use db::database_service::{DatabaseService, DatabaseServiceImpl};
use std::sync::{Arc, RwLock};
use tower_http::cors::{Any, CorsLayer};

pub(crate) type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone)]
struct AppState {
    pub env_config: EnvConfig,
    pub calculator: CalculatorService,
}

#[tokio::main]
async fn main() {
    let env_config = config::env_config::load();

    let port = env_config.port.clone();
    let state = create_app_state(&env_config);

    // TODO: it would be enough to protect the services,
    // not the whole state.
    let shared_state: SharedState = Arc::new(RwLock::new(state));

    // initialize tracing
    tracing_subscriber::fmt::init();

    let origins: Vec<HeaderValue> = env_config
        .cors_whitelist
        .iter()
        .map(|s| s.as_str().parse::<HeaderValue>().unwrap())
        .collect();

    let app = Router::new()
        .route("/health", get(health))
        .merge(routes::root::create(Arc::clone(&shared_state)))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::OPTIONS, Method::GET, Method::POST])
                .allow_headers(Any)
                .allow_origin(origins), // .allow_credentials(true),
        )
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

fn create_app_state(env_config: &EnvConfig) -> AppState {
    let db_path = utils::file_utils::resolve_file_path(&env_config.database_name);

    let db = Arc::new(DatabaseServiceImpl::new(&db_path));
    let calculator = CalculatorService::new(Arc::clone(&db));

    AppState {
        calculator,
        env_config: env_config.clone(),
    }
}
