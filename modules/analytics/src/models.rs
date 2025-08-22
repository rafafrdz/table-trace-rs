use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeQueryRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeQueryExtractTableNamesResponse {
    pub tables: Vec<String>,
}
