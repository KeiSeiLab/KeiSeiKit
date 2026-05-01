use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossEdge {
    pub id: i64,
    pub from_uri: String,
    pub to_uri: String,
    pub edge_type: String,
    pub weight: f64,
    pub evidence: String,
    pub metadata: String,
    pub created_at: i64,
}

/// Extract "domain" from a "domain://…" URI. Empty string if malformed.
pub fn extract_domain(uri: &str) -> &str {
    match uri.find("://") {
        Some(0) => "",
        Some(i) => &uri[..i],
        None => "",
    }
}
