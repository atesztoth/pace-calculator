use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct EnvConfig {
    pub port: String,
    pub api_key: String,
    pub database_name: String,
    pub cors_whitelist: Vec<String>,
}

pub fn load() -> EnvConfig {
    if let Some(path) = dotenv().ok() {
        println!(
            "Env loaded from .env file {}",
            path.to_str().unwrap_or("Unable to convert path")
        );
    } else {
        println!("Did not load env from .env file, assuming values are already set.");
    }

    let cors_whitelist_str =
        env::var("CORS_WHITELIST").expect("CORS whitelist has not been provided!");

    EnvConfig {
        port: env::var("PORT").unwrap_or_else(|_| "3000".to_string()),
        api_key: env::var("API_KEY").expect("Environment variable 'API_KEY' was not provided!"),
        database_name: env::var("DATABASE_NAME")
            .expect("Environment variable 'DATABASE_NAME' was not provided!"),
        cors_whitelist: cors_whitelist_str
            .split(',')
            .map(|s| s.to_string())
            .collect(),
    }
}
