//! Param extraction — regex scans the raw query for path / limit / id / URI / KV.
//!
//! Ported from LBM pkg/keirouter/extract.go.

use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Default, Clone)]
pub struct Extracted {
    pub path: String,
    pub paths: String,
    pub limit: i64,
    pub depth: i64,
    pub id: i64,
    pub query: String,
    pub text: String,
    pub text_clean: String,
    pub uri: String,
    pub kv: HashMap<String, String>,
}

fn re(pat: &str) -> Regex {
    Regex::new(pat).expect("invalid regex pattern in kei-router")
}

fn re_abs_path() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"(?:^|\s)((?:/[\w.~-]+)+(?:\.\w+)?)"))
}
fn re_rel_path() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"(?:^|\s)((?:[\w.-]+/)+[\w.-]+\.\w+)"))
}
fn re_json_arr() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r#"\[(?:\s*"[^"]*"\s*,?\s*)+\]"#))
}
fn re_number() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"\b(?:limit|max|top)\s*[=:]?\s*(\d+)"))
}
fn re_depth() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"\b(?:depth)\s*[=:]?\s*(\d+)"))
}
fn re_id_num() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"\b(?:id|unit)\s*[=:#]?\s*(\d+)"))
}
fn re_bare_num() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"\b(\d{1,4})\b"))
}
fn re_vault_uri() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"\bnote://vault/[\w/.\-]+"))
}
fn re_domain_uri() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"\b(\w+://[\w/.+\-]+)"))
}
fn re_kv() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| re(r"\b(\w+)=([\w://._+\-]+)"))
}

fn parse_i64(s: &str) -> i64 {
    s.parse::<i64>().unwrap_or(0)
}

fn extract_paths(query: &str, e: &mut Extracted) {
    if let Some(m) = re_json_arr().find(query) {
        e.paths = m.as_str().to_string();
    }
    if let Some(c) = re_abs_path().captures(query) {
        if let Some(m) = c.get(1) {
            e.path = m.as_str().to_string();
        }
    }
    if e.path.is_empty() {
        if let Some(c) = re_rel_path().captures(query) {
            if let Some(m) = c.get(1) {
                e.path = m.as_str().to_string();
            }
        }
    }
    if let Some(m) = re_vault_uri().find(query) {
        if e.path.is_empty() {
            e.path = m.as_str().to_string();
        }
    }
}

fn extract_numbers(text: &str, e: &mut Extracted) {
    if let Some(c) = re_number().captures(text) {
        if let Some(m) = c.get(1) {
            e.limit = parse_i64(m.as_str());
        }
    }
    if let Some(c) = re_depth().captures(text) {
        if let Some(m) = c.get(1) {
            e.depth = parse_i64(m.as_str());
        }
    }
    if let Some(c) = re_id_num().captures(text) {
        if let Some(m) = c.get(1) {
            e.id = parse_i64(m.as_str());
        }
    }
    if e.limit == 0 && e.id == 0 {
        if let Some(c) = re_bare_num().captures(text) {
            if let Some(m) = c.get(1) {
                let n = parse_i64(m.as_str());
                if n > 0 && n <= 500 {
                    e.limit = n;
                }
            }
        }
    }
}

fn extract_uri_kv(query: &str, e: &mut Extracted) {
    if let Some(m) = re_domain_uri().find(query) {
        let s = m.as_str();
        if !s.starts_with("note://") {
            e.uri = s.to_string();
        }
    }
    for c in re_kv().captures_iter(query) {
        if let (Some(k), Some(v)) = (c.get(1), c.get(2)) {
            e.kv.insert(k.as_str().to_string(), v.as_str().to_string());
        }
    }
}

fn build_clean_query(e: &mut Extracted) {
    let mut q = e.text.clone();
    if !e.path.is_empty() {
        q = q.replacen(&e.path.to_lowercase(), "", 1);
    }
    q = re_number().replace_all(&q, "").to_string();
    q = re_depth().replace_all(&q, "").to_string();
    q = re_id_num().replace_all(&q, "").to_string();
    q = q.trim().to_string();
    if !q.is_empty() {
        e.query = q;
    }
    e.text_clean = e.text.clone();
    if !e.path.is_empty() {
        e.text_clean = e.text_clean.replacen(&e.path.to_lowercase(), " ", 1).trim().to_string();
    }
}

/// Parse a raw NL query into structured [`Extracted`] params.
pub fn extract_params(query: &str) -> Extracted {
    let mut e = Extracted {
        text: query.trim().to_lowercase(),
        ..Default::default()
    };
    extract_paths(query, &mut e);
    let text_copy = e.text.clone();
    extract_numbers(&text_copy, &mut e);
    extract_uri_kv(query, &mut e);
    build_clean_query(&mut e);
    e
}
