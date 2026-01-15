mod handlers;
mod models;
mod routes;
use async_trait::async_trait;
use axum::Router;
use common::error::Result;
use common::{config::Config, modules::Module};

pub struct AnalyticsModule {}

#[async_trait]
impl Module for AnalyticsModule {
    async fn create(_config: &Config) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    fn routes(&self) -> Router {
        routes::create_routes()
    }
}
