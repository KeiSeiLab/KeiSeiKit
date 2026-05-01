//! Shared value types for knowledge units + edges + BFS results.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Unit {
    pub id: i64,
    pub unit_type: String,
    pub title: String,
    pub content: String,
    pub evidence_grade: String,
    pub source_path: String,
    pub vault_path: String,
    pub category: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: i64,
    pub src_path: String,
    pub dst_path: String,
    pub edge_type: String,
    pub weight: f64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Related {
    pub path: String,
    pub edge_type: String,
    pub depth: i64,
}
