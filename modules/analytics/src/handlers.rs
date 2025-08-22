use super::models::AnalyzeQueryRequest;
use analytics_core::core::extract_table_names;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use common::error::AppError;

pub async fn analyze_sql(Json(payload): Json<AnalyzeQueryRequest>) -> impl IntoResponse {
    let query = &payload.query;

    let tables = extract_table_names(query.as_str());
    match tables {
        Ok(value) => (StatusCode::OK, Json(value)).into_response(),
        Err(e) => {
            let msg: String = format!("Error processing the query `{}`. {}", query.as_str(), e);
            AppError::Validation(msg).into_response()
        }
    }
}
