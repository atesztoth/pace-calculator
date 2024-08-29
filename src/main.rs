mod calculation_handlers;

use crate::calculation_handlers::calculation_api_handlers::store_calculation;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/calculate", post(store_calculation));

    // Retrieve the port number from the environment, defaulting to 3000 if not set
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // Construct the address string "0.0.0.0:<port>"
    let address = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Server is listening on address {}", &address);

    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
