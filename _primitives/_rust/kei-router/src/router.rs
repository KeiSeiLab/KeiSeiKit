//! Router — holds keyword rules, dispatches queries to tool calls.

use crate::extract::{extract_params, Extracted};
use crate::keywords::default_rules;
use crate::rules::{always, DynRule, KeywordRule};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    Keyword,
    Fallback,
    Remote,
}

/// Canonical route outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResult {
    pub tool: String,
    pub params: BTreeMap<String, serde_json::Value>,
    pub confidence: f64,
    pub method: Method,
}

/// Router holds the static + dynamic keyword rules.
pub struct Router {
    rules: Vec<KeywordRule>,
    dynamic: Vec<DynRule>,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    pub fn new() -> Self {
        Self {
            rules: default_rules(),
            dynamic: Vec::new(),
        }
    }

    /// Append user-supplied rules at runtime (domain extension).
    pub fn add_dynamic(&mut self, dyn_rules: Vec<DynRule>) {
        self.dynamic.extend(dyn_rules);
    }

    /// Route a natural language query. Always returns a result — falls back to search tools.
    pub fn route(&self, query: &str) -> RouteResult {
        let ext = extract_params(query);
        if let Some(r) = self.keyword_match(&ext) {
            return r;
        }
        if let Some(r) = self.dynamic_match(&ext) {
            return r;
        }
        self.fallback(query, &ext)
    }

    /// Convenience wrapper — useful for remote MCP forwarders that want a hint.
    pub fn route_with_hint(&self, query: &str) -> RouteResult {
        let mut r = self.route(query);
        if r.method == Method::Fallback {
            // Remote-MCP stub: caller may inspect params["_forward"] to decide.
            r.params.insert("_forward".into(), serde_json::Value::Bool(true));
        }
        r
    }

    fn keyword_match(&self, ext: &Extracted) -> Option<RouteResult> {
        for rule in &self.rules {
            if !(rule.require)(ext) {
                continue;
            }
            for kw in rule.keywords {
                if ext.text_clean.contains(kw) || ext.text.contains(kw) {
                    return Some(make_route(rule.tool, ext, Method::Keyword, 0.9));
                }
            }
        }
        None
    }

    fn dynamic_match(&self, ext: &Extracted) -> Option<RouteResult> {
        for rule in &self.dynamic {
            for kw in &rule.keywords {
                if ext.text.contains(kw.as_str()) {
                    return Some(make_route(&rule.tool, ext, Method::Keyword, 0.75));
                }
            }
        }
        None
    }

    fn fallback(&self, query: &str, ext: &Extracted) -> RouteResult {
        if !ext.path.is_empty() {
            make_route("search_code", ext, Method::Fallback, 0.3)
        } else {
            let mut params = BTreeMap::new();
            params.insert(
                "query".into(),
                serde_json::Value::String(query.to_string()),
            );
            RouteResult {
                tool: "search_knowledge".into(),
                params,
                confidence: 0.2,
                method: Method::Fallback,
            }
        }
    }
}

fn make_route(tool: &str, ext: &Extracted, method: Method, confidence: f64) -> RouteResult {
    RouteResult {
        tool: tool.to_string(),
        params: merge_params(ext),
        confidence,
        method,
    }
}

fn merge_params(ext: &Extracted) -> BTreeMap<String, serde_json::Value> {
    let mut m = BTreeMap::new();
    // KV pairs first — typed extraction below takes precedence on collisions
    // (e.g. "id=42" → kv["id"]="42" string, but ext.id=42 wins as i64).
    for (k, v) in &ext.kv {
        m.insert(k.clone(), v.clone().into());
    }
    if !ext.path.is_empty() {
        m.insert("path".into(), ext.path.clone().into());
    }
    if ext.limit > 0 {
        m.insert("limit".into(), ext.limit.into());
    }
    if ext.depth > 0 {
        m.insert("depth".into(), ext.depth.into());
    }
    if ext.id > 0 {
        m.insert("id".into(), ext.id.into());
    }
    if !ext.query.is_empty() {
        m.insert("query".into(), ext.query.clone().into());
    }
    if !ext.uri.is_empty() {
        m.insert("uri".into(), ext.uri.clone().into());
    }
    m
}

// Silence unused import in some build modes.
#[allow(dead_code)]
fn _always_keep(_e: &Extracted) -> bool {
    always(_e)
}
