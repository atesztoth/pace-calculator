use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct EnvConfig {
    pub port: String,
    pub api_key: String,
    pub database_url: String,
}

pub fn load() -> EnvConfig {
    if let Some(path) = dotenv().ok() {
        println!(
            "Env loaded from .env file {}",
            path.to_str().unwrap_or("Unable to convert path")
        );
    } else {
        println!("Did not load env from .env file!");
    }

    EnvConfig {
        port: env::var("PORT").unwrap_or_else(|_| "3000".to_string()),
        api_key: env::var("API_KEY").expect("Environment variable 'API_KEY' was not provided!"),
        database_url: env::var("DATABASE_URL")
            .expect("Environment variable 'API_KEY' was not provided!"),
    }
}
