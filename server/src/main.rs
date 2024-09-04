mod calculation;
mod config;
mod db;
mod error;
mod middleware;
mod models;
mod response;
mod routes;
mod schema;
mod validation;

use crate::calculation::calculation_service::CalculatorService;
use crate::config::env_config::EnvConfig;
use crate::db::database_service::{DatabaseService, DatabaseServiceImpl};
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use std::sync::{Arc, RwLock};

pub(crate) type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone)]
struct AppState {
    pub env_config: EnvConfig,
    // Not using dynamic dispatch.
    pub calculator: CalculatorService,
    pub db: Arc<DatabaseServiceImpl>,
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

fn create_app_state(env_config: &EnvConfig) -> AppState {
    let db = Arc::new(DatabaseServiceImpl::new(&env_config.database_url));
    let calculator = CalculatorService::new(Arc::clone(&db));

    AppState {
        db,
        calculator,
        env_config: env_config.clone(),
    }
}
