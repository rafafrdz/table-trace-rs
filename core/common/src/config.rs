use dotenvy::dotenv;
use std::env;
use tracing_subscriber::EnvFilter;

pub static PORT: &str = "PORT";

#[derive(Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_target(true)
            .with_level(true)
            .with_line_number(true)
            .pretty()
            .init();

        let port = env::var(PORT)
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(9876);

        Self { port }
    }
}
