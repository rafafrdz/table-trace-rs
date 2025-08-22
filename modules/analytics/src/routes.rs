use crate::handlers::analyze_sql;
use axum::{routing::post, Router};

pub fn create_routes() -> Router {
    Router::new().route("/analyze", post(analyze_sql))
}
