use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub category: String,
    pub tags: Vec<String>,
    pub connections: usize,
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Edge {
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub weight: f32,
}

#[derive(Debug, Serialize)]
pub struct SpaceData {
    pub nodes: Vec<Node>,
    pub links: Vec<Edge>,
}

#[derive(Debug, Serialize)]
pub struct Space {
    pub name: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
    pub colors: HashMap<String, String>,
    pub data: SpaceData,
}

pub fn sanitize_id(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() || "-_:/.".contains(c) { c } else { '_' })
        .collect()
}

pub fn dna_prefix(dna: &str) -> String {
    sanitize_id(&dna.chars().take(30).collect::<String>())
}

pub fn truncate_chars(s: &str, max: usize) -> &str {
    if s.chars().count() <= max { return s; }
    let end = s.char_indices().nth(max).map(|(i, _)| i).unwrap_or(s.len());
    &s[..end]
}
