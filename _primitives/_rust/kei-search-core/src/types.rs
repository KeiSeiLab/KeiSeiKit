use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Research {
    pub id: i64,
    pub query_original: String,
    pub status: String,
    pub result_markdown: String,
    pub total_cost_mc: i64,
    pub created_at: i64,
    pub completed_at: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Source {
    pub id: i64,
    pub research_id: i64,
    pub url: String,
    pub title: String,
    pub content: String,
    pub provider: String,
    pub domain: String,
    pub relevance_score: f64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Claim {
    pub id: i64,
    pub research_id: i64,
    pub claim_text: String,
    pub support: f64,
    pub contradict: f64,
    pub consensus: f64,
    pub grade: String,
    pub created_at: i64,
}
